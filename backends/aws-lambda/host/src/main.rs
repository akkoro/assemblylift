use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::env;
use std::fs;
use std::process;
use std::sync::{Arc, Mutex};

use clap::crate_version;
use once_cell::sync::Lazy;
use tokio::sync::mpsc;
use zip;

use assemblylift_core::buffers::LinearBuffer;
use assemblylift_core_iomod::{package::IomodManifest, registry};
use runtime::AwsLambdaRuntime;
use std::io::{BufReader, Read, Write};
use std::ffi::OsStr;
use std::fs::File;

mod runtime;
mod wasm;

pub static LAMBDA_RUNTIME: Lazy<AwsLambdaRuntime> = Lazy::new(|| AwsLambdaRuntime::new());
pub static LAMBDA_REQUEST_ID: Lazy<Mutex<RefCell<String>>> =
    Lazy::new(|| Mutex::new(RefCell::new(String::new())));

#[tokio::main]
async fn main() {
    println!(
        "Starting AssemblyLift AWS Lambda runtime {}",
        crate_version!()
    );

    let registry_channel = mpsc::channel(100);
    let tx = registry_channel.0.clone();
    let rx = registry_channel.1;
    registry::spawn_registry(rx).unwrap();

    // load plugins from runtime dir, which should contain merged contents of Lambda layers
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
                                let mut archive = RefCell::new(zip::ZipArchive::new(reader).unwrap());
                                let mut manifest_str: String = Default::default();
                                {
                                    let mut archive = archive.borrow_mut();
                                    for name in archive.file_names() {
                                        println!("DEBUG file_name={}", name);
                                    }
                                    let mut manifest = archive.by_name("iomod.toml")
                                        .expect("could not find IOmod manifest");
                                    manifest.read_to_string(&mut manifest_str);
                                }
                                {
                                    let mut archive = archive.borrow_mut();
                                    let iomod_manifest = IomodManifest::from(manifest_str);
                                    let entrypoint = iomod_manifest.process.entrypoint;
                                    let mut entrypoint_binary = archive.by_name(&*entrypoint)
                                        .expect("could not find entrypoint in package");
                                    let path = &*format!(
                                        "/opt/iomod/{}@{}/{}",
                                        iomod_manifest.iomod.coordinates,
                                        iomod_manifest.iomod.version,
                                        entrypoint
                                    );
                                    let path = std::path::Path::new(path);
                                    let path_prefix = path.parent().unwrap();
                                    fs::create_dir_all(path_prefix)
                                        .expect(&*format!("unable to create directory {:?}", path_prefix));
                                    let mut entrypoint_file = File::create(path)
                                        .expect(&*format!("unable to create file at {:?}", path));
                                    std::io::copy(&mut entrypoint_binary, &mut entrypoint_file)
                                        .expect("unable to copy entrypoint");
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

    let task_set = tokio::task::LocalSet::new();
    task_set
        .run_until(async move {
            let (instance, env) = match wasm::build_instance(tx) {
                Ok(instance) => (Arc::new(instance.0), instance.1),
                Err(why) => panic!("PANIC {}", why.to_string()),
            };

            while let Ok(event) = LAMBDA_RUNTIME.get_next_event().await {
                {
                    let ref_cell = LAMBDA_REQUEST_ID.lock().unwrap();
                    if ref_cell.borrow().clone() == event.request_id.clone() {
                        continue;
                    }
                    ref_cell.replace(event.request_id.clone());
                }

                env.clone().host_input_buffer.clone().lock().unwrap().initialize(event.event_body.into_bytes());

                let thread_env = env.clone();
                let instance = instance.clone();
                tokio::task::spawn_local(async move {
                    // handler coordinates are expected to be <file name>.<function name>
                    let handler_coordinates = std::env::var("_HANDLER").unwrap();
                    let coords = handler_coordinates.split(".").collect::<Vec<&str>>();
                    let handler_name = coords[1];

                    thread_env.threader.lock().unwrap().__reset_memory();

                    let handler_call = instance.exports.get_function(handler_name).unwrap();
                    match handler_call.call(&[]) {
                        Ok(result) => println!("SUCCESS: handler returned {:?}", result),
                        Err(error) => println!("ERROR: {}", error.to_string()),
                    }
                })
                .await
                .unwrap();
            }
            
            std::mem::drop(env.clone().threader);
        })
        .await;
}
