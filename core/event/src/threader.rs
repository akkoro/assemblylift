use std::collections::HashMap;
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use crossbeam_utils::atomic::AtomicCell;
use futures::{FutureExt, TryFutureExt};
use indexmap::map::IndexMap;
use tokio::prelude::*;
use tokio::runtime::Runtime;

use crate::constants::EVENT_BUFFER_SIZE_BYTES;
use crate::Event;

pub struct Threader {
    memory: ExecutorMemory,
    runtime: Runtime,
}

impl Threader {
    pub fn new() -> Self {
        Threader {
            memory: ExecutorMemory::new(),
            runtime: Runtime::new().unwrap(),
        }
    }

    pub fn run_with(&mut self, root: impl Future<Output=()>) {
        self.runtime.block_on(root)
    }

    pub fn next_event_id(&mut self) -> Option<u32> {
        self.memory.next_id()
    }

    pub fn spawn_with_event_id(&self, writer: Arc<*const AtomicCell<u8>>, future: impl Future<Output=Vec<u8>> + 'static + Send, event_id: u32) {
        println!("TRACE: spawn_with_event_id");

        // clone is fine, as long as we're sure that the addresses aren't stale
        // TODO not sure of performance of clone here though
        let memory = self.memory.clone();

        let mut wr = writer.clone();
        // FIXME this is suuuuper kludgy
        let slc = unsafe { std::slice::from_raw_parts(*wr, EVENT_BUFFER_SIZE_BYTES) };

        self.runtime.spawn(async move {
            println!("TRACE: awaiting IO...");
            let serialized = future.await;
            memory.write_vec_at(slc, serialized, event_id);
            println!("TRACE: wrote to WASM memory");
        });

        ()
    }
}

#[derive(Clone)]
struct Document {
    start: usize,
    length: usize
}

#[derive(Clone)]
struct ExecutorMemory {
    _next_id: u32,
    document_map: IndexMap<usize, Document>
}

impl ExecutorMemory {
    pub fn new() -> Self {
        ExecutorMemory {
            _next_id: 0,
            document_map: Default::default()
        }
    }

    pub fn next_id(&mut self) -> Option<u32> {
        let next_id = self._next_id.clone();
        self._next_id += 1;

        Some(next_id)
    }

    pub fn write_vec_at(&self, writer: &[AtomicCell<u8>], vec: Vec<u8>, event_id: u32) {
        println!("TRACE: write_vec_at");

        let index = event_id as usize;
        let required_length = vec.len();

        let start = self.find_with_length(required_length);
        let end = start + required_length;
        for i in start..end {
            writer[i].store(vec[i - start]);
        }
    }

    fn find_with_length(&self, length: usize) -> usize {
        0usize
    }
}
