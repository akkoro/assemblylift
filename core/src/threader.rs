use std::collections::HashMap;
use std::sync::{mpsc, Mutex};

use crossbeam_utils::atomic::AtomicCell;

use assemblylift_core_event_common::constants::EVENT_BUFFER_SIZE_BYTES;
use assemblylift_core_event_common::EventMemoryDocument;
use assemblylift_core_iomod::registry::{RegistryChannelMessage, RegistryTx};

lazy_static! {
    static ref EVENT_MEMORY: Mutex<EventMemory> = Mutex::new(EventMemory::new());
}

pub struct Threader {
    registry_tx: RegistryTx
}

impl Threader {
    pub fn new(tx: RegistryTx) -> Self {
        Threader {
            registry_tx: tx
        }
    }

    pub fn next_event_id(&mut self) -> Option<u32> {
        match EVENT_MEMORY.lock() {
            Ok(mut memory) => memory.next_id(),
            Err(_) => None,
        }
    }

    pub fn is_event_ready(&self, event_id: u32) -> bool {
        match EVENT_MEMORY.lock() {
            Ok(memory) => memory.is_ready(event_id),
            Err(_) => false,
        }
    }

    pub fn get_event_memory_document(&mut self, event_id: u32) -> Option<EventMemoryDocument> {
        println!("TRACE: get_event_memory_document event_id={}", event_id);
        match EVENT_MEMORY.lock() {
            Ok(memory) => {
                println!(
                    "DEBUG: num keys in document map: {}",
                    memory.document_map.keys().len()
                );
                match memory.document_map.get(&event_id) {
                    Some(doc) => Some(doc.clone()),
                    None => None,
                }
            }
            Err(_) => None,
        }
    }

    pub fn spawn_with_event_id(
        &mut self,
        method_path: &str,
        method_input: Vec<u8>,
        writer: *const AtomicCell<u8>,
        event_id: u32,
    ) {
        println!("TRACE: spawn_with_event_id");

        // FIXME this is a kludge -- I feel like the raw pointer shouldn't be needed
        let slc = unsafe { std::slice::from_raw_parts(writer, EVENT_BUFFER_SIZE_BYTES) };

        let coords = method_path.split(".").collect::<Vec<&str>>();
        if coords.len() != 4 {
            panic!("Malformed method path @ spawn_with_event_id") // TODO don't panic
        }

        let iomod_coords = format!("{}.{}.{}", coords[0], coords[1], coords[2]);
        let method_name = format!("{}", coords[3]);

        let channel = mpsc::channel();
        self.registry_tx.send(RegistryChannelMessage {
            iomod_coords,
            method_name,
            payload_type: "IOMOD_REQUEST",
            payload: method_input,
            responder: Some(channel.0.clone())
        }).unwrap();

        tokio::task::spawn_local(async move {
            if let Ok(response) = channel.1.recv() {
                EVENT_MEMORY
                    .lock()
                    .unwrap()
                    .write_vec_at(slc, response.payload, event_id);
            }
        });

        println!("TRACE: spawned");
    }

    pub fn __reset_memory() {
        if let Ok(mut memory) = EVENT_MEMORY.lock() {
            memory.__reset();
        }
    }
}

struct EventMemory {
    _next_id: u32,
    document_map: HashMap<u32, EventMemoryDocument>,
    event_map: HashMap<u32, bool>,
}

impl EventMemory {
    pub fn new() -> Self {
        EventMemory {
            _next_id: 1, // id 0 is reserved (null)
            document_map: Default::default(),
            event_map: Default::default(),
        }
    }

    pub fn next_id(&mut self) -> Option<u32> {
        let next_id = self._next_id.clone();
        self._next_id += 1;

        self.event_map.insert(next_id, false);

        Some(next_id)
    }

    pub fn is_ready(&self, event_id: u32) -> bool {
        match self.event_map.get(&event_id) {
            Some(status) => *status,
            None => false,
        }
    }

    pub fn write_vec_at(&mut self, writer: &[AtomicCell<u8>], vec: Vec<u8>, event_id: u32) {
        println!("TRACE: write_vec_at");

        // Serialize the response
        let response_len = vec.len();
        println!("DEBUG: response is {} bytes", response_len);

        let start = self.find_with_length(response_len);
        let end = start + response_len;
        for i in start..end {
            writer[i].store(vec[i - start]);
        }
        println!("TRACE: stored response");

        // Update document map
        self.document_map
            .insert(event_id, EventMemoryDocument { start, length: end });
        println!(
            "TRACE: updated document map id={} start={} end={}",
            event_id, start, end
        );

        // Update event status table
        self.event_map.insert(event_id, true);
    }

    pub fn __reset(&mut self) {
        self._next_id = 1;
        self.document_map = Default::default();
        self.event_map = Default::default();
        println!("DEBUG: reset Threader memory");
    }

    fn find_with_length(&self, _length: usize) -> usize {
        // TODO this less stupidly
        let mut max_end = 0usize;
        for doc in self.document_map.values().into_iter() {
            let next_end = doc.start + doc.length;
            if next_end > max_end {
                max_end = next_end
            }
        }
        max_end
    }
}
