use std::{env, io};
use std::cell::Cell;
use std::error::Error;
use std::ffi::c_void;
use std::fs::canonicalize;
use std::io::ErrorKind;

use wasmer_runtime::memory::MemoryView;
use wasmer_runtime_core::Instance;
use wasmer_runtime_core::vm::Ctx;

use assemblylift_core::abi::{asml_abi_event_len, asml_abi_event_ptr, asml_abi_invoke, asml_abi_poll, get_threader};
use assemblylift_core::threader::Threader;
use assemblylift_core_iomod::registry::RegistryTx;

pub fn build_instance(tx: RegistryTx) -> Result<Instance, io::Error> {
    // let panic if these aren't set
    let handler_coordinates = env::var("_HANDLER").unwrap();
    let lambda_path = env::var("LAMBDA_TASK_ROOT").unwrap();

    // handler coordinates are expected to be <file name>.<function name>
    let coords = handler_coordinates.split(".").collect::<Vec<&str>>();
    let file_name = coords[0];
    let file_path = format!("{}/{}.wasm", lambda_path, file_name);

    let get_instance = canonicalize(file_path)
        .and_then(|path| std::fs::read(path))
        .and_then(|buffer| {
            use wasmer_runtime::{func, imports, instantiate};
            let import_object = imports! {
                "env" => {
                    "__asml_abi_console_log" => func!(runtime_console_log),
                    "__asml_abi_success" => func!(runtime_success),
                    "__asml_abi_invoke" => func!(asml_abi_invoke),
                    "__asml_abi_poll" => func!(asml_abi_poll),
                    "__asml_abi_event_ptr" => func!(asml_abi_event_ptr),
                    "__asml_abi_event_len" => func!(asml_abi_event_len),
                },
            };

            instantiate(&buffer[..], &import_object).map_err(to_io_error)
        });

    match get_instance {
        Ok(mut instance) => {
            let threader = Box::into_raw(Box::from(Threader::new(tx)));
            instance.context_mut().data = threader as *mut _ as *mut c_void;

            Ok(instance)
        }
        Err(error) => Err(to_io_error(error)),
    }
}

fn to_io_error<E: Error>(err: E) -> io::Error {
    io::Error::new(ErrorKind::Other, err.to_string())
}

fn runtime_console_log(ctx: &mut Ctx, ptr: u32, len: u32) {
    let string = runtime_ptr_to_string(ctx, ptr, len).unwrap();
    println!("LOG: {}", string);
}

fn runtime_success(ctx: &mut Ctx, ptr: u32, len: u32) -> Result<(), io::Error> {
    let lambda_runtime = &crate::LAMBDA_RUNTIME;
    let response = runtime_ptr_to_string(ctx, ptr, len).unwrap();

    let threader: *mut Threader = get_threader(ctx);
    let threader = unsafe { threader.as_mut().unwrap() };

    let respond = lambda_runtime.respond(response.to_string());
    threader.spawn(respond);
    Ok(())
}

fn runtime_ptr_to_string(ctx: &mut Ctx, ptr: u32, len: u32) -> Result<String, io::Error> {
    let memory = ctx.memory(0);
    let view: MemoryView<u8> = memory.view();

    let mut str_vec: Vec<u8> = Vec::new();
    for byte in view[ptr as usize..(ptr + len) as usize]
        .iter()
        .map(Cell::get)
    {
        str_vec.push(byte);
    }

    std::str::from_utf8(str_vec.as_slice())
        .map(String::from)
        .map_err(to_io_error)
}
