#[macro_use]
extern crate asml_awslambda;

use direct_executor;
use asml_core::GuestCore;
use asml_awslambda::{*, AwsLambdaClient, LambdaContext};

handler!(context: LambdaContext, async {
    AwsLambdaClient::console_log("test".to_string())
});
