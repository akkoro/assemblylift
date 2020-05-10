#[macro_use]
extern crate lazy_static;

use std::borrow::Borrow;
use std::cell::Cell;
use std::env;
use std::error::Error;
use std::ffi::c_void;
use std::fs::{canonicalize, File};
use std::io;
use std::io::ErrorKind;
use std::io::prelude::*;
use std::str::Utf8Error;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::sync_channel;

use crossbeam_utils::atomic::AtomicCell;
use crossbeam_utils::thread::scope;
use wasmer_runtime::{Array, Ctx, Export, Instance, LikeNamespace, WasmPtr};
use wasmer_runtime::memory::MemoryView;
use wasmer_runtime::types::{FuncSig, Type};
use wasmer_runtime_core::backend::SigRegistry;
use wasmer_runtime_core::export::{Context, FuncPointer};
use wasmer_runtime_core::Func;
use wasmer_runtime_core::typed_func::Wasm;

use assemblylift_core::WasmBufferPtr;
use assemblylift_core::iomod::*;
use assemblylift_core_event::constants::EVENT_BUFFER_SIZE_BYTES;
use assemblylift_core_event::threader::Threader;
use runtime::AwsLambdaRuntime;

mod runtime;
mod wasm;

lazy_static! {
    pub static ref LAMBDA_RUNTIME: Mutex<AwsLambdaRuntime> = Mutex::new(AwsLambdaRuntime::new());
}

fn write_event_buffer(instance: &Instance, event: String) {
    use wasmer_runtime::{Func};

    let wasm_instance_context = instance.context();
    let wasm_instance_memory = wasm_instance_context.memory(0);

    let mut get_pointer: Func<(), WasmBufferPtr> = instance
        .func("__al_get_aws_event_string_buffer_pointer")
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
    // let panic if these aren't set
    let handler_coordinates = env::var("_HANDLER").unwrap();
    let lambda_path = env::var("LAMBDA_TASK_ROOT").unwrap();

    println!("Using Lambda root: {}", lambda_path);

    // handler coordinates are expected to be <file name>.<function name>
    let coords =  handler_coordinates.split(".").collect::<Vec<&str>>();
    let handler_name = coords[1];

    if let Ok(mut instance) = wasm::build_instance() {
        // init modules -- these will eventually be plugins
        awsio::database::MyModule::register(&mut MODULE_REGISTRY.lock().unwrap());

        // loop {
        //     LAMBDA_RUNTIME
        //         .lock().unwrap()
        //         .get_next_event()
        //         .and_then(|event| {
        scope(|s| {
            s.spawn(|_| {
                let locked = instance.lock().unwrap();

                write_event_buffer(&locked, "{}".to_string() /* event.event_body */);
                locked.call(handler_name, &[]);
                println!("TRACE: handler returned");
            });
        });

        // all threads spawned in the scope join here automatically
        // the side-effect of which is that a hang in the handler will block the lambda
        // runtime loop

        // Ok(())
        // });
        // }
    }
}
