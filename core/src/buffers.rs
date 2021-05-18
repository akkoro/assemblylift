use std::sync::{Arc, Mutex};
use crossbeam_utils::atomic::AtomicCell;
use assemblylift_core_io_common::constants::FUNCTION_INPUT_BUFFER_SIZE;

use crate::threader::ThreaderEnv;

pub struct FunctionInputBuffer {
    buffer: Vec<u8>,
    buffer_idx: usize,
}

impl FunctionInputBuffer {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            buffer_idx: 0usize,
        }
    }

    pub fn set_buffer(&mut self, buffer: Vec<u8>) {
        self.buffer = buffer;
        println!("DEBUG: set_buffer len={}", self.buffer.len());
    }

    pub fn start(&mut self, env: &ThreaderEnv) -> i32 {
        let end: usize = match self.buffer.len() < FUNCTION_INPUT_BUFFER_SIZE {
            true => self.buffer.len(),
            false => FUNCTION_INPUT_BUFFER_SIZE,
        };
        self.write_wasm_buffer(
            env,
            &self.buffer[0..end],
        );
        self.buffer_idx = 0usize;
        0
    }

    pub fn next(&mut self, env: &ThreaderEnv) -> i32 {
        if self.buffer.len() > FUNCTION_INPUT_BUFFER_SIZE {
            self.buffer_idx += 1;
            self.write_wasm_buffer(
                env,
                &self.buffer[FUNCTION_INPUT_BUFFER_SIZE * self.buffer_idx
                    ..std::cmp::min(FUNCTION_INPUT_BUFFER_SIZE * (self.buffer_idx + 1), self.buffer.len())],
            );
        }
        0
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

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

