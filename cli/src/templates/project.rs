use std::sync::Arc;

use once_cell::sync::Lazy;

use crate::templates::Document;

static ROOT_GITIGNORE: &str = r#".asml/
.terraform/
.DS_Store
net/
**/target/
**/build/
"#;

static ASSEMBLYLIFT_TOML: &str = r#"[project]
name = "{{project_name}}"

[[services]]
name = "{{default_service_name}}"
"#;

pub static ROOT_DOCUMENTS: Lazy<Arc<Vec<Document>>> = Lazy::new(|| Arc::new(Vec::from([
    Document {
        file_name: "assemblylift.toml",
        document: String::from(ASSEMBLYLIFT_TOML),
    },
    Document {
        file_name: ".gitignore",
        document: String::from(ROOT_GITIGNORE),
    },
])));

static SERVICE_TOML: &str = r#"[service]
name = "{{service_name}}"

[[api.functions]]
name = "my-function"
language = {{function_language}}"
"#;

pub static SERVICE_DOCUMENTS: Lazy<Arc<Vec<Document>>> = Lazy::new(|| Arc::new(Vec::from([
    Document {
        file_name: "service.toml",
        document: String::from(SERVICE_TOML),
    },
])));

static FUNCTION_CARGO_TOML: &str = r#"[package]
name = "{{function_name}}"
version = "0.0.0"
edition = "2021"

[dependencies]
direct-executor = "0.3.0"
serde = "1"
serde_json = "1"
asml_core = { version = "0.4.0-alpha", package = "assemblylift-core-guest" }
assemblylift_core_io_guest = { version = "0.4.0-alpha", package = "assemblylift-core-io-guest" }
z85 = "3"
"#;

static FUNCTION_MAIN_RS: &str = r#"use asml_core::*;

#[handler]
async fn main() {
    // `ctx` is a value injected by the `handler` attribute macro
    let event: serde_json::Value = serde_json::from_str(&ctx.input)
        .expect("could not parse function input as JSON");

    FunctionContext::success("Function returned OK!".to_string());
}
"#;

static FUNCTION_HANDLER_RB: &str = r#"require 'asml'
require 'base64'
require 'json'

def main(input)
    # TODO implement your function code here!
    Asml.success(input.to_s)
end

main(JSON.parse(Asml.get_function_input()))
"#;

pub static RUST_FUNCTION_DOCUMENTS: Lazy<Arc<Vec<Document>>> = Lazy::new(|| Arc::new(Vec::from([
    Document {
        file_name: "Cargo.toml",
        document: String::from(FUNCTION_CARGO_TOML),
    },
    Document {
        file_name: "src/main.rs",
        document: String::from(FUNCTION_MAIN_RS),
    },
])));

pub static RUBY_FUNCTION_DOCUMENTS: Lazy<Arc<Vec<Document>>> = Lazy::new(|| Arc::new(Vec::from([
    Document {
        file_name: "handler.rb",
        document: String::from(FUNCTION_HANDLER_RB),
    },
])));
