extern crate assemblylift_core_event;
use assemblylift_core_event::Event;
use assemblylift_core_event::manager::EventManager;

use crate::iomod::ModuleRegistry;

use std::io::Read;

use wasmer_runtime::Instance;
use wasmer_runtime::{Func, Ctx, WasmPtr, Array};
use std::ffi::c_void;
use std::any::Any;
use wasmer_runtime_core::module::ExportIndex;
use wasmer_runtime_core::{structures::TypedIndex, types::TableIndex, DynFunc};
use wasmer_runtime_core::backend::SigRegistry;
use std::env::Args;
use wasmer_runtime_core::typed_func::Wasm;
use std::borrow::Borrow;
use std::ops::Deref;
use std::pin::Pin;

pub mod iomod;

pub type WasmBufferPtr = WasmPtr<u8, Array>;

pub struct InstanceData<'a> {
    pub instance: *mut Instance,

    pub module_registry: &'a mut ModuleRegistry,
    pub event_manager: &'a mut EventManager,
}

/* Cloud interface */

pub trait Database {
    // TODO: general-purpose database api ?
}
