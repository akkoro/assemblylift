use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};

use capnp::capability::Promise;
use capnp_rpc::{rpc_twoparty_capnp, RpcSystem, twoparty};
use futures::{AsyncReadExt, FutureExt, TryFutureExt};
use once_cell::sync::Lazy;
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio_util::compat::Tokio02AsyncReadCompatExt;

use crate::iomod_capnp::{iomod, registry};
use crate::Agent;
use std::rc::Rc;

pub trait RegistryService {
    fn spawn_service(self) -> Result<(), RegistryError>;
}

pub struct Registry {
    modules: Arc<RefCell<HashMap<String, iomod::Client>>>,
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

impl Registry {
    pub fn new() -> Self {
        Self {
            modules: Arc::new(RefCell::new(HashMap::new())),
        }
    }
}

impl RegistryService for Registry {
    fn spawn_service(self) -> Result<(), RegistryError> {
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

        // TODO capnproto TCP thread
        Ok(())
    }
}

impl registry::Server for Registry {
    fn register(
        &mut self,
        params: registry::RegisterParams,
        mut results: registry::RegisterResults,
    ) -> Promise<(), capnp::Error> {
        let coordinates: String = String::from(params.get().unwrap().get_coordinates().unwrap());
        let module: iomod::Client = params.get().unwrap().get_iomod().unwrap();
        let rc_module: Rc<RefCell<iomod::Client>> = Rc::new(RefCell::new(
            params.get().unwrap().get_iomod().unwrap()));

        self.modules.borrow_mut().entry(coordinates).or_insert(module);

        results.get().set_agent(capnp_rpc::new_client(Agent::new(rc_module)));

        Promise::ok(())
    }
}
