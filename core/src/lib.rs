extern crate assemblylift_core_event;
#[macro_use]
extern crate lazy_static;
extern crate paste;

use std::any::Any;
use std::borrow::Borrow;
use std::env::Args;
use std::ffi::c_void;
use std::io::Read;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::SyncSender;

use crossbeam_utils::atomic::AtomicCell;
use wasmer_runtime::{Array, Ctx, Func, WasmPtr};
use wasmer_runtime::Instance;
use wasmer_runtime_core::{DynFunc, structures::TypedIndex, types::TableIndex};
use wasmer_runtime_core::backend::SigRegistry;
use wasmer_runtime_core::module::ExportIndex;
use wasmer_runtime_core::typed_func::Wasm;

use assemblylift_core_event::threader::Threader;

use crate::iomod::ModuleRegistry;

pub mod iomod;

pub type WasmBufferPtr = WasmPtr<u8, Array>;
