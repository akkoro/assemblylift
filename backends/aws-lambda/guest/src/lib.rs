extern crate assemblylift_core_event_guest;
extern crate assemblylift_core_guest;

use assemblylift_core_guest::*;
use serde::{Deserialize, Serialize};
use serde_json;

const AWS_EVENT_STRING_BUFFER_SIZE: usize = 2048;
static mut AWS_EVENT_STRING_BUFFER: [u8; AWS_EVENT_STRING_BUFFER_SIZE] = [0; AWS_EVENT_STRING_BUFFER_SIZE];

// provided TO the wasm runtime (host)
#[no_mangle]
pub fn __al_get_aws_event_string_buffer_pointer() -> *const u8 {
    unsafe { AWS_EVENT_STRING_BUFFER.as_ptr() }
}

// these are provided BY the wasm runtime (host)
extern {
    fn __asml_abi_console_log(ptr: *const u8, len: usize);
    fn __asml_abi_success(ptr: *const u8, len: usize);
}

pub fn get_lambda_event() -> String {
    unsafe { std::str::from_utf8(&AWS_EVENT_STRING_BUFFER[..AWS_EVENT_STRING_BUFFER_SIZE]).unwrap().to_string() }
}

pub struct AwsLambdaClient(Guest);

impl AwsLambdaClient {
    pub fn new() -> AwsLambdaClient {
        AwsLambdaClient { 0: Guest {} }
    }
}

impl GuestCore for AwsLambdaClient {
    fn console_log(message: String) {
        unsafe { __asml_abi_console_log(message.as_ptr(), message.len()) }
    }

    fn success(response: String) {
        unsafe { __asml_abi_success(response.as_ptr(), response.len()) }
    }
}

#[derive(Serialize, Deserialize, Clone, std::fmt::Debug)]
pub struct ApiGatewayEvent {
    pub body: Option<String>
}

pub struct LambdaContext {
    pub client: AwsLambdaClient,
    pub event: ApiGatewayEvent,
}

#[macro_export]
macro_rules! handler {
    ($context:ident: $type:ty, $async_handler:expr) => {
        #[no_mangle]
        pub fn handler() -> i32 {

            AwsLambdaClient::console_log("Started handler...".to_string());

            let client = AwsLambdaClient::new();
            let event: ApiGatewayEvent = serde_json::from_str(&get_lambda_event()).unwrap();
            let $context: $type = LambdaContext { client, event };

            direct_executor::run_spinning($async_handler);

            0
        }
    }
}
