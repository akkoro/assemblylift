extern crate guest;
use tokio::prelude::*;
use tokio::runtime::Runtime;
use awsio::*;
use guest::*;
use core_guest::*;

#[no_mangle]
pub fn handler() -> i32 {
    let _client = AwsLambdaClient::new();
    let _event = get_lambda_event();

    let mut rt = Runtime::new().unwrap();
    rt.block_on(async {
        let ret = database::aws_dynamodb_list_tables().unwrap().await;
        AwsLambdaClient::console_log("Tables:".to_string());
    });

    0
}
