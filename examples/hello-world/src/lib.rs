extern crate clientlib;
use core::client::*;
use clientlib::{AwsLambdaClient as Client, get_event};

#[no_mangle]
pub fn handler() -> i32 {
    let _client = Client::new();
    let _event = get_event();

    Client::success("OK".to_string());
    return 0;
}
