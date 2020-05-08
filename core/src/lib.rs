#[macro_use]
extern crate lazy_static;
extern crate assemblylift_core_event;

use std::any::Any;
use std::borrow::Borrow;
use std::env::Args;
use std::ffi::c_void;
use std::io::Read;
use std::ops::Deref;
use std::pin::Pin;

use wasmer_runtime::{Array, Ctx, Func, WasmPtr};
use wasmer_runtime::Instance;
use wasmer_runtime_core::{DynFunc, structures::TypedIndex, types::TableIndex};
use wasmer_runtime_core::backend::SigRegistry;
use wasmer_runtime_core::module::ExportIndex;
use wasmer_runtime_core::typed_func::Wasm;

use assemblylift_core_event::threader::Threader;

use crate::iomod::ModuleRegistry;
use std::sync::mpsc::SyncSender;
use std::sync::Arc;

pub mod iomod;

pub type WasmBufferPtr = WasmPtr<u8, Array>;

pub struct InstanceData<'a> {
    pub instance: *mut c_void,
    pub module_registry: &'a mut ModuleRegistry,
    pub threader: &'a mut Threader
}

/* Cloud interface */

pub trait Database {
    // TODO: general-purpose database api ?
}
