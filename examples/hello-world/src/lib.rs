extern crate guest;
use guest::*;
use core_guest::*;

#[no_mangle]
pub fn handler() -> i32 {
    let _client = AwsLambdaClient::new();
    let _event = get_lambda_event();

    AwsLambdaClient::success("OK".to_string());
    return 0;
}
