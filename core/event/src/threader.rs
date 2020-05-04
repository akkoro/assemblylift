use std::collections::HashMap;
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use bincode::serialize;
use crossbeam_utils::atomic::AtomicCell;
use futures::{FutureExt, TryFutureExt};
use indexmap::map::IndexMap;
use serde::Serialize;
use tokio::prelude::*;
use tokio::runtime::Runtime;

use crate::constants::EVENT_BUFFER_SIZE_BYTES;

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
        let mut memory = self.memory.clone();

        // FIXME this is suuuuper kludgy
        let mut wr = writer.clone();
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

#[derive(Clone, Serialize)]
struct EventStatus {
    pub events: [(u32, bool); 20] // this should be sorted by id (u32)
}

#[derive(Clone)]
struct ExecutorMemory {
    _next_id: u32,
    document_map: IndexMap<usize, Document>,
    event_status: EventStatus,
}

impl ExecutorMemory {
    pub fn new() -> Self {
        ExecutorMemory {
            _next_id: 1, // id 0 is reserved (null)
            document_map: Default::default(),
            event_status: EventStatus { events: [(0, false); 20] }
        }
    }

    // TODO this needs to be smarter if there's going to be a finite number of handles
    pub fn next_id(&mut self) -> Option<u32> {
        let next_id = self._next_id.clone();
        self._next_id += 1;

        Some(next_id)
    }

    pub fn write_vec_at(&mut self, writer: &[AtomicCell<u8>], vec: Vec<u8>, event_id: u32) {
        println!("TRACE: write_vec_at");

        let required_length = vec.len();

        let start = self.find_with_length(required_length);
        let end = start + required_length;
        for i in start..end {
            writer[i].store(vec[i - start]);
        }

        for (idx, e) in self.event_status.events.iter().enumerate() {
            if e.0 == 0 {
                self.event_status.events[idx] = (event_id, true);
                break;
            }
        }

        if let Ok(serialized_event_status) = serialize(&self.event_status) {
            for i in 0..serialized_event_status.len() {
                writer[i].store(serialized_event_status[i]);
            }
        }
    }

    fn find_with_length(&self, length: usize) -> usize {
        1000usize
    }
}
