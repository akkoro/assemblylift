extern crate guest;
// extern crate core_event_guest;
use awsio::*;
use guest::*;
use core_guest::*;

extern {
    fn __asml_abi_init(fn_index: u32) -> i32;
}

#[no_mangle]
pub fn test_fn() -> i32 {
    AwsLambdaClient::console_log("from test_fn".to_string());
    0
}

#[no_mangle]
pub fn handler() -> i32 {
    unsafe {
        __asml_abi_init(test_fn as usize as u32);
    }

    let _client = AwsLambdaClient::new();
    let _event = get_lambda_event();

    AwsLambdaClient::console_log("Hello, World!".to_string());

    database::aws_dynamodb_list_tables();

    // AwsLambdaClient::success("OK".to_string());

    0
}
