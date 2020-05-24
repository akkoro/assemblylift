extern crate guest;
extern crate core_event_guest;
use serde_json;
use awsio::*;
use guest::*;
use core_guest::GuestCore;

use std::collections::HashMap;

handler!(context: LambdaContext, async {
    AwsLambdaClient::console_log(format!("Got event: {:?}", context.event));

    AwsLambdaClient::console_log("Calling...".to_string());


    let mut item = HashMap::<String, structs::AttributeValue>::new();
    let mut value: structs::AttributeValue = Default::default();
    value.s = Some("hello world".to_string());
    item.insert("pk".to_string(), value);

    let mut input: structs::PutItemInput = Default::default();
    input.table_name = "asml-test".to_string();
    input.item = item;

    let ret = database::aws_dynamodb_put_item(input).await;

    AwsLambdaClient::console_log("IO complete!".to_string());
    AwsLambdaClient::console_log(format!("Got response {:?}", ret).to_string());
});
