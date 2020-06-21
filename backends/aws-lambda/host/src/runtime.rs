use std::env;
use std::io::{Error, ErrorKind};

use reqwest::blocking;

use crate::LAMBDA_REQUEST_ID;

// https://docs.aws.amazon.com/lambda/latest/dg/runtimes-api.html

pub struct AwsLambdaEvent {
    pub request_id: String,
    pub event_body: String,
}

pub struct AwsLambdaRuntime {
    client: blocking::Client,
    api_endpoint: String,
}

impl AwsLambdaRuntime {
    pub fn new() -> AwsLambdaRuntime {
        AwsLambdaRuntime {
            client: blocking::Client::new(),
            api_endpoint: env::var("AWS_LAMBDA_RUNTIME_API").unwrap(),
        }
    }

    pub fn get_next_event(&self) -> Result<AwsLambdaEvent, Error> {
        let url = &format!(
            "http://{}/2018-06-01/runtime/invocation/next",
            self.api_endpoint
        )
        .to_string();

        self.client
            .get(url)
            .send()
            .map(|res| {
                let request_id = res.headers()["Lambda-Runtime-Aws-Request-Id"]
                    .to_str()
                    .unwrap()
                    .to_string();

                AwsLambdaEvent {
                    request_id,
                    event_body: res.text().unwrap(),
                }
            })
            .map_err(|err| Error::new(ErrorKind::Other, err.to_string()))
    }

    pub fn respond(&self, response: String) -> Result<(), Error> {
        let ref_cell = LAMBDA_REQUEST_ID.lock().unwrap();
        let request_id = ref_cell.borrow();
        let url = &format!(
            "http://{}/2018-06-01/runtime/invocation/{}/response",
            self.api_endpoint, request_id
        )
        .to_string();

        println!("Responding to APIGW endpoint: {}", url);

        self.client
            .post(url)
            .body(response)
            .send()
            .map(|_| ())
            .map_err(|err| Error::new(ErrorKind::Other, err.to_string()))
    }
}
