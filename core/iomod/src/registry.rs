use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::sync::{Arc, mpsc};

use capnp::capability::Promise;
use capnp_rpc::{rpc_twoparty_capnp, RpcSystem, twoparty};
use futures::{AsyncReadExt, FutureExt, TryFutureExt};
use once_cell::sync::Lazy;
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio_util::compat::Tokio02AsyncReadCompatExt;

use crate::Agent;
use crate::iomod_capnp::{agent, iomod, registry};
use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;

// pub type RegistryChannelMessage = &'static str;
pub type RegistryTx = mpsc::Sender<RegistryChannelMessage>;
pub type RegistryRx = mpsc::Receiver<RegistryChannelMessage>;
pub type RegistryChannel = (RegistryTx, RegistryRx);

pub type ClientPair = (iomod::Client, agent::Client);

pub struct Registry {
    modules: Arc<RefCell<HashMap<String, agent::Client>>>,
}

#[derive(Debug)]
pub struct RegistryError;

impl RegistryError {
    pub fn new() -> Self {
        Self {}
    }
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RegistryError")
    }
}

impl std::error::Error for RegistryError {}

#[derive(Debug)]
pub struct RegistryChannelMessage {
    pub iomod_coords: &'static str,
    pub method_name: &'static str,
    pub payload_type: &'static str,
    pub payload: Vec<u8>,
    pub responder: Option<RegistryTx>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            modules: Arc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn spawn_service(&mut self, rx: RegistryRx) -> Result<(), RegistryError> {
        tokio::task::spawn_local(async move {
            let mut listener = TcpListener::bind("127.0.0.1:13555").await.unwrap();
            let registry_client: registry::Client = capnp_rpc::new_client(Registry::new());

            loop {
                let (stream, _) = listener.accept().await.unwrap();
                stream.set_nodelay(true).unwrap();

                let (reader, writer) =
                    tokio_util::compat::Tokio02AsyncReadCompatExt::compat(stream).split();

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

        let modules = self.modules.clone();
        tokio::task::spawn_local(async move {
            while let Ok(msg) = rx.recv() {
                let responder = msg.responder.unwrap();
                let coords = msg.iomod_coords;
                let method = msg.method_name;

                let modules = RefCell::borrow(&modules);
                let agent = modules.get(coords).unwrap();
                let mut invoke = agent.invoke_request();

                invoke.get().set_coordinates(method);
                let results = invoke.send().promise.await.unwrap();
                let payload = Vec::from(results.get().unwrap().get_result().unwrap());

                responder.send(RegistryChannelMessage {
                    iomod_coords: "",
                    method_name: "",
                    payload_type: "",
                    payload,
                    responder: None
                }).unwrap();
            }
        });

        Ok(())
    }
}

impl registry::Server for Registry {
    fn register(
        &mut self,
        params: registry::RegisterParams,
        mut _results: registry::RegisterResults,
    ) -> Promise<(), capnp::Error> {
        let coordinates: String = String::from(params.get().unwrap().get_coordinates().unwrap());

        let module: Rc<RefCell<iomod::Client>> = Rc::new(RefCell::new(
            params.get().unwrap().get_iomod().unwrap()));

        let agent: agent::Client = capnp_rpc::new_client(Agent::new(module));

        let modules = self.modules.clone();
        let mut modules_ref = RefCell::borrow_mut(&modules);
        modules_ref.entry(coordinates).or_insert(agent);

        Promise::ok(())
    }
}
