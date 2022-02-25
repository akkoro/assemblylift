use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use async_trait::async_trait;
use clap::crate_version;
use kubelet::{Kubelet, node};
use kubelet::config::Config;
use kubelet::container::Container;
use kubelet::container::state::prelude::SharedState;
use kubelet::log::Sender;
use kubelet::node::Builder;
use kubelet::plugin_watcher::PluginRegistry;
use kubelet::pod::{Handle, Pod, PodKey};
use kubelet::provider::{DevicePluginSupport, PluginSupport, Provider, ProviderError, VolumeSupport};
use kubelet::resources::DeviceManager;
use kubelet::state::common::{GenericProvider, GenericProviderState};
use kubelet::state::common::registered::Registered;
use kubelet::state::common::terminated::Terminated;
use kubelet::store::composite::ComposableStore;
use kubelet::store::oci::FileStore;
use kubelet::store::Store;
use kubelet::volume::VolumeRef;
use tempfile::NamedTempFile;
use tokio::sync::{mpsc, RwLock};

use assemblylift_core_iomod::registry;
use assemblylift_core_iomod::registry::RegistryTx;

use crate::runtime::{HandleFactory, Runtime, RuntimeHandle};
use crate::states::pod::PodState;

mod abi;
mod runtime;
mod states;

const LOG_DIR_NAME: &str = "wasi-logs";
const VOLUME_DIR: &str = "volumes";

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
pub struct RuntimeProvider {
    shared: ProviderState,
}

impl RuntimeProvider {
    pub async fn new(
        store: Arc<dyn Store + Sync + Send>,
        config: &kubelet::config::Config,
        kubeconfig: kube::Config,
        plugin_registry: Arc<PluginRegistry>,
        device_plugin_manager: Arc<DeviceManager>,
        registry_tx: RegistryTx,
    ) -> anyhow::Result<Self> {
        let log_path = config.data_dir.join(LOG_DIR_NAME);
        let volume_path = config.data_dir.join(VOLUME_DIR);
        tokio::fs::create_dir_all(&log_path).await?;
        tokio::fs::create_dir_all(&volume_path).await?;
        let client = kube::Client::try_from(kubeconfig)?;
        Ok(Self {
            shared: ProviderState {
                handles: Default::default(),
                store,
                log_path,
                volume_path,
                client,
                plugin_registry,
                device_plugin_manager,
                registry_tx,
            },
        })
    }
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

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let version = crate_version!();
    println!(
        "Starting AssemblyLift Kubelet runtime {}",
        version,
    );

    let registry_channel = mpsc::channel(32);
    let tx = registry_channel.0.clone();
    let rx = registry_channel.1;
    registry::spawn_registry(rx).unwrap();

    let config = Config::new_from_file_and_flags(version, None);

    // Initialize the logger
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let kube_config = kubelet::bootstrap(&config, &config.bootstrap_file, notify_bootstrap).await?;

    let store = make_store(&config);
    let plugin_registry = Arc::new(PluginRegistry::new(&config.plugins_dir));
    let device_plugin_manager = Arc::new(DeviceManager::new(
        &config.device_plugins_dir,
        kube::Client::try_from(kube_config.clone())?,
        &config.node_name,
    ));

    let provider = RuntimeProvider::new(
        store,
        &config,
        kube_config.clone(),
        plugin_registry,
        device_plugin_manager,
        tx,
    )
        .await?;
    let kubelet = Kubelet::new(provider, kube_config, config).await?;
    kubelet.start().await
}

fn make_store(config: &Config) -> Arc<dyn kubelet::store::Store + Send + Sync> {
    let client = oci_distribution::Client::from_source(config);
    let mut store_path = config.data_dir.join(".oci");
    store_path.push("modules");
    let file_store = Arc::new(FileStore::new(client, &store_path));

    if config.allow_local_modules {
        file_store.with_override(Arc::new(kubelet::store::fs::FileSystemStore {}))
    } else {
        file_store
    }
}

fn notify_bootstrap(message: String) {
    println!("BOOTSTRAP: {}", message);
}
