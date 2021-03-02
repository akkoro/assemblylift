use crate::templates::Document;

static ROOT_GITIGNORE: &str = r#".asml/
net/
"#;

static ASSEMBLYLIFT_TOML: &str = r#"# Generated with assemblylift-cli {{asml_version}}

[project]
name = "{{project_name}}"
version = "0.1.0"

[services]
default = { name = "{{default_service_name}}" }
"#;

pub static ROOT_DOCUMENTS: Vec<Document> = Vec::from([
    Document {
        file_name: "assemblylift.toml",
        document: String::from(ASSEMBLYLIFT_TOML),
    },
    Document {
        file_name: ".gitignore",
        document: String::from(ROOT_GITIGNORE),
    },
]);

static SERVICE_TOML: &str = r#"# Generated with assemblylift-cli {{asml_version}}

[service]
name = "{{service_name}}"

[api.functions.my-function]
name = "my-function"
handler_name = "handler"
"#;

pub static SERVICE_DOCUMENTS: Vec<Document> = Vec::from([
    Document {
        file_name: "service.toml",
        document: String::from(SERVICE_TOML),
    },
]);

static FUNCTION_CARGO_TOML: &str = r#"# Generated with assemblylift-cli {{asml_version}}

[package]
name = "{{function_name}}"
version = "0.1.0"
edition = "2018"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
direct-executor = "0.3.0"
serde_json = "1.0.61"
asml_core = { version = "0.2.0", package = "assemblylift-core-guest" }
asml_core_io = { version = "0.2.1", package = "assemblylift-core-io-guest" }
asml_awslambda = { version = "0.2.4", package = "assemblylift-awslambda-guest" }

"#;

static FUNCTION_CARGO_CONFIG: &str = r#"# Generated with assemblylift-cli {{asml_version}}

[build]
target = "wasm32-unknown-unknown"
"#;

static FUNCTION_LIB_RS: &str = r#"// Generated with assemblylift-cli {{asml_version}}

extern crate asml_awslambda;

use asml_core::GuestCore;
use asml_awslambda::{*, AwsLambdaClient, LambdaContext};

handler!(context: LambdaContext<()>, async {
    let event = context.event;
    AwsLambdaClient::console_log(format!("Read event: {:?}", event));

    AwsLambdaClient::success("OK".to_string());
});
"#;

static FUNCTION_GITIGNORE: &str = r#"// Generated with assemblylift-cli {{asml_version}}
.DS_Store
*.wasm
target/
build/
"#;

pub static RUST_FUNCTION_DOCUMENTS: Vec<Document> = Vec::from([
    Document {
        file_name: "Cargo.toml",
        document: String::from(FUNCTION_CARGO_TOML),
    },
    Document {
        file_name: ".cargo/config",
        document: String::from(FUNCTION_CARGO_CONFIG),
    },
    Document {
        file_name: "src/lib.rs",
        document: String::from(FUNCTION_LIB_RS),
    },
    Document {
        file_name: ".gitignore",
        document: String::from(FUNCTION_GITIGNORE),
    },
]);
