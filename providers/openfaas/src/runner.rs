use std::sync::Arc;
use tokio::sync::mpsc;

use wasmer::Module;
use assemblylift_core::buffers::LinearBuffer;
use assemblylift_core::threader::ThreaderEnv;

use assemblylift_core::wasm;
use assemblylift_core::wasm::Resolver;

use crate::Status;

pub type RunnerTx = mpsc::Sender<RunnerMessage>;
pub type RunnerRx = mpsc::Receiver<RunnerMessage>;

#[derive(Clone)]
pub struct RunnerMessage {
    pub input: Vec<u8>,
}

pub fn spawn_runner(mut rx: RunnerRx, module: Arc<Module>, resolver: Resolver, env: ThreaderEnv<Status>) {
    std::thread::spawn(move || {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        tokio::task::LocalSet::new().block_on(&mut rt, async {
            while let Some(msg) = rx.recv().await {
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
                    start.call(&[]).expect("WASM handler exited with error");
                });
            }
        });
    });
}
