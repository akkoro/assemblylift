use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

use serde::Deserialize;
use serde_json;

use assemblylift_core_io_common::constants::{FUNCTION_INPUT_BUFFER_SIZE, IO_BUFFER_SIZE_BYTES};

extern "C" {
    // IO
    fn __asml_abi_poll(id: u32) -> i32; // TODO rename __asml_abi_io_poll for consistency in prefixing
    fn __asml_abi_io_ptr(id: u32) -> u32;
    fn __asml_abi_io_len(id: u32) -> u32;

    // System clock
    fn __asml_abi_clock_time_get() -> u64;

    // Console
    fn __asml_abi_console_log(ptr: *const u8, len: usize);

    // Input
    fn __asml_abi_input_start() -> i32;
    fn __asml_abi_input_next() -> i32;
    fn __asml_abi_input_length_get() -> u64;

    // Z85
    fn __asml_expabi_z85_encode(ptr: *const u8, len: usize, out_ptr: *const u8) -> i32;
    pub fn __asml_expabi_z85_decode(ptr: *const u8, len: usize, out_ptr: *const u8) -> i32;
}

// Raw buffer holding serialized IO data
pub static mut IO_BUFFER: [u8; IO_BUFFER_SIZE_BYTES] = [0; IO_BUFFER_SIZE_BYTES];

fn console_log(message: String) {
    unsafe { __asml_abi_console_log(message.as_ptr(), message.len()) }
}

pub fn get_time() -> u64 {
    unsafe { __asml_abi_clock_time_get() }
}

#[derive(Clone)]
pub struct Io<'a, R> {
    pub id: u32,
    waker: Box<Option<Waker>>,
    _phantom: PhantomData<&'a R>,
}

impl<'a, R: Deserialize<'a>> Io<'_, R> {
    pub fn new(id: u32) -> Self {
        Io {
            id,
            waker: Box::new(None),
            _phantom: PhantomData,
        }
    }
}

impl<'a, R: Deserialize<'a>> Future for Io<'_, R> {
    type Output = R;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match unsafe { __asml_abi_poll(self.id) } {
            1 => Poll::Ready(unsafe { read_response::<Self::Output>(self.id).unwrap() }),
            _ => {
                self.waker = Box::new(Some(cx.waker().clone()));
                Poll::Pending
            }
        }
    }
}

unsafe fn read_response<'a, R: Deserialize<'a>>(id: u32) -> Option<R> {
    let ptr = __asml_abi_io_ptr(id) as usize;
    let end = __asml_abi_io_len(id) as usize + ptr;

    match serde_json::from_slice::<R>(&IO_BUFFER[ptr..end]) {
        Ok(response) => Some(response),
        Err(why) => {
            console_log(format!("[ERROR] {}", why.to_string()));
            None
        }
    }
}

// Function Input Buffer

pub static mut FUNCTION_INPUT_BUFFER: [u8; FUNCTION_INPUT_BUFFER_SIZE] =
    [0; FUNCTION_INPUT_BUFFER_SIZE];

// provided TO the wasm runtime (host)
#[no_mangle]
pub fn __asml_guest_get_function_input_buffer_pointer() -> *const u8 {
    unsafe { FUNCTION_INPUT_BUFFER.as_ptr() }
}

pub struct FunctionInputBuffer {
    total_bytes_read: usize,
    window_start: usize,
    window_end: usize,
}

impl FunctionInputBuffer {
    pub fn new() -> Self {
        unsafe { __asml_abi_input_start() };
        Self { total_bytes_read: 0usize, window_start: 0usize, window_end: 0usize }
    }
}

impl std::io::Read for FunctionInputBuffer {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        // console_log(format!("WASM: buf.len={}", buf.len()));
        let is_buffer_short: bool = buf.len() < FUNCTION_INPUT_BUFFER_SIZE;
        return if is_buffer_short {
            // if buffer is short, we need windowed read
            if self.window_end == 0 {
                self.window_end = buf.len();
            }
            let mut bytes_read: usize = 0;
            for (i, wi) in (self.window_start..self.window_end).enumerate() {
                if self.total_bytes_read >= unsafe { __asml_abi_input_length_get() as usize } {
                    break;
                }
                if wi < FUNCTION_INPUT_BUFFER_SIZE {
                    buf[i] = unsafe { FUNCTION_INPUT_BUFFER[wi] };
                    bytes_read += 1;
                    self.total_bytes_read += 1;
                } else {
                    let r = unsafe { __asml_abi_input_next() };
                    if r == -1 {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "host could not get next input chunk",
                        ));
                    }
                    self.window_start = 1usize;
                    self.window_end = buf.len() + 1usize;
                    buf[i] = unsafe { FUNCTION_INPUT_BUFFER[0] };
                    bytes_read += 1;
                    self.total_bytes_read += 1;
                    return Ok(bytes_read)
                }
            }
            self.window_start = self.window_end;
            self.window_end = self.window_end + buf.len();
            // console_log(format!("WASM: bytes_read={}", bytes_read));
            Ok(bytes_read)
        } else {
            // else we can read whole FIB on this read
            let mut bytes_read: usize = 0;
            for idx in 0..FUNCTION_INPUT_BUFFER_SIZE {
                if self.total_bytes_read >= unsafe { __asml_abi_input_length_get() as usize } {
                    return Ok(bytes_read);
                }
                buf[idx] = unsafe { FUNCTION_INPUT_BUFFER[idx] };
                bytes_read += 1;
                self.total_bytes_read += 1;
            }
            let r = unsafe { __asml_abi_input_next() };
            if r == -1 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "host could not get next input chunk",
                ));
            }
            // console_log(format!("WASM: bytes_read={}", bytes_read));
            Ok(bytes_read)
        }
    }
}

// FIXME This requires some manual intervention above as the abi calls are not mocked.
//       For the same reason, this doesn't properly test the functionality around input_next().
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn short_buffer_read() {
        // Setup
        let mut FIB = [0u8; FUNCTION_INPUT_BUFFER_SIZE];
        {
            let s = "hello world! this is a test";
            for (i, c) in s.as_bytes().iter().enumerate() {
                 FIB[i] = *c;
            }
        }

        let mut buf = [0; 13];
        let mut fib = FunctionInputBuffer::new();
        
        let n = fib.read(&mut buf).unwrap();
        assert_eq!(n, 13);
        assert_eq!(buf[0..n], FIB[0..13]);
        
        let n = fib.read(&mut buf).unwrap();
        assert_eq!(n, 13);
        assert_eq!(buf[0..n], FIB[13..26]);
        
        let n = fib.read(&mut buf).unwrap();
        assert_eq!(n, 1);
        assert_eq!(buf[0..n], FIB[26..27]);
    }

    #[test]
    fn long_buffer_read() {
        // Setup
        let mut FIB = [0u8; FUNCTION_INPUT_BUFFER_SIZE];
        {
            let s = "hello world! this is a test";
            for (i, c) in s.as_bytes().iter().enumerate() {
                 FIB[i] = *c;
            }
        }

        let mut buf = [0; FUNCTION_INPUT_BUFFER_SIZE];
        let mut fib = FunctionInputBuffer::new();
        let n = fib.read(&mut buf).unwrap();
        
        assert_eq!(n, 27);
        assert_eq!(buf[0..n], FIB[0..27]);
    }
}
