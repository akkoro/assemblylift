use crossbeam_utils::atomic::AtomicCell;
use assemblylift_core_io_common::constants::{FUNCTION_INPUT_BUFFER_SIZE, IO_BUFFER_SIZE_BYTES};

use crate::threader::ThreaderEnv;

pub trait LinearBuffer {
    fn initialize(&mut self, buffer: Vec<u8>);
    fn write(&mut self, bytes: &[u8], at_offset: usize) -> usize;
    fn erase(&mut self, offset: usize, len: usize) -> usize;
    fn len(&self) -> usize;
    fn capacity(&self) -> usize;
}

pub trait WasmBuffer {
    fn copy_to_wasm(&self, env: &ThreaderEnv, src: (usize, usize), dst: (usize, usize)) -> Result<(), ()>;
}

pub trait PagedWasmBuffer: WasmBuffer {
    fn first(&mut self, env: &ThreaderEnv, offset: Option<usize>) -> i32;
    fn next(&mut self, env: &ThreaderEnv) -> i32;
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
    fn first(&mut self, env: &ThreaderEnv, _offset: Option<usize>) -> i32 {
        let end: usize = match self.buffer.len() < FUNCTION_INPUT_BUFFER_SIZE {
            true => self.buffer.len(),
            false => FUNCTION_INPUT_BUFFER_SIZE,
        };
        self.copy_to_wasm(env, (0usize, end), (0usize, FUNCTION_INPUT_BUFFER_SIZE)).unwrap();
        self.page_idx = 0usize;
        0
    }

    fn next(&mut self, env: &ThreaderEnv) -> i32 {
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

impl WasmBuffer for FunctionInputBuffer {
    fn copy_to_wasm(&self, env: &ThreaderEnv, src: (usize, usize), dst: (usize, usize)) -> Result<(), ()> {
        let wasm_memory = env.memory_ref().unwrap();
        let input_buffer = env
            .get_function_input_buffer
            .get_ref()
            .unwrap()
            .call()
            .unwrap();
        let memory_writer: &[AtomicCell<u8>] = input_buffer
            .deref(
                &wasm_memory,
                dst.0 as u32,
                dst.1 as u32,
            )
            .unwrap();

        for (i, b) in self.buffer[src.0..src.1].iter().enumerate() {
            let idx = i + dst.0;
            memory_writer[idx].store(*b);
        }

        Ok(())
    }
}

pub struct IoBuffer {
    buffer: Vec<u8>,
    page_idx: usize,
}

impl IoBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            page_idx: 0usize,
        }
    }

    pub fn double(&mut self) {
        self.buffer.reserve(self.buffer.capacity());
    }

    #[inline(always)]
    fn push(&mut self, idx: usize) {
        if idx > self.buffer.len() {
            let diff = idx - self.buffer.len();
            for _ in 0..diff {
                self.buffer.push(0)
            }
        }
    }

    fn set_byte(&mut self, byte: u8, idx: usize) {
        self.push(idx);
        self.buffer[idx] = byte;
    }

    fn erase_byte(&mut self, idx: usize) {
        self.push(idx);
        self.buffer[idx] = 0;
    }
}

impl LinearBuffer for IoBuffer {
    fn initialize(&mut self, buffer: Vec<u8>) {
        self.buffer = buffer;
    }

    fn write(&mut self, bytes: &[u8], at_offset: usize) -> usize {
        println!("DEBUG: writing {} bytes from {}", bytes.len(), at_offset);
        let mut bytes_written = 0usize;
        for idx in at_offset..bytes.len() {
            self.set_byte(bytes[idx - at_offset], idx);
            bytes_written += 1;
        }
        bytes_written
    }
    
    fn erase(&mut self, offset: usize, len: usize) -> usize {
        println!("DEBUG: erasing bytes from {} to {}", offset, offset + len);
        let mut bytes_erased = 0usize;
        for idx in offset..len {
            self.erase_byte(idx);
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

impl PagedWasmBuffer for IoBuffer {
    fn first(&mut self, env: &ThreaderEnv, offset: Option<usize>) -> i32 {
        use std::cmp::min;
        let offset = offset.unwrap_or(0);
        if offset > IO_BUFFER_SIZE_BYTES {
            self.page_idx = (offset as f32 / IO_BUFFER_SIZE_BYTES as f32).floor() as usize;
        } else {
            self.page_idx = 0usize;
        }

        let page_offset = self.page_idx * IO_BUFFER_SIZE_BYTES;
        self.copy_to_wasm(
            env, 
            (page_offset, min(page_offset + IO_BUFFER_SIZE_BYTES, self.buffer.len())), 
            (0usize, IO_BUFFER_SIZE_BYTES),
        ).unwrap();
        0
    }

    fn next(&mut self, env: &ThreaderEnv) -> i32 {
        use std::cmp::min;
        if self.buffer.len() > IO_BUFFER_SIZE_BYTES {
            self.page_idx += 1;
            let page_offset = self.page_idx * IO_BUFFER_SIZE_BYTES;
            self.copy_to_wasm(
                env, 
                (page_offset, min(page_offset + IO_BUFFER_SIZE_BYTES, self.buffer.len())), 
                (0usize, IO_BUFFER_SIZE_BYTES),
            ).unwrap();
        }
        0
    }
}

impl WasmBuffer for IoBuffer {
    fn copy_to_wasm(&self, env: &ThreaderEnv, src: (usize, usize), dst: (usize, usize)) -> Result<(), ()> {
        let wasm_memory = env.memory_ref().unwrap();
        let io_buffer = env
            .get_io_buffer
            .get_ref()
            .unwrap()
            .call()
            .unwrap();
        let memory_writer: &[AtomicCell<u8>] = io_buffer
            .deref(
                &wasm_memory,
                dst.0 as u32,
                dst.1 as u32,
            )
            .unwrap();

        for (i, b) in self.buffer[src.0..src.1].iter().enumerate() {
            let idx = i + dst.0;
            memory_writer[idx].store(*b);
        }

        Ok(())
    }
}
