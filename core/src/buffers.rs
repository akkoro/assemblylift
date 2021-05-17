use std::sync::{Arc, Mutex};

use crossbeam_utils::atomic::AtomicCell;

use assemblylift_core_io_common::constants::FUNCTION_INPUT_BUFFER_SIZE;

pub struct FunctionInputBuffer {
    buffer: Vec<u8>,
    buffer_idx: usize,
    env: Option<Arc<Mutex<crate::threader::ThreaderEnv>>>,
}

impl FunctionInputBuffer {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            buffer_idx: 0usize,
            env: None,
        }
    }

    pub fn set_env(&mut self, env: Arc<Mutex<crate::threader::ThreaderEnv>>) {
        self.env = Some(env);
    }

    pub fn set_buffer(&mut self, buffer: Vec<u8>) {
        self.buffer = buffer;
    }

    pub fn start(&mut self) -> i32 {
        let end: usize = match self.buffer.len() < FUNCTION_INPUT_BUFFER_SIZE {
            true => self.buffer.len(),
            false => FUNCTION_INPUT_BUFFER_SIZE,
        };
        self.write_wasm_buffer(
            &self.buffer[0..end],
        );
        self.buffer_idx = 0usize;
        0
    }

    pub fn next(&mut self) -> i32 {
        if self.buffer.len() > FUNCTION_INPUT_BUFFER_SIZE {
            self.buffer_idx += 1;
            self.write_wasm_buffer(
                &self.buffer[FUNCTION_INPUT_BUFFER_SIZE * self.buffer_idx
                    ..FUNCTION_INPUT_BUFFER_SIZE * (self.buffer_idx + 1)],
            );
        }
        0
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    #[inline(always)]
    fn write_wasm_buffer(&self, input: &[u8]) {
        let env = self.env
            .as_ref()
            .unwrap()
            .clone();
        let env = env
            .lock()
            .unwrap();
        let input_buffer = env
            .get_function_input_buffer
            .get_ref()
            .unwrap()
            .call()
            .unwrap();
        let wasm_memory = env.memory.get_ref().unwrap();
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

