use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use kubelet::container::Handle as ContainerHandle;
use kubelet::container::Status;
use kubelet::handle::StopHandler;
use tokio::sync::mpsc::Sender;

use assemblylift_core::threader::ThreaderEnv;
use assemblylift_core::wasm;
use assemblylift_core::wasm::Resolver;

use crate::HandleFactory;

pub struct Runtime {
    module: Arc<wasmer::Module>,
    resolver: Resolver,
    threader_env: ThreaderEnv,
}

impl Runtime {
    pub async fn new<L: AsRef<Path> + Send + Sync + 'static>(
        name: String,
        module_data: Vec<u8>,
        env: HashMap<String, String>,
        args: Vec<String>,
        dirs: HashMap<PathBuf, Option<PathBuf>>,
        log_dir: L,
        status_sender: Sender<Status>,
    ) -> anyhow::Result<Self> {
        // TODO `tx` comes from IOmod registry, where does that live?
        match wasm::build_module_from_bytes(tx, &module_data, &name) {
            Ok((module, resolver, threader_env)) => {
                Ok(Runtime {
                    module: Arc::new(module),
                    resolver,
                    threader_env,
                })
            }
            Err(e) => Err(e),
        }
    }

    pub async fn start(&self) -> anyhow::Result<ContainerHandle<Runtime, HandleFactory>> {
        // TODO this might work better if the instance is created in Runtime::new along with a tokio runtime,
        //      then here we can spawn call(_start) on the tk runtime and return the JoinHandle to the kube layer
        let instance = wasm::new_instance(self.module.clone(), self.resolver.clone()).unwrap(); // FIXME catch error
    }
}

#[async_trait::async_trait]
impl StopHandler for Runtime {
    async fn stop(&mut self) -> anyhow::Result<()> {
        todo!()
    }

    async fn wait(&mut self) -> anyhow::Result<()> {
        todo!()
    }
}
