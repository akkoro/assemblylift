use wasmer::{Array, WasmPtr};

use crate::threader::ThreaderEnv;

pub type WasmBufferPtr = WasmPtr<u8, Array>;

pub mod abi;
pub mod buffers;
pub mod threader;
pub mod wasm;

#[inline(always)]
/// Invoke an IOmod call at coordinates `method_path` with input `method_input`
pub fn invoke_io<S>(env: &ThreaderEnv<S>, method_path: &str, method_input: Vec<u8>) -> i32
where
    S: Clone + Send + Sized + 'static,
{
    let ioid = env
        .threader
        .clone()
        .lock()
        .unwrap()
        .next_ioid()
        .expect("unable to get a new IO ID");

    env.threader
        .clone()
        .lock()
        .unwrap()
        .invoke(method_path, method_input, ioid);

    ioid as i32
}
