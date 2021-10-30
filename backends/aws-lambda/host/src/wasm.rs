use std::{env, io};
use std::cell::Cell;
use std::error::Error;
use std::io::ErrorKind;
use std::mem::ManuallyDrop;
use std::sync::{Arc, Mutex};

use wasmer::{Function, imports, Instance, InstantiationError, MemoryView, Store, Cranelift, ChainableNamedResolver};
//use wasmer_engine_native::Native;
use wasmer_engine_universal::Universal;
use wasmer_wasi::WasiState;

use assemblylift_core::abi::{asml_abi_clock_time_get, asml_abi_input_length_get, asml_abi_input_next, asml_abi_input_start, asml_abi_invoke, asml_abi_io_len, asml_abi_io_load, asml_abi_io_next, asml_abi_io_poll, asml_abi_z85_decode, asml_abi_z85_encode};
use assemblylift_core::buffers::FunctionInputBuffer;
use assemblylift_core::threader::{Threader, ThreaderEnv};
use assemblylift_core_iomod::registry::RegistryTx;

pub fn build_instance(tx: RegistryTx) -> Result<(Instance, ThreaderEnv), InstantiationError> {
    // let panic if these aren't set
    let handler_coordinates = env::var("_HANDLER").unwrap();
    let lambda_path = env::var("LAMBDA_TASK_ROOT").unwrap();

    // handler coordinates are expected to be <file name>.<function name>
    let coords = handler_coordinates.split(".").collect::<Vec<&str>>();
    let file_name = coords[0];
    let file_path = format!("{}/{}.wasm.bin", lambda_path, file_name);

    //    let store = Store::new(&Native::headless().engine());
    let compiler = Cranelift::default();
    let store = Store::new(&Universal::new(compiler).engine());
    let module = unsafe { wasmer::Module::deserialize_from_file(&store, file_path.clone()) }
        .expect(&format!("could not load wasm from {}", file_path.clone()));

    let env = ThreaderEnv {
        threader: ManuallyDrop::new(Arc::new(Mutex::new(Threader::new(tx)))),
        memory: Default::default(),
        get_function_input_buffer: Default::default(),
        get_io_buffer: Default::default(),
        host_input_buffer: Arc::new(Mutex::new(FunctionInputBuffer::new())),
    };

    let mut wasi_env = WasiState::new(coords[0])
        .finalize()
        .expect("could not init WASI env");
    let wasi_imports = wasi_env.import_object(&module)
        .expect("could not get WASI import object");

    let asml_imports = imports! {
        "env" => {
            "__asml_abi_console_log" => Function::new_native_with_env(&store, env.clone(), runtime_console_log),
            "__asml_abi_success" => Function::new_native_with_env(&store, env.clone(), runtime_success),
            "__asml_abi_invoke" => Function::new_native_with_env(&store, env.clone(), asml_abi_invoke),
            "__asml_abi_io_poll" => Function::new_native_with_env(&store, env.clone(), asml_abi_io_poll),
            "__asml_abi_io_len" => Function::new_native_with_env(&store, env.clone(), asml_abi_io_len),
            "__asml_abi_io_load" => Function::new_native_with_env(&store, env.clone(), asml_abi_io_load),
            "__asml_abi_io_next" => Function::new_native_with_env(&store, env.clone(), asml_abi_io_next),
            "__asml_abi_clock_time_get" => Function::new_native_with_env(&store, env.clone(), asml_abi_clock_time_get),
            "__asml_abi_input_start" => Function::new_native_with_env(&store, env.clone(), asml_abi_input_start),
            "__asml_abi_input_next" => Function::new_native_with_env(&store, env.clone(), asml_abi_input_next),
            "__asml_abi_input_length_get" => Function::new_native_with_env(&store, env.clone(), asml_abi_input_length_get),
            "__asml_expabi_z85_encode" => Function::new_native_with_env(&store, env.clone(), asml_abi_z85_encode),
            "__asml_expabi_z85_decode" => Function::new_native_with_env(&store, env.clone(), asml_abi_z85_decode),
        },
    };

    let import_object = asml_imports.chain_back(wasi_imports);

    match Instance::new(&module, &import_object) {
        Ok(instance) => Ok((instance, env)),
        Err(err) => Err(err),
    }
}

fn to_io_error<E: Error>(err: E) -> io::Error {
    io::Error::new(ErrorKind::Other, err.to_string())
}

fn runtime_console_log(env: &ThreaderEnv, ptr: u32, len: u32) {
    let string = runtime_ptr_to_string(env, ptr, len).unwrap();
    println!("LOG: {}", string);
}

fn runtime_success(env: &ThreaderEnv, ptr: u32, len: u32) -> Result<(), io::Error> {
    let lambda_runtime = &crate::LAMBDA_RUNTIME;
    let response = runtime_ptr_to_string(env, ptr, len).unwrap();

    let threader = env.threader.clone();

    let respond = lambda_runtime.respond(response.to_string());
    threader.lock().unwrap().spawn(respond);
    Ok(())
}

fn runtime_ptr_to_string(env: &ThreaderEnv, ptr: u32, len: u32) -> Result<String, io::Error> {
    let memory = env.memory_ref().unwrap();
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
