use std::collections::HashMap;

use wasmer_runtime_core::vm;

use crate::WasmBufferPtr;

pub type AsmlAbiFn = fn(&mut vm::Ctx, WasmBufferPtr, WasmBufferPtr, u32) -> i32;
pub type ModuleMap = HashMap<String, HashMap<String, HashMap<String, AsmlAbiFn>>>;

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
