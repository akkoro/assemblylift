extern crate assemblylift_core_event;
#[macro_use]
extern crate lazy_static;
extern crate paste;

use wasmer_runtime::{Array, WasmPtr};

pub mod iomod;

pub type WasmBufferPtr = WasmPtr<u8, Array>;
