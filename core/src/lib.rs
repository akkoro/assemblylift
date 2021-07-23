extern crate lazy_static;

use wasmer::{Array, WasmPtr};

use crate::threader::ThreaderEnv;

pub type WasmBufferPtr = WasmPtr<u8, Array>;

pub mod abi;
pub mod buffers;
pub mod threader;

#[inline(always)]
pub fn invoke_io(
    env: &ThreaderEnv,
    method_path: &str,
    method_input: Vec<u8>,
) -> i32 {
    let ioid = env.threader.clone().lock().unwrap().next_ioid().unwrap();

    env.threader
        .clone()
        .lock()
        .unwrap()
        .invoke(method_path, method_input, ioid);

    ioid as i32
}
