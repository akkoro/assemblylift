use std::collections::HashMap;

use wasmer_runtime_core::vm;

use crate::WasmBufferPtr;
use tokio::runtime::Runtime;
use std::sync::Arc;

pub type AsmlRuntime = Arc<Runtime>;
pub type AsmlAbiFn = fn(&mut vm::Ctx, WasmBufferPtr, WasmBufferPtr, u32) -> i32;
pub type AsmlRuntimePair = (AsmlAbiFn, AsmlRuntime);

// org -> namespace -> name -> fn
pub type ModuleMap =
    HashMap<String, HashMap<String, HashMap<String, AsmlRuntimePair>>>;

#[derive(Clone)]
pub struct ModuleRegistry {
    pub modules: ModuleMap,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        ModuleRegistry {
            modules: Default::default(),
        }
    }
}
