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
    memory_size   = {{size}}

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
