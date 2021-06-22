use std::collections::HashMap;
use std::future::Future;
use std::iter::Extend;
use std::sync::{Arc, Mutex};
use std::mem::ManuallyDrop;

use tokio::sync::mpsc;
use wasmer::{Array, LazyInit, Memory, NativeFunc, WasmerEnv, WasmPtr};

use assemblylift_core_iomod::registry::{RegistryChannelMessage, RegistryTx};

use crate::buffers::{LinearBuffer, IoBuffer, PagedWasmBuffer};

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
    io_memory: Arc<Mutex<IoMemory>>,
    registry_tx: RegistryTx,
    runtime: tokio::runtime::Runtime,
}

impl Threader {
    pub fn new(tx: RegistryTx) -> Self {
        Threader {
            io_memory: Arc::new(Mutex::new(IoMemory::new(256, 16384))),
            registry_tx: tx,
            runtime: tokio::runtime::Runtime::new().unwrap(),
        }
    }

    pub fn next_ioid(&mut self) -> Option<u32> {
        match self.io_memory.clone().lock() {
            Ok(mut memory) => memory.next_id(),
            Err(_) => None,
        }
    }

    pub fn get_io_memory_document(&mut self, ioid: u32) -> Option<IoMemoryDocument> {
        match self.io_memory.clone().lock() {
            Ok(memory) => match memory.document_map.get(&ioid) {
                Some(doc) => Some(doc.clone()),
                None => None,
            },
            Err(_) => None,
        }
    }

    pub fn document_load(&mut self, env: &ThreaderEnv, ioid: u32) -> Result<(), ()> {
        let doc = self.get_io_memory_document(ioid).unwrap();
        self.io_memory.lock().unwrap().buffer.first(env, Some(doc.start));
        Ok(())
    }

    pub fn document_next(&mut self, env: &ThreaderEnv) -> Result<(), ()> {
        self.io_memory.lock().unwrap().buffer.next(env);
        Ok(())
    }
    
    pub fn poll(&mut self, ioid: u32) -> bool {
        match self.io_memory.clone().lock() {
            Ok(mut memory) => {
                match memory.poll(ioid) {
                    true => {
                        // At this point, the document "contents" have already been written to the WASM buffer
                        //    and are read on the guest side immediately after poll() exits.
                        // We can free the host-side memory structure here.
//                        memory.free(ioid);
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
        ioid: u32,
    ) {
        let io_memory = self.io_memory.clone();
        
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
                    io_memory
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

    pub fn __reset_memory(&self) {
        if let Ok(mut memory) = self.io_memory.clone().lock() {
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
struct BlockList {
    block_size: usize,
    list: Vec<Block>,
}

impl BlockList {
    fn new(block_size: usize, num_blocks: usize) -> Self {
        Self {
            block_size,
            list: Vec::with_capacity(num_blocks),
        }
    }

    #[inline(always)]
    fn push(&mut self, idx: usize) {
        if idx >= self.list.len() {
            let diff = std::cmp::max(idx - self.list.len(), 1);
            for _ in 0..diff {
                self.list.push(Block {
                    event_ptr: None,
                    offset: None,
                    status: BlockStatus::Free,
                })
            }
        }
    }
    
    fn set(&mut self, idx: usize, ioid: u32) {
        self.push(idx);
        self.list.get_mut(idx)
            .expect(&format!("could not get block idx {} for ioid {}", idx, ioid))
            .set(ioid, idx * self.block_size);
    }

    fn reserve(&mut self, num_blocks: usize) {
        self.list.reserve(num_blocks);
    }
}

impl Extend<Block> for BlockList {
    fn extend<T: IntoIterator<Item=Block>>(&mut self, iter: T) {
        println!("DEBUG: extending BlockList");
        let mut count = 0usize;
        for e in iter {
            self.list.push(e);
            count += 1;
        }
        println!("DEBUG: added {} blocks", count);
    }
}

impl IntoIterator for BlockList {
    type Item = Block;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.list.into_iter()
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
        let mut blocks = BlockList::new(block_size, num_blocks);
        blocks.extend(vec![Block {
            event_ptr: None,
            offset: None,
            status: BlockStatus::Free,
        }; num_blocks]);

        IoMemory {
            _next_id: 1, // id 0 is reserved (null)
            blocks,
            block_size,
            buffer: IoBuffer::new(),
            document_map: Default::default(),
            io_status: Default::default(),
            num_blocks,
        }
    }

    fn reset(&mut self) {
        self._next_id = 1;
        self.blocks = BlockList::new(self.block_size, self.num_blocks);
        self.buffer = IoBuffer::new();
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
        println!("DEBUG: handle response for {}", ioid);
        self.buffer.write(ioid as usize, response.as_slice());
        self.io_status.insert(ioid, true);
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
