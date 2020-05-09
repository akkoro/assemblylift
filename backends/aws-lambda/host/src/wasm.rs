use std::{env, io};
use std::cell::Cell;
use std::error::Error;
use std::fs::{canonicalize, File};
use std::io::{ErrorKind, Read};

use wasmer_runtime::memory::MemoryView;
use wasmer_runtime_core::Instance;
use wasmer_runtime_core::vm::Ctx;

use assemblylift_core::{InstanceData, WasmBufferPtr};
use assemblylift_core::iomod::*;
use assemblylift_core_event::threader::Threader;
use std::sync::{Mutex, Arc};
use std::ffi::c_void;
use assemblylift_core_event::constants::EVENT_BUFFER_SIZE_BYTES;
use crossbeam_utils::atomic::AtomicCell;
use wasmer_runtime::Func;

pub fn build_instance() -> Result<Mutex<Box<Instance>>, io::Error> {
    // let panic if these aren't set
    let handler_coordinates = env::var("_HANDLER").unwrap();
    let lambda_path = env::var("LAMBDA_TASK_ROOT").unwrap();

    // handler coordinates are expected to be <file name>.<function name>
    let coords =  handler_coordinates.split(".").collect::<Vec<&str>>();
    let file_name = coords[0];

    let get_instance = canonicalize(format!("{}/{}.wasm", lambda_path, file_name))
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
                    "__asml_abi_console_log" => func!(runtime_console_log),
                    "__asml_abi_success" => func!(runtime_success),
                    "__asml_abi_invoke" => func!(asml_abi_invoke),
                    "__asml_abi_poll" => func!(asml_abi_poll),
                    "__asml_abi_event_ptr" => func!(asml_abi_event_ptr),
                    "__asml_abi_event_len" => func!(asml_abi_event_len),
                },
            };

            instantiate(&buffer[..], &import_object)
                .map_err(to_io_error)
        });

    match get_instance {
        Ok(mut instance) => {
            let mut threader = &mut Threader::new();

            let mut boxed_instance = Box::new(instance);

            println!("TRACE: building wasm memory writer");
            let mut __asml_get_event_buffer_pointer_func: Func<(), WasmBufferPtr> = boxed_instance.exports
                .get("__asml_get_event_buffer_pointer")
                .expect("__asml_get_event_buffer_pointer");

            let ctx = boxed_instance.context();
            let wasm_instance_memory = ctx.memory(0);
            let event_buffer = __asml_get_event_buffer_pointer_func.call().unwrap();
            let memory_writer: &[AtomicCell<u8>] = event_buffer
                .deref(wasm_instance_memory, 0, EVENT_BUFFER_SIZE_BYTES as u32)
                .unwrap();

            unsafe {
                let mut instance_data = &mut InstanceData { threader, memory_writer: &memory_writer[0] as *const AtomicCell<u8> };
                boxed_instance.context_mut().data = &mut instance_data as *mut _ as *mut c_void;
            }

            let guarded_instance = Mutex::new(boxed_instance);

            Ok(guarded_instance)
        },
        Err(error) => Err(to_io_error(error))
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
    unsafe {
        let lambda_runtime = crate::LAMBDA_RUNTIME.lock().unwrap();
        let request_id = lambda_runtime.current_request_id.borrow().clone();
        let response = runtime_ptr_to_string(ctx, ptr, len).unwrap();
        lambda_runtime.respond(request_id, response.to_string())
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
