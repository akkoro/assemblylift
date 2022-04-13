use std::cell::RefCell;
use std::convert::Infallible;
use std::{fs, process};
use std::fs::File;
use std::io::{BufReader, Read};
use std::net::SocketAddr;
use std::os::unix::fs::PermissionsExt;
use std::sync::Arc;

use clap::crate_version;
use hyper::{Body, Method, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use tokio::sync::mpsc;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use assemblylift_core::wasm;
use assemblylift_core_iomod::package::IomodManifest;
use assemblylift_core_iomod::registry;

use crate::abi::GenericDockerAbi;
use crate::runner::{RunnerMessage, RunnerTx, spawn_runner};
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
    if req.method() != Method::POST {
        return Ok(Response::builder()
            .status(500)
            .body(Body::default())
            .unwrap());
    }

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
pub async fn main() {
    println!("Starting AssemblyLift generic runtime {}", crate_version!());

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");


    if let Ok(rd) = fs::read_dir("/opt/iomod") {
        for entry in rd {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                // this makes the assumption that the
                // IOmod entrypoint is always an executable binary
                match entry.path().extension() {
                    Some(os_str) => {
                        match os_str.to_str() {
                            Some("iomod") => {
                                let file = fs::File::open(&entry.path()).unwrap();
                                let reader = BufReader::new(file);
                                let archive = RefCell::new(zip::ZipArchive::new(reader).unwrap());
                                let mut manifest_str: String = Default::default();
                                {
                                    let mut archive = archive.borrow_mut();
                                    let mut manifest = archive.by_name("./iomod.toml")
                                        .expect("could not find IOmod manifest");
                                    manifest.read_to_string(&mut manifest_str)
                                        .expect("could not read iomod.toml");
                                }
                                {
                                    let mut archive = archive.borrow_mut();
                                    let iomod_manifest = IomodManifest::from(manifest_str);
                                    let entrypoint = format!("./{}", iomod_manifest.process.entrypoint);
                                    let mut entrypoint_binary = archive.by_name(&*entrypoint)
                                        .expect("could not find entrypoint in package");
                                    let path = &*format!(
                                        "/tmp/iomod/{}@{}/{}",
                                        iomod_manifest.iomod.coordinates,
                                        iomod_manifest.iomod.version,
                                        entrypoint
                                    );
                                    let path = std::path::Path::new(path);
                                    {
                                        let path_prefix = path.parent().unwrap();
                                        fs::create_dir_all(path_prefix)
                                            .expect(&*format!("unable to create directory {:?}", path_prefix));
                                        let mut entrypoint_file = File::create(path)
                                            .expect(&*format!("unable to create file at {:?}", path));
                                        std::io::copy(&mut entrypoint_binary, &mut entrypoint_file)
                                            .expect("unable to copy entrypoint");
                                        let mut perms: std::fs::Permissions = fs::metadata(&path).unwrap().permissions();
                                        perms.set_mode(0o755);
                                        entrypoint_file.set_permissions(perms)
                                            .expect("could not set IOmod binary executable (octal 755) permissions");
                                    }
                                    process::Command::new(path).spawn().unwrap();
                                }
                            }
                            _ => {}
                        }
                    }
                    None => {
                        process::Command::new(entry.path()).spawn().unwrap();
                    }
                }
            }
        }
    }


    let addr = SocketAddr::from(([0, 0, 0, 0], 5543));

    let make_svc = make_service_fn(|_conn| async {
        let (runner_tx, runner_rx) = mpsc::channel(32);
        let (status_tx, status_rx) = mpsc::channel::<Status>(32);
        let (registry_tx, registry_rx) = mpsc::channel(32);
        registry::spawn_registry(registry_rx).unwrap();

        let (module, resolver, threader_env) =
            match wasm::build_module_from_path::<GenericDockerAbi, Status>(
                registry_tx,
                status_tx.clone(),
                "/opt/assemblylift", // TODO get from env
                "handler",           // TODO get from env
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
        Ok::<_, Infallible>(service_fn(move |req| {
            launcher(req, runner_tx.clone(), rx.take().unwrap())
        }))
    });

    if let Err(e) = Server::bind(&addr).serve(make_svc).await {
        eprintln!("server error: {}", e);
    }
}
