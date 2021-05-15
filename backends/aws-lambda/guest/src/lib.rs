extern crate assemblylift_core_guest;
extern crate assemblylift_core_io_guest;

use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use assemblylift_core_guest::*;

//pub const AWS_EVENT_STRING_BUFFER_SIZE: usize = 8192;
//pub static mut AWS_EVENT_STRING_BUFFER: [u8; AWS_EVENT_STRING_BUFFER_SIZE] =
//    [0; AWS_EVENT_STRING_BUFFER_SIZE];
//
//// provided TO the wasm runtime (host)
//#[no_mangle]
//pub fn __asml_guest_get_aws_event_string_buffer_pointer() -> *const u8 {
//    unsafe { AWS_EVENT_STRING_BUFFER.as_ptr() }
//}

// these are provided BY the wasm runtime (host)
extern "C" {
    fn __asml_abi_console_log(ptr: *const u8, len: usize);
    fn __asml_abi_success(ptr: *const u8, len: usize);
}

pub struct AwsLambdaClient(Guest);

impl AwsLambdaClient {
    pub fn new() -> AwsLambdaClient {
        AwsLambdaClient { 0: Guest {} }
    }
}

impl GuestCore for AwsLambdaClient {
    fn console_log(message: String) {
        unsafe { __asml_abi_console_log(message.as_ptr(), message.len()) }
    }

    fn success(response: String) {
        unsafe { __asml_abi_success(response.as_ptr(), response.len()) }
    }
}

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

impl fmt::Display for ApiGatewayErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiGatewayErrorCode::FunctionError => write!(f, "Function Error"),
        }
    }
}

impl ApiGatewayResponse {
    pub fn ok(body: String, content_type: Option<String>) -> Self {
        let mut headers = HashMap::default();
        headers.insert(
            "content-type".to_string(),
            content_type.unwrap_or_else(|| String::from("application/json")),
        );

        Self {
            status_code: 200,
            is_base64_encoded: false,
            headers,
            body,
        }
    }

    pub fn error(message: String, code: ApiGatewayErrorCode) -> Self {
        let mut headers = HashMap::default();
        headers.insert(
            String::from("content-type"),
            String::from("application/json"),
        );

        Self {
            status_code: code as StatusCode,
            is_base64_encoded: false,
            headers,
            body: serde_json::to_string(&ApiGatewayError {
                code: code as StatusCode,
                desc: code.to_string(),
                message,
            })
            .unwrap(),
        }
    }
}

pub struct LambdaContext<'a, E: Serialize + Deserialize<'a> + Clone + Debug> {
    pub client: AwsLambdaClient,
    pub event: E,
    pub _phantom: std::marker::PhantomData<&'a E>,
}

#[macro_export]
macro_rules! handler {
    ($context:ident: $type:ty, $async_handler:expr) => {
        #[no_mangle]
        pub fn handler() -> i32 {
            use assemblylift_core_io_guest;
            //use asml_awslambda::{AWS_EVENT_STRING_BUFFER, AWS_EVENT_STRING_BUFFER_SIZE};
            use direct_executor;

            let client = AwsLambdaClient::new();
            let mut fib = assemblylift_core_io_guest::FunctionInputBuffer::new();
            
            //let mut event_ptr: i32 = -1;
            //let mut event_end: i32 = -1;
            //unsafe {
            //    for (i, &b) in AWS_EVENT_STRING_BUFFER.iter().enumerate() {
            //        if event_ptr == -1 {
            //            if b != '\0' as u8 {
            //                event_ptr = i as i32;
            //            }
            //        } else {
            //            if event_end == -1 {
            //                if b == '\0' as u8 {
            //                    event_end = i as i32;
            //                    break;
            //                }
            //            }
            //        }
            //    }
            //}

            //if event_ptr == -1 || event_end == -1 {
            //    AwsLambdaClient::console_log(format!("ERROR reading Lambda Event from buffer"));
            //    return -1;
            //}

            //let slice = unsafe { &AWS_EVENT_STRING_BUFFER[event_ptr as usize..event_end as usize] };
            let event = match serde_json::from_reader(fib) {
                Ok(event) => event,
                Err(why) => {
                    AwsLambdaClient::console_log(format!(
                        "ERROR deserializing Lambda Event: {}",
                        why.to_string()
                    ));
                    return -1;
                }
            };

            let $context: $type = LambdaContext { client, event, _phantom: std::marker::PhantomData };

            direct_executor::run_spinning($async_handler);

            0
        }
    };
}

#[macro_export]
macro_rules! http_ok {
    ($response:ident) => {
        AwsLambdaClient::success(
            serde_json::to_string(&ApiGatewayResponse::ok(
                serde_json::to_string(&$response).unwrap(),
                None,
            ))
            .unwrap(),
        );
    };
}

#[macro_export]
macro_rules! http_error {
    ($message:expr) => {
        AwsLambdaClient::success(
            serde_json::to_string(&ApiGatewayResponse::error(
                $message,
                ApiGatewayErrorCode::FunctionError,
            ))
            .unwrap(),
        );
    };
}
