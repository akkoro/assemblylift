use std::io;
use std::path;
use std::path::PathBuf;

use clap::crate_version;
use handlebars::{Handlebars, to_json};
use serde_derive::{Deserialize, Serialize};
use serde_json::value::{Map, Value as Json};

use crate::terraform::write_to_file;

pub static TERRAFORM_FUNCTION: &str = r#"# Generated with assemblylift-cli {{asml_version}}

variable "runtime_layer_arn" {
  type = string
}
{{#if service_has_layer}}
variable "service_layer_arn" {
  type = string
}
{{/if}}
variable "lambda_role_arn" {
  type = string
}

variable "lambda_role_name" {
  type = string
}
{{#if service_has_http_api}
variable "service_http_api_id" {
  type = sring
}

resource "aws_apigatewayv2_integration" "asml_{{name}}" {
  api_id           = var.service_http_api_id
  integration_type = "AWS_PROXY"

  connection_type           = "INTERNET"
  integration_method        = "POST"
  integration_uri           = aws_lambda_function.asml_{{name}}_lambda.invoke_arn
}
{{/if}}

resource "aws_lambda_function" "asml_{{name}}_lambda" {
    function_name = "{{name}}"
    role          = var.lambda_role_arn
    runtime       = "provided"
    handler       = "{{name}}.{{handler_name}}"
    filename      = "${path.module}/{{name}}.zip"
    timeout       = 10

    {{#if service_has_layer}}
    layers = [var.runtime_layer_arn, var.service_layer_arn]
    {{else}}
    layers = [var.runtime_layer_arn]
    {{/if}}

    source_code_hash = filebase64sha256("${path.module}/{{name}}.zip")
}

resource "aws_iam_policy" "asml_{{name}}_lambda_logging" {
  name        = "asml_{{name}}_lambda_logging"
  path        = "/"
  description = "IAM policy for logging from a lambda"

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": [
        "logs:CreateLogGroup",
        "logs:CreateLogStream",
        "logs:PutLogEvents"
      ],
      "Resource": "arn:aws:logs:*:*:*",
      "Effect": "Allow"
    }
  ]
}
EOF
}

resource "aws_iam_role_policy_attachment" "asml_{{name}}_lambda_logs" {
  role       = var.lambda_role_name
  policy_arn = aws_iam_policy.asml_{{name}}_lambda_logging.arn
}
"#;

#[derive(Clone, Serialize, Deserialize)]
pub struct TerraformFunction {
    pub name: String,
    pub handler_name: String,
    pub service: String,
    pub service_has_layer: bool,
    pub service_has_http_api: bool,
}

pub fn write(project_path: &PathBuf, function: &TerraformFunction) -> Result<(), io::Error> {
    let file_name = "function.tf";

    let mut reg = Handlebars::new();
    reg.register_template_string(file_name, TERRAFORM_FUNCTION)
        .unwrap(); // templates are known at compile-time

    let mut data = Map::<String, Json>::new();
    data.insert("asml_version".to_string(), to_json(crate_version!()));
    data.insert("name".to_string(), to_json(&function.name));
    data.insert("handler_name".to_string(), to_json(&function.handler_name));
    data.insert("service".to_string(), to_json(&function.service));
    data.insert(
        "service_has_layer".to_string(),
        to_json(function.service_has_layer),
    );
    data.insert(
        "service_has_http_api".to_string(),
        to_json(function.service_has_http_api),
    );

    let render = reg.render(file_name, &data).unwrap();

    let path_str = &format!(
        "{}/net/services/{}/{}/{}",
        project_path.clone().into_os_string().into_string().unwrap(),
        function.service,
        function.name,
        file_name
    );
    let path = path::Path::new(path_str);

    write_to_file(&path, render)
}
