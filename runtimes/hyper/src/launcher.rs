use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::{Body, Method, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use tokio::sync::mpsc;
use tracing::{debug, info};

use crate::{Failure, RunnerMessage, RunnerTx, StatusRx, StatusTx, Success};

pub struct Launcher {
    runtime: tokio::runtime::Runtime,
}

impl Launcher {
    pub fn new() -> Self {
        Self {
            runtime: tokio::runtime::Runtime::new().unwrap(),
        }
    }

    pub fn spawn(&mut self, runner_tx: RunnerTx) {
        info!("Spawning launcher");
        tokio::task::LocalSet::new().block_on(&self.runtime, async {
            let make_svc = make_service_fn(|_| {
                debug!("called make_service_fn");
                let channel = mpsc::channel(32);
                let runner_tx = runner_tx.clone();
                let tx = channel.0.clone();
                let mut rx = Some(channel.1);
                async {
                    Ok::<_, Infallible>(service_fn(move |req| {
                        launch(req, runner_tx.clone(), tx.clone(), rx.take().unwrap())
                    }))
                }
            });

            let addr = SocketAddr::from(([0, 0, 0, 0], 5543));
            info!("Serving from {}", addr.to_string());
            if let Err(e) = Server::bind(&addr).serve(make_svc).await {
                eprintln!("server error: {}", e);
            }
        });
    }
}

async fn launch(
    req: Request<Body>,
    runner_tx: RunnerTx,
    status_tx: StatusTx,
    mut status_rx: StatusRx,
) -> Result<Response<Body>, Infallible> {
    if req.method() != Method::POST {
        return Ok(Response::builder()
            .status(500)
            .body(Body::default())
            .unwrap());
    }

    let input_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();

    let msg = RunnerMessage {
        input: input_bytes.to_vec(),
        status_sender: status_tx.clone(),
    };
    tokio::spawn(async move {
        if let Err(e) = runner_tx.send(msg).await {
            println!("could not send to runner: {}", e.to_string())
        }
    });

    if let Some(result) = status_rx.recv().await {
        return Ok(match result {
            Success(response) => Response::builder()
                .status(200)
                .body(Body::from(response))
                .unwrap(),
            Failure(response) => Response::builder()
                .status(500)
                .body(Body::from(response))
                .unwrap(),
        });
    }

    Ok(Response::builder()
        .status(500)
        .body(Body::default())
        .unwrap())
}
