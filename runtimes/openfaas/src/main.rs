use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use clap::crate_version;
use crossbeam_channel::bounded;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use tokio::sync::mpsc;

use assemblylift_core::wasm;
use assemblylift_core_iomod::registry;

use crate::abi::OpenFaasAbi;
use crate::runner::{spawn_runner, RunnerMessage, RunnerTx};
use crate::Status::{Failure, Success};

mod abi;
mod runner;

pub type StatusTx = crossbeam_channel::Sender<Status>;
pub type StatusRx = crossbeam_channel::Receiver<Status>;

#[derive(Clone)]
pub enum Status {
    Success(String),
    Failure(String),
}

async fn launcher(
    req: Request<Body>,
    runner_tx: RunnerTx,
    status_rx: StatusRx,
) -> Result<Response<Body>, Infallible> {
    let input_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();

    let msg = RunnerMessage {
        input: input_bytes.to_vec(),
    };
    tokio::spawn(async move {
        if let Err(e) = runner_tx.send(msg).await {
            println!("could not send to runner: {}", e.to_string())
        }
    });

    if let Ok(result) = status_rx.recv() {
        return Ok(match result {
            Success(_) => Response::builder()
                .status(200)
                .body(Body::default())
                .unwrap(),
            Failure(_) => Response::builder()
                .status(500)
                .body(Body::default())
                .unwrap(),
        });
    }

    Ok(Response::builder()
        .status(500)
        .body(Body::default())
        .unwrap())
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
        let (status_tx, status_rx) = bounded::<Status>(32);
        let (registry_tx, registry_rx) = mpsc::channel(32);
        registry::spawn_registry(registry_rx).unwrap();

        let (module, store) =
            match wasm::deserialize_module_from_path::<OpenFaasAbi, Status>(
                "/opt/assemblylift", // TODO get from env
                "handler",          // TODO get from env
            ) {
                Ok(module) => (Arc::new(module.0), Arc::new(module.1)),
                Err(_) => panic!("Unable to build WASM module"),
            };

        let (resolver, threader_env) = wasm::build_module::<OpenFaasAbi, Status>(
            registry_tx.clone(),
            status_tx.clone(),
            module.clone(),
            "handler",
            store.clone(),
        ).expect("could not build WASM module");

        spawn_runner(
            status_tx.clone(),
            runner_rx,
            module.clone(),
            resolver.clone(),
            threader_env.clone(),
        );

        let mut rx = Some(status_rx);
        Ok::<_, Infallible>(service_fn(move |req| launcher(req, runner_tx.clone(), rx.take().unwrap())))
    });

    if let Err(e) = Server::bind(&addr).serve(make_svc).await {
        eprintln!("server error: {}", e);
    }
}
