#[macro_use]
extern crate lazy_static;

use std::sync::Arc;

use once_cell::sync::Lazy;
use tokio::runtime::Runtime;
use wasmer_runtime::{Array, WasmPtr};

use crate::registry::ModuleRegistry;

pub type WasmBufferPtr = WasmPtr<u8, Array>;

pub struct IoModulePlugin {
    pub organization: &'static str,
    pub namespace: &'static str,
    pub name: &'static str,

    pub rustc_version: &'static str,
    pub asml_core_version: &'static str,

    pub runtime: Lazy<Runtime>,
    pub register: unsafe extern "C" fn(&mut ModuleRegistry),
}

pub mod registry;
pub mod threader;
