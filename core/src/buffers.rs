use std::collections::HashMap;

use wasmer::WasmCell;
use assemblylift_core_io_common::constants::{FUNCTION_INPUT_BUFFER_SIZE, IO_BUFFER_SIZE_BYTES};

use crate::threader::ThreaderEnv;

/// A trait representing a linear byte buffer, such as Vec<u8>
pub trait LinearBuffer {
    /// Initialize the buffer with the contents of `buffer`
    fn initialize(&mut self, buffer: Vec<u8>);
    /// Write bytes to the buffer at an offset
    fn write(&mut self, bytes: &[u8], at_offset: usize) -> usize;
    /// Erase `len` bytes starting from `offset`
    fn erase(&mut self, offset: usize, len: usize) -> usize;
    /// The length of the buffer in bytes
    fn len(&self) -> usize;
    /// The capacity of the buffer in bytes
    fn capacity(&self) -> usize;
}

/// A trait representing a buffer in WASM guest memory
pub trait WasmBuffer<S: Clone + Send + Sized + 'static> {
    fn copy_to_wasm(&self, env: &ThreaderEnv<S>, src: (usize, usize), dst: (usize, usize)) -> Result<(), ()>;
}

/// Implement paging data into a `WasmBuffer`
pub trait PagedWasmBuffer<S: Clone + Send + Sized + 'static>: WasmBuffer<S> {
    fn first(&mut self, env: &ThreaderEnv<S>, offset: Option<usize>) -> i32;
    fn next(&mut self, env: &ThreaderEnv<S>) -> i32;
}

pub struct FunctionInputBuffer {
    buffer: Vec<u8>,
    page_idx: usize,
}

impl FunctionInputBuffer {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            page_idx: 0usize,
        }
    }
}

impl LinearBuffer for FunctionInputBuffer {
    fn initialize(&mut self, buffer: Vec<u8>) {
        self.buffer = buffer;
    }

    fn write(&mut self, bytes: &[u8], at_offset: usize) -> usize {
        let mut bytes_written = 0usize;
        for idx in at_offset..bytes.len() {
            self.buffer[idx] = bytes[idx - at_offset];
            bytes_written += 1;
        }
        bytes_written
    }

    fn erase(&mut self, offset: usize, len: usize) -> usize {
        let mut bytes_erased = 0usize;
        for idx in offset..len {
            self.buffer[idx] = 0;
            bytes_erased += 1;
        }
        bytes_erased
    }

    fn len(&self) -> usize {
        self.buffer.len()
    }

    fn capacity(&self) -> usize {
        self.buffer.capacity()
    }
}

impl<S> PagedWasmBuffer<S> for FunctionInputBuffer
where
    S: Clone + Send + Sized + 'static,
{
    fn first(&mut self, env: &ThreaderEnv<S>, _offset: Option<usize>) -> i32 {
        let end: usize = match self.buffer.len() < FUNCTION_INPUT_BUFFER_SIZE {
            true => self.buffer.len(),
            false => FUNCTION_INPUT_BUFFER_SIZE,
        };
        self.copy_to_wasm(env, (0usize, end), (0usize, FUNCTION_INPUT_BUFFER_SIZE)).unwrap();
        self.page_idx = 0usize;
        0
    }

    fn next(&mut self, env: &ThreaderEnv<S>) -> i32 {
        use std::cmp::min;
        if self.buffer.len() > FUNCTION_INPUT_BUFFER_SIZE {
            self.page_idx += 1;
            self.copy_to_wasm(
                env,
                (FUNCTION_INPUT_BUFFER_SIZE * self.page_idx, min(FUNCTION_INPUT_BUFFER_SIZE * (self.page_idx + 1), self.buffer.len())),
                (0usize, FUNCTION_INPUT_BUFFER_SIZE)
            ).unwrap();
        }
        0
    }
}

impl<S> WasmBuffer<S> for FunctionInputBuffer
where
    S: Clone + Send + Sized + 'static,
{
    fn copy_to_wasm(&self, env: &ThreaderEnv<S>, src: (usize, usize), dst: (usize, usize)) -> Result<(), ()> {
        let wasm_memory = env.memory_ref().unwrap();
        let input_buffer = env
            .get_function_input_buffer
            .get_ref()
            .unwrap()
            .call()
            .unwrap();
        let memory_writer: Vec<WasmCell<u8>> = input_buffer
            .deref(
                &wasm_memory,
                dst.0 as u32,
                dst.1 as u32,
            )
            .unwrap();

        for (i, b) in self.buffer[src.0..src.1].iter().enumerate() {
            let idx = i + dst.0;
            memory_writer[idx].set(*b);
        }

        Ok(())
    }
}

pub struct IoBuffer {
    active_buffer: usize,
    buffers: HashMap<usize, Vec<u8>>,
    page_indices: HashMap<usize, usize>,
}

impl IoBuffer {
    pub fn new() -> Self {
        Self {
            active_buffer: 0usize,
            buffers: Default::default(),
            page_indices: Default::default(),
        }
    }

    pub fn len(&self, ioid: usize) -> usize {
        self.buffers.get(&ioid).unwrap().len()
    }

    pub fn with_capacity(num_buffers: usize, buffer_capacity: usize) -> Self {
        let mut buffers: HashMap<usize, Vec<u8>> = HashMap::new();
        let mut indices: HashMap<usize, usize> = HashMap::new();
        for idx in 0..num_buffers {
            buffers.insert(idx, Vec::with_capacity(buffer_capacity));
            indices.insert(idx, 0);
        }
        Self {
            active_buffer: 0usize,
            buffers,
            page_indices: indices,
        }
    }

    pub fn write(&mut self, ioid: usize, bytes: &[u8]) -> usize {
        let mut bytes_written = 0usize;
        match self.buffers.get_mut(&ioid) {
            Some(buffer) => {
                for idx in 0..bytes.len() {
                    buffer.push(bytes[idx]);
                    bytes_written += 1;
                }
            }
            None => {
                self.buffers.insert(ioid, Vec::new());
                return self.write(ioid, bytes);
            }
        }
        bytes_written
    }
}

impl<S> PagedWasmBuffer<S> for IoBuffer
where
    S: Clone + Send + Sized + 'static,
{
    fn first(&mut self, env: &ThreaderEnv<S>, offset: Option<usize>) -> i32 {
        self.active_buffer = offset.unwrap_or(0);
        self.page_indices.insert(self.active_buffer, 0usize);

        self.copy_to_wasm(
            env,
            (self.active_buffer, 0usize),
            (0usize, IO_BUFFER_SIZE_BYTES),
        ).unwrap();
        0
    }

    fn next(&mut self, env: &ThreaderEnv<S>) -> i32 {
        let page_idx = self.page_indices.get(&self.active_buffer).unwrap() + 1;
        let page_offset = page_idx * IO_BUFFER_SIZE_BYTES;
        self.copy_to_wasm(
            env,
            (self.active_buffer, page_offset),
            (0usize, IO_BUFFER_SIZE_BYTES),
        ).unwrap();
        *self.page_indices.get_mut(&self.active_buffer).unwrap() = page_idx;
        0
    }
}

impl<S> WasmBuffer<S> for IoBuffer
where
    S: Clone + Send + Sized + 'static,
{
    fn copy_to_wasm(&self, env: &ThreaderEnv<S>, src: (usize, usize), dst: (usize, usize)) -> Result<(), ()> {
        use std::cmp::min;
        let wasm_memory = env.memory_ref().unwrap();
        let io_buffer = env
            .get_io_buffer
            .get_ref()
            .unwrap()
            .call()
            .unwrap();
        let memory_writer: Vec<WasmCell<u8>> = io_buffer
            .deref(
                &wasm_memory,
                dst.0 as u32,
                dst.1 as u32,
            )
            .unwrap();

        let buffer = self.buffers.get(&src.0).unwrap();
        for (i, b) in buffer[src.1..min(src.1 + IO_BUFFER_SIZE_BYTES, buffer.len())].iter().enumerate() {
            memory_writer[i].set(*b);
        }

        Ok(())
    }
}
