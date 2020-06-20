pub(crate) static ROOT_GITIGNORE: &str = 
r#".asml/
net/
"#;

pub(crate) static ASSEMBLYLIFT_TOML: &str =
r#"# Generated with assemblylift-cli {{asml_version}}

[project]
name = "{{project_name}}"
version = "0.1.0"

[services]
default = { name = "{{default_service_name}}" }
"#;

pub(crate) static SERVICE_TOML: &str =
r#"# Generated with assemblylift-cli {{asml_version}}

[service]
name = "{{service_name}}"
version = ""

[api]
name = "{{service_name}}-api"

[api.functions.my-function]
name = "my-function"
handler_name = "handler"
"#;

pub(crate) static FUNCTION_CARGO_TOML: &str =
r#"# Generated with assemblylift-cli {{asml_version}}

[package]
name = "{{function_name}}"
version = "0.1.0"
edition = "2018"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
direct-executor = "0.3.0"
serde_json = "1.0.53"
asml_core = { package = "assemblylift-core-guest", git = "https://github.com/akkoro/assemblylift", branch="research" }
asml_core_event = { package = "assemblylift-core-event-guest", git = "https://github.com/akkoro/assemblylift", branch="research" }
asml_awslambda = { package = "assemblylift-awslambda-guest", git = "https://github.com/akkoro/assemblylift", branch="research" }
asml_awslambda_iomod = { package = "assemblylift-awslambda-iomod-guest", git = "https://github.com/akkoro/assemblylift", branch="research" }
"#;

pub(crate) static FUNCTION_CARGO_CONFIG: &str =
r#"# Generated with assemblylift-cli {{asml_version}}

[build]
target = "wasm32-unknown-unknown"
"#;

pub(crate) static FUNCTION_LIB_RS: &str =
r#"// Generated with assemblylift-cli {{asml_version}}

extern crate asml_awslambda;

use direct_executor;
use asml_core::GuestCore;
use asml_awslambda::{*, AwsLambdaClient, LambdaContext};

handler!(context: LambdaContext, async {
    let event = context.event;
    AwsLambdaClient::console_log(format!("Read event: {:?}", event));

    AwsLambdaClient::success("OK".to_string());
});
"#;

pub(crate) static FUNCTION_GITIGNORE: &str = 
r#"// Generated with assemblylift-cli {{asml_version}}
.DS_Store
*.wasm
target/
build/
"#;

pub(crate) static TERRAFORM_ROOT: &str = 
r#"# Generated with assemblylift-cli {{asml_version}}

provider "aws" {
    region = "{{aws_region}}"
}

resource "aws_iam_role" "lambda_iam_role" {
    name = "asml_lambda_iam_role"
  
    assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      },
      "Effect": "Allow",
      "Sid": ""
    }
  ]
}
EOF
}

resource "aws_lambda_layer_version" "asml_runtime_layer" {
  filename   = "${path.module}/../.asml/runtime/bootstrap.zip"
  layer_name = "assemblylift-runtime"
}

{{#each functions}}
module "{{this.name}}" {
  source = "./services/{{this.service}}/{{this.name}}"

  lambda_role_arn   = aws_iam_role.lambda_iam_role.arn
  runtime_layer_arn = aws_lambda_layer_version.asml_runtime_layer.arn
}
{{/each}}

"#;

pub(crate) static TERRAFORM_FUNCTION: &str = 
r#"# Generated with assemblylift-cli {{asml_version}}

variable "runtime_layer_arn" {
  type = string
}

variable "lambda_role_arn" {
  type = string
}

resource "aws_lambda_function" "{{name}}_lambda" {
    function_name = "{{name}}"
    role          = var.lambda_role_arn
    runtime       = "provided"
    handler       = "{{name}}.{{handler_name}}"
    filename      = "${path.module}/{{name}}.zip"

    layers = [var.runtime_layer_arn]

    source_code_hash = filebase64sha256("${path.module}/{{name}}.zip")
}
"#;
