extern crate assemblylift_core;
use assemblylift_core::client::*;

const EVENT_BUFFER_SIZE: usize = 2048;
static mut EVENT_BUFFER: [u8; EVENT_BUFFER_SIZE] = [0; EVENT_BUFFER_SIZE];

// provided to the wasm runtime for memory sharing
#[no_mangle]
pub fn __al_get_event_buffer_pointer() -> *const u8 {
    unsafe { EVENT_BUFFER.as_ptr() }
}

// these are provided by the wasm runtime (host)
extern {
    fn __al_console_log(ptr: *const u8, len: usize);
    fn __al_success(ptr: *const u8, len: usize);
}

pub fn get_event() -> String {
    unsafe { std::str::from_utf8(&EVENT_BUFFER[..EVENT_BUFFER_SIZE]).unwrap().to_string() }
}

pub struct AwsLambdaClient(Client);

impl AwsLambdaClient {
    pub fn new() -> AwsLambdaClient {
        AwsLambdaClient { 0: Client {} }
    }
}

impl ClientCore for AwsLambdaClient {
    fn console_log(message: String) {
        unsafe { __al_console_log(message.as_ptr(), message.len()) }
    }

    fn success(response: String) {
        unsafe { __al_success(response.as_ptr(), response.len()) }
    }

    fn test(&self) {
        self.0.get_response();
    }
}

// TODO: parse string into struct
// pub fn aws_dynamodb_list_tables() -> &'static str {
//     unsafe { __aws_dynamodb_list_tables() }
//     get_response()
// }
