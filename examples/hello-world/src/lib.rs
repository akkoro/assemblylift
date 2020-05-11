#[macro_use]
extern crate guest;
extern crate core_event_guest;
use awsio::*;
use guest::*;
use core_guest::GuestCore;

handler!(context, async {
    AwsLambdaClient::console_log(format!("Got event: {}", context.event));

    AwsLambdaClient::console_log("Calling...".to_string());
    let ret = database::aws_dynamodb_list_tables().await;
    AwsLambdaClient::console_log("IO complete!".to_string());
    AwsLambdaClient::console_log(format!("Got {:?}", ret).to_string());
});
