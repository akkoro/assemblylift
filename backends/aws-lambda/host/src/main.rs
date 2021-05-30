use std::cell::RefCell;
use std::env;
use std::fs;
use std::process;
use std::sync::{Arc, Mutex};

use clap::crate_version;
use once_cell::sync::Lazy;
use tokio::sync::mpsc;

use assemblylift_core::buffers::LinearBuffer;
use assemblylift_core::threader::Threader;
use assemblylift_core_iomod::registry;
use runtime::AwsLambdaRuntime;

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
                process::Command::new(entry.path()).spawn().unwrap();
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

                let instance = instance.clone();
                tokio::task::spawn_local(async move {
                    // handler coordinates are expected to be <file name>.<function name>
                    let handler_coordinates = std::env::var("_HANDLER").unwrap();
                    let coords = handler_coordinates.split(".").collect::<Vec<&str>>();
                    let handler_name = coords[1];

                    Threader::__reset_memory();

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
