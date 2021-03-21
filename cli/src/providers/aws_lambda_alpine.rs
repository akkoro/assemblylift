use std::rc::Rc;
use std::sync::Arc;

use clap::crate_version;
use handlebars::{to_json, Handlebars};
use serde::Serialize;

use crate::transpiler::{asml, Artifact};
use crate::providers::{render_string_list, Options, Provider, ProviderArtifact, ProviderError};

pub struct ServiceProvider {
    options: Arc<Options>,
}

impl ServiceProvider {
    pub fn new() -> Self {
        Self { options: Arc::new(Options::new()) }
    }
}

impl Provider for ServiceProvider {
    fn name(&self) -> String {
        String::from("aws-lambda-alpine")
    }

    fn init(&self, ctx: Rc<asml::Context>, name: String) -> Result<(), ProviderError> {
        use std::io::Write;
        use crate::docker;

        let registry_url = self.options.get("registry")
            .expect("service provider requires `registry` option");
        let version = crate_version!();

        let public = &format!("public.ecr.aws/akkoro/assemblylift/asml-lambda-alpine:{}", version);
        let local = &format!("{}/assemblylift/asml-lambda-alpine:{}", registry_url, version);
        let layer_name = format!("{}-{}", ctx.project.name.clone(), name.clone()); 
        let service_tag = &format!("{}/assemblylift/asml-lambda-alpine:{}-{}", registry_url, version, layer_name);

        {
            let mut contents: String = format!("FROM {}\n", public);
            for iomod in ctx.iomods.iter().filter(|i| i.service_name == name.clone()) {
                contents.push_str(&format!("ADD ./.asml/runtime/{} /opt/iomod/\n", iomod.name.clone()));
            }

            std::fs::create_dir_all("./.asml/runtime").unwrap();
            let mut file = std::fs::File::create("./.asml/runtime/Dockerfile")
                .expect("could not create runtime Dockerfile");
            file.write_all(contents.as_bytes()).expect("could not write runtime Dockerfile");
        }

        docker::build(local, "./.asml/runtime/Dockerfile").expect("could not build docker image");
        docker::tag(local, service_tag).expect("could not tag docker image");
        docker::push(service_tag).expect("could not push docker image");

        Ok(())
    }
    
    fn transform(&self, ctx: Rc<asml::Context>, name: String) -> Result<Box<dyn Artifact>, ProviderError> {
        let mut reg = Box::new(Handlebars::new()); 
        reg.register_template_string("service", SERVICE_TEMPLATE)
            .unwrap();

        let layer_name = format!("asml-{}-{}-{}-runtime", 
            ctx.project.name.clone(), 
            name.clone(), 
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

        let data = ServiceData { 
            name: name.clone(),
            aws_region: String::from("us-east-1"),
            hcl_provider: String::from("aws"),
            layer_name,
            use_apigw,
            authorizers,
        };
        let data = to_json(data);
        
        let rendered = reg.render("service", &data).unwrap();

        Ok(Box::new(ProviderArtifact::new(rendered)))
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

impl Provider for FunctionProvider {
    fn name(&self) -> String {
        String::from("aws-lambda")
    }
    
    fn init(&self, _ctx: Rc<asml::Context>, _name: String) -> Result<(), ProviderError> {
        Ok(())
    }

    fn transform(&self, ctx: Rc<asml::Context>, name: String) -> Result<Box<dyn Artifact>, ProviderError> {
        let mut reg = Box::new(Handlebars::new()); 
        reg.register_template_string("function", FUNCTION_TEMPLATE)
            .unwrap();

        match ctx.functions.iter().find(|&f| *f.name == name.clone()) {
            Some(function) => {
                let service = function.service_name.clone();

                // find dependencies for service
                let iomod_names: Vec<String> = ctx.iomods.iter()
                    .filter(|&m| *m.service_name == service.clone())
                    .map(|m| m.name.clone())
                    .collect();

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
        
                let registry_url = self.options.get("registry")
                    .expect("service provider requires `registry` option");
                let version = crate_version!();
                let layer_name = format!("{}-{}", ctx.project.name.clone(), service.clone()); 
                let image_uri = format!("{}/assemblylift/asml-lambda-alpine:{}-{}", registry_url, version, layer_name);

                let data = FunctionData {
                    name: function.name.clone(),
                    handler_name: function.handler_name.clone(),
                    service: service.clone(),
                    image_uri,
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

                Ok(Box::new(ProviderArtifact::new(rendered)))
            }
            None => Err(ProviderError::TransformationError(format!("unable to find function {} in context", name.clone()))),
        }

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
    pub aws_region: String,
    pub hcl_provider: String,
    pub layer_name: String,
    pub use_apigw: bool,
    pub authorizers: Vec<ServiceAuthData>,
}

#[derive(Serialize)]
pub struct FunctionData {
    pub name: String,
    pub handler_name: String,
    pub service: String,
    pub image_uri: String,
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
r#"provider "aws" {
    alias  = "{{name}}"
    region = "{{aws_region}}"
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
r#"resource "aws_lambda_function" "asml_{{service}}_{{name}}" {
    provider = aws.{{service}}

    function_name = "asml-{{project_name}}-{{service}}-{{name}}"
    role          = aws_iam_role.{{service}}_{{name}}_lambda_iam_role.arn
    runtime       = "provided"
    handler       = "{{name}}.{{handler_name}}"
    timeout       = {{timeout}}
    memory_size   = {{size}}

    package_type  = "Image"
    image_uri     = "{{image_uri}}"
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
