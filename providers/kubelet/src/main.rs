use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use async_trait::async_trait;
use kubelet::container::Container;
use kubelet::container::state::prelude::SharedState;
use kubelet::log::Sender;
use kubelet::node;
use kubelet::node::Builder;
use kubelet::plugin_watcher::PluginRegistry;
use kubelet::pod::{Handle, Pod, PodKey};
use kubelet::provider::{DevicePluginSupport, PluginSupport, Provider, ProviderError, VolumeSupport};
use kubelet::resources::DeviceManager;
use kubelet::state::common::{GenericProvider, GenericProviderState};
use kubelet::state::common::registered::Registered;
use kubelet::state::common::terminated::Terminated;
use kubelet::store::Store;
use kubelet::volume::VolumeRef;
use tempfile::NamedTempFile;
use tokio::sync::RwLock;
use assemblylift_core_iomod::registry::RegistryTx;

use crate::runtime::{HandleFactory, Runtime, RuntimeHandle};
use crate::states::pod::PodState;

mod abi;
mod runtime;
mod states;

pub(crate) type PodHandleMap = Arc<RwLock<HashMap<PodKey, Arc<Handle<RuntimeHandle, HandleFactory>>>>>;

pub(crate) struct ModuleRunContext {
    modules: HashMap<String, Vec<u8>>,
    volumes: HashMap<String, VolumeRef>,
    env_vars: HashMap<String, HashMap<String, String>>,
}

/// Provider-level state shared between all pods
#[derive(Clone)]
pub struct ProviderState {
    handles: PodHandleMap,
    store: Arc<dyn Store + Sync + Send>,
    log_path: PathBuf,
    client: kube::Client,
    volume_path: PathBuf,
    plugin_registry: Arc<PluginRegistry>,
    device_plugin_manager: Arc<DeviceManager>,
    registry_tx: RegistryTx,
}

#[async_trait]
impl GenericProviderState for ProviderState {
    fn client(&self) -> kube::Client {
        self.client.clone()
    }

    fn store(&self) -> std::sync::Arc<(dyn Store + Send + Sync + 'static)> {
        self.store.clone()
    }

    async fn stop(&self, pod: &Pod) -> anyhow::Result<()> {
        let key = PodKey::from(pod);
        let mut handle_writer = self.handles.write().await;
        if let Some(handle) = handle_writer.get_mut(&key) {
            handle.stop().await
        } else {
            Ok(())
        }
    }
}

impl VolumeSupport for ProviderState {
    fn volume_path(&self) -> Option<&Path> {
        Some(self.volume_path.as_ref())
    }
}

impl PluginSupport for ProviderState {
    fn plugin_registry(&self) -> Option<Arc<PluginRegistry>> {
        Some(self.plugin_registry.clone())
    }
}

impl DevicePluginSupport for ProviderState {
    fn device_plugin_manager(&self) -> Option<Arc<DeviceManager>> {
        Some(self.device_plugin_manager.clone())
    }
}

#[derive(Clone)]
pub(crate) struct RuntimeProvider {
    shared: ProviderState,
}

#[async_trait]
impl Provider for RuntimeProvider {
    type ProviderState = ProviderState;
    type PodState = PodState;
    type InitialState = Registered<Self>;
    type TerminatedState = Terminated<Self>;

    const ARCH: &'static str = "wasm32-wasi";

    fn provider_state(&self) -> SharedState<ProviderState> {
        Arc::new(RwLock::new(self.shared.clone()))
    }

    async fn node(&self, builder: &mut Builder) -> anyhow::Result<()> {
        builder.set_architecture("wasm-wasi");
        builder.add_taint("NoSchedule", "kubernetes.io/arch", Self::ARCH);
        builder.add_taint("NoExecute", "kubernetes.io/arch", Self::ARCH);
        Ok(())
    }

    async fn initialize_pod_state(&self, pod: &Pod) -> anyhow::Result<Self::PodState> {
        Ok(PodState::new(pod))
    }

    // Evict all pods upon shutdown
    async fn shutdown(&self, node_name: &str) -> anyhow::Result<()> {
        node::drain(&self.shared.client, node_name).await?;
        Ok(())
    }

    async fn logs(
        &self,
        namespace: String,
        pod_name: String,
        container_name: String,
        sender: kubelet::log::Sender,
    ) -> anyhow::Result<()> {
        let mut handles = self.shared.handles.write().await;
        let handle = handles
            .get_mut(&PodKey::new(&namespace, &pod_name))
            .ok_or_else(|| ProviderError::PodNotFound {
                pod_name: pod_name.clone(),
            })?;
        handle.output(&container_name, sender).await
    }
}

impl GenericProvider for RuntimeProvider {
    type ProviderState = ProviderState;
    type PodState = PodState;
    type RunState = crate::states::pod::initializing::Initializing;

    fn validate_pod_runnable(_pod: &Pod) -> anyhow::Result<()> {
        Ok(())
    }

    fn validate_container_runnable(
        container: &kubelet::container::Container,
    ) -> anyhow::Result<()> {
        if let Some(image) = container.image()? {
            if image.whole().starts_with("k8s.gcr.io/kube-proxy") {
                return Err(anyhow::anyhow!("Cannot run kube-proxy"));
            }
        }
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
    // TODO bootstrap kubelet with RuntimeProvider
}
