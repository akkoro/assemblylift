use std::time::{SystemTime, UNIX_EPOCH};

use wasmtime::{Caller, Val};

use itertools::Itertools;

use crate::buffers::PagedWasmBuffer;
use crate::wasm::{State, Wasmtime};

pub trait RuntimeAbi<S: Clone + Send + Sized + 'static> {
    fn log(caller: Caller<'_, State<S>>, ptr: u32, len: u32);
    fn success(caller: Caller<'_, State<S>>, ptr: u32, len: u32);
}

pub fn asml_abi_io_invoke<R, S>(
    mut caller: Caller<'_, State<S>>,
    name_ptr: u32,
    name_len: u32,
    input_ptr: u32,
    input_len: u32,
) -> i32
where
    R: RuntimeAbi<S> + 'static,
    S: Clone + Send + Sized + 'static,
{
    if let Ok(method_path) = Wasmtime::<R, S>::ptr_to_string(&mut caller, name_ptr, name_len) {
        if let Ok(method_input) = Wasmtime::<R, S>::ptr_to_bytes(&mut caller, input_ptr, input_len)
        {
            return invoke_io(caller, &*method_path, method_input);
        }
    }

    -1i32 // error
}

pub fn asml_abi_io_poll<S>(mut caller: Caller<'_, State<S>>, id: u32) -> i32
where
    S: Clone + Send + Sized + 'static,
{
    let state = caller.data_mut();
    state.threader.clone().lock().unwrap().poll(id) as i32
}

pub fn asml_abi_io_len<S>(mut caller: Caller<'_, State<S>>, id: u32) -> u32
where
    S: Clone + Send + Sized + 'static,
{
    let state = caller.data_mut();
    match state
        .threader
        .clone()
        .lock()
        .unwrap()
        .get_io_memory_document(id)
    {
        Some(doc) => doc.length as u32,
        None => 0,
    }
}

pub fn asml_abi_io_load<S>(mut caller: Caller<'_, State<S>>, id: u32) -> i32
where
    S: Clone + Send + Sized + 'static,
{
    let state = caller.data_mut();
    let io_buffer_ptr = state.io_buffer_ptr.unwrap();

    let mut ptr: Vec<Val> = vec![Val::I32(0)];
    if let Err(_err) = io_buffer_ptr.call(&mut caller, &[], &mut ptr) {
        // TODO log with info! when tracing is added
        return -1;
    }
    let ptr = *&ptr[0].i32().unwrap();

    let memory_offset = ptr as usize;
    let data = {
        let state = caller.data_mut();
        state
            .threader
            .lock()
            .unwrap()
            .document_load(memory_offset, id)
            .unwrap()
    };
    let memory = caller
        .get_export("memory")
        .expect("could not find the default memory export named \"memory\"")
        .into_memory()
        .unwrap();
    match data.len() > 0 {
        true => {
            let offset = data[0].0;
            let buffer = data.iter().map(|e| e.1).collect_vec();
            match memory.write(&mut caller, offset, &buffer) {
                Ok(_) => 0,
                Err(_err) => -1,
            }
        }
        false => -1,
    }
}

pub fn asml_abi_io_next<S>(mut caller: Caller<'_, State<S>>) -> i32
where
    S: Clone + Send + Sized + 'static,
{
    let ptr = {
        let state = caller.data_mut();
        let io_buffer_ptr = state.io_buffer_ptr.unwrap();

        let mut ptr: Vec<Val> = vec![Val::I32(0)];
        if let Err(_err) = io_buffer_ptr.call(&mut caller, &[], &mut ptr) {
            // TODO log with info! when tracing is added
            return -1;
        }
        *&ptr[0].i32().unwrap()
    };

    let memory_offset = ptr as usize;
    let data = {
        let state = caller.data_mut();
        state
            .threader
            .lock()
            .unwrap()
            .document_next(memory_offset)
            .unwrap()
    };
    let memory = caller
        .get_export("memory")
        .expect("could not find the default memory export named \"memory\"")
        .into_memory()
        .unwrap();
    match data.len() > 0 {
        true => {
            let offset = data[0].0;
            let buffer = data.iter().map(|e| e.1).collect_vec();
            match memory.write(&mut caller, offset, &buffer) {
                Ok(_) => 0,
                Err(_err) => -1,
            }
        }
        false => -1,
    }
}

pub fn asml_abi_clock_time_get<S>(_caller: Caller<'_, State<S>>) -> u64
where
    S: Clone + Send + Sized + 'static,
{
    let start = SystemTime::now();
    let unix_time = start.duration_since(UNIX_EPOCH).expect("time is broken");
    unix_time.as_secs() * 1000u64
}

pub fn asml_abi_input_start<S>(mut caller: Caller<'_, State<S>>) -> i32
where
    S: Clone + Send + Sized + 'static,
{
    let state = caller.data_mut();

    let mut ptr: Vec<Val> = vec![Val::I32(0)];
    if let Err(_err) = state
        .function_input_buffer_ptr
        .unwrap()
        .call(&mut caller, &[], &mut ptr)
    {
        // TODO log with info! when tracing is added
        return -1;
    }
    let ptr = *&ptr[0].i32().unwrap();

    let offset = ptr as usize;
    let data = {
        let state = caller.data_mut();
        state.function_input_buffer.first(0, offset)
    };
    let memory = caller
        .get_export("memory")
        .expect("could not find the default memory export named \"memory\"")
        .into_memory()
        .unwrap();
    match data.len() > 0 {
        true => {
            let offset = data[0].0;
            let buffer = data.iter().map(|e| e.1).collect_vec();
            memory.write(&mut caller, offset, &buffer).expect("");
            0
        }
        false => -1,
    }
}

pub fn asml_abi_input_next<S>(mut caller: Caller<'_, State<S>>) -> i32
where
    S: Clone + Send + Sized + 'static,
{
    let state = caller.data_mut();

    let mut ptr: Vec<Val> = vec![Val::I32(0)];
    if let Err(_err) = state
        .function_input_buffer_ptr
        .unwrap()
        .call(&mut caller, &[], &mut ptr)
    {
        // TODO log with info! when tracing is added
        return -1;
    }
    let ptr = *&ptr[0].i32().unwrap();

    let offset = ptr as usize;
    let data = {
        let state = caller.data_mut();
        state.function_input_buffer.next(offset)
    };
    let memory = caller
        .get_export("memory")
        .expect("could not find the default memory export named \"memory\"")
        .into_memory()
        .unwrap();
    match data.len() > 0 {
        true => {
            let offset = data[0].0;
            let buffer = data.iter().map(|e| e.1).collect_vec();
            memory.write(&mut caller, offset, &buffer).expect("");
            0
        }
        false => -1,
    }
}

pub fn asml_abi_input_length_get<S>(mut caller: Caller<'_, State<S>>) -> u64
where
    S: Clone + Send + Sized + 'static,
{
    let state = caller.data_mut();
    state.function_input_buffer.len() as u64
}

#[inline(always)]
/// Invoke an IOmod call at coordinates `method_path` with input `method_input`
fn invoke_io<S>(caller: Caller<'_, State<S>>, method_path: &str, method_input: Vec<u8>) -> i32
where
    S: Clone + Send + Sized + 'static,
{
    let ioid = caller
        .data()
        .threader
        .clone()
        .lock()
        .unwrap()
        .next_ioid()
        .expect("unable to get a new IO ID");

    caller
        .data()
        .threader
        .clone()
        .lock()
        .unwrap()
        .invoke(method_path, method_input, ioid);

    ioid as i32
}
