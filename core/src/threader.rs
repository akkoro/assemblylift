use std::collections::HashMap;
use std::sync::Mutex;

use crossbeam_utils::atomic::AtomicCell;
use once_cell::sync::Lazy;
use tokio::sync::mpsc;

use assemblylift_core_io_common::constants::IO_BUFFER_SIZE_BYTES;
use assemblylift_core_io_common::IoMemoryDocument;
use assemblylift_core_iomod::registry::{RegistryChannelMessage, RegistryTx};

const BLOCK_SIZE_BYTES: usize = 64;
const NUM_BLOCKS: usize = IO_BUFFER_SIZE_BYTES / BLOCK_SIZE_BYTES;

static IO_MEMORY: Lazy<Mutex<IoMemory>> = Lazy::new(|| Mutex::new(IoMemory::new()));

pub struct Threader {
    registry_tx: RegistryTx,
    runtime: tokio::runtime::Runtime,
}

impl Threader {
    pub fn new(tx: RegistryTx) -> Self {
        Threader {
            registry_tx: tx,
            runtime: tokio::runtime::Runtime::new().unwrap(),
        }
    }

    pub fn next_ioid(&mut self) -> Option<u32> {
        match IO_MEMORY.lock() {
            Ok(mut memory) => memory.next_id(),
            Err(_) => None,
        }
    }

    pub fn get_io_memory_document(&mut self, ioid: u32) -> Option<IoMemoryDocument> {
        match IO_MEMORY.lock() {
            Ok(memory) => match memory.document_map.get(&ioid) {
                Some(doc) => Some(doc.clone()),
                None => None,
            },
            Err(_) => None,
        }
    }

    pub fn poll(&mut self, ioid: u32) -> bool {
        match IO_MEMORY.lock() {
            Ok(mut memory) => {
                match memory.poll(ioid) {
                    true => {
                        // At this point, the document "contents" have already been written to the WASM buffer
                        //    and are read on the guest side immediately after poll() exits.
                        // We can free the host-side memory structure here.
                        memory.free(ioid);
                        true
                    }
                    false => false,
                }
            }
            Err(_) => false,
        }
    }

    pub fn invoke(
        &mut self,
        method_path: &str,
        method_input: Vec<u8>,
        writer: *const AtomicCell<u8>,
        ioid: u32,
    ) {
        // FIXME this is a kludge -- I feel like the raw pointer shouldn't be needed
        let slc = unsafe { std::slice::from_raw_parts(writer, IO_BUFFER_SIZE_BYTES) };

        let coords = method_path.split(".").collect::<Vec<&str>>();
        if coords.len() != 4 {
            panic!("Malformed method path @ Threader::invoke") // TODO don't panic
        }

        let iomod_coords = format!("{}.{}.{}", coords[0], coords[1], coords[2]);
        let method_name = format!("{}", coords[3]);

        let mut registry_tx = self.registry_tx.clone();
        let (local_tx, mut local_rx) = mpsc::channel(100);

        let hnd = self.runtime.handle().clone();
        std::thread::spawn(move || {
            hnd.enter(|| {
                tokio::spawn(async move {
                    registry_tx
                        .send(RegistryChannelMessage {
                            iomod_coords,
                            method_name,
                            payload_type: "IOMOD_REQUEST",
                            payload: method_input,
                            responder: Some(local_tx.clone()),
                        })
                        .await
                        .unwrap();
                });

                tokio::spawn(async move {
                    if let Some(response) = local_rx.recv().await {
                        IO_MEMORY
                            .lock()
                            .unwrap()
                            .write_vec_at(slc, response.payload, ioid);
                    }
                });
            });
        });
    }

    pub fn __reset_memory() {
        if let Ok(mut memory) = IO_MEMORY.lock() {
            memory.reset();
        }
    }
}

#[derive(Copy, Clone)]
enum BlockStatus {
    Free,
    Used,
}

#[derive(Copy, Clone)]
struct Block {
    status: BlockStatus,
    event_ptr: Option<u32>,
}

struct BlockList {
    list: Box<[Block; NUM_BLOCKS]>,
}

impl Default for BlockList {
    fn default() -> Self {
        let list: Box<[Block; NUM_BLOCKS]> = Box::new(
            [Block {
                status: BlockStatus::Free,
                event_ptr: None,
            }; NUM_BLOCKS],
        );

        Self { list }
    }
}

struct IoMemory {
    _next_id: u32,
    document_map: HashMap<u32, IoMemoryDocument>,
    io_status: HashMap<u32, bool>,
    blocks: BlockList,
}

impl IoMemory {
    fn new() -> Self {
        IoMemory {
            _next_id: 1, // id 0 is reserved (null)
            document_map: Default::default(),
            io_status: Default::default(),
            blocks: Default::default(),
        }
    }

    fn reset(&mut self) {
        self._next_id = 1;
        self.document_map = Default::default();
        self.io_status = Default::default();
        self.blocks = Default::default();
    }

    fn next_id(&mut self) -> Option<u32> {
        let next_id = self._next_id.clone();
        self._next_id += 1;

        self.io_status.insert(next_id, false);

        Some(next_id)
    }

    fn poll(&self, ioid: u32) -> bool {
        match self.io_status.get(&ioid) {
            Some(status) => *status,
            None => false,
        }
    }

    fn write_vec_at(&mut self, writer: &[AtomicCell<u8>], vec: Vec<u8>, ioid: u32) {
        // Serialize the response
        let response_len = vec.len();
        let start = self.alloc(writer, response_len, ioid);
        let end = start + response_len;
        for i in start..end {
            writer[i].store(vec[i - start]);
        }

        // Update document map
        self.document_map.insert(
            ioid,
            IoMemoryDocument {
                start,
                length: response_len,
            },
        );

        // Update io status table
        self.io_status.insert(ioid, true);
    }

    fn alloc(&mut self, writer: &[AtomicCell<u8>], byte_length: usize, ioid: u32) -> usize {
        let needed_blocks = (byte_length as f64 / BLOCK_SIZE_BYTES as f64).ceil() as usize;
        let mut available_blocks = 0usize;
        let mut block_list_offset = 0usize;

        for i in 0..NUM_BLOCKS {
            match self.blocks.list[i].status {
                BlockStatus::Free => {
                    if available_blocks == 0 {
                        block_list_offset = i;
                    }
                    available_blocks += 1;
                    if available_blocks >= needed_blocks {
                        break;
                    }
                }

                BlockStatus::Used => {
                    available_blocks = 0;
                }
            }
        }

        if available_blocks < needed_blocks {
            panic!("unable to allocate memory in Threader")
        }

        let block_range = block_list_offset..(block_list_offset + needed_blocks);
        for i in block_range {
            let byte_range = (i * BLOCK_SIZE_BYTES)..((i * BLOCK_SIZE_BYTES) + BLOCK_SIZE_BYTES);
            for b in byte_range {
                writer[b].store(0);
            }
            self.blocks.list[i] = Block {
                status: BlockStatus::Used,
                event_ptr: Some(ioid),
            }
        }

        block_list_offset * BLOCK_SIZE_BYTES
    }

    fn free(&mut self, ioid: u32) {
        for i in 0..NUM_BLOCKS {
            if let Some(event_ptr) = self.blocks.list[i].event_ptr {
                if event_ptr == ioid {
                    self.blocks.list[i] = Block {
                        status: BlockStatus::Free,
                        event_ptr: None,
                    }
                }
            }
        }
    }
}
