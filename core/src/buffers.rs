use crossbeam_utils::atomic::AtomicCell;
use assemblylift_core_io_common::constants::{FUNCTION_INPUT_BUFFER_SIZE, IO_BUFFER_SIZE_BYTES};

use crate::threader::ThreaderEnv;

pub trait PagedBuffer {
    fn initialize(&mut self, buffer: Vec<u8>);
    fn first(&mut self, env: &ThreaderEnv) -> i32;
    fn next(&mut self, env: &ThreaderEnv) -> i32;
    fn len(&self) -> usize;
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

    // TODO should this be here or outside as a pub fn
    fn write_wasm_buffer(&self, env: &ThreaderEnv, input: &[u8]) {
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
                0,
                FUNCTION_INPUT_BUFFER_SIZE as u32,
            )
            .unwrap();

        for (i, b) in input.iter().enumerate() {
            memory_writer[i].store(*b);
        }
    }
}

impl PagedBuffer for FunctionInputBuffer {
    fn initialize(&mut self, buffer: Vec<u8>) {
        self.buffer = buffer;
        println!("DEBUG: FunctionInputBuffer len={}", self.buffer.len());
    }

    fn first(&mut self, env: &ThreaderEnv) -> i32 {
        let end: usize = match self.buffer.len() < FUNCTION_INPUT_BUFFER_SIZE {
            true => self.buffer.len(),
            false => FUNCTION_INPUT_BUFFER_SIZE,
        };
        self.write_wasm_buffer(
            env,
            &self.buffer[0..end],
        );
        self.page_idx = 0usize;
        0
    }

    fn next(&mut self, env: &ThreaderEnv) -> i32 {
        if self.buffer.len() > FUNCTION_INPUT_BUFFER_SIZE {
            self.page_idx += 1;
            self.write_wasm_buffer(
                env,
                &self.buffer[FUNCTION_INPUT_BUFFER_SIZE * self.page_idx
                    ..std::cmp::min(FUNCTION_INPUT_BUFFER_SIZE * (self.page_idx + 1), self.buffer.len())],
            );
        }
        0
    }

    fn len(&self) -> usize {
        self.buffer.len()
    }
}

pub struct IoBuffer {
    buffer: Vec<u8>,
    page_idx: usize,
}

impl IoBuffer {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            page_idx: 0usize,
        }
    }
    
    fn write_wasm_buffer(&self, env: &ThreaderEnv, input: &[u8]) {
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
                0,
                IO_BUFFER_SIZE_BYTES as u32,
            )
            .unwrap();

        for (i, b) in input.iter().enumerate() {
            memory_writer[i].store(*b);
        }
    }
}

impl PagedBuffer for IoBuffer {
    fn initialize(&mut self, buffer: Vec<u8>) {
        self.buffer = buffer;
    }

    fn first(&mut self, env: &ThreaderEnv) -> i32 {
        let end: usize = match self.buffer.len() < IO_BUFFER_SIZE_BYTES {
            true => self.buffer.len(),
            false => IO_BUFFER_SIZE_BYTES,
        };
        self.write_wasm_buffer(
            env,
            &self.buffer[0..end],
        );
        self.page_idx = 0usize;
        0
    }

    fn next(&mut self, env: &ThreaderEnv) -> i32 {
        if self.buffer.len() > IO_BUFFER_SIZE_BYTES {
            self.page_idx += 1;
            self.write_wasm_buffer(
                env,
                &self.buffer[IO_BUFFER_SIZE_BYTES * self.page_idx
                    ..std::cmp::min(IO_BUFFER_SIZE_BYTES * (self.page_idx + 1), self.buffer.len())],
            );
        }
        0
    }

    fn len(&self) -> usize {
        self.buffer.len()
    }
}
