use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::rc::Rc;

use tokio::sync::mpsc;
use tracing::{debug, info};

use assemblylift_core::wasm::{StatusTx, Wasmtime};
use assemblylift_core_iomod::registry::RegistryTx;

use crate::abi::Abi;
use crate::Status;

pub type RunnerTx<S> = mpsc::Sender<RunnerMessage<S>>;
pub type RunnerRx<S> = mpsc::Receiver<RunnerMessage<S>>;
pub type RunnerChannel<S> = (RunnerTx<S>, RunnerRx<S>);

#[derive(Clone)]
pub struct RunnerMessage<S>
where
    S: Clone + Send + Sized + 'static,
{
    pub input: Vec<u8>,
    pub status_sender: StatusTx<S>,
    pub wasm_path: PathBuf,
    pub env_vars: BTreeMap<String, String>,
    pub bind_paths: BTreeMap<String, String>,
    pub runtime_environment: Option<String>,
}

pub struct Runner<S>
where
    S: Clone + Send + Sized + 'static,
{
    channel: RunnerChannel<S>,
    registry_tx: RegistryTx,
    runtime: tokio::runtime::Runtime,
}

impl Runner<Status> {
    pub fn new(registry_tx: RegistryTx) -> Self {
        Runner {
            channel: mpsc::channel(32),
            registry_tx,
            runtime: tokio::runtime::Builder::new_current_thread()
                .enable_io()
                .build()
                .unwrap(),
        }
    }

    pub fn spawn<'a>(&mut self) {
        info!("Spawning runner");
        tokio::task::LocalSet::new().block_on(&self.runtime, async {
            let mut functions: BTreeMap<PathBuf, Rc<RefCell<Wasmtime<Abi, Status>>>> =
                BTreeMap::new();

            while let Some(msg) = self.channel.1.recv().await {
                debug!("received runner message");

                let runtime_environment = match std::env::var("ASML_FUNCTION_ENV") {
                    Ok(env) => env,
                    Err(_) => msg.runtime_environment.unwrap_or("default".to_string()),
                };

                let wasm_path = msg.wasm_path;
                info!("Loading module at {}", wasm_path.clone().display());

                // Environment vars prefixed with __ASML_ are defined in the function definition;
                // the prefix indicates that they are to be mapped to the function environment.
                // In a single-function environment (Lambda or Docker), these are the only function env-vars;
                // in a multi-function environment, these are global across all functions and function-specific
                // vars are passed thru the runner request from the launcher.
                let mut env_vars: Vec<(String, String)> = Vec::from_iter(
                    std::env::vars()
                        .into_iter()
                        .filter(|e| e.0.starts_with("__ASML_"))
                        .map(|e| (e.0.replace("__ASML_", ""), e.1))
                        .into_iter(),
                );
                env_vars.append(
                    &mut Vec::from_iter(
                        msg.env_vars
                            .into_iter()
                            .map(|e| (e.0, e.1))
                            .into_iter(),
                    )
                );

                let bind_paths: Vec<(String, String)> = Vec::from_iter(
                    msg
                    .bind_paths
                    .into_iter()
                    .map(|e| (e.0, e.1))
                    .into_iter(),
                );

                let wasmtime = match functions.contains_key(&*wasm_path) {
                    false => {
                        let wt = Rc::new(RefCell::new(
                            // FIXME this should return an error response via status_sender instead of panicing
                            Wasmtime::<Abi, Status>::new_from_path(wasm_path.as_ref())
                                .expect("could not create WASM runtime from module path"),
                        ));
                        functions.insert(wasm_path, wt.clone());
                        wt
                    }
                    true => functions.get(&*wasm_path).unwrap().clone(),
                };

                let (command, mut store) = wasmtime
                    .borrow_mut()
                    .link_wasi_component(
                        self.registry_tx.clone(),
                        msg.status_sender.clone(),
                        env_vars,
                        runtime_environment.clone(),
                        bind_paths,
                        None,
                        &msg.input,
                    )
                    .await
                    .expect("could not link wasm component");

                let wasmtime = wasmtime.clone();
                tokio::task::spawn_local(async move {
                    match wasmtime
                        .borrow_mut()
                        .run_component(command, &mut store)
                        .await
                    {
                        Ok(_) => msg.status_sender.send(Status::Exited(0)),
                        Err(err) => msg.status_sender.send(Status::Failure(
                            format!("WASM module exited in error: {}", err.to_string()).as_bytes().to_vec(),
                        )),
                    }
                });
            }
        });
    }

    pub fn sender(&self) -> RunnerTx<Status> {
        self.channel.0.clone()
    }
}
