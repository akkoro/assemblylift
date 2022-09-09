extern crate assemblylift_core_guest_macros;

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use assemblylift_core_guest_macros::handler;

extern "C" {
    fn __asml_abi_runtime_log(ptr: *const u8, len: usize);
    fn __asml_abi_runtime_success(ptr: *const u8, len: usize);
}

pub struct FunctionContext {
    pub input: String,
}

impl FunctionContext {
    pub fn log(message: String) {
        unsafe { __asml_abi_runtime_log(message.as_ptr(), message.len()) }
    }

    pub fn success(response: String) {
        unsafe { __asml_abi_runtime_success(response.as_ptr(), response.len()) }
    }
}

pub type StatusCode = u16;
#[derive(Serialize, Deserialize)]
pub struct HttpResponse {
    #[serde(rename = "isBase64Encoded")]
    is_base64_encoded: bool,
    #[serde(rename = "statusCode")]
    status_code: StatusCode,
    headers: HashMap<String, String>,
    body: String,
}

#[derive(Serialize, Deserialize)]
pub struct HttpError {
    pub code: StatusCode,
    pub desc: String,
    pub message: String,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum HttpErrorCode {
    NotFound = 404,
    FunctionError = 520,
}

impl fmt::Display for HttpErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpErrorCode::NotFound => write!(f, "Missing Resource"),
            HttpErrorCode::FunctionError => write!(f, "Function Error"),
        }
    }
}

impl HttpResponse {
    pub fn ok(
        body: String,
        content_type: Option<String>,
        is_base64_encoded: bool,
        gzip: bool,
    ) -> Self {
        let mut headers = HashMap::default();
        headers.insert(
            "content-type".to_string(),
            content_type.unwrap_or_else(|| String::from("application/json")),
        );
        if gzip {
            headers.insert("content-encoding".to_string(), "gzip".to_string());
        }

        Self {
            status_code: 200,
            is_base64_encoded,
            headers,
            body,
        }
    }

    pub fn error(message: String, code: HttpErrorCode) -> Self {
        let mut headers = HashMap::default();
        headers.insert(
            String::from("content-type"),
            String::from("application/json"),
        );

        Self {
            status_code: code as StatusCode,
            is_base64_encoded: false,
            headers,
            body: serde_json::to_string(&HttpError {
                code: code as StatusCode,
                desc: code.to_string(),
                message,
            })
            .unwrap(),
        }
    }
}

#[macro_export]
macro_rules! http_ok {
    ($response:expr) => {
        FunctionContext::success(
            serde_json::to_string(&HttpResponse::ok(
                serde_json::to_string(&$response).unwrap(),
                None,
                false,
                false,
            ))
            .unwrap(),
        );
    };

    ($response:expr, $type:expr, $isb64:expr, $isgzip:expr) => {
        FunctionContext::success(
            serde_json::to_string(&HttpResponse::ok($response, $type, $isb64, $isgzip)).unwrap(),
        );
    };
}

#[macro_export]
macro_rules! http_error {
    ($message:expr) => {
        FunctionContext::success(
            serde_json::to_string(&HttpResponse::error($message, HttpErrorCode::FunctionError))
                .unwrap(),
        );
    };
}

#[macro_export]
macro_rules! http_not_found {
    ($resource_name:expr) => {
        FunctionContext::success(
            serde_json::to_string(&HttpResponse::error(
                format!("missing resource {:?}", $resource_name),
                HttpErrorCode::NotFound,
            ))
            .unwrap(),
        );
    };
}
