extern crate assemblylift_core_iomod;

use std::collections::HashMap;

use capnp::capability::Promise;
use capnp::{Error, ErrorKind};
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::future::BoxFuture;
use futures::{AsyncReadExt, FutureExt};
use futures_util::TryFutureExt;
use once_cell::sync::Lazy;
use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;
use tokio::net::TcpStream;
use tokio::sync::mpsc;

use assemblylift_core_iomod::iomod_capnp::*;

static IOMOD_COORDS: &str = "akkoro.aws.dynamodb";
static DYNAMODB: Lazy<DynamoDbClient> = Lazy::new(|| DynamoDbClient::new(Region::UsEast1));

// TODO look at moving a lot of this out or to new macros

struct CallRequest {
    pub coords: String,
    pub input: Vec<u8>,
    pub responder: mpsc::Sender<CallResponse>,
}

struct CallResponse {
    pub coords: String,
    pub payload: Vec<u8>,
}

type CallChannel = (mpsc::Sender<CallRequest>, mpsc::Receiver<CallRequest>);

type Call<F> = fn(Vec<u8>) -> F;

struct CallPtr<F>
where
    F: std::future::Future<Output = Vec<u8>> + Send,
{
    call: Call<F>,
}

impl<F> CallPtr<F>
where
    F: std::future::Future<Output = Vec<u8>> + Send,
{
    fn new(call: Call<F>) -> Self {
        Self { call }
    }
}

struct CallMap<'a> {
    map: HashMap<&'a str, CallPtr<BoxFuture<'a, Vec<u8>>>>,
}

impl<'a> CallMap<'a> {
    fn new() -> Self {
        Self {
            map: HashMap::default(),
        }
    }

    fn get(&self, coords: String, with_input: Vec<u8>) -> BoxFuture<'a, Vec<u8>> {
        let call = self.map[coords.as_str()].call;
        call(with_input)
    }
}

struct Iomod {
    tx: mpsc::Sender<CallRequest>,
}

impl Iomod {
    fn new(tx: mpsc::Sender<CallRequest>) -> Self {
        Self { tx }
    }
}

impl iomod::Server for Iomod {
    fn invoke(
        &mut self,
        params: iomod::InvokeParams,
        mut results: iomod::InvokeResults,
    ) -> Promise<(), Error> {
        let mut tx = self.tx.clone();

        Promise::from_future(async move {
            let coords = params.get().unwrap().get_coordinates().unwrap().to_owned();
            let input = params.get().unwrap().get_input().unwrap();

            let mut channel: (mpsc::Sender<CallResponse>, mpsc::Receiver<CallResponse>) =
                mpsc::channel(100);

            tx.send(CallRequest {
                coords,
                input: Vec::from(input),
                responder: channel.0.clone(),
            })
            .and_then(|_| async move {
                // wait for response from executor thread
                if let Some(response) = channel.1.recv().await {
                    results.get().set_result(response.payload.as_slice());
                }

                Ok(())
            })
            .or_else(|why| async move {
                Err(capnp::Error {
                    kind: ErrorKind::Failed,
                    description: why.to_string(),
                })
            })
            .await
        })
    }
}

#[tokio::main]
async fn main() {
    println!("Starting AssemblyLift IO module {}", IOMOD_COORDS);

    // TODO macro
    let mut call_map: CallMap = CallMap::new();
    call_map
        .map
        .insert("list_tables", CallPtr::new(aws_dynamodb_list_tables));

    let mut call_channel: CallChannel = mpsc::channel(100);

    let stream = TcpStream::connect("127.0.0.1:13555").await.unwrap();
    stream.set_nodelay(true).unwrap();

    let (reader, writer) = tokio_util::compat::Tokio02AsyncReadCompatExt::compat(stream).split();

    let rpc_network = Box::new(twoparty::VatNetwork::new(
        reader,
        writer,
        rpc_twoparty_capnp::Side::Client,
        Default::default(),
    ));

    let mut rpc_system = RpcSystem::new(rpc_network, None);
    let registry: registry::Client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

    let local = tokio::task::LocalSet::new();
    local
        .run_until(async move {
            let rpc_task = tokio::task::spawn_local(Box::pin(rpc_system.map(|_| ())));

            println!("IOMOD: registering dynamodb IOmod");
            let mut register = registry.register_request();
            register
                .get()
                .set_iomod(capnp_rpc::new_client(Iomod::new(call_channel.0.clone())));
            register.get().set_coordinates(IOMOD_COORDS);
            register.send().promise.await.unwrap();

            println!("IOMOD: spawning call receiver");
            let call_task = tokio::task::spawn_local(async move {
                while let Some(mut call) = call_channel.1.recv().await {
                    let coords = call.coords.as_str();
                    let call_ptr = call_map.get(String::from(coords), call.input);

                    let response = call_ptr.await;

                    if let Err(why) = call
                        .responder
                        .send(CallResponse {
                            coords: String::from(coords),
                            payload: response,
                        })
                        .await
                    {
                        println!("ERROR {}", why)
                    }
                }
            });

            let (_, _) = tokio::join!(rpc_task, call_task);
        })
        .await;
}

fn aws_dynamodb_list_tables(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    Box::pin(async move {
        use rusoto_dynamodb::*;

        let deserialized = serde_json::from_slice(input.as_slice()).unwrap();
        let result = DYNAMODB.list_tables(deserialized).await.unwrap();
        serde_json::to_vec(&result).unwrap()
    })
}
