use std::sync::Arc;

use tokio::sync::mpsc;
use tracing::{debug, info};
use wasmer::{Module, Store};

use assemblylift_core::buffers::LinearBuffer;
use assemblylift_core::wasm;
use assemblylift_core_iomod::registry::RegistryTx;

use crate::{GenericDockerAbi, Status, StatusTx};

pub type RunnerTx = mpsc::Sender<RunnerMessage>;
pub type RunnerRx = mpsc::Receiver<RunnerMessage>;
pub type RunnerChannel = (RunnerTx, RunnerRx);

#[derive(Clone)]
pub struct RunnerMessage {
    pub input: Vec<u8>,
    pub status_sender: StatusTx,
}

pub struct Runner {
    channel: RunnerChannel,
    registry_tx: RegistryTx,
    runtime: tokio::runtime::Runtime,
}

impl Runner {
    pub fn new(registry_tx: RegistryTx) -> Self {
        Runner {
            channel: mpsc::channel(32),
            registry_tx,
            runtime: tokio::runtime::Runtime::new().unwrap(),
        }
    }

    pub fn spawn<'a>(&mut self, module: Arc<Module>, store: Arc<Store>) {
        info!("Spawning runner");
        tokio::task::LocalSet::new().block_on(&self.runtime, async {
            while let Some(msg) = self.channel.1.recv().await {
                debug!("received runner message");
                let mt = wasm::build_module::<GenericDockerAbi, Status>(
                    self.registry_tx.clone(),
                    msg.status_sender.clone(),
                    module.clone(),
                    module.name().unwrap_or("handler"),
                    store.clone(),
                )
                .expect("could not assemble module environment");

                mt.1.host_input_buffer
                    .clone()
                    .lock()
                    .unwrap()
                    .initialize(msg.input);

                let instance = match wasm::new_instance(module.clone(), mt.0.clone()) {
                    Ok(instance) => Arc::new(instance),
                    Err(why) => {
                        panic!("Unable to spin new WASM instance {}", why.to_string())
                    }
                };
                tokio::task::spawn_local(async move {
                    let start = instance.exports.get_function("_start").unwrap();
                    match start.call(&[]) {
                        Ok(_) => msg.status_sender.send(Status::Exited(0)),
                        Err(_) => msg
                            .status_sender
                            .send(Status::Failure("WASM module exited in error".to_string())),
                    }
                });
            }
        });
    }

    pub fn sender(&self) -> RunnerTx {
        self.channel.0.clone()
    }
}
