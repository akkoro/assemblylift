use std::collections::HashMap;
use std::path::{Path, PathBuf};
use kubelet::container::Handle as ContainerHandle;
use kubelet::container::Status;
use kubelet::handle::StopHandler;
use tokio::sync::mpsc::Sender;
use crate::HandleFactory;

pub struct Runtime;

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
        todo!()
    }

    pub async fn start(&self) -> anyhow::Result<ContainerHandle<Runtime, HandleFactory>> {
        todo!()
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
