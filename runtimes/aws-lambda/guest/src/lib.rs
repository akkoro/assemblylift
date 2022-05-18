extern crate assemblylift_core_guest;
extern crate assemblylift_core_io_guest;

use std::collections::HashMap;
use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiGatewayEvent {
    pub resource: String,
    pub path: String,
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    pub headers: HashMap<String, String>,
    #[serde(rename = "queryStringParameters")]
    pub query_string_parameters: Option<HashMap<String, String>>,
    #[serde(rename = "pathParameters")]
    pub path_parameters: Option<HashMap<String, String>>,
    #[serde(rename = "stageVariables")]
    pub stage_variables: Option<HashMap<String, String>>,
    #[serde(rename = "requestContext")]
    pub request_context: Option<ApiGatewayRequestContext>,
    pub body: Option<String>,
}

pub type StatusCode = u16;
#[derive(Serialize, Deserialize)]
pub struct ApiGatewayResponse {
    #[serde(rename = "isBase64Encoded")]
    is_base64_encoded: bool,
    #[serde(rename = "statusCode")]
    status_code: StatusCode,
    headers: HashMap<String, String>,
    body: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiGatewayError {
    pub code: StatusCode,
    pub desc: String,
    pub message: String,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ApiGatewayErrorCode {
    NotFound = 404,
    FunctionError = 520,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiGatewayRequestContext {
    pub authorizer: Option<ApiGatewayRequestContextAuthorizer>,
    pub identity: Option<ApiGatewayRequestContextIdentity>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiGatewayRequestContextAuthorizer {
    pub claims: Option<HashMap<String, String>>,
    pub scopes: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiGatewayRequestContextIdentity {
    #[serde(rename = "accessKey")]
    pub access_key: Option<String>,
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    pub caller: Option<String>,
    #[serde(rename = "cognitoAmr")]
    pub cognito_amr: Option<String>,
    #[serde(rename = "cognitoAuthenticationProvider")]
    pub cognito_authentication_provider: Option<String>,
    #[serde(rename = "cognitoAuthenticationType")]
    pub cognito_authentication_type: Option<String>,
    #[serde(rename = "cognitoIdentityId")]
    pub cognito_identity_id: Option<String>,
    #[serde(rename = "cognitoIdentityPoolId")]
    pub cognito_identity_pool_id: Option<String>,
    #[serde(rename = "principalOrgId")]
    pub principal_org_id: Option<String>,
    #[serde(rename = "sourceIp")]
    pub source_ip: String,
    pub user: Option<String>,
    #[serde(rename = "userAgent")]
    pub user_agent: Option<String>,
    #[serde(rename = "userArn")]
    pub user_arn: Option<String>,
}

// #[macro_export]
// macro_rules! handler {
//     ($context:ident: $type:ty, $async_handler:expr) => {
//         #[no_mangle]
//         pub fn handler() -> i32 {
//             use assemblylift_core_io_guest;
//             use assemblylift_core_io_guest::get_time;
//             use direct_executor;
//
//             let client = AwsLambdaClient::new();
//             let mut fib = std::io::BufReader::new(assemblylift_core_io_guest::FunctionInputBuffer::new());
//
//             let event = match serde_json::from_reader(fib) {
//                 Ok(event) => event,
//                 Err(why) => {
//                     AwsLambdaClient::console_log(format!(
//                         "ERROR deserializing Lambda Event: {}",
//                         why.to_string()
//                     ));
//                     return -1;
//                 }
//             };
//
//             let $context: $type = LambdaContext { client, event, _phantom: std::marker::PhantomData };
//
//             direct_executor::run_spinning($async_handler);
//
//             0
//         }
//     };
// }
