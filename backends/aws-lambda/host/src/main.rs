#[macro_use]
extern crate lazy_static;

use std::cell::RefCell;
use std::env;
use std::fs;
use std::sync::Mutex;

use clap::crate_version;
use crossbeam_utils::atomic::AtomicCell;
use crossbeam_utils::thread::scope;
use once_cell::sync::Lazy;
use wasmer_runtime::Instance;

use assemblylift_core::threader::Threader;
use assemblylift_core::WasmBufferPtr;
use assemblylift_core_iomod::{plugin, MODULE_REGISTRY};
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
        .get("__asml_guest_get_aws_event_string_buffer_pointer")
        .expect(
            "could not find export in wasm named __asml_guest_get_aws_event_string_buffer_pointer",
        );

    let event_buffer = get_pointer.call().unwrap();
    let memory_writer: &[AtomicCell<u8>] = event_buffer
        .deref(wasm_instance_memory, 0, event.len() as u32)
        .unwrap();

    for (i, b) in event.bytes().enumerate() {
        memory_writer[i].store(b);
    }
}

fn main() {
    println!(
        "Starting AssemblyLift AWS Lambda runtime {}",
        crate_version!()
    );

    // let panic if these aren't set
    let handler_coordinates = env::var("_HANDLER").unwrap();
    let lambda_path = env::var("LAMBDA_TASK_ROOT").unwrap();
    // let runtime_dir = "/opt/iomod".to_string();

    println!("Using Lambda root: {}", lambda_path);

    // handler coordinates are expected to be <file name>.<function name>
    let coords = handler_coordinates.split(".").collect::<Vec<&str>>();
    let handler_name = coords[1];

    // load plugins from runtime dir, which should contain merged contents of Lambda layers
    // for entry in fs::read_dir(runtime_dir).unwrap() {
    //     let entry = entry.unwrap();
    //     println!("Found entry: {}", entry.path().display());
    //     if entry.file_type().unwrap().is_file()
    //         && entry.file_name().into_string().unwrap().contains(".so")
    //     {
    //         plugin::load(&mut MODULE_REGISTRY, entry.path()).unwrap();
    //     }
    // }

    println!("TRACE: building Wasmer instance");
    let instance = match wasm::build_instance() {
        Ok(instance) => instance,
        Err(why) => panic!("PANIC {}", why.to_string()),
    };

    println!("TRACE: starting main Lambda runtime loop");
    while let Ok(event) = LAMBDA_RUNTIME.get_next_event() {
        println!("DEBUG: got Lambda event {}", event.request_id.clone());

        let ref_cell = LAMBDA_REQUEST_ID.lock().unwrap();
        ref_cell.replace(event.request_id.clone());
        std::mem::drop(ref_cell);

        scope(|s| {
            s.spawn(|_| {
                let locked = instance.lock().unwrap();
                write_event_buffer(&locked, event.event_body);
                Threader::__reset_memory();

                println!("DEBUG: calling handler {}", handler_name);
                match locked.call(handler_name, &[]) {
                    Ok(_result) => println!("TRACE: handler returned Ok()"),
                    Err(error) => println!("ERROR: {}", error.to_string()),
                }
            });

            s.spawn(|_| {
                let tokio_local = tokio::task::LocalSet::new();
                tokio_local.spawn_local(async move {
                    let registry = &MODULE_REGISTRY;
                    registry.start_service()
                })
            });
        })
        .unwrap();

        // all threads spawned in the scope join here automatically
        // the side-effect of which is that a hang in the handler will block the lambda
        // runtime loop
    }
}
