use std::cell::RefCell;
use std::env;
use std::io::{Error, ErrorKind};

use crossbeam_utils::atomic::AtomicCell;
use reqwest::blocking;

// FIXME this is a kludge -- I have no real idea if this is safe
pub struct SendString(String);
unsafe impl Send for SendString {}

// https://docs.aws.amazon.com/lambda/latest/dg/runtimes-api.html

pub struct AwsLambdaEvent {
    pub request_id: String,
    pub event_body: String,
}

pub struct AwsLambdaRuntime {
    client: blocking::Client,
    api_endpoint: String,

    pub current_request_id: SendString,
}

impl AwsLambdaRuntime {
    pub fn new() -> AwsLambdaRuntime {
        AwsLambdaRuntime {
            client: blocking::Client::new(),
            api_endpoint: env::var("AWS_LAMBDA_RUNTIME_API").unwrap(),
            current_request_id: SendString { 0: String::new() },
        }
    }

    pub fn get_next_event(&self) -> Result<AwsLambdaEvent, Error> {
        let url = &format!("http://{}/2018-06-01/runtime/invocation/next", self.api_endpoint).to_string();
        self.client
            .get(url)
            .send()
            .map(|res| {
                AwsLambdaEvent {
                    request_id: res.headers()["Lambda-Runtime-Aws-Request-Id"].to_str().unwrap().to_string(),
                    event_body: res.text().unwrap(),
                }
            })
            .map_err(|err| Error::new(ErrorKind::Other, err.to_string()))
    }

    pub fn respond(&self, request_id: &SendString, response: String) -> Result<(), Error> {
        let url = &format!("http://{}/2018-06-01/runtime/invocation/{}/response", self.api_endpoint, request_id.0).to_string();
        println!("Responding to APIGW endpoint: {}", url);
        self.client
            .post(url)
            .body(response)
            .send()
            .map(|_| ())
            .map_err(|err| Error::new(ErrorKind::Other, err.to_string()))
    }
}
