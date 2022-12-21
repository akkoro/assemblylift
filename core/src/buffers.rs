//! AssemblyLift WASM Buffers
//! See [core-buffers doc](../../docs/core-buffers.md) for more details

use std::collections::HashMap;

use assemblylift_core_io_common::constants::{FUNCTION_INPUT_BUFFER_SIZE, IO_BUFFER_SIZE_BYTES};

use crate::wasm::BufferElement;

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

/// Implement paging data into a `WasmBuffer`
pub trait PagedWasmBuffer {
    fn first(&mut self, offset: Option<Vec<usize>>) -> Vec<BufferElement>;
    fn next(&mut self, offset: Option<Vec<usize>>) -> Vec<BufferElement>;
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

impl PagedWasmBuffer for FunctionInputBuffer {
    fn first(&mut self, offset: Option<Vec<usize>>) -> Vec<BufferElement> {
        let offset = offset.unwrap();
        let end: usize = match self.buffer.len() < FUNCTION_INPUT_BUFFER_SIZE {
            true => self.buffer.len(),
            false => FUNCTION_INPUT_BUFFER_SIZE,
        };
        self.page_idx = 0usize;

        let mut out: Vec<BufferElement> = Vec::with_capacity(end);
        for (i, b) in self.buffer[0..end].iter().enumerate() {
            let idx = i + offset[0];
            out.push((idx, *b));
        }
        out
    }

    fn next(&mut self, offset: Option<Vec<usize>>) -> Vec<BufferElement> {
        use std::cmp::min;

        let offset = offset.unwrap();
        let start = FUNCTION_INPUT_BUFFER_SIZE * self.page_idx;
        let end = min(FUNCTION_INPUT_BUFFER_SIZE * (self.page_idx + 1), self.buffer.len());
        let mut out: Vec<BufferElement> = Vec::with_capacity(end);
        if self.buffer.len() > FUNCTION_INPUT_BUFFER_SIZE {
            self.page_idx += 1;
            for (i, b) in self.buffer[start..end].iter().enumerate() {
                let idx = i + offset[0];
                out.push((idx, *b));
            }
        }
        out
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

impl PagedWasmBuffer for IoBuffer {
    fn first(&mut self, offset: Option<Vec<usize>>) -> Vec<BufferElement> {
        use std::cmp::min;

        match offset {
            Some(offset) => {
                self.active_buffer = offset[0];
                self.page_indices.insert(self.active_buffer, 0usize);
                let buffer = self.buffers.get(&self.active_buffer).unwrap();
                let end = min(IO_BUFFER_SIZE_BYTES, buffer.len());
                let mut out: Vec<BufferElement> = Vec::with_capacity(end);

                for (i, b) in buffer[0..end]
                    .iter()
                    .enumerate()
                {
                    let idx = i + offset[1];
                    out.push((idx, *b));
                }
                out
            }
            None => Vec::default(),
        }
    }

    fn next(&mut self, offset: Option<Vec<usize>>) -> Vec<BufferElement> {
        use std::cmp::min;

        let offset = offset.unwrap();
        let buffer = self.buffers.get(&self.active_buffer).unwrap();
        let page_idx = self.page_indices.get(&self.active_buffer).unwrap() + 1;
        let page_offset = page_idx * IO_BUFFER_SIZE_BYTES;
        let end = min(page_offset + IO_BUFFER_SIZE_BYTES, buffer.len());
        let mut out: Vec<BufferElement> = Vec::with_capacity(end);

        for (i, b) in buffer[page_offset..end]
            .iter()
            .enumerate()
        {
            let idx = i + offset[0];
            out.push((idx, *b));
        }
        *self.page_indices.get_mut(&self.active_buffer).unwrap() = page_idx;
        out
    }
}
