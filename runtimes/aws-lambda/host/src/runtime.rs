use std::env;
use std::io::{Error, ErrorKind};

use reqwest::Client;

use crate::LAMBDA_REQUEST_ID;

// https://docs.aws.amazon.com/lambda/latest/dg/runtimes-api.html

#[derive(Debug)]
pub struct AwsLambdaEvent {
    pub request_id: String,
    pub event_body: String,
}

pub struct AwsLambdaRuntime {
    client: Client,
    api_endpoint: String,
}

impl AwsLambdaRuntime {
    pub fn new() -> AwsLambdaRuntime {
        AwsLambdaRuntime {
            client: Client::new(),
            api_endpoint: env::var("AWS_LAMBDA_RUNTIME_API").unwrap(),
        }
    }

    pub async fn get_next_event(&self) -> Result<AwsLambdaEvent, Error> {
        let url = &format!(
            "http://{}/2018-06-01/runtime/invocation/next",
            self.api_endpoint
        )
        .to_string();

        match self.client.get(url).send().await {
            Ok(res) => {
                let request_id = match res.headers().get("Lambda-Runtime-Aws-Request-Id") {
                    Some(request_id) => request_id.to_str().unwrap().to_string(),
                    None => {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            "missing header \"Lambda-Runtime-Aws-Request-Id\"",
                        ))
                    }
                };

                let event_body = res.text().await.unwrap();

                Ok(AwsLambdaEvent {
                    request_id,
                    event_body,
                })
            }

            Err(why) => Err(Error::new(ErrorKind::Other, why.to_string())),
        }
    }

    pub async fn respond(&self, response: String) -> Result<(), Error> {
        let request_id: String;
        {
            let ref_cell = LAMBDA_REQUEST_ID.lock().unwrap();
            request_id = ref_cell.borrow().clone();
        }
        let url = &format!(
            "http://{}/2018-06-01/runtime/invocation/{}/response",
            self.api_endpoint, request_id
        )
        .to_string();

        match self.client.post(url).body(response).send().await {
            Ok(_) => Ok(()),
            Err(why) => Err(Error::new(ErrorKind::Other, why.to_string())),
        }
    }
}
