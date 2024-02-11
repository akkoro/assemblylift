use std::str::FromStr;

use clap::crate_version;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use assemblylift_core_iomod::registry;
use assemblylift_core_iomod::registry::registry_channel;
use assemblylift_hyper_runtime::spawn_runtime;

fn main() {
    let default_level = "info".to_string();
    let log_level = Level::from_str(
        std::env::args()
            .collect::<Vec<String>>()
            .get(1)
            .unwrap_or(&default_level),
    )
    .unwrap_or(Level::INFO);
    let subscriber = FmtSubscriber::builder().with_max_level(log_level).finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!(
        "Starting AssemblyLift Hyper HTTP runtime v{}",
        crate_version!()
    );

    let (registry_tx, registry_rx) = registry_channel(32);
    registry::spawn_registry(registry_rx).unwrap();

    spawn_runtime(registry_tx)
}
