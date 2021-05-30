use std::collections::HashMap;
use std::future::Future;
use std::iter::Extend;
use std::sync::{Arc, Mutex};
use std::mem::ManuallyDrop;

use crossbeam_utils::atomic::AtomicCell;
use once_cell::sync::Lazy;
use tokio::sync::mpsc;
use wasmer::{Array, LazyInit, Memory, NativeFunc, WasmerEnv, WasmPtr};

use assemblylift_core_io_common::constants::IO_BUFFER_SIZE_BYTES;
use assemblylift_core_iomod::registry::{RegistryChannelMessage, RegistryTx};

use crate::buffers::{LinearBuffer, IoBuffer};

static IO_MEMORY: Lazy<Mutex<IoMemory>> = Lazy::new(|| Mutex::new(IoMemory::new(64, 8192)));

#[derive(WasmerEnv, Clone)]
pub struct ThreaderEnv {
    pub threader: ManuallyDrop<Arc<Mutex<Threader>>>,
    pub host_input_buffer: Arc<Mutex<crate::buffers::FunctionInputBuffer>>,
    #[wasmer(export)]
    pub memory: LazyInit<Memory>,
    #[wasmer(export(name = "__asml_guest_get_function_input_buffer_pointer"))]
    pub get_function_input_buffer: LazyInit<NativeFunc<(), WasmPtr<u8, Array>>>,
    #[wasmer(export(name = "__asml_guest_get_io_buffer_pointer"))]
    pub get_io_buffer: LazyInit<NativeFunc<(), WasmPtr<u8, Array>>>,
}

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
                        // TODO below maybe not true now
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
//        memory: *const AtomicCell<u8>,
        ioid: u32,
    ) {
//        let slc = unsafe { std::slice::from_raw_parts(memory, IO_BUFFER_SIZE_BYTES) };
        
        let coords = method_path.split(".").collect::<Vec<&str>>();
        if coords.len() != 4 {
            panic!("Malformed method path @ Threader::invoke") // TODO don't panic
        }

        let iomod_coords = format!("{}.{}.{}", coords[0], coords[1], coords[2]);
        let method_name = format!("{}", coords[3]);

        let registry_tx = self.registry_tx.clone();
        let (local_tx, mut local_rx) = mpsc::channel(100);

        let hnd = self.runtime.handle().clone();
        hnd.spawn(async move {
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
                        .handle_response(response.payload, ioid);
                }
            });
        });
    }

    pub fn spawn(&self, future: impl Future<Output = Result<(), std::io::Error>> + Send + 'static) {
        let hnd = self.runtime.handle();
        hnd.spawn(future);
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
    event_ptr: Option<u32>,
    offset: Option<usize>,
    status: BlockStatus,
}

impl Block {
    fn free(&mut self) {
        self.status = BlockStatus::Free;
        self.offset = None;
        self.event_ptr = None;
    }

    fn set(&mut self, ioid: u32, offset: usize) {
        self.status = BlockStatus::Used;
        self.event_ptr = Some(ioid);
        self.offset = Some(offset);
    }
}

#[derive(Clone)]
struct BlockList(Vec<Block>);

impl BlockList {
    fn new(num_blocks: usize) -> Self {
        Self(Vec::with_capacity(num_blocks))
    }

    fn reserve(&mut self, num_blocks: usize) {
        self.0.reserve(num_blocks);
    }
}

impl Extend<Block> for BlockList {
    fn extend<T: IntoIterator<Item=Block>>(&mut self, iter: T) {
        for e in iter {
            self.0.push(e);
        }
    }
}

impl IntoIterator for BlockList {
    type Item = Block;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Clone)]
pub struct IoMemoryDocument {
    pub start: usize,
    pub length: usize,
}

struct IoMemory {
    _next_id: u32,
    blocks: BlockList,
    block_size: usize,
    buffer: IoBuffer,
    document_map: HashMap<u32, IoMemoryDocument>,
    io_status: HashMap<u32, bool>,
    num_blocks: usize,
}

impl IoMemory {
    fn new(block_size: usize, num_blocks: usize) -> Self {
        IoMemory {
            _next_id: 1, // id 0 is reserved (null)
            blocks: BlockList::new(num_blocks),
            block_size,
            buffer: IoBuffer::new(block_size * num_blocks),
            document_map: Default::default(),
            io_status: Default::default(),
            num_blocks,
        }
    }

    fn reset(&mut self) {
        self._next_id = 1;
        self.blocks = BlockList::new(self.num_blocks);
        self.buffer = IoBuffer::new(self.block_size * self.num_blocks);
        self.document_map.clear();
        self.io_status.clear();
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

    fn handle_response(&mut self, response: Vec<u8>, ioid: u32) {
        let offset = self.alloc(response.len(), ioid);
        self.buffer.write(response.as_slice(), offset);
        self.io_status.insert(ioid, true);
    }

    fn alloc(&mut self, byte_length: usize, ioid: u32) -> usize {
        let needed_blocks = (byte_length as f64 / self.block_size as f64).ceil() as usize;
        let needed_bytes = needed_blocks * self.block_size;
        let mut available_blocks = 0usize;
        let mut block_list_offset = 0usize;

        if self.buffer.capacity() < needed_bytes {
            self.grow();
        }

        for (i, block) in self.blocks.clone().into_iter().enumerate() {
            match block.status {
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

        let block_range = block_list_offset..(block_list_offset + needed_blocks);
        for i in block_range {
            self.buffer.erase(i * self.block_size, (i * self.block_size) + self.block_size);
            self.blocks.0.get_mut(i).unwrap().set(ioid, i * self.block_size);
        }

        let start = block_list_offset * self.block_size;

        self.document_map.insert(
            ioid,
            IoMemoryDocument {
                start,
                length: byte_length,
            },
        );

        start
    }

    fn grow(&mut self) {
        self.buffer.double();
        self.blocks.reserve(self.num_blocks);
        self.blocks.extend(vec![Block {
            event_ptr: None,
            offset: None,
            status: BlockStatus::Free,
        }; self.num_blocks]);
        self.num_blocks *= 2;
    }

    fn free(&mut self, ioid: u32) {
         for mut block in self.blocks.clone().into_iter() {
            if let Some(event_ptr) = block.event_ptr {
                if event_ptr == ioid {
                    block.free();
                }
            }
        }
    }
}
