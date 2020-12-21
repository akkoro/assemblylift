use std::path::PathBuf;

use clap::crate_version;
use handlebars::to_json;
use serde::Deserialize;
use serde_json::value::{Map, Value as Json};

use crate::bom::{write_documents, Document, DocumentSet};

static FUNCTION_CARGO_TOML: &str = r#"# Generated with assemblylift-cli {{asml_version}}

[package]
name = "{{function_name}}"
version = "0.1.0"
edition = "2018"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
direct-executor = "0.3.0"
serde_json = "1.0.53"
asml_core = { version = "0.2.0", package = "assemblylift-core-guest" }
asml_core_io = { version = "0.2.1", package = "assemblylift-core-io-guest" }
asml_awslambda = { version = "0.2.2", package = "assemblylift-awslambda-guest" }

"#;

static FUNCTION_CARGO_CONFIG: &str = r#"# Generated with assemblylift-cli {{asml_version}}

[build]
target = "wasm32-unknown-unknown"
"#;

static FUNCTION_LIB_RS: &str = r#"// Generated with assemblylift-cli {{asml_version}}

extern crate asml_awslambda;

use asml_core::GuestCore;
use asml_awslambda::{*, AwsLambdaClient, LambdaContext};

handler!(context: LambdaContext, async {
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

#[derive(Deserialize)]
pub struct RustFunction;

impl DocumentSet<'_, RustFunction> for RustFunction {
    fn file_names() -> Vec<Document> {
        Vec::from([
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
        ])
    }

    fn read(_path: &PathBuf) -> RustFunction {
        unimplemented!()
    }

    fn write(path: &PathBuf, data: &mut Map<String, Json>) {
        data.insert("asml_version".to_string(), to_json(crate_version!()));
        write_documents(path, RustFunction::file_names(), data)
    }
}
