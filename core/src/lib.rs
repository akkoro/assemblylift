extern crate lazy_static;

use wasmer::{Array, WasmPtr};

use assemblylift_core_io_common::constants::IO_BUFFER_SIZE_BYTES;

use crate::threader::ThreaderEnv;

pub type WasmBufferPtr = WasmPtr<u8, Array>;

pub mod abi;
pub mod threader;

#[inline(always)]
pub fn invoke_io(
    env: &ThreaderEnv,
    ptr: WasmBufferPtr,
    method_path: &str,
    method_input: Vec<u8>,
) -> i32 {
    let memory = env.memory_ref().unwrap();
    let mem = ptr.deref(memory, 0, IO_BUFFER_SIZE_BYTES as u32).unwrap();
    let ioid = env.threader.clone().lock().unwrap().next_ioid().unwrap();

    env.threader
        .clone()
        .lock()
        .unwrap()
        .invoke(method_path, method_input, mem.as_ptr(), ioid);

    ioid as i32
}
