//! Not to be confused with the IO Module Public Registry; IOmods register themselves
//! on the start of their execution with `Registry`.
//!
//! The call registry is maintained in-memory. A thread is spawned which handles
//! RPC connections from IOmods and handles IOmod registration. This thread also
//! services call invocations to registered IOmods via MPSC receiver (sent from `Threader`).

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::sync::Arc;

use capnp::capability::Promise;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, FutureExt, TryFutureExt};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tracing::{error, info};

use crate::iomod_capnp::{agent, iomod, registry};
use crate::Agent;

pub type RegistryTx = mpsc::Sender<RegistryChannelMessage>;
pub type RegistryRx = mpsc::Receiver<RegistryChannelMessage>;
pub type RegistryChannel = (RegistryTx, RegistryRx);

pub type ClientPair = (iomod::Client, agent::Client);

pub struct Registry {
    modules: ModuleMap,
}

#[derive(Debug)]
pub struct RegistryError {
    why: String,
}

impl RegistryError {
    pub fn new(why: String) -> Self {
        Self { why }
    }
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RegistryError: {}", self.why)
    }
}

impl std::error::Error for RegistryError {}

#[derive(Debug)]
pub struct RegistryChannelMessage {
    pub iomod_coords: String,
    pub method_name: String,
    pub payload_type: &'static str,
    pub payload: Vec<u8>,
    pub responder: Option<RegistryTx>,
}

pub type ModuleMap = Arc<Box<RefCell<HashMap<String, agent::Client>>>>;

pub fn spawn_registry(mut rx: RegistryRx) -> Result<(), RegistryError> {
    std::thread::spawn(|| {
        let mut rt = tokio::runtime::Runtime::new().unwrap();

        tokio::task::LocalSet::new().block_on(&mut rt, async {
            let modules: ModuleMap = Arc::new(Box::new(RefCell::new(HashMap::new())));

            let rpc_modules = modules.clone();
            let rpc_task = tokio::task::spawn_local(async move {
                let listener = TcpListener::bind("0.0.0.0:13555").await.unwrap();
                let registry_client: registry::Client =
                    capnp_rpc::new_client(Registry::new(rpc_modules));

                while let Ok((stream, _)) = listener.accept().await {
                    stream.set_nodelay(true).unwrap();

                    let (reader, writer) =
                        tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();

                    let rpc_network = twoparty::VatNetwork::new(
                        reader,
                        writer,
                        rpc_twoparty_capnp::Side::Server,
                        Default::default(),
                    );

                    let rpc_system =
                        RpcSystem::new(Box::new(rpc_network), Some(registry_client.clone().client));

                    tokio::task::spawn_local(Box::pin(
                        rpc_system
                            .map_err(|e| println!("error: {:?}", e))
                            .map(|_| ()),
                    ));
                }
            });

            let rx_modules = modules.clone();
            let rx_task = tokio::task::spawn_local(async move {
                while let Some(msg) = rx.recv().await {
                    let responder = msg.responder.unwrap();
                    let coords = msg.iomod_coords;
                    let method = msg.method_name;
                    let input = msg.payload.as_slice();

                    let modules = RefCell::borrow(&rx_modules);
                    match modules.get(&coords) {
                        Some(agent) => {
                            info!("invoking call @ {}.{}", coords.clone(), method.clone());
                            let mut invoke = agent.invoke_request();
                            invoke.get().set_coordinates(&method);
                            invoke.get().set_input(input);
                            let results = invoke.send().promise.await.unwrap();
                            let response_payload =
                                Vec::from(results.get().unwrap().get_result().unwrap());

                            responder
                                .send(RegistryChannelMessage {
                                    iomod_coords: coords,
                                    method_name: method,
                                    payload_type: "IOMOD_RESPONSE",
                                    payload: response_payload,
                                    responder: None,
                                })
                                .await
                                .unwrap();
                        }
                        None => panic!("no IOmod registered at {}", coords),
                    }
                }
            });

            let (rpc_result, rx_result) = tokio::join!(rpc_task, rx_task);

            if rpc_result.is_err() {
                error!(
                    "registry RPC task exited with error {:?}",
                    Some(rpc_result.err())
                );
            }

            if rx_result.is_err() {
                error!(
                    "registry rx task exited with error {:?}",
                    Some(rx_result.err())
                );
            }
        })
    });

    Ok(())
}

impl Registry {
    pub fn new(modules: ModuleMap) -> Self {
        Self { modules }
    }
}

impl registry::Server for Registry {
    fn register(
        &mut self,
        params: registry::RegisterParams,
        mut _results: registry::RegisterResults,
    ) -> Promise<(), capnp::Error> {
        let coordinates: String = String::from(params.get().unwrap().get_coordinates().unwrap());
        let module: Rc<RefCell<iomod::Client>> =
            Rc::new(RefCell::new(params.get().unwrap().get_iomod().unwrap()));

        let agent: agent::Client = capnp_rpc::new_client(Agent::new(module));

        let modules = self.modules.clone();
        let mut modules_ref = RefCell::borrow_mut(&modules);
        modules_ref.entry(coordinates.clone()).or_insert(agent);
        info!("registered IOmod at coordinates {}", coordinates.clone());

        Promise::ok(())
    }
}
