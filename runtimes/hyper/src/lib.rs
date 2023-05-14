use std::sync::{Arc, Mutex};

use assemblylift_core_iomod::registry::RegistryTx;

use crate::launcher::Launcher;
use crate::runner::Runner;

pub mod abi;
pub mod launcher;
pub mod runner;

#[derive(Debug, Clone)]
pub enum Status {
    Exited(i32),
    Success(Vec<u8>),
    Failure(Vec<u8>),
}

pub fn spawn_runtime(registry_tx: RegistryTx) {
    // Mapped to /tmp inside the WASM module
    std::fs::create_dir_all("/tmp/asmltmp").expect("could not create /tmp/asmltmp");

    crossbeam_utils::thread::scope(|s| {
        let runner = Arc::new(Mutex::new(Runner::<Status>::new(registry_tx)));
        let tx = { runner.clone().lock().unwrap().sender() };

        let r = runner.clone();
        s.spawn(move |_| r.lock().unwrap().spawn());

        s.spawn(move |_| {
            let mut launcher = Launcher::new();
            launcher.spawn(tx);
        });
    })
    .unwrap();
}
