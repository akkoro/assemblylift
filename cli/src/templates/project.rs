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

# Platforms define an environment in which to deploy providers.
# [[platforms]]
# id = "my-id" # A unique identifier for this platform
# name = "platform-name" # Name of the platform, one of "aws" or "kubernetes"
# options = {} # Map of options to pass to the platform

[[services]]
name = "{{default_service_name}}" # Must correspond to a service in the services/ directory
provider = { name = "my-provider", platform_id = "my-id" } # `platform_id` is the `id` of one of the platforms defined above
# registry_id = "my-registry-id" # Optional; needed for Service providers which deploy using containers
# domain_name = "my.example-domain.com" # Optional; needed if deploying service with a Domain name

# [[registries]]
# id = "my-registry-id"
# provider = { name = "my-provider", platform_id = "my-id" }

# [[domains]]
# dns_name = "my.example-domain.com" # An existing hosted zone with the DNS Provider specified
# [domains.provider]
# name = "my-provider"
# platform_id = "my-id"
# options = {} # Map of options to pass to the provider

"#;

pub static ROOT_DOCUMENTS: Lazy<Arc<Vec<Document>>> = Lazy::new(|| {
    Arc::new(Vec::from([
        Document {
            file_name: "assemblylift.toml",
            document: String::from(ASSEMBLYLIFT_TOML),
        },
        Document {
            file_name: ".gitignore",
            document: String::from(ROOT_GITIGNORE),
        },
    ]))
});

static SERVICE_TOML: &str = r#"[gateway]
provider = { name = "{{service_name}}" }

#[[functions]]
#name = "rusty-fn" # Must correspond to a function in the service's functions/ directory
#language = "rust" # Kind of source code for function; one of "rust" or "ruby"
#http = { verb = "GET", path = "/rustyfn" } # HTTP route to the function from the Service's Gateway
#environment = { var1 = "val1" } # Map of environment variables to pass to the function
"#;

pub static SERVICE_DOCUMENTS: Lazy<Arc<Vec<Document>>> = Lazy::new(|| {
    Arc::new(Vec::from([Document {
        file_name: "service.toml",
        document: String::from(SERVICE_TOML),
    }]))
});

static FUNCTION_CARGO_TOML: &str = r#"[package]
name = "{{function_name}}"
version = "0.0.0"
edition = "2021"

[dependencies]
serde = "1"
serde_json = "1"
assemblylift-core-guest = { version = "0.4.0-beta" }
# You can omit core-io-guest if you will not be using IOmods in your function
assemblylift-core-io-guest = { version = "0.4.0-beta" }
"#;

static FUNCTION_MAIN_RS: &str = r#"use assemblylift_core_guest::*;

#[handler]
async fn main() {
    // `ctx` is a value injected by the `handler` attribute macro
    let event: serde_json::Value = match serde_json::from_slice(&ctx.input) {
        Ok(val) => val,
        Err(err) => return FunctionContext::failure(format!("could not parse function input as JSON: {}", err.to_string()));
    };

    FunctionContext::success("\"Function returned OK!\"".to_string());
}
"#;

static FUNCTION_HANDLER_RB: &str = r#"require 'asml'
require 'base64'
require 'json'

def main(input)
    # TODO implement your function code here!
    Asml.success(JSON.generate(input.to_s))
end

main(JSON.parse(Asml.get_function_input()))
"#;

pub static RUST_FUNCTION_DOCUMENTS: Lazy<Arc<Vec<Document>>> = Lazy::new(|| {
    Arc::new(Vec::from([
        Document {
            file_name: "Cargo.toml",
            document: String::from(FUNCTION_CARGO_TOML),
        },
        Document {
            file_name: "src/main.rs",
            document: String::from(FUNCTION_MAIN_RS),
        },
    ]))
});

pub static RUBY_FUNCTION_DOCUMENTS: Lazy<Arc<Vec<Document>>> = Lazy::new(|| {
    Arc::new(Vec::from([Document {
        file_name: "handler.rb",
        document: String::from(FUNCTION_HANDLER_RB),
    }]))
});
