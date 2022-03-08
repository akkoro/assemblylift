use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use clap::crate_version;
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

pub type StatusTx = mpsc::Sender<Status>;
pub type StatusRx = mpsc::Receiver<Status>;

#[derive(Clone)]
pub enum Status {
    Success(String),
    Failure(String),
}

async fn launcher(
    req: Request<Body>,
    runner_tx: RunnerTx,
    mut status_rx: StatusRx,
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

    if let Some(result) = status_rx.recv().await {
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
        let (status_tx, status_rx) = mpsc::channel::<Status>(32);
        let (registry_tx, registry_rx) = mpsc::channel(32);
        registry::spawn_registry(registry_rx).unwrap();

        let (module, resolver, threader_env) =
            match wasm::build_module_from_path::<OpenFaasAbi, Status>(
                registry_tx,
                status_tx.clone(),
                "/opt/assemblylift", // TODO get from env
                "handler",          // TODO get from env
            ) {
                Ok(module) => (Arc::new(module.0), module.1, module.2),
                Err(_) => panic!("Unable to build WASM module"),
            };

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
