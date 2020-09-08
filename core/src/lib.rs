#[macro_use]
extern crate lazy_static;

use wasmer_runtime::{Array, WasmPtr};

pub type WasmBufferPtr = WasmPtr<u8, Array>;

pub mod abi;
pub mod threader;
