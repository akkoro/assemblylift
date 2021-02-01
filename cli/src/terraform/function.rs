use std::io;
use std::path;
use std::path::PathBuf;

use clap::crate_version;
use handlebars::{to_json, Handlebars};
use serde::{Deserialize, Serialize};
use serde_json::value::{Map, Value as Json};

use crate::terraform::write_to_file;

pub static TERRAFORM_FUNCTION: &str = r#"# Generated with assemblylift-cli {{asml_version}}

variable "runtime_layer_arn" {
  type = string
}

locals {
  lambda_name = "asml-{{project_name}}-{{service}}-{{name}}"
}

{{#if service_has_layer}}
variable "service_layer_arn" {
  type = string
}
{{/if}}
{{#if service_has_http_api}}
variable "service_http_api_id" {
  type = string
}

variable "http_verb" {
  type = string
}

variable "http_path" {
  type = string
}

variable "http_api_execution_arn" {
  type = string
}

variable "http_authorizer_id" {
  type = string
  default = ""
}

resource "aws_apigatewayv2_route" "asml_{{name}}_http_route" {
  api_id    = var.service_http_api_id
  route_key = "${var.http_verb} ${var.http_path}"
  target    = "integrations/${aws_apigatewayv2_integration.asml_{{name}}.id}"

  authorization_type = "{{auth_type}}"
  {{#if auth_has_id}}
  authorizer_id = var.http_authorizer_id
  authorization_scopes = ["email", "openid"]
  {{/if}}
}

resource "aws_apigatewayv2_integration" "asml_{{name}}" {
  api_id           = var.service_http_api_id
  integration_type = "AWS_PROXY"

  connection_type           = "INTERNET"
  integration_method        = "POST"
  integration_uri           = aws_lambda_function.asml_{{service}}_{{name}}_lambda.invoke_arn
}

resource "aws_lambda_permission" "lambda_permission" {
  action        = "lambda:InvokeFunction"
  function_name = local.lambda_name
  principal     = "apigateway.amazonaws.com"

  source_arn = "${var.http_api_execution_arn}/*"
}
{{/if}}

resource "aws_lambda_function" "asml_{{service}}_{{name}}_lambda" {
    function_name = local.lambda_name
    role          = aws_iam_role.lambda_iam_role.arn
    runtime       = "provided"
    handler       = "{{name}}.{{handler_name}}"
    filename      = "${path.module}/{{name}}.zip"
    timeout       = {{timeout}}

    {{#if service_has_layer}}
    layers = [var.runtime_layer_arn, var.service_layer_arn]
    {{else}}
    layers = [var.runtime_layer_arn]
    {{/if}}

    source_code_hash = filebase64sha256("${path.module}/{{name}}.zip")
}

resource "aws_iam_role" "lambda_iam_role" {
    name = local.lambda_name

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

resource "aws_iam_policy" "asml_{{service}}_{{name}}_lambda_logging" {
  name        = "asml-{{project_name}}-{{service}}-{{name}}-logging"
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
  role       = aws_iam_role.lambda_iam_role.name
  policy_arn = aws_iam_policy.asml_{{service}}_{{name}}_lambda_logging.arn
}
"#;

#[derive(Clone, Serialize, Deserialize)]
pub struct TerraformFunction {
    pub name: String,
    pub handler_name: String,
    pub service: String,
    pub service_has_layer: bool,
    pub service_has_http_api: bool,
    pub http_verb: Option<String>,
    pub http_path: Option<String>,

    pub auth_name: String,
    pub auth_type: String,
    pub auth_has_id: bool,

    pub timeout: Option<u16>,

    pub project_name: String,
}

pub fn write(project_path: &PathBuf, function: &TerraformFunction) -> Result<(), io::Error> {
    let file_name = "function.tf";

    let mut reg = Handlebars::new();
    reg.register_template_string(file_name, TERRAFORM_FUNCTION)
        .unwrap(); // templates are known at compile-time

    let mut data = Map::<String, Json>::new();
    data.insert("asml_version".to_string(), to_json(crate_version!()));
    data.insert("project_name".to_string(), to_json(&function.project_name));
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
    data.insert("auth_type".to_string(), to_json(&function.auth_type));
    data.insert(
        "auth_has_id".to_string(), 
        to_json(&function.auth_has_id)
    );
    data.insert("timeout".to_string(), to_json(&function.timeout));

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
