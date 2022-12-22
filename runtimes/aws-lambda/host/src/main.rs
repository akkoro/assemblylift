use std::cell::RefCell;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process;
use std::sync::{Arc, Mutex};

use clap::crate_version;
use crossbeam_channel::bounded;
use once_cell::sync::Lazy;
use tokio::sync::mpsc;
use zip;

use assemblylift_core::wasm::Wasmtime;
use assemblylift_core_iomod::{package::IomodManifest, registry};
use runtime::AwsLambdaRuntime;

use crate::abi::LambdaAbi;

mod abi;
mod runtime;

pub static LAMBDA_RUNTIME: Lazy<AwsLambdaRuntime> = Lazy::new(|| AwsLambdaRuntime::new());
pub static LAMBDA_REQUEST_ID: Lazy<Mutex<RefCell<String>>> =
    Lazy::new(|| Mutex::new(RefCell::new(String::new())));

#[tokio::main]
async fn main() {
    println!(
        "Starting AssemblyLift AWS Lambda runtime v{}",
        crate_version!()
    );

    let registry_channel = mpsc::channel(32);
    let tx = registry_channel.0.clone();
    let rx = registry_channel.1;
    registry::spawn_registry(rx).unwrap();

    // load plugins from runtime dir, which should contain merged contents of Lambda layers
    if let Ok(rd) = fs::read_dir("/opt") {
        for entry in rd {
            let entry = entry.unwrap();
            println!("DEBUG entry={:?}", entry);
            if entry.file_type().unwrap().is_file() {
                // FIXME this makes the assumption that the
                //       IOmod entrypoint is always an executable binary
                if let Some(os_path) = entry.path().extension() {
                    if let Some(ext) = os_path.to_str() {
                        if ext == "iomod" {
                            let file = File::open(&entry.path()).unwrap();
                            let reader = BufReader::new(file);
                            let archive = RefCell::new(zip::ZipArchive::new(reader).unwrap());
                            let mut manifest_str: String = Default::default();
                            {
                                let mut archive = archive.borrow_mut();
                                let mut manifest = archive
                                    .by_name("./iomod.toml")
                                    .expect("could not find IOmod manifest");
                                manifest
                                    .read_to_string(&mut manifest_str)
                                    .expect("could not read iomod.toml");
                            }
                            {
                                let mut archive = archive.borrow_mut();
                                let iomod_manifest = IomodManifest::from(manifest_str);
                                let entrypoint = format!("./{}", iomod_manifest.process.entrypoint);
                                let mut entrypoint_binary = archive
                                    .by_name(&*entrypoint)
                                    .expect("could not find entrypoint in package");
                                let path = &*format!(
                                    "/tmp/iomod/{}@{}/{}",
                                    iomod_manifest.iomod.coordinates,
                                    iomod_manifest.iomod.version,
                                    entrypoint
                                );
                                let path = Path::new(path);
                                if !path.exists() {
                                    {
                                        let path_prefix = path.parent().unwrap();
                                        fs::create_dir_all(path_prefix).expect(&*format!(
                                            "unable to create directory {:?}",
                                            path_prefix
                                        ));
                                        let mut entrypoint_file = File::create(path)
                                            .expect(&*format!("unable to create file at {:?}", path));
                                        std::io::copy(&mut entrypoint_binary, &mut entrypoint_file)
                                            .expect("unable to copy entrypoint");
                                        let mut perms: fs::Permissions =
                                            fs::metadata(&path).unwrap().permissions();
                                        perms.set_mode(0o755);
                                        entrypoint_file.set_permissions(perms)
                                            .expect("could not set IOmod binary executable (octal 755) permissions");
                                    }
                                    process::Command::new(path).spawn().unwrap();
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        println!("WARN Could not find dir /opt/iomod");
    }

    let module_path = env::var("LAMBDA_TASK_ROOT").unwrap();
    let handler_name = env::var("_HANDLER").unwrap();

    // Mapped to /tmp inside the WASM module
    fs::create_dir_all("/tmp/asmltmp").expect("could not create /tmp/asmltmp");

    if let Ok("ruby-lambda") = env::var("ASML_FUNCTION_ENV").as_deref() {
        let rubysrc_path = "/tmp/rubysrc";
        if !Path::new(&rubysrc_path).exists() {
            fs::create_dir_all(rubysrc_path)
                .expect(&*format!("unable to create directory {:?}", rubysrc_path));
        }
        let rubyusr_path = "/tmp/rubyusr";
        if !Path::new(&rubyusr_path).exists() {
            fs::create_dir_all(rubyusr_path)
                .expect(&*format!("unable to create directory {:?}", rubyusr_path));
        }

        fn copy_entries(dir: &PathBuf, to: &PathBuf) {
            for entry in fs::read_dir(dir).unwrap() {
                let entry = entry.unwrap();
                if entry.file_type().unwrap().is_file() {
                    let copy_to = format!(
                        "{}/{}",
                        to.to_str().unwrap(),
                        entry.file_name().to_str().unwrap()
                    );
                    fs::copy(entry.path(), copy_to).unwrap();
                } else if entry.file_type().unwrap().is_dir() {
                    let mut copy_to = PathBuf::from(to);
                    copy_to.push(entry.path().iter().last().unwrap());
                    fs::create_dir_all(&copy_to).unwrap();
                    copy_entries(&entry.path(), &copy_to);
                }
            }
        }
        copy_entries(
            &PathBuf::from(format!("{}/rubysrc", &module_path)),
            &PathBuf::from(rubysrc_path),
        );
        copy_entries(
            &PathBuf::from("/opt/ruby-wasm32-wasi/usr"),
            &PathBuf::from(rubyusr_path),
        );
    }

    let (status_sender, _status_receiver) = bounded::<()>(1);

    tokio::task::LocalSet::new().run_until(async move {
        let mut full_path = PathBuf::from(&module_path);
        full_path.push(&handler_name);
        let wasmtime = Arc::new(Mutex::new(
            Wasmtime::<LambdaAbi, ()>::new_from_path(Path::new(full_path.as_path()))
                .expect("could not create WASM runtime from module path")
        ));

        while let Ok(event) = LAMBDA_RUNTIME.get_next_event().await {
            {
                let ref_cell = LAMBDA_REQUEST_ID.lock().unwrap();
                if ref_cell.borrow().clone() == event.request_id.clone() {
                    continue;
                }
                ref_cell.replace(event.request_id.clone());
            }

            let (instance, mut store) = wasmtime
                .lock()
                .unwrap()
                .link_module(tx.clone(), status_sender.clone())
                .expect("could not link wasm module");

            wasmtime
                .lock()
                .unwrap()
                .initialize_function_input_buffer(&mut store, &event.event_body.into_bytes())
                .expect("could not initialize input buffer");

            let wasmtime = wasmtime.clone();
            tokio::task::spawn_local(async move {
                // env.clone().threader.lock().unwrap().__reset_memory();

                match wasmtime.lock().unwrap().start(&mut store, instance) {
                    Ok(result) => println!("SUCCESS: handler returned {:?}", result),
                    Err(error) => println!("ERROR: {}", error.to_string()),
                }
            })
            .await
            .unwrap();
            // std::mem::drop(env.clone().threader);
        }
    })
    .await;
}
