use std::future::Future;
use std::io::BufReader;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

use serde::{de::DeserializeOwned, Deserialize};

use assemblylift_core_io_common::constants::{FUNCTION_INPUT_BUFFER_SIZE, IO_BUFFER_SIZE_BYTES};

/// The ABI exported by the AssemblyLift runtime host
extern "C" {
    // IO
    fn __asml_abi_io_poll(id: u32) -> i32;
    fn __asml_abi_io_len(id: u32) -> u32;
    fn __asml_abi_io_load(id: u32) -> i32;
    fn __asml_abi_io_next() -> i32;

    // System clock
    fn __asml_abi_clock_time_get() -> u64;

    // Console
    fn __asml_abi_runtime_log(ptr: *const u8, len: usize);

    // Function Input
    fn __asml_abi_input_start() -> i32;
    fn __asml_abi_input_next() -> i32;
    fn __asml_abi_input_length_get() -> u64;

    // Z85
    fn __asml_expabi_z85_encode(ptr: *const u8, len: usize, out_ptr: *const u8) -> i32;
    fn __asml_expabi_z85_decode(ptr: *const u8, len: usize, out_ptr: *const u8) -> i32;
}

#[doc(hidden)]
pub static mut IO_BUFFER: [u8; IO_BUFFER_SIZE_BYTES] = [0; IO_BUFFER_SIZE_BYTES];

#[no_mangle]
#[doc(hidden)]
pub fn __asml_guest_get_io_buffer_pointer() -> *const u8 {
    unsafe { IO_BUFFER.as_ptr() }
}

fn console_log(message: String) {
    unsafe { __asml_abi_runtime_log(message.as_ptr(), message.len()) }
}

/// Get the host clock time in seconds since UNIX epoch
pub fn get_time() -> u64 {
    unsafe { __asml_abi_clock_time_get() }
}

/// A struct representing data returned by an IOmod call.
/// An IoDocument is initialized with the IOID of the call the document "belongs" to. On `new` the
/// first page of data returned by the call is loaded into `IO_BUFFER`; `read()` is expected to be called
/// immediately to continue paging data in. Initializing another document with `new` will cause the
/// data of that call to overwrite the existing data in `IO_BUFFER`.
pub struct IoDocument {
    bytes_read: usize,
    pages_read: usize,
    length: usize,
}

impl IoDocument {
    /// Create a new document for call ID `ioid`
    pub fn new(ioid: u32) -> Self {
        unsafe { __asml_abi_io_load(ioid) };
        Self {
            bytes_read: 0,
            pages_read: 0,
            length: unsafe { __asml_abi_io_len(ioid) } as usize,
        }
    }

    /// Get the length of the document
    pub fn len(&self) -> usize {
        self.length
    }
}

impl std::io::Read for IoDocument {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        let mut bytes_read = 0usize;
        if self.bytes_read < self.length {
            for idx in 0..std::cmp::min(self.length, buf.len()) {
                // unsafe: bytes_read is always positive, mod IO_BUFFER_SIZE_BYTES
                //         is always less than IO_BUFFER_SIZE_BYTES
                buf[idx] = unsafe { IO_BUFFER[self.bytes_read % IO_BUFFER_SIZE_BYTES] };
                bytes_read += 1;
                self.bytes_read += 1;
                if self.bytes_read % IO_BUFFER_SIZE_BYTES == 0 {
                    unsafe { __asml_abi_io_next() };
                    self.pages_read += 1;
                }
            }
        }
        Ok(bytes_read)
    }
}

#[derive(Clone)]
/// A handle implementing `std::future::Future` for an in-flight IOmod call
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

impl<'a, R> Future for Io<'_, R>
where
    R: DeserializeOwned,
{
    type Output = R;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match unsafe { __asml_abi_io_poll(self.id) } {
            1 => Poll::Ready(read_response::<Self::Output>(self.id).unwrap()),
            _ => {
                self.waker = Box::new(Some(cx.waker().clone()));
                Poll::Pending
            }
        }
    }
}

fn read_response<'a, T>(id: u32) -> Option<T>
where
    T: DeserializeOwned,
{
    let doc = IoDocument::new(id);
    let doc = BufReader::with_capacity(doc.len(), doc);
    match serde_json::from_reader::<BufReader<IoDocument>, T>(doc) {
        Ok(response) => Some(response),
        Err(why) => {
            console_log(format!("[ERROR] ioid={} {}", id, why.to_string()));
            None
        }
    }
}

// Function Input Buffer

#[doc(hidden)]
pub static mut FUNCTION_INPUT_BUFFER: [u8; FUNCTION_INPUT_BUFFER_SIZE] =
    [0; FUNCTION_INPUT_BUFFER_SIZE];

// provided TO the wasm runtime (host)
#[no_mangle]
pub fn __asml_guest_get_function_input_buffer_pointer() -> *const u8 {
    unsafe { FUNCTION_INPUT_BUFFER.as_ptr() }
}

/// A struct representing the Function Input Buffer (FIB).
/// Reads from the host via the `input` ABI function group.
pub struct FunctionInputBuffer {
    bytes_read: usize,
    pages_read: usize,
    length: usize,
}

impl FunctionInputBuffer {
    pub fn new() -> Self {
        unsafe { __asml_abi_input_start() };
        Self {
            bytes_read: 0usize,
            pages_read: 0usize,
            length: unsafe { __asml_abi_input_length_get() as usize },
        }
    }
}

impl std::io::Read for FunctionInputBuffer {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        let mut bytes_read = 0usize;
        if self.bytes_read < self.length {
            for idx in 0..std::cmp::min(self.length, buf.len()) {
                // unsafe: bytes_read is always positive, mod FUNCTION_INPUT_BUFFER_SIZE
                //         is always less than FUNCTION_INPUT_BUFFER_SIZE
                buf[idx] =
                    unsafe { FUNCTION_INPUT_BUFFER[self.bytes_read % FUNCTION_INPUT_BUFFER_SIZE] };
                bytes_read += 1;
                self.bytes_read += 1;
                if self.bytes_read % FUNCTION_INPUT_BUFFER_SIZE == 0 {
                    unsafe { __asml_abi_input_next() };
                    self.pages_read += 1;
                }
            }
        }
        Ok(bytes_read)
    }
}
