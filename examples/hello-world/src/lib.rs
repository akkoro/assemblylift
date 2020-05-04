extern crate guest;
use awsio::*;
use guest::*;
use core_guest::*;

use wasm_bindgen_futures::spawn_local;

#[no_mangle]
pub fn handler() -> i32 {
    AwsLambdaClient::console_log("Started handler...".to_string());

    let _client = AwsLambdaClient::new();
    let _event = get_lambda_event();

    spawn_local(async {
        AwsLambdaClient::console_log("Calling...".to_string());
        let ret = database::aws_dynamodb_list_tables().unwrap().await;
        AwsLambdaClient::console_log("IO complete!".to_string());
    });

    0
}
