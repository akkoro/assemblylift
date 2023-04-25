use std::env;

use anyhow::anyhow;
use reqwest::Client;
use tracing::debug;

// https://docs.aws.amazon.com/lambda/latest/dg/runtimes-api.html

#[derive(Debug)]
pub struct AwsLambdaEvent {
    pub request_id: String,
    pub event_body: String,
}

#[derive(Clone)]
pub struct AwsLambdaRuntime {
    client: Client,
    api_endpoint: String,
//    last_request_id: Option<String>,
}

impl AwsLambdaRuntime {
    pub fn new() -> AwsLambdaRuntime {
        AwsLambdaRuntime {
            client: Client::new(),
            api_endpoint: env::var("AWS_LAMBDA_RUNTIME_API").unwrap(),
 //           last_request_id: None,
        }
    }

    pub async fn get_next_event(&mut self) -> anyhow::Result<AwsLambdaEvent> {
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
                        return Err(anyhow!("missing header \"Lambda-Runtime-Aws-Request-Id\""))
                    }
                };
                debug!("got next event request ID {}", &request_id);
//                if let Some(last_request_id) = self.last_request_id.as_ref() {
//                    if last_request_id.eq_ignore_ascii_case(&request_id) {
//                        return Err(anyhow!("already processed"));
//                    }
//                }
//                self.last_request_id = Some(request_id.clone());

                let event_body = res.text().await.unwrap();

                Ok(AwsLambdaEvent {
                    request_id,
                    event_body,
                })
            }

            Err(why) => Err(anyhow!(why.to_string())),
        }
    }

    pub async fn respond(&self, response: String, request_id: String) -> anyhow::Result<()> {
        debug!("responding to request ID {}", &request_id);
        let url = &format!(
            "http://{}/2018-06-01/runtime/invocation/{}/response",
            self.api_endpoint, request_id
        )
        .to_string();

        match self.client.post(url).body(response).send().await {
            Ok(_) => Ok(()),
            Err(why) => Err(anyhow!(why.to_string())),
        }
    }

    pub async fn error(&self, message: String, request_id: String) -> anyhow::Result<()> {
        debug!("posting error to request ID {}", &request_id);
        let url = &format!(
            "http://{}/2018-06-01/runtime/invocation/{}/error",
            self.api_endpoint, request_id
        )
        .to_string();

        match self
            .client
            .post(url)
            .body(format!(
                "{{\"errorMessage\":\"{}\", \"errorType\":\"Unknown\"}}",
                &message
            ))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(why) => Err(anyhow!(why.to_string())),
        }
    }
}
