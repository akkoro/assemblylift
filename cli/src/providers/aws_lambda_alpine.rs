use std::rc::Rc;
use std::sync::Arc;

use clap::crate_version;
use handlebars::{Handlebars, to_json};
use serde::Serialize;

use crate::providers::{Options, Provider, ProviderError, render_string_list};
use crate::transpiler::{Artifact, Bindable, Castable, CastError, ContentType};
use crate::transpiler::context::Context;

pub struct ServiceProvider {
    options: Arc<Options>,
}

impl ServiceProvider {
    pub fn new() -> Self {
        Self { options: Arc::new(Options::new()) }
    }
}

impl Castable for ServiceProvider {
    fn cast(&self, ctx: Rc<Context>, selector: Option<&str>) -> Result<Vec<Artifact>, CastError> {
        let mut reg = Box::new(Handlebars::new());
        reg.register_template_string("service", SERVICE_TEMPLATE)
            .unwrap();

        let layer_name = format!("asml-{}-{}-{}-runtime",
                                 ctx.project.name.clone(),
                                 selector.expect("selector must be a service name").to_string(),
                                 self.name().clone(),
        );

        let use_apigw = ctx.functions.iter().find(|f| f.http.is_some()).is_some();

        let authorizers: Vec<ServiceAuthData> = ctx.authorizers.iter()
            .filter(|a| a.r#type.to_lowercase() != "aws_iam")
            .map(|a| {
                ServiceAuthData {
                    id: a.id.clone(),
                    r#type: a.r#type.clone(),
                    jwt_config: match &a.jwt_config {
                        Some(jwt) => {
                            let audience = render_string_list(jwt.audience.clone());

                            Some(ServiceAuthDataJwtConfig {
                                audience,
                                issuer: jwt.issuer.clone(),
                            })
                        }
                        None => None,
                    },
                }
            })
            .collect();

        let aws_account_id = self.options.get("aws_account_id")
            .expect("service provider requires `aws_account_id` option");
        let data = ServiceData {
            name: selector.expect("selector must be a service name").to_string(),
            aws_account_id: aws_account_id.clone(),
            aws_region: String::from("us-east-1"),
            hcl_provider: String::from("aws"),
            layer_name,
            use_apigw,
            authorizers,
        };
        let data = to_json(data);

        let rendered = reg.render("service", &data).unwrap();
        let hcl = Artifact {
            content_type: ContentType::HCL("HCL"),
            content: rendered,
            write_path: "net/plan.tf".into(),
        };
        Ok(vec![hcl])
    }
}

impl Bindable for ServiceProvider {
    fn bind(&self, ctx: Rc<Context>) -> Result<(), CastError> {
        todo!()
    }
}

impl Provider for ServiceProvider {
    fn name(&self) -> String {
        String::from("aws-lambda-alpine")
    }

    fn options(&self) -> Arc<Options> {
        self.options.clone()
    }

    fn set_options(&mut self, opts: Arc<Options>) -> Result<(), ProviderError> {
        self.options = opts;
        Ok(())
    }
}

pub struct FunctionProvider {
    options: Arc<Options>,
}

impl FunctionProvider {
    pub fn new() -> Self {
        Self { options: Arc::new(Options::new()) }
    }
}

impl Castable for FunctionProvider {
    fn cast(&self, ctx: Rc<Context>, selector: Option<&str>) -> Result<Vec<Artifact>, CastError> {
        use std::io::Write;

        let mut reg = Box::new(Handlebars::new());
        reg.register_template_string("function", FUNCTION_TEMPLATE)
            .unwrap();

        let name = selector.expect("selector must be a function name").to_string();
        match ctx.functions.iter().find(|&f| f.name == name) {
            Some(function) => {
                let service = function.service_name.clone();

                // write Dockerfile for function
                {
                    let version = crate_version!();
                    let public = &format!("public.ecr.aws/akkoro/assemblylift/asml-lambda-alpine:{}", version);
                    let mut contents: String = format!("FROM {}\n", public);
                    contents.push_str(&format!("ENV _HANDLER \"{}.handler\"\n", function.name.clone()));
                    //contents.push_str("ENV LAMBDA_TASK_ROOT /var/task\n");
                    //contents.push_str("ENV RUST_BACKTRACE full\n");
                    for iomod in ctx.iomods.iter().filter(|i| i.service_name == service.clone()) {
                        contents.push_str(&format!("ADD ./iomods/{} /opt/iomod/\n", iomod.name.clone()));
                    }
                    contents.push_str(&format!("ADD ./{}/{}.wasm.bin /var/task/{}.wasm.bin\n",
                                               function.name.clone(), function.name.clone(), function.name.clone()));
                    contents.push_str("RUN chmod -R 755 /opt\n");

                    let mut file = std::fs::File::create(format!("./net/services/{}/{}/Dockerfile", service.clone(), function.name.clone()))
                        .expect("could not create runtime Dockerfile");
                    file.write_all(contents.as_bytes()).expect("could not write runtime Dockerfile");
                }

                // find authorizers for service
                let auth = match &function.authorizer_id {
                    Some(id) => {
                        let authorizer = ctx.authorizers.iter()
                            .filter(|a| a.service_name == service.clone())
                            .find(|a| a.id == id.clone())
                            .expect(&format!("could not find authorizer by id \"{}\" in context", id.clone()));
                        Some(FunctionAuthData {
                            id: match authorizer.r#type.clone().to_lowercase().as_str() {
                                "aws_iam" => None,
                                _ => Some(format!("aws_apigatewayv2_authorizer.{}_{}.id", service.clone(), id)),
                            },
                            r#type: authorizer.r#type.clone(),
                            scopes: match authorizer.r#type.clone().to_lowercase().as_str() {
                                "aws_iam" => None,
                                _ => Some(render_string_list(Rc::new(vec!["email".to_string(), "openid".to_string()]))),
                            },
                        })
                    },
                    None => None,
                };

                let data = FunctionData {
                    name: function.name.clone(),
                    service: service.clone(),
                    project_name: ctx.project.name.clone(),
                    size: function.size,
                    timeout: function.timeout,
                    http: match &function.http {
                        Some(http) => {
                            Some(HttpData {
                                verb: http.verb.clone(),
                                path: http.path.clone(),
                            })
                        }
                        None => None,
                    },
                    auth,
                };
                let data = to_json(data);

                let rendered = reg.render("function", &data).unwrap();
                let hcl = Artifact {
                    content_type: ContentType::HCL("HCL"),
                    content: rendered,
                    write_path: "net/plan.tf".into(),
                };
                Ok(vec![hcl])
            }
            None => Err(CastError(format!("unable to find function {} in context", name.clone()))),
        }
    }
}

impl Bindable for FunctionProvider {
    fn bind(&self, ctx: Rc<Context>) -> Result<(), CastError> {
        todo!()
    }
}

impl Provider for FunctionProvider {
    fn name(&self) -> String {
        String::from("aws-lambda")
    }

    fn options(&self) -> Arc<Options> {
        self.options.clone()
    }

    fn set_options(&mut self, opts: Arc<Options>) -> Result<(), ProviderError> {
        self.options = opts;
        Ok(())
    }
}

#[derive(Serialize)]
pub struct ServiceData {
    pub name: String,
    pub aws_account_id: String,
    pub aws_region: String,
    pub hcl_provider: String,
    pub layer_name: String,
    pub use_apigw: bool,
    pub authorizers: Vec<ServiceAuthData>,
}

#[derive(Serialize)]
pub struct FunctionData {
    pub name: String,
    pub service: String,
    pub http: Option<HttpData>,
    pub auth: Option<FunctionAuthData>,
    pub size: u16,
    pub timeout: u16,
    pub project_name: String,
}

#[derive(Serialize)]
pub struct FunctionAuthData {
    pub id: Option<String>,
    pub r#type: String,
    pub scopes: Option<String>,
}

#[derive(Serialize)]
pub struct ServiceAuthData {
    pub id: String,
    pub r#type: String,
    pub jwt_config: Option<ServiceAuthDataJwtConfig>,
}

#[derive(Serialize)]
pub struct ServiceAuthDataJwtConfig {
    pub audience: String,
    pub issuer: String,
}

#[derive(Serialize)]
pub struct HttpData {
    pub verb: String,
    pub path: String,
}

static SERVICE_TEMPLATE: &str =
r#"locals {
    ecr = "{{aws_account_id}}.dkr.ecr.{{aws_region}}.amazonaws.com"
}

provider "aws" {
    alias  = "{{name}}"
    region = "{{aws_region}}"
}

data "aws_ecr_authorization_token" "{{name}}_token" {
    provider = aws.{{name}}
}

provider "docker" {
    alias   = "{{name}}"
    registry_auth {
        address  = local.ecr
        password = data.aws_ecr_authorization_token.{{name}}_token.password
        username = data.aws_ecr_authorization_token.{{name}}_token.user_name
    }
}

{{#if use_apigw}}
resource "aws_apigatewayv2_api" "{{name}}_http_api" {
    provider      = aws.{{name}}
    name          = "asml-${local.project_name}-{{name}}"
    protocol_type = "HTTP"
}

resource "aws_apigatewayv2_stage" "{{name}}_default_stage" {
    provider    = aws.{{name}}
    api_id      = aws_apigatewayv2_api.{{name}}_http_api.id
    name        = "$default"
    auto_deploy = true
}
{{/if}}
{{#each authorizers}}
resource "aws_apigatewayv2_authorizer" "{{../name}}_{{this.id}}" {
    provider    = aws.{{../name}}

    api_id           = aws_apigatewayv2_api.{{../name}}_http_api.id
    authorizer_type  = "{{this.type}}"
    identity_sources = ["$request.header.Authorization"]
    name             = "{{../name}}-{{this.id}}"
  
    {{#if this.jwt_config}}jwt_configuration {
        audience = {{{this.jwt_config.audience}}}
        issuer   = "{{this.jwt_config.issuer}}"
    }{{/if}}
}
{{/each}}
"#;

static FUNCTION_TEMPLATE: &str =
r#"resource "aws_ecr_repository" "{{service}}_{{name}}" {
    provider = aws.{{service}}
    name = "asml-{{project_name}}-{{service}}-{{name}}"
}

data "archive_file" "{{service}}_{{name}}_iomods" {
    type        = "zip"
    source_dir  = "${path.module}/services/{{service}}/iomods"
    output_path = "${path.module}/services/{{service}}/iomods.zip"
}

resource "random_id" "{{service}}_{{name}}_image" {
    byte_length = 8
    keepers = {
        dockerfile_hash = filebase64sha256("${path.module}/services/{{service}}/{{name}}/Dockerfile")
        wasm_hash       = filebase64sha256("${path.module}/services/{{service}}/{{name}}/{{name}}.wasm.bin")
        iomods_hash     = data.archive_file.{{service}}_{{name}}_iomods.output_sha
    }
}

resource "docker_registry_image" "{{service}}_{{name}}" {
    provider = docker.{{service}}
    name = "${aws_ecr_repository.{{service}}_{{name}}.repository_url}:${random_id.{{service}}_{{name}}_image.hex}"
  
    build {
        context      = "${path.module}/services/{{service}}"
        dockerfile   = "{{name}}/Dockerfile"
        pull_parent  = true
        force_remove = true
    }
}

resource "aws_lambda_function" "asml_{{service}}_{{name}}" {
    provider = aws.{{service}}

    function_name = "asml-{{project_name}}-{{service}}-{{name}}"
    role          = aws_iam_role.{{service}}_{{name}}_lambda_iam_role.arn
    timeout       = {{timeout}}
    memory_size   = {{size}}
    package_type  = "Image"
    image_uri     = docker_registry_image.{{service}}_{{name}}.name

    image_config {
        command = ["{{name}}.handler"]
        working_directory = "/"
    }
}

resource "aws_iam_role" "{{service}}_{{name}}_lambda_iam_role" {
    provider = aws.{{service}}
    name     = "asml-{{project_name}}-{{service}}-{{name}}"

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

    inline_policy {
        name = "allow-cloudwatch-logging"
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
}
{{#if http}}
resource "aws_apigatewayv2_route" "asml_{{service}}_{{name}}" {
    provider = aws.{{service}}

    api_id    = aws_apigatewayv2_api.{{service}}_http_api.id
    route_key = "{{http.verb}} {{http.path}}"
    target    = "integrations/${aws_apigatewayv2_integration.asml_{{service}}_{{name}}.id}"
{{#if auth}}  
    authorization_type   = "{{auth.type}}"
    {{#if auth.id}}authorizer_id        = {{auth.id}}{{/if}}
    {{#if auth.scopes}}authorization_scopes = {{{auth.scopes}}}{{/if}}
{{else}}
    authorization_type = "NONE"
{{/if}}
}

resource "aws_apigatewayv2_integration" "asml_{{service}}_{{name}}" {
    provider = aws.{{service}}

    api_id           = aws_apigatewayv2_api.{{service}}_http_api.id
    integration_type = "AWS_PROXY"
  
    connection_type    = "INTERNET"
    integration_method = "POST"
    integration_uri    = aws_lambda_function.asml_{{service}}_{{name}}.invoke_arn
}

resource "aws_lambda_permission" "asml_{{service}}_{{name}}" {
    provider = aws.{{service}}

    action        = "lambda:InvokeFunction"
    function_name = "asml-{{project_name}}-{{service}}-{{name}}"
    principal     = "apigateway.amazonaws.com"
  
    source_arn = aws_apigatewayv2_api.{{service}}_http_api.execution_arn
}
{{/if}}
"#;
