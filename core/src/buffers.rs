//! AssemblyLift WASM Buffers
//! See [core-buffers doc](../../docs/core-buffers.md) for more details

use std::collections::HashMap;

use assemblylift_core_io_common::constants::{FUNCTION_INPUT_BUFFER_SIZE, IO_BUFFER_SIZE_BYTES};

use crate::wasm::BufferElement;

/// Implement paging data into a `WasmBuffer`
pub trait PagedWasmBuffer {
    fn first(&mut self, buffer_id: usize, offset: usize) -> Vec<BufferElement>;
    fn next(&mut self, offset: usize) -> Vec<BufferElement>;
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

    pub fn set(&mut self, buffer: Vec<u8>) {
        self.buffer = buffer;
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }
}

impl PagedWasmBuffer for FunctionInputBuffer {
    fn first(&mut self, _buffer_id: usize, offset: usize) -> Vec<BufferElement> {
        let end: usize = match self.buffer.len() < FUNCTION_INPUT_BUFFER_SIZE {
            true => self.buffer.len(),
            false => FUNCTION_INPUT_BUFFER_SIZE,
        };
        self.page_idx = 0usize;

        let mut out: Vec<BufferElement> = Vec::with_capacity(end);
        for (i, b) in self.buffer[0..end].iter().enumerate() {
            let idx = i + offset;
            out.push((idx, *b));
        }
        out
    }

    fn next(&mut self, offset: usize) -> Vec<BufferElement> {
        use std::cmp::min;

        let start = FUNCTION_INPUT_BUFFER_SIZE * self.page_idx;
        let end = min(FUNCTION_INPUT_BUFFER_SIZE * (self.page_idx + 1), self.buffer.len());
        let mut out: Vec<BufferElement> = Vec::with_capacity(end);
        if self.buffer.len() > FUNCTION_INPUT_BUFFER_SIZE {
            self.page_idx += 1;
            for (i, b) in self.buffer[start..end].iter().enumerate() {
                let idx = i + offset;
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

    pub fn set(&mut self, ioid: usize, bytes: Vec<u8>) -> usize {
        self.buffers.insert(ioid, bytes.clone());
        bytes.len()
    }
}

impl PagedWasmBuffer for IoBuffer {
    fn first(&mut self, buffer_id: usize, offset: usize) -> Vec<BufferElement> {
        use std::cmp::min;

        self.active_buffer = buffer_id;
        self.page_indices.insert(self.active_buffer, 0usize);
        let buffer = self.buffers.get(&self.active_buffer).unwrap();
        let end = min(IO_BUFFER_SIZE_BYTES, buffer.len());
        let mut out: Vec<BufferElement> = Vec::with_capacity(end);

        for (i, b) in buffer[0..end]
            .iter()
            .enumerate()
        {
            let idx = i + offset;
            out.push((idx, *b));
        }
        out
    }

    fn next(&mut self, offset: usize) -> Vec<BufferElement> {
        use std::cmp::min;

        let buffer = self.buffers.get(&self.active_buffer).unwrap();
        let page_idx = self.page_indices.get(&self.active_buffer).unwrap() + 1;
        let page_offset = page_idx * IO_BUFFER_SIZE_BYTES;
        let end = min(page_offset + IO_BUFFER_SIZE_BYTES, buffer.len());
        let mut out: Vec<BufferElement> = Vec::with_capacity(end);

        for (i, b) in buffer[page_offset..end]
            .iter()
            .enumerate()
        {
            let idx = i + offset;
            out.push((idx, *b));
        }
        *self.page_indices.get_mut(&self.active_buffer).unwrap() = page_idx;
        out
    }
}
