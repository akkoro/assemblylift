use std::fs;

use clap::crate_version;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use assemblylift_core_iomod::registry;
use assemblylift_core_iomod::registry::registry_channel;
use assemblylift_hyper_runtime::spawn_runtime;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Starting AssemblyLift Hyper HTTP runtime v{}", crate_version!());

    // Mapped to /tmp inside the WASM module
    fs::create_dir_all("/tmp/asmltmp").expect("could not create /tmp/asmltmp");

    let (registry_tx, registry_rx) = registry_channel(32);
    registry::spawn_registry(registry_rx).unwrap();

    spawn_runtime(registry_tx)
}
