use std::fs;
use std::sync::{Arc, Mutex};

use clap::crate_version;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use assemblylift_core::wasm::StatusTx;
use assemblylift_core_iomod::registry;
use assemblylift_core_iomod::registry::registry_channel;

use crate::abi::Abi;
use crate::launcher::Launcher;
use crate::runner::{Runner, RunnerMessage, RunnerTx};
use crate::Status::{Failure, Success};

mod abi;
mod launcher;
mod runner;

#[derive(Debug, Clone)]
pub enum Status {
    Exited(i32),
    Success(Vec<u8>),
    Failure(Vec<u8>),
}

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

    crossbeam_utils::thread::scope(|s| {
        let runner = Arc::new(Mutex::new(Runner::<Status>::new(registry_tx)));
        let tx = { runner.clone().lock().unwrap().sender() };

        let r = runner.clone();
        s.spawn(move |_| r.lock().unwrap().spawn());

        s.spawn(move |_| {
            let mut launcher = Launcher::<Status>::new();
            launcher.spawn(tx);
        });
    })
    .unwrap();
}
