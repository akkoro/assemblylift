use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::anyhow;
use hyper::header::{HeaderName, HeaderValue};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};
use url::Url;

use assemblylift_core::wasm::{status_channel, StatusRx, StatusTx};

use crate::runner::{RunnerMessage, RunnerTx};
use crate::Status;
use crate::Status::{Exited, Failure, Success};

pub struct Launcher<S>
where
    S: Clone + Send + Sized + 'static,
{
    runtime: tokio::runtime::Runtime,
    _phantom: std::marker::PhantomData<S>,
}

impl Launcher<Status> {
    pub fn new() -> Self {
        Self {
            runtime: tokio::runtime::Builder::new_current_thread()
                .enable_io()
                .build()
                .unwrap(),
            _phantom: std::marker::PhantomData::default(),
        }
    }

    pub fn spawn(&mut self, runner_tx: RunnerTx<Status>) {
        info!("Spawning launcher");
        tokio::task::LocalSet::new().block_on(&self.runtime, async {
            let channel = status_channel(32);

            let make_svc = make_service_fn(|_| {
                debug!("called make_service_fn");
                let runner_tx = runner_tx.clone();
                let tx = channel.0.clone();
                let rx = channel.1.clone();
                async {
                    Ok::<_, anyhow::Error>(service_fn(move |req| {
                        launch(req, runner_tx.clone(), tx.clone(), rx.clone())
                    }))
                }
            });

            let addr = SocketAddr::from(([0, 0, 0, 0], 5543));
            info!("Serving from {}", addr.to_string());
            if let Err(e) = Server::bind(&addr).serve(make_svc).await {
                error!("server error: {}", e);
            }
        });
    }
}

async fn launch(
    req: Request<Body>,
    runner_tx: RunnerTx<Status>,
    status_tx: StatusTx<Status>,
    status_rx: StatusRx<Status>,
) -> anyhow::Result<Response<Body>> {
    debug!("launching function...");
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let mut headers = BTreeMap::new();
    for h in req.headers().iter() {
        headers.insert(h.0.as_str().to_string(), h.1.to_str().unwrap().to_string());
    }
    // FIXME cap input length to limit DoS attacks
    let input_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let launcher_req = LauncherRequest {
        method,
        path,
        headers: headers.clone(),
        body_encoding: "base64".into(),
        body: Some(base64::encode(input_bytes.as_ref())),
    };

    // TODO check for x-assemblylift-function-coordinates header, and if found then load from a
    //      projects dir, $INSTALL/assemblylift/projects, or failing that from the cwd if assemblylift.toml
    //      is found and the project name matches
    let wasm_uri = Url::from_str(&**headers.get("x-assemblylift-wasm-uri").unwrap())
        .map_err(|e| anyhow!(e))?;
    if !wasm_uri.scheme().eq_ignore_ascii_case("file") {
        unimplemented!("{} scheme not yet supported", wasm_uri.scheme());
    }
    let msg = RunnerMessage {
        input: serde_json::to_vec(&launcher_req).unwrap(),
        status_sender: status_tx.clone(),
        wasm_path: PathBuf::from(wasm_uri.path()),
    };

    debug!("sending runner request...");
    runner_tx
        .send(msg)
        .await
        .map_err(|e| anyhow!("could not send to runner: {}", e.to_string()))?;

    debug!("waiting for runner response...");
    while let Ok(result) = status_rx.recv() {
        debug!("launcher received response from runner");
        return Ok(match result {
            Exited(_status) => continue, // TODO start timeout to default response
            Success(response) => match serde_json::from_slice::<serde_json::Value>(&response) {
                Ok(json) => match json.get("isBase64Encoded").is_some() {
                    true => {
                        let b64 = json.get("isBase64Encoded").unwrap().as_bool().unwrap();
                        let body = json
                            .get("body")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .as_bytes()
                            .to_vec();
                        let mut response = Response::builder()
                            .status(json.get("statusCode").unwrap().as_i64().unwrap() as u16)
                            .body(Body::from(match b64 {
                                true => base64::decode(body).unwrap(),
                                false => body,
                            }))
                            .unwrap();
                        let headers = json.get("headers").unwrap().as_object().unwrap().clone();
                        for header in headers.into_iter() {
                            let key = header.0.clone();
                            response.headers_mut().insert(
                                HeaderName::from_str(key.as_str()).unwrap(),
                                HeaderValue::from_str(header.1.as_str().unwrap()).unwrap(),
                            );
                        }
                        response
                    }
                    false => Response::builder()
                        .status(200)
                        .header("content-type", "application/json")
                        .body(Body::from(response))
                        .unwrap(),
                },
                Err(_) => Response::builder()
                    .status(200)
                    .body(Body::from(response))
                    .unwrap(),
            },
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
    path: String,
    headers: BTreeMap<String, String>,
    body_encoding: String,
    body: Option<String>,
}
