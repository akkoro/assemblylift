#[macro_use]
extern crate lazy_static;

use std::future::Future;

use crossbeam_utils::atomic::AtomicCell;

use wasmer_runtime_core::vm;

use assemblylift_core::threader::Threader;
use assemblylift_core::WasmBufferPtr;
use assemblylift_core_event_common::constants::EVENT_BUFFER_SIZE_BYTES;
