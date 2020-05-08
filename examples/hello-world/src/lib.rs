extern crate guest;
extern crate core_event_guest;
use awsio::*;
use guest::*;
use core_guest::*;

use direct_executor::run_spinning;

#[no_mangle]
pub fn handler() -> i32 {
    AwsLambdaClient::console_log("Started handler...".to_string());

    let _client = AwsLambdaClient::new();
    let _event = get_lambda_event();

    run_spinning(async {
        AwsLambdaClient::console_log("Calling...".to_string());
        let ret = database::aws_dynamodb_list_tables().await;
        AwsLambdaClient::console_log("IO complete!".to_string());
        AwsLambdaClient::console_log(format!("Got {:?}", ret).to_string());
    });

    0
}
