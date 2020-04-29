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

use wasmer_runtime::{Array, Ctx, Export, Instance, LikeNamespace, WasmPtr};
use wasmer_runtime::memory::MemoryView;
use wasmer_runtime::types::{FuncSig, Type};
use wasmer_runtime_core::backend::SigRegistry;
use wasmer_runtime_core::export::{Context, FuncPointer};
use wasmer_runtime_core::Func;
use wasmer_runtime_core::typed_func::Wasm;

use assemblylift_core::{InstanceData, WasmBufferPtr};
use assemblylift_core::iomod::*;
use assemblylift_core_event::executor::Executor;
use assemblylift_core_event::manager::*;
use runtime::AwsLambdaRuntime;
use std::sync::mpsc::sync_channel;

mod runtime;

lazy_static! {
    pub static ref LAMBDA_RUNTIME: Mutex<AwsLambdaRuntime> = Mutex::new(AwsLambdaRuntime::new());
}

fn to_io_error<E: Error>(err: E) -> io::Error {
    io::Error::new(ErrorKind::Other, err.to_string())
}

fn write_event_buffer(instance: &Instance, event: String) {
    use wasmer_runtime::{Func};

    let wasm_instance_context = instance.context();
    let wasm_instance_memory = wasm_instance_context.memory(0);

    let mut get_pointer: Func<(), WasmBufferPtr> = instance
        .func("__al_get_aws_event_string_buffer_pointer")
        .expect("__al_get_aws_event_string_buffer_pointer");

    let event_buffer = get_pointer.call().unwrap();
    let memory_writer: &[Cell<u8>] = event_buffer
        .deref(wasm_instance_memory, 0, event.len() as u32)
        .unwrap();

    for (i, b) in event.bytes().enumerate() {
        memory_writer[i].set(b);
    }
}

fn runtime_ptr_to_string(ctx: &mut Ctx, ptr: u32, len: u32) -> Result<String, io::Error> {
    let memory = ctx.memory(0);
    let view: MemoryView<u8> = memory.view();

    let mut str_vec: Vec<u8> = Vec::new();
    for byte in view[ptr as usize .. (ptr + len) as usize].iter().map(Cell::get) {
        str_vec.push(byte);
    }

    std::str::from_utf8(str_vec.as_slice())
        .map(String::from)
        .map_err(to_io_error)
}

fn runtime_console_log(ctx: &mut Ctx, ptr: u32, len: u32) {
    let string = runtime_ptr_to_string(ctx, ptr, len).unwrap();
    println!("LOG: {}", string);
}

fn runtime_success(ctx: &mut Ctx, ptr: u32, len: u32) -> Result<(), io::Error> {
    unsafe {
        let lambda_runtime = LAMBDA_RUNTIME.lock().unwrap();
        let request_id = lambda_runtime.current_request_id.borrow().clone();
        let response = runtime_ptr_to_string(ctx, ptr, len).unwrap();
        lambda_runtime.respond(request_id, response.to_string())
    }
}

fn main() {
    // let panic if these aren't set
    let handler_coordinates = env::var("_HANDLER").unwrap();
    let lambda_path = env::var("LAMBDA_TASK_ROOT").unwrap();

    println!("Using Lambda root: {}", lambda_path);

    // handler coordinates are expected to be <file name>.<function name>
    let coords =  handler_coordinates.split(".").collect::<Vec<&str>>();
    let file_name = coords[0];
    let handler_name = coords[1];

    let mut get_instance =
        canonicalize(format!("{}/{}.wasm", lambda_path, file_name))
            .and_then(|path| File::open(path))
            .and_then(|mut file: File| {
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)
                    .map(move |_| buffer)
                    .map_err(to_io_error)
            })
            .and_then(|buffer| {
                use wasmer_runtime::{instantiate, func, imports};
                let mut import_object = imports! {
                    "env" => {
                        "__al_console_log" => func!(runtime_console_log),
                        "__al_success" => func!(runtime_success),
                        "__asml_abi_invoke" => func!(asml_abi_invoke),
                    },
                };
                import_object.allow_missing_functions = true;

                instantiate(&buffer[..], &import_object)
                    .map_err(to_io_error)
            });

    match get_instance {
        Ok(mut instance) => {
            use wasmer_runtime::func;

            let mut module_registry = &mut ModuleRegistry::new();
            let mut event_executor = Box::new(Executor::new());

            let (sender, receiver) = sync_channel(10_000);

            unsafe {
                let mut instance_data = &mut InstanceData {
                    instance: unsafe { std::mem::transmute(&instance) },
                    module_registry,
                    event_executor: event_executor.as_mut(),
                    memory_writer: &sender
                };

                instance.context_mut().data = &mut instance_data as *mut _ as *mut c_void;
            }

            awsio::database::MyModule::register(module_registry);

            // loop {
            //     LAMBDA_RUNTIME
            //         .get_next_event()
            //         .and_then(|event| {
            //             write_event_buffer(&instance, event.event_body);
            write_event_buffer(&instance, "{}".to_string());

            let executor_thread = std::thread::spawn(move || {
                event_executor.run()
            });

            let guest_return_value = instance.call(handler_name, &[]);

            // while let.. {}

            executor_thread.join();

            // guest_return_value
            //     .map(|v| println!("GUEST EXIT CODE: {:?}", v))
            //     .map_err(to_io_error);
            // });
            // }
        },
        Err(error) => {
            println!("ERROR: {:?}", error);
        },
        _ => {
            panic!("uh oh")
        }
    }
}
