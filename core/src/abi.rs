use std::cell::Cell;
use std::error::Error;
use std::io;
use std::io::ErrorKind;

use crate::threader::ThreaderEnv;
use crate::{invoke_io, WasmBufferPtr};
use std::time::{SystemTime, UNIX_EPOCH};
use wasmer::MemoryView;

pub type AsmlAbiFn = fn(&ThreaderEnv, WasmBufferPtr, WasmBufferPtr, u32) -> i32;

fn to_io_error<E: Error>(err: E) -> io::Error {
    io::Error::new(ErrorKind::Other, err.to_string())
}

pub fn asml_abi_invoke(
    env: &ThreaderEnv,
    mem: WasmBufferPtr,
    name_ptr: u32,
    name_len: u32,
    input: u32,
    input_len: u32,
) -> i32 {
    if let Ok(method_path) = env_ptr_to_string(env, name_ptr, name_len) {
        if let Ok(input) = env_ptr_to_bytes(env, input, input_len) {
            return invoke_io(env, mem, &*method_path, input);
        }
    }

    -1i32 // error
}

pub fn asml_abi_poll(env: &ThreaderEnv, id: u32) -> i32 {
    env.threader
        .clone()
        .lock()
        .unwrap()
        .poll(id) as i32
}

pub fn asml_abi_io_ptr(env: &ThreaderEnv, id: u32) -> u32 {
    env.threader
        .clone()
        .lock()
        .unwrap()
        .get_io_memory_document(id)
        .unwrap()
        .start as u32
}

pub fn asml_abi_io_len(env: &ThreaderEnv, id: u32) -> u32 {
    env.threader
        .clone()
        .lock()
        .unwrap()
        .get_io_memory_document(id)
        .unwrap()
        .length as u32
}

pub fn asml_abi_clock_time_get(_env: &ThreaderEnv) -> u64 {
    let start = SystemTime::now();
    let unix_time = start
        .duration_since(UNIX_EPOCH)
        .expect("time is broken");
    unix_time.as_secs() * 1000u64
}

pub fn asml_abi_input_start(env: &ThreaderEnv) -> i32 {
    env.host_input_buffer
        .get_ref()
        .unwrap()
        .clone()
        .lock()
        .unwrap()
        .start()
}

pub fn asml_abi_input_next(env: &ThreaderEnv) -> i32 {
    env.host_input_buffer
        .get_ref()
        .unwrap()
        .clone()
        .lock()
        .unwrap()
        .next()
}

pub fn asml_abi_input_length_get(env: &ThreaderEnv) -> i64 {
    env.host_input_buffer
        .get_ref()
        .unwrap()
        .clone()
        .lock()
        .unwrap()
        .len() as i64
}

#[inline]
fn env_ptr_to_string(env: &ThreaderEnv, ptr: u32, len: u32) -> Result<String, io::Error> {
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

#[inline]
fn env_ptr_to_bytes(env: &ThreaderEnv, ptr: u32, len: u32) -> Result<Vec<u8>, io::Error> {
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
