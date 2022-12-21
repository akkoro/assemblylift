use std::sync::{Arc, Mutex};

use tokio::sync::mpsc;
use tracing::{debug, info};

use assemblylift_core::wasm::Wasmtime;
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
    wasmtime: Arc<Mutex<Wasmtime<GenericDockerAbi, Status>>>,
}

impl Runner {
    pub fn new(
        registry_tx: RegistryTx,
        wasmtime: Arc<Mutex<Wasmtime<GenericDockerAbi, Status>>>,
    ) -> Self {
        Runner {
            channel: mpsc::channel(32),
            registry_tx,
            runtime: tokio::runtime::Runtime::new().unwrap(),
            wasmtime,
        }
    }

    pub fn spawn<'a>(&mut self) {
        info!("Spawning runner");
        tokio::task::LocalSet::new().block_on(&self.runtime, async {
            while let Some(msg) = self.channel.1.recv().await {
                debug!("received runner message");

                let (instance, mut store) = self
                    .wasmtime
                    .lock()
                    .unwrap()
                    .link_module(self.registry_tx.clone(), msg.status_sender.clone())
                    .expect("could not link wasm module");

                self.wasmtime
                    .lock()
                    .unwrap()
                    .initialize_function_input_buffer(&mut store, &msg.input)
                    .expect("could not initialize input buffer");

                let wasmtime = self.wasmtime.clone();
                tokio::task::spawn_local(async move {
                    match wasmtime.lock().unwrap().start(&mut store, instance) {
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
