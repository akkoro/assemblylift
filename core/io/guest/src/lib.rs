extern crate lazy_static;

use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

use serde::Deserialize;
use serde_json;

use assemblylift_core_io_common::constants::IO_BUFFER_SIZE_BYTES;

extern "C" {
    fn __asml_abi_poll(id: u32) -> i32;
    fn __asml_abi_io_ptr(id: u32) -> u32;
    fn __asml_abi_io_len(id: u32) -> u32;
    fn __asml_abi_clock_time_get() -> u64;

    fn __asml_abi_console_log(ptr: *const u8, len: usize);
}

// Raw buffer holding serialized IO data
pub static mut IO_BUFFER: [u8; IO_BUFFER_SIZE_BYTES] = [0; IO_BUFFER_SIZE_BYTES];

#[no_mangle]
pub fn __asml_get_event_buffer_pointer() -> *const u8 {
    unsafe { IO_BUFFER.as_ptr() }
}

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
