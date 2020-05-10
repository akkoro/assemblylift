use std::borrow::Borrow;
use std::collections::hash_map::Entry::Occupied;
use std::collections::HashMap;
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, sync_channel, SyncSender};
use std::time::Duration;

use bincode::serialize;
use crossbeam_utils::atomic::AtomicCell;
use futures::{FutureExt, TryFutureExt};
use indexmap::map::IndexMap;
use serde::Serialize;
use tokio::prelude::*;
use tokio::runtime::{Builder, Runtime};

use assemblylift_core_event_common::{EventHandles, EventMemoryDocument, EventStatus, NUM_EVENT_HANDLES};

use crate::constants::EVENT_BUFFER_SIZE_BYTES;

lazy_static! {
    static ref EVENT_MEMORY: Mutex<EventMemory> = Mutex::new(EventMemory::new());
}

pub struct Threader {
    runtime: Runtime,
}

impl Threader {
    pub fn new() -> Self {
        let runtime = Builder::new()
            .threaded_scheduler()
            .build()
            .unwrap();

        Threader {
            runtime
        }
    }

    pub fn next_event_id(&mut self) -> Option<u32> {
        match EVENT_MEMORY.lock() {
            Ok(mut memory) => memory.next_id(),
            Err(_) => None
        }
    }

    pub fn is_event_ready(&self, event_id: u32) -> bool {
        match EVENT_MEMORY.lock() {
            Ok(mut memory) => memory.is_ready(event_id),
            Err(_) => false
        }
    }

    pub fn get_event_memory_document(&mut self, event_id: u32) -> Option<EventMemoryDocument> {
        println!("TRACE: get_event_memory_document event_id={}", event_id);
        match EVENT_MEMORY.lock() {
            Ok(mut memory) => {
                println!("DEBUG: num keys in document map: {}", memory.document_map.keys().len());
                match memory.document_map.get(&event_id) {
                    Some(doc) => Some(doc.clone()),
                    None => None
                }
            },
            Err(_) => None
        }

    }

    pub fn spawn_with_event_id(&mut self, writer: *const AtomicCell<u8>, future: impl Future<Output=Vec<u8>> + 'static + Send, event_id: u32) {
        println!("TRACE: spawn_with_event_id");

        // FIXME this is suuuuper kludgy
        // let mut wr = writer.clone();
        let slc = unsafe { std::slice::from_raw_parts(writer, EVENT_BUFFER_SIZE_BYTES) };

        self.runtime.spawn(async move {
            println!("TRACE: awaiting IO...");
            let serialized = future.await;
            println!("TRACE: IO complete");

            EVENT_MEMORY.lock().unwrap().write_vec_at(slc, serialized, event_id);
        });
    }
}

struct EventMemory {
    _next_id: u32,
    document_map: HashMap<u32, EventMemoryDocument>,
    event_status: EventStatus,
}

impl EventMemory {
    pub fn new() -> Self {
        EventMemory {
            _next_id: 1, // id 0 is reserved (null)
            document_map: Default::default(),
            event_status: EventStatus([(0, false); NUM_EVENT_HANDLES]) // TODO I think this can be a map now
        }
    }

    pub fn next_id(&mut self) -> Option<u32> {
        let next_id = self._next_id.clone();
        self._next_id += 1;

        Some(next_id)
    }

    pub fn is_ready(&self, event_id: u32) -> bool {
        for evt in self.event_status.0.iter() {
            if evt.0 == event_id {
                return evt.1;
            }
        }

        false
    }

    pub fn write_vec_at(&mut self, writer: &[AtomicCell<u8>], vec: Vec<u8>, event_id: u32) {
        println!("TRACE: write_vec_at");

        // Serialize the response
        let required_length = vec.len();
        println!("DEBUG: response is {} bytes", required_length);

        let start = self.find_with_length(required_length);
        let end = start + required_length;
        for i in start..end {
            writer[i].store(vec[i - start]);
        }
        println!("TRACE: stored response");

        // Update document map
        self.document_map.insert(event_id, EventMemoryDocument { start, length: end });
        println!("TRACE: updated document map id={} start={} end={}", event_id, start, end);

        // Update event status table
        for (idx, e) in self.event_status.0.iter().enumerate() {
            if e.0 == 0 {
                self.event_status.0[idx] = (event_id, true);
                break;
            }
        }
    }

    fn find_with_length(&self, length: usize) -> usize {
        0
    }
}
