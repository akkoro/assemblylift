#[macro_use]
extern crate lazy_static;

use std::cell::RefCell;
use std::env;
use std::sync::Mutex;

use clap::crate_version;
use crossbeam_utils::atomic::AtomicCell;
use crossbeam_utils::thread::scope;
use once_cell::sync::Lazy;
use wasmer_runtime::Instance;

use assemblylift_core::iomod::*;
use assemblylift_core::WasmBufferPtr;
use runtime::AwsLambdaRuntime;

mod runtime;
mod wasm;

lazy_static! {
    pub static ref LAMBDA_RUNTIME: AwsLambdaRuntime = AwsLambdaRuntime::new();
}

pub static LAMBDA_REQUEST_ID: Lazy<Mutex<RefCell<String>>> =
    Lazy::new(|| Mutex::new(RefCell::new(String::new())));

fn write_event_buffer(instance: &Instance, event: String) {
    use wasmer_runtime::Func;

    let wasm_instance_context = instance.context();
    let wasm_instance_memory = wasm_instance_context.memory(0);

    let get_pointer: Func<(), WasmBufferPtr> = instance
        .exports
        .get("__al_get_aws_event_string_buffer_pointer")
        .expect("__al_get_aws_event_string_buffer_pointer");

    let event_buffer = get_pointer.call().unwrap();
    let memory_writer: &[AtomicCell<u8>] = event_buffer
        .deref(wasm_instance_memory, 0, event.len() as u32)
        .unwrap();

    for (i, b) in event.bytes().enumerate() {
        memory_writer[i].store(b);
    }
}

fn main() {
    println!("Starting AssemblyLift AWS Lambda runtime {}", crate_version!());

    // let panic if these aren't set
    let handler_coordinates = env::var("_HANDLER").unwrap();
    let lambda_path = env::var("LAMBDA_TASK_ROOT").unwrap();

    println!("Using Lambda root: {}", lambda_path);

    // handler coordinates are expected to be <file name>.<function name>
    let coords = handler_coordinates.split(".").collect::<Vec<&str>>();
    let handler_name = coords[1];

    if let Ok(instance) = wasm::build_instance() {
        // init modules -- these will eventually be plugins specified in a manifest of some kind
        awsio::database::MyModule::register(&mut MODULE_REGISTRY.lock().unwrap());

        while let Ok(event) = LAMBDA_RUNTIME.get_next_event() {
            let ref_cell = LAMBDA_REQUEST_ID.lock().unwrap();
            ref_cell.replace(event.request_id.clone());
            std::mem::drop(ref_cell);

            scope(|s| {
                s.spawn(|_| {
                    let locked = instance.lock().unwrap();

                    write_event_buffer(&locked, event.event_body);

                    match locked.call(handler_name, &[]) {
                        Ok(_result) => println!("TRACE: handler returned Ok()"),
                        Err(error) => println!("ERROR: {}", error.to_string()),
                    }
                });
            });

            // all threads spawned in the scope join here automatically
            // the side-effect of which is that a hang in the handler will block the lambda
            // runtime loop
        }
    }
}
