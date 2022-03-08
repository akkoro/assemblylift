use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use clap::crate_version;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use tokio::sync::mpsc;
use wasmer::Instance;

use assemblylift_core::buffers::LinearBuffer;
use assemblylift_core::threader::ThreaderEnv;
use assemblylift_core::wasm;
use assemblylift_core_iomod::registry;
use assemblylift_core_iomod::registry::RegistryTx;

use crate::abi::OpenFaasAbi;
use crate::runner::{RunnerMessage, RunnerTx, spawn_runner};

mod abi;
mod runner;

pub type Status = ();
pub type StatusTx = mpsc::Sender<Status>;

async fn launcher(
    req: Request<Body>,
    runner_tx: RunnerTx,
) -> Result<Response<Body>, Infallible> {
    let input_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();

    let msg = RunnerMessage { input: input_bytes.to_vec() };
    let result = runner_tx.send(msg).await;

    // TODO this should wait for a response from the runner before replying!
    Ok(match result {
        Ok(_) => Response::builder()
            .status(200)
            .body(Body::default())
            .unwrap(),
        Err(e) => Response::builder()
            .status(500)
            .body(Body::from(e.to_string()))
            .unwrap(),
    })
}

#[tokio::main]
async fn main() {
    println!(
        "Starting AssemblyLift OpenFaas runtime {}",
        crate_version!()
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn| async {
        let (runner_tx, runner_rx) = mpsc::channel(32);
        let (status_tx, _status_receiver) = mpsc::channel::<Status>(32);
        let (registry_tx, registry_rx) = mpsc::channel(100);
        registry::spawn_registry(registry_rx).unwrap();

        let (module, resolver, threader_env) = match wasm::build_module_from_path::<OpenFaasAbi, ()>(
            registry_tx,
            status_tx,
            "/opt/assemblylift/handler.wasm.bin", // TODO get from env
            "handler", // TODO get from env
        ) {
            Ok(module) => (Arc::new(module.0), module.1, module.2),
            Err(_) => panic!("Unable to build WASM module"),
        };

        spawn_runner(runner_rx, module.clone(), resolver.clone(), threader_env.clone());

        Ok::<_, Infallible>(service_fn(move |req| launcher(req, runner_tx.clone())))
    });

    if let Err(e) = Server::bind(&addr).serve(make_svc).await {
        eprintln!("server error: {}", e);
    }
}
