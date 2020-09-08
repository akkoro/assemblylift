use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::sync::{mpsc, Arc};

use capnp::capability::Promise;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, FutureExt, TryFutureExt};
use once_cell::sync::Lazy;
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio_util::compat::Tokio02AsyncReadCompatExt;

use crate::iomod_capnp::{agent, iomod, registry};
use crate::Agent;
use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;

pub type RegistryTx = mpsc::Sender<RegistryChannelMessage>;
pub type RegistryRx = mpsc::Receiver<RegistryChannelMessage>;
pub type RegistryChannel = (RegistryTx, RegistryRx);

pub type ClientPair = (iomod::Client, agent::Client);

pub struct Registry {
    modules: Rc<RefCell<HashMap<String, agent::Client>>>,
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

impl Registry {
    pub fn new() -> Self {
        Self {
            modules: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn start_service(&mut self, rx: RegistryRx) -> Result<(), RegistryError> {
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
                let input = msg.payload.as_slice();

                let modules = RefCell::borrow(&modules);
                let agent = modules.get(&coords).unwrap();
                let mut invoke = agent.invoke_request();

                invoke.get().set_coordinates(&method);
                invoke.get().set_input(input);
                let results = invoke.send().promise.await.unwrap();
                let response_payload = Vec::from(results.get().unwrap().get_result().unwrap());

                responder
                    .send(RegistryChannelMessage {
                        iomod_coords: coords,
                        method_name: method,
                        payload_type: "IOMOD_RESPONSE",
                        payload: response_payload,
                        responder: None,
                    })
                    .unwrap();
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

        let module: Rc<RefCell<iomod::Client>> =
            Rc::new(RefCell::new(params.get().unwrap().get_iomod().unwrap()));

        let agent: agent::Client = capnp_rpc::new_client(Agent::new(module));

        let modules = self.modules.clone();
        let mut modules_ref = RefCell::borrow_mut(&modules);
        modules_ref.entry(coordinates).or_insert(agent);

        Promise::ok(())
    }
}
