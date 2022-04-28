use std::collections::HashMap;
use std::convert::Infallible;
use std::future::Future;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process;
use std::process::Stdio;
use std::sync::Arc;
use futures::TryFutureExt;

use kubelet::container::Handle as ContainerHandle;
use kubelet::container::Status;
use kubelet::handle::StopHandler;
use tempfile::NamedTempFile;
use tokio::task::JoinHandle;
use tracing::{info, warn};

use assemblylift_core::buffers::LinearBuffer;
use assemblylift_core::threader::ThreaderEnv;
use assemblylift_core::wasm;
use assemblylift_core::wasm::Resolver;
use assemblylift_core_iomod::registry::RegistryTx;

use crate::abi::KubeletAbi;

pub type HandleResult = Result<(), crossbeam_channel::SendError<Status>>;

/// Holds our tempfile handle.
pub struct HandleFactory {
    temp: Arc<NamedTempFile>,
}

impl kubelet::log::HandleFactory<tokio::fs::File> for HandleFactory {
    /// Creates `tokio::fs::File` on demand for log reading.
    fn new_handle(&self) -> tokio::fs::File {
        tokio::fs::File::from_std(self.temp.reopen().unwrap())
    }
}

pub struct RuntimeHandle {
    wasm_handle: JoinHandle<HandleResult>,
}

pub struct Runtime {
    module: Arc<wasmer::Module>,
    resolver: Resolver,
    threader_env: ThreaderEnv<Status>,
    output: Arc<NamedTempFile>,
}

impl Runtime {
    pub async fn new<L: AsRef<Path> + Send + Sync + 'static>(
        name: String,
        module_data: Vec<u8>,
        registry_tx: RegistryTx,
        env: HashMap<String, String>,
        args: Vec<String>,
        dirs: HashMap<PathBuf, Option<PathBuf>>,
        log_dir: L,
        status_sender: crossbeam_channel::Sender<Status>,
    ) -> anyhow::Result<Self> {
        let temp_file = tokio::task::spawn_blocking(move || -> anyhow::Result<NamedTempFile> {
            Ok(NamedTempFile::new_in(log_dir)?)
        }).await??;
        let (module, store) = wasm::deserialize_module_from_bytes::<KubeletAbi, Status>(&module_data)?;
        let module = Arc::new(module);
        let store = Arc::new(store);
        let (resolver, threader_env) = wasm::build_module::<KubeletAbi, Status>(
            registry_tx.clone(),
            status_sender.clone(),
            module.clone(),
            &name,
            store,
        )?;

        Ok(Runtime {
            module,
            resolver: resolver.clone(),
            threader_env,
            output: Arc::new(temp_file),
        })
    }

    pub async fn start(&self) -> anyhow::Result<ContainerHandle<RuntimeHandle, HandleFactory>> {
        let instance = wasm::new_instance(self.module.clone(), self.resolver.clone())
            .expect("TODO handle this panic");
        let env = self.threader_env.clone();

        let wasm_handle: JoinHandle<HandleResult> = tokio::task::spawn_blocking(move || -> HandleResult {
            let start = instance.exports.get_function("_start").unwrap();

            let status_sender = env.status_sender;
            status_sender.send(Status::Running {
                timestamp: chrono::Utc::now(),
            });

            match start.call(&[]) {
                Ok(_) => {
                    status_sender.send(Status::Terminated {
                        timestamp: chrono::Utc::now(),
                        message: "WASM exited successfully".to_string(),
                        failed: false,
                    })
                }
                Err(error) => {
                    status_sender.send(Status::Terminated {
                        timestamp: chrono::Utc::now(),
                        message: error.message(),
                        failed: true,
                    })
                }
            }
        });

        let log_handle_factory = HandleFactory {
            temp: self.output.clone(),
        };

        Ok(ContainerHandle::new(
            RuntimeHandle { wasm_handle },
            log_handle_factory,
        ))
    }
}

#[async_trait::async_trait]
impl StopHandler for RuntimeHandle {
    async fn stop(&mut self) -> anyhow::Result<()> {
        self.wasm_handle.abort();
        Ok(())
    }

    async fn wait(&mut self) -> anyhow::Result<()> {
        (&mut self.wasm_handle).await.unwrap();
        Ok(())
    }
}
