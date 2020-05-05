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

    // let mut exec = EventExecutor::new();

    // exec.spawn(async {
    //     AwsLambdaClient::console_log("Calling...".to_string());
    //     database::aws_dynamodb_list_tables().unwrap().await;
    //     AwsLambdaClient::console_log("IO complete!".to_string());
    // });

    // exec.run();

    run_spinning(async {
        AwsLambdaClient::console_log("Calling...".to_string());
        database::aws_dynamodb_list_tables().unwrap().await;
        AwsLambdaClient::console_log("IO complete!".to_string());
    });

    0
}
