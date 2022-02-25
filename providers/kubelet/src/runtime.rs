use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use kubelet::container::Handle as ContainerHandle;
use kubelet::container::Status;
use kubelet::handle::StopHandler;
use tempfile::NamedTempFile;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;

use assemblylift_core::threader::ThreaderEnv;
use assemblylift_core::wasm;
use assemblylift_core::wasm::Resolver;
use assemblylift_core_iomod::registry::RegistryTx;

use crate::abi::KubeletAbi;

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
    handle: JoinHandle<Result<(), SendError<Status>>>,
}

pub struct Runtime {
    module: Arc<wasmer::Module>,
    resolver: Resolver,
    threader_env: ThreaderEnv<Status>,
    tokio: tokio::runtime::Runtime,
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
        status_sender: Sender<Status>,
    ) -> anyhow::Result<Self> {
        let temp_file = tokio::task::spawn_blocking(move || -> anyhow::Result<NamedTempFile> {
            Ok(NamedTempFile::new_in(log_dir)?)
        }).await??;
        match wasm::build_module_from_bytes::<KubeletAbi, Status>(registry_tx, status_sender, &module_data, &name) {
            Ok((module, resolver, threader_env)) => {
                Ok(Runtime {
                    module:  Arc::new(module),
                    resolver: resolver.clone(),
                    threader_env,
                    tokio: tokio::runtime::Runtime::new().expect("TODO handle this panic"),
                    output: Arc::new(temp_file),
                })
            }
            Err(e) => Err(e),
        }
    }

    pub async fn start(&self) -> anyhow::Result<ContainerHandle<RuntimeHandle, HandleFactory>> {
        let hnd = self.tokio.handle().clone();
        let instance = wasm::new_instance(self.module.clone(), self.resolver.clone())
            .expect("TODO handle this panic");
        let threader = self.threader_env.threader.clone();
        let handle: JoinHandle<Result<(), SendError<Status>>> = hnd.spawn_blocking(move || -> Result<(), SendError<Status>> {
            let start = instance.exports.get_function("_start").unwrap();

            let threader = threader.lock().unwrap();
            let status_sender = threader.status_sender.as_ref().unwrap();

            status_sender.blocking_send(Status::Running {
                timestamp: chrono::Utc::now(),
            });

            match start.call(&[]) {
                Ok(result) => status_sender.blocking_send(Status::Terminated {
                    timestamp: chrono::Utc::now(),
                    message: "WASM exited successfully".to_string(),
                    failed: false
                }),
                Err(error) => status_sender.blocking_send(Status::Terminated {
                    timestamp: chrono::Utc::now(),
                    message: error.message(),
                    failed: true
                }),
            }
        });

        let log_handle_factory = HandleFactory {
            temp: self.output.clone(),
        };

        Ok(ContainerHandle::new(
            RuntimeHandle { handle },
            log_handle_factory,
        ))
    }
}

#[async_trait::async_trait]
impl StopHandler for RuntimeHandle {
    async fn stop(&mut self) -> anyhow::Result<()> {
        Ok(self.handle.abort())
    }

    async fn wait(&mut self) -> anyhow::Result<()> {
        (&mut self.handle).await?;
        Ok(())
    }
}
