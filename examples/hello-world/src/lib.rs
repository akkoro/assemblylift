extern crate guest;
// extern crate core_event_guest;
use awsio::*;
use guest::*;
use core_guest::*;

#[no_mangle]
pub fn handler() -> i32 {
    let _client = AwsLambdaClient::new();
    let _event = get_lambda_event();

    AwsLambdaClient::console_log("Hello, World!".to_string());

    // TODO await
    database::aws_dynamodb_list_tables();

    // AwsLambdaClient::success("OK".to_string());

    0
}
