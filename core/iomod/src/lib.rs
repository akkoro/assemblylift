//! Host-side IOmod RPC protocol implementation

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use capnp::capability::Promise;
use capnp::{Error, ErrorKind};
use futures::future::BoxFuture;
use futures_util::TryFutureExt;
use tokio::sync::mpsc;

use crate::iomod_capnp::{agent, iomod};

pub mod iomod_capnp;
pub mod macros;
pub mod package;
pub mod registry;

pub struct CallRequest {
    pub coords: String,
    pub input: Vec<u8>,
    pub responder: mpsc::Sender<CallResponse>,
}

pub struct CallResponse {
    pub coords: String,
    pub payload: Vec<u8>,
}

pub type CallChannel = (mpsc::Sender<CallRequest>, mpsc::Receiver<CallRequest>);

pub type Call<F> = fn(Vec<u8>) -> F;

pub struct CallPtr<F>
where
    F: std::future::Future<Output = Vec<u8>> + Send,
{
    call: Call<F>,
}

impl<F> CallPtr<F>
where
    F: std::future::Future<Output = Vec<u8>> + Send,
{
    pub fn new(call: Call<F>) -> Self {
        Self { call }
    }
}

pub struct CallMap<'a> {
    pub map: HashMap<&'a str, CallPtr<BoxFuture<'a, Vec<u8>>>>,
}

impl<'a> CallMap<'a> {
    pub fn new() -> Self {
        Self {
            map: HashMap::default(),
        }
    }

    pub fn get(&self, coords: String, with_input: Vec<u8>) -> BoxFuture<'a, Vec<u8>> {
        let call = self.map[coords.as_str()].call;
        call(with_input)
    }
}

pub struct Iomod {
    tx: mpsc::Sender<CallRequest>,
}

impl Iomod {
    pub fn new(tx: mpsc::Sender<CallRequest>) -> Self {
        Self { tx }
    }
}

impl iomod::Server for Iomod {
    fn invoke(
        &mut self,
        params: iomod::InvokeParams,
        mut results: iomod::InvokeResults,
    ) -> Promise<(), Error> {
        let tx = self.tx.clone();

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

pub struct Agent {
    iomod_client: Rc<RefCell<iomod::Client>>,
}

impl Agent {
    pub fn new(iomod_client: Rc<RefCell<iomod::Client>>) -> Self {
        Self { iomod_client }
    }
}

impl agent::Server for Agent {
    fn invoke(
        &mut self,
        params: agent::InvokeParams,
        mut results: agent::InvokeResults,
    ) -> Promise<(), capnp::Error> {
        let client = self.iomod_client.clone();

        Promise::from_future(async move {
            let mut invoke = client.borrow_mut().invoke_request();
            invoke
                .get()
                .set_coordinates(params.get().unwrap().get_coordinates().unwrap());
            invoke
                .get()
                .set_input(params.get().unwrap().get_input().unwrap());

            let invoke_response = invoke.send().promise.await.unwrap();
            results
                .get()
                .set_result(invoke_response.get().unwrap().get_result().unwrap());

            Ok(())
        })
    }
}
