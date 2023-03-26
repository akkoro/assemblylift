use clap::ArgMatches;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use assemblylift_core_iomod::registry::{registry_channel, spawn_registry};
use assemblylift_hyper_runtime::spawn_runtime;

pub fn command(_matches: Option<&ArgMatches>) {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let (registry_tx, registry_rx) = registry_channel(8);
    spawn_registry(registry_rx).expect("unable to spawn IOmod registry");
    spawn_runtime(registry_tx);
}
