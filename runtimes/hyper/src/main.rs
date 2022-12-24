use std::fs;
use std::sync::{Arc, Mutex};

use clap::crate_version;
use tokio::sync::mpsc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use assemblylift_core::wasm::Wasmtime;
use assemblylift_core_iomod::registry;

use crate::abi::GenericDockerAbi;
use crate::launcher::Launcher;
use crate::runner::{Runner, RunnerMessage, RunnerTx};
use crate::Status::{Failure, Success};

mod abi;
mod launcher;
mod runner;

// pub type StatusTx = mpsc::Sender<Status>;
// pub type StatusRx = mpsc::Receiver<Status>;
pub type StatusTx = crossbeam_channel::Sender<Status>;
pub type StatusRx = crossbeam_channel::Receiver<Status>;
pub type StatusChannel = (StatusTx, StatusRx);

#[derive(Debug, Clone)]
pub enum Status {
    Exited(i32),
    Success(String),
    Failure(String),
}

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Starting AssemblyLift hyper runtime v{}", crate_version!());

    // Mapped to /tmp inside the WASM module
    fs::create_dir_all("/tmp/asmltmp").expect("could not create /tmp/asmltmp");

    let (registry_tx, registry_rx) = mpsc::channel(32);
    registry::spawn_registry(registry_rx).unwrap();

    let wasmtime = Arc::new(Mutex::new(
        Wasmtime::<GenericDockerAbi, Status>::new_from_path(
            format!(
                "/opt/assemblylift/{}",
                std::env::var("ASML_WASM_MODULE_NAME").unwrap_or("handler.wasm.bin".into())
            )
            .as_ref(),
        )
        .expect("could not create WASM runtime from module path"),
    ));

    crossbeam_utils::thread::scope(|s| {
        let runner = Arc::new(Mutex::new(Runner::new(registry_tx, wasmtime)));
        let tx = { runner.clone().lock().unwrap().sender() };

        let r = runner.clone();
        s.spawn(move |_| {
            r.lock().unwrap().spawn()
        });

        s.spawn(move |_| {
            let mut launcher = Launcher::new();
            launcher.spawn(tx);
        });
    })
    .unwrap();
}
