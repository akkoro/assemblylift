use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::ops::Deref;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::anyhow;
use hyper::body::HttpBody;
use hyper::header::{HeaderName, HeaderValue};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, warn};
use url::Url;

use assemblylift_core::wasm::{status_channel, StatusRx, StatusTx};

use crate::runner::{RunnerMessage, RunnerTx};
use crate::Status;
use crate::Status::{Exited, Failure, Success};

pub const INSTALL_DIR: Lazy<String> =
    Lazy::new(|| std::env::var("ASML_INSTALL_DIR").unwrap_or("/opt/assemblylift".to_string()));
pub const FUNCTION_BIND_PATHS: Lazy<Option<String>> = 
    Lazy::new(|| std::env::var("ASML_FUNCTION_BIND_PATHS").ok());
pub const FUNCTION_COORDINATES: Lazy<Option<String>> = 
    Lazy::new(|| std::env::var("ASML_FUNCTION_COORDINATES").ok());
pub const FUNCTION_PRECOMPILED: Lazy<Option<String>> = 
    Lazy::new(|| std::env::var("ASML_FUNCTION_PRECOMPILED").ok());
pub const MAX_ALLOWED_REQUEST_SIZE: u64 = 10_485_760;

pub struct Launcher {
    runtime: tokio::runtime::Runtime,
}

impl Launcher {
    pub fn new() -> Self {
        Self {
            runtime: tokio::runtime::Builder::new_current_thread()
                .enable_io()
                .build()
                .unwrap(),
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
    let request_content_length = match req.body().size_hint().upper() {
        Some(v) => v,
        None => MAX_ALLOWED_REQUEST_SIZE + 1,
    };
    let input_bytes = match request_content_length < MAX_ALLOWED_REQUEST_SIZE {
        true => hyper::body::to_bytes(req.into_body()).await.unwrap(),
        false => {
            warn!(
                "function request payload exceeds limit of {} bytes",
                MAX_ALLOWED_REQUEST_SIZE
            );
            hyper::body::to_bytes(Body::from(Vec::<u8>::new()))
                .await
                .unwrap()
        }
    };
    let launcher_req = LauncherRequest {
        method,
        path,
        headers: headers.clone(),
        body_encoding: "base64".into(),
        body: Some(base64::encode(input_bytes.as_ref())),
    };

    let wasm_ext = match FUNCTION_PRECOMPILED.deref() {
        Some(precompiled) => match !precompiled.eq_ignore_ascii_case("false") {
            true => "wasm.bin",
            false => "wasm",
        }
        None => "wasm.bin",
    };

    fn uri_from_coords(coords: &String, ext: &str) -> anyhow::Result<Url> {
        // coordinate is the triple project.service.function
        let coordinates = coords.split('.').collect::<Vec<&str>>();
        if coordinates.len() != 3 {
            return Err(anyhow!("malformed coordinates in header"));
        }
        let project_dir = PathBuf::from(format!(
            "{}/projects/{}",
            INSTALL_DIR.as_str(),
            coordinates[0]
        ));
        match project_dir.exists() {
            true => Url::from_str(
                format!(
                    "file://{}/services/{}/{}.component.{}",
                    project_dir.to_str().unwrap(),
                    coordinates[1],
                    coordinates[2],
                    ext,
                )
                .as_str(),
            )
            .map_err(|e| anyhow!(e)),

            false => match PathBuf::from("./assemblylift.toml").exists() {
                true => Url::from_str(
                    format!(
                        "file://{}/net/services/{}/functions/{}/{}.component.{}",
                        std::env::current_dir().unwrap().to_str().unwrap(),
                        coordinates[1],
                        coordinates[2],
                        coordinates[2],
                        ext,
                    )
                    .as_str(),
                )
                .map_err(|e| anyhow!(e)),
                false => return Err(anyhow!("cannot find function to run; assemblylift is not installed or we are not in a project directory")),
            },
        }
    }

    let wasm_uri: Url = match FUNCTION_COORDINATES.deref() {
        Some(coords) => uri_from_coords(coords, wasm_ext)?,
        None => match headers.get("x-assemblylift-function-coordinates") {
            Some(coords) => uri_from_coords(coords, wasm_ext)?,
            None => Url::from_str(&**headers.get("x-assemblylift-wasm-uri").unwrap())
                .map_err(|e| anyhow!(e))?,
        }
    };

    if !wasm_uri.scheme().eq_ignore_ascii_case("file") {
        unimplemented!("{} scheme not yet supported", wasm_uri.scheme());
    }

    let env_vars: BTreeMap<String, String> = match headers.get("x-assemblylift-function-env-vars") {
        Some(vars) => parse_map(vars),
        None => Default::default(),
    };

    let mut bind_paths: BTreeMap<String, String> = match headers.get("x-assemblylift-function-bind-paths") {
        Some(paths) => parse_map(paths),
        None => Default::default(),
    };
    if let Some(paths) =  FUNCTION_BIND_PATHS.deref() {
        bind_paths.append(&mut parse_map(paths));
    }

    let runtime_environment = headers.get("x-assemblylift-function-runtime-env").cloned();
    let msg = RunnerMessage {
        input: serde_json::to_vec(&launcher_req).unwrap(),
        status_sender: status_tx.clone(),
        wasm_path: PathBuf::from(wasm_uri.path()),
        env_vars,
        bind_paths,
        runtime_environment,
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
            Exited(status) => {
                debug!("exit code {}", status);
                let timer = timer::Timer::new();
                let tx = status_tx.clone();
                // FIXME this doesn't work
                timer.schedule_with_delay(chrono::Duration::seconds(3), move || {
                    tx.send(Status::Failure("No Response".as_bytes().to_vec()))
                        .unwrap();
                });
                continue;
            }
            Success(response) => match serde_json::from_slice::<serde_json::Value>(&response) {
                Ok(json) => match json.get("isBase64Encoded").is_some() {
                    true => {
                        debug!("function response detected as Lambda-APIGW format");
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

fn parse_map(vars: &String) -> BTreeMap<String, String> {
    let mut map = BTreeMap::<String, String>::new();
    let pairs = vars.split(',');
    for pair in pairs {
        let kv = pair.split('=').collect::<Vec<&str>>();
        map.insert(kv[0].into(), kv[1].into());
    }
    map
}

#[derive(Serialize, Deserialize)]
struct LauncherRequest {
    method: String,
    path: String,
    headers: BTreeMap<String, String>,
    body_encoding: String,
    body: Option<String>,
}
