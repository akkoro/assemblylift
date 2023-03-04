//! AssemblyLift WASM Buffers
//! See [core-buffers doc](../../docs/core-buffers.md) for more details

use std::collections::HashMap;

pub struct IoBuffer {
    buffers: HashMap<usize, Vec<u8>>,
}

impl IoBuffer {
    pub fn new() -> Self {
        Self {
            buffers: Default::default(),
        }
    }

    pub fn set(&mut self, ioid: usize, bytes: Vec<u8>) -> usize {
        self.buffers.insert(ioid, bytes.clone());
        bytes.len()
    }

    pub fn get(&self, ioid: usize) -> Vec<u8> {
        self.buffers.get(&ioid).unwrap().clone()
    }
}
