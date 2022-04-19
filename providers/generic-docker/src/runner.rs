use std::sync::Arc;
use tokio::sync::mpsc;

use wasmer::Module;
use assemblylift_core::buffers::LinearBuffer;
use assemblylift_core::threader::ThreaderEnv;

use assemblylift_core::wasm;
use assemblylift_core::wasm::Resolver;

use crate::{Status, StatusTx};

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
}

impl Runner {
    pub fn new() -> Self {
        Runner {
            channel: mpsc::channel(32), // TODO move out? or make crossbeam channel
        }
    }

    pub fn spawn(&mut self, module: Arc<Module>, resolver: Resolver, env: ThreaderEnv<Status>) {
        crossbeam_utils::thread::scope(|s| {
            s.spawn(move |_| {
                let rt = tokio::runtime::Runtime::new().unwrap();
                tokio::task::LocalSet::new().block_on(&rt, async {
                    while let Some(msg) = self.channel.1.recv().await {
                        env.host_input_buffer
                            .clone()
                            .lock()
                            .unwrap()
                            .initialize(msg.input);
                        // TODO instance pooling
                        let instance = match wasm::new_instance(module.clone(), resolver.clone()) {
                            Ok(instance) => Arc::new(instance),
                            Err(why) => panic!("Unable to spin new WASM instance {}", why.to_string()),
                        };
                        tokio::task::spawn_local(async move {
                            let start = instance.exports.get_function("_start").unwrap();
                            match start.call(&[]) {
                                Ok(_) => msg.status_sender.send(Status::Success("".to_string())).unwrap(),
                                Err(_) => msg.status_sender.send(Status::Failure("WASM module exited in error".to_string())).unwrap(),
                            }
                        });
                    }
                });
            });
        }).unwrap()
    }

    pub fn sender(&self) -> RunnerTx {
        self.channel.0.clone()
    }
}
