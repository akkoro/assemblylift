use std::cell::Cell;
use std::error::Error;
use std::io;
use std::io::ErrorKind;
use std::time::{SystemTime, UNIX_EPOCH};

use wasmer::{Array, MemoryView, WasmPtr};

use crate::buffers::{LinearBuffer, PagedWasmBuffer};
use crate::threader::ThreaderEnv;
use crate::{invoke_io, WasmBufferPtr};

pub type AsmlAbiFn<S> = fn(&ThreaderEnv<S>, WasmBufferPtr, WasmBufferPtr, u32) -> i32;

pub trait RuntimeAbi<S: Clone + Send + Sized + 'static> {
    fn log(env: &ThreaderEnv<S>, ptr: u32, len: u32);
    fn success(env: &ThreaderEnv<S>, ptr: u32, len: u32);
}

fn to_io_error<E: Error>(err: E) -> io::Error {
    io::Error::new(ErrorKind::Other, err.to_string())
}

pub fn asml_abi_io_invoke<S>(
    env: &ThreaderEnv<S>,
    name_ptr: u32,
    name_len: u32,
    input: u32,
    input_len: u32,
) -> i32
where
    S: Clone + Send + Sized + 'static,
{
    if let Ok(method_path) = env_ptr_to_string(env, name_ptr, name_len) {
        if let Ok(input) = env_ptr_to_bytes(env, input, input_len) {
            return invoke_io(env, &*method_path, input);
        }
    }

    -1i32 // error
}

pub fn asml_abi_io_poll<S>(env: &ThreaderEnv<S>, id: u32) -> i32
where
    S: Clone + Send + Sized + 'static,
{
    env.threader.clone().lock().unwrap().poll(id) as i32
}

pub fn asml_abi_io_len<S>(env: &ThreaderEnv<S>, id: u32) -> u32
where
    S: Clone + Send + Sized + 'static,
{
    env.threader
        .clone()
        .lock()
        .unwrap()
        .get_io_memory_document(id)
        .unwrap()
        .length as u32
}

pub fn asml_abi_io_load<S>(env: &ThreaderEnv<S>, id: u32) -> i32
where
    S: Clone + Send + Sized + 'static,
{
    match env.threader.lock().unwrap().document_load(env, id) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

pub fn asml_abi_io_next<S>(env: &ThreaderEnv<S>) -> i32
where
    S: Clone + Send + Sized + 'static,
{
    match env.threader.lock().unwrap().document_next(env) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

pub fn asml_abi_clock_time_get<S>(_env: &ThreaderEnv<S>) -> u64
where
    S: Clone + Send + Sized + 'static,
{
    let start = SystemTime::now();
    let unix_time = start.duration_since(UNIX_EPOCH).expect("time is broken");
    unix_time.as_secs() * 1000u64
}

pub fn asml_abi_input_start<S>(env: &ThreaderEnv<S>) -> i32
where
    S: Clone + Send + Sized + 'static,
{
    env.host_input_buffer
        .clone()
        .lock()
        .unwrap()
        .first(env, None)
}

pub fn asml_abi_input_next<S>(env: &ThreaderEnv<S>) -> i32
where
    S: Clone + Send + Sized + 'static,
{
    env.host_input_buffer.clone().lock().unwrap().next(env)
}

pub fn asml_abi_input_length_get<S>(env: &ThreaderEnv<S>) -> u64
where
    S: Clone + Send + Sized + 'static,
{
    env.host_input_buffer.clone().lock().unwrap().len() as u64
}

pub fn asml_abi_z85_encode<S>(
    env: &ThreaderEnv<S>,
    ptr: u32,
    len: u32,
    out_ptr: WasmPtr<u8, Array>,
) -> i32
where
    S: Clone + Send + Sized + 'static,
{
    if let Ok(input) = env_ptr_to_bytes(env, ptr, len) {
        let output = z85::encode(input);
        return match write_bytes_to_ptr(env, output.into_bytes(), out_ptr) {
            Ok(_) => 0i32,
            Err(_) => -1i32,
        };
    }
    -1i32
}

pub fn asml_abi_z85_decode<S>(
    env: &ThreaderEnv<S>,
    ptr: u32,
    len: u32,
    out_ptr: WasmPtr<u8, Array>,
) -> i32
where
    S: Clone + Send + Sized + 'static,
{
    if let Ok(input) = env_ptr_to_bytes(env, ptr, len) {
        if let Ok(output) = z85::decode(input) {
            return match write_bytes_to_ptr(env, output, out_ptr) {
                Ok(_) => 0i32,
                Err(_) => -1i32,
            };
        }
    }
    -1i32
}

fn env_ptr_to_string<S>(env: &ThreaderEnv<S>, ptr: u32, len: u32) -> Result<String, io::Error>
where
    S: Clone + Send + Sized + 'static,
{
    let mem = env.memory_ref().unwrap();
    let view: MemoryView<u8> = mem.view();

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

fn write_bytes_to_ptr<S>(
    env: &ThreaderEnv<S>,
    s: Vec<u8>,
    ptr: WasmPtr<u8, Array>,
) -> Result<(), io::Error>
where
    S: Clone + Send + Sized + 'static,
{
    let mem = env.memory_ref().unwrap();
    let memory_writer = ptr
        .deref(&mem, 0u32, s.len() as u32)
        .expect("could not deref wasm memory");
    for (i, b) in s.iter().enumerate() {
        memory_writer[i].set(*b);
    }
    Ok(())
}

fn env_ptr_to_bytes<S>(env: &ThreaderEnv<S>, ptr: u32, len: u32) -> Result<Vec<u8>, io::Error>
where
    S: Clone + Send + Sized + 'static,
{
    let mem = env.memory_ref().unwrap();
    let view: MemoryView<u8> = mem.view();

    let mut bytes: Vec<u8> = Vec::new();
    for byte in view[ptr as usize..(ptr + len) as usize]
        .iter()
        .map(Cell::get)
    {
        bytes.push(byte);
    }

    Ok(bytes)
}
