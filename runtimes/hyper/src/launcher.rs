use std::collections::BTreeMap;
use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use serde::{Serialize, Deserialize};
use tokio::sync::mpsc;
use tracing::{debug, error, info};

use crate::{Failure, RunnerMessage, RunnerTx, StatusRx, StatusTx, Success};
use crate::Status::Exited;

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
    let method = req.method().to_string();
    let mut headers = BTreeMap::new();
    for h in req.headers().iter() {
        headers.insert(h.0.as_str().to_string(), h.1.to_str().unwrap().to_string());
    }
    let input_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let launcher_req = LauncherRequest {
        method,
        headers,
        body: Some(z85::encode(input_bytes.as_ref())),
    };

    let msg = RunnerMessage {
        input: serde_json::to_vec(&launcher_req).unwrap(),
        status_sender: status_tx.clone(),
    };
    tokio::spawn(async move {
        if let Err(e) = runner_tx.send(msg).await {
            error!("could not send to runner: {}", e.to_string())
        }
    });

    while let Some(result) = status_rx.recv().await {
        debug!("launcher received status response from runner: {:?}", result);
        return Ok(match result {
            Exited(_status) => continue, // TODO start timeout to default response
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

#[derive(Serialize, Deserialize)]
struct LauncherRequest {
    method: String,
    headers: BTreeMap<String, String>,
    body: Option<String>,
}