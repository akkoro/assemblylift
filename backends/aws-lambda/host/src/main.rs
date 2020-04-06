mod runtime;
use runtime::AwsLambdaRuntime;
use assemblylift_core::iomod::*;

use std::io;
use std::io::prelude::*;
use std::fs::{canonicalize, File};
use std::env;
use std::error::Error;
use std::io::ErrorKind;
use std::cell::Cell;
use wasmer_runtime::{Array, Ctx, Instance, WasmPtr, LikeNamespace, Export};
use wasmer_runtime::memory::MemoryView;
use std::ffi::c_void;
use std::str::Utf8Error;
use std::sync::{Mutex, Arc};
use wasmer_runtime_core::backend::SigRegistry;
use wasmer_runtime::types::{FuncSig, Type};
use wasmer_runtime_core::export::{Context, FuncPointer};

type WasmBufferPtr = WasmPtr<u8, Array>;

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
    let memory_writer = event_buffer
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
        let lambda_runtime = &*ctx.data.cast::<AwsLambdaRuntime>();
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

    use awsio;
    awsio::database::Module::register();

    // handler coordinates are expected to be <file name>.<function name>
    let coords =  handler_coordinates.split(".").collect::<Vec<&str>>();
    let file_name = coords[0];
    let handler_name = coords[1];

    // let mut lambda_runtime = AwsLambdaRuntime::new();

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
                        "__asml_abi_invoke" => func!(asml_abi_invoke)
                    },
                };
                import_object.allow_missing_functions = true;

                instantiate(&buffer[..], &import_object)
                    .map_err(to_io_error)
            });

    match get_instance {
        Ok(mut instance) => {
            use wasmer_runtime::func;

            // instance.context_mut().data = &mut lambda_runtime as *mut _ as *mut c_void;

            // let params: Vec<Type> = [].iter().cloned().map(|x: Type| x.into()).collect();
            // let returns: Vec<Type> = [Type::I32].iter().cloned().map(|x| x.into()).collect();
            //
            // let func = func!(awsio::database::aws_dynamodb_list_tables_impl);
            //
            // unsafe {
            //     if let None = instance.maybe_insert("__wsw_list_tables", Export::Function {
            //         func: FuncPointer::new(func.get_vm_func().as_ptr()),
            //         ctx: Context::Internal,
            //         signature: Arc::new(FuncSig::new(params, returns))
            //     }) {
            //         println!("MAYBE_INSERT_ERROR: __wsw_list_tables");
            //     }
            // }

            // loop {
            //     lambda_runtime
            //         .get_next_event()
            //         .and_then(|event| {
            //             write_event_buffer(&instance, event.event_body);
            write_event_buffer(&instance, "{}".to_string());

            // lambda_runtime.current_request_id.replace(event.request_id);

            let value = instance.call(handler_name, &[]);
            value
                .map(|v| println!("EXIT CODE: {:?}", v))
                .map_err(to_io_error);
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
