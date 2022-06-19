use std::fs;
use std::io::Read;
use std::rc::Rc;
use std::sync::Arc;

use handlebars::{Handlebars, to_json};
use registry_common::models::GetIomodAtResponse;
use serde::Serialize;

use crate::archive;
use crate::providers::{Options, Provider, ProviderError, render_string_list};
use crate::transpiler::{Artifact, Castable, CastError, ContentType, context};
use crate::transpiler::context::Context;

pub struct ServiceProvider;

impl ServiceProvider {
    pub fn new() -> Self {
        let runtime_url = &*format!(
            "http://public.assemblylift.akkoro.io/runtime/{}/aws-lambda/bootstrap.zip",
            clap::crate_version!(),
        );
        let mut response =
            reqwest::blocking::get(runtime_url).expect("could not download bootstrap.zip");
        if !response.status().is_success() {
            panic!("unable to fetch asml runtime from {}", runtime_url);
        }
        let mut response_buffer = Vec::new();
        response.read_to_end(&mut response_buffer).unwrap();

        std::fs::create_dir_all("./.asml/runtime").unwrap();
        std::fs::write("./.asml/runtime/bootstrap.zip", response_buffer).unwrap();

        Self
    }

    pub fn cast_iomods(ctx: Rc<Context>, service_name: &str) -> Result<(), CastError> {
        let project_path = ctx.project.path.clone();
        let iomod_path = format!("{}/net/services/{}/iomods", project_path, service_name);
        let _ = fs::remove_dir_all(iomod_path.clone()); // we don't care about this result
        fs::create_dir_all(iomod_path.clone()).expect("could not create iomod directory");

        let mut dependencies: Vec<String> = Vec::new();
        for iomod in &ctx.iomods {
            // let dependency_coords: Vec<&str> = iomod.coordinates.split('.').collect();
            // let dependency_name = dependency_coords.get(2).unwrap().to_string();

            // TODO try from file
            // if _ {
            //     let dependency_path = format!("{}/net/services/{}/iomods/{}", project_path, service_name, dependency_name);
            //     let dependency_from = dependency.from.as_ref()
            //         .expect("`from` must be defined when dependency type is `file`");
            //     match fs::metadata(dependency_from.clone()) {
            //         Ok(_) => {
            //             fs::copy(dependency_from.clone(), &dependency_path).unwrap();
            //             ()
            //         }
            //         Err(_) => panic!("ERROR: could not find file-type dependency named {} (check path)", dependency_name),
            //     }
            //
            //     dependencies.push(dependency_path);
            // } else {
            let dependency_path = format!(
                "{}/net/services/{}/iomods/{}@{}.iomod",
                project_path, service_name, iomod.coordinates, iomod.version,
            );
            let client = reqwest::blocking::ClientBuilder::new()
                .build()
                .expect("could not build blocking HTTP client");
            let registry_url = format!(
                "https://registry.assemblylift.akkoro.io/iomod/{}/{}",
                iomod.coordinates, iomod.version
            );
            let res: GetIomodAtResponse = client.get(registry_url).send().unwrap().json().unwrap();
            let bytes = client.get(res.url).send().unwrap().bytes().unwrap();
            fs::write(&dependency_path, &*bytes).expect("could not write iomod package");
            dependencies.push(dependency_path);
        }

        archive::zip_files(
            dependencies,
            format!("./.asml/runtime/{}.zip", &service_name),
            Some("iomod/"),
            false,
        );

        Ok(())
    }
}

impl Castable for ServiceProvider {
    fn cast(&self, ctx: Rc<Context>, selector: Option<&str>) -> Result<Vec<Artifact>, CastError> {
        let mut reg = Box::new(Handlebars::new());
        reg.register_template_string("service", SERVICE_TEMPLATE)
            .unwrap();

        let name = selector
            .expect("selector must be a service name")
            .to_string();
        let layer_name = format!(
            "asml-{}-{}-{}-runtime",
            ctx.project.name.clone(),
            name.clone(),
            self.name().clone(),
        );

        ServiceProvider::cast_iomods(ctx.clone(), &name).unwrap();

        let use_apigw = ctx.functions.iter().find(|f| f.http.is_some()).is_some();
        let has_service_layer = ctx.iomods.len() > 0;

        let authorizers: Vec<ServiceAuthData> = ctx
            .authorizers
            .iter()
            .filter(|a| a.r#type.to_lowercase() != "aws_iam")
            .map(|a| ServiceAuthData {
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
            })
            .collect();

        let data = ServiceData {
            name: name.clone(),
            aws_region: String::from("us-east-1"),
            hcl_provider: String::from("aws"),
            layer_name,
            use_apigw,
            has_service_layer,
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

impl Provider for ServiceProvider {
    fn name(&self) -> String {
        String::from("aws-lambda")
    }

    fn options(&self) -> Arc<Options> {
        Arc::new(Options::new())
    }

    fn set_options(&mut self, _opts: Arc<Options>) -> Result<(), ProviderError> {
        Ok(())
    }
}

pub struct FunctionProvider;

impl Castable for FunctionProvider {
    fn cast(&self, ctx: Rc<Context>, selector: Option<&str>) -> Result<Vec<Artifact>, CastError> {
        let mut reg = Box::new(Handlebars::new());
        reg.register_template_string("function", FUNCTION_TEMPLATE)
            .unwrap();

        let name = selector
            .expect("selector must be a function name")
            .to_string();
        match ctx.functions.iter().find(|&f| f.name == name) {
            Some(function) => {
                let service = function.service_name.clone();

                // find dependencies for service
                let iomod_names: Vec<String> = ctx
                    .iomods
                    .iter()
                    .filter(|&m| *m.service_name == service.clone())
                    .map(|m| m.name.clone())
                    .collect();

                // find authorizers for service
                let auth = match &function.authorizer_id {
                    Some(id) => {
                        let authorizer = ctx
                            .authorizers
                            .iter()
                            .filter(|a| a.service_name == service.clone())
                            .find(|a| a.id == id.clone())
                            .expect(&format!(
                                "could not find authorizer by id \"{}\" in context",
                                id.clone()
                            ));
                        let auth_type = authorizer.r#type.clone();
                        Some(FunctionAuthData {
                            id: match auth_type.to_lowercase().as_str() {
                                "aws_iam" => None,
                                _ => Some(format!(
                                    "aws_apigatewayv2_authorizer.{}_{}.id",
                                    service.clone(),
                                    id
                                )),
                            },
                            r#type: auth_type.clone(),
                            scopes: match auth_type.to_lowercase().as_str() {
                                "aws_iam" => None,
                                _ => Some(render_string_list(authorizer.scopes.clone())),
                            },
                        })
                    }
                    None => None,
                };

                let data = FunctionData {
                    name: function.name.clone(),
                    service: service.clone(),
                    runtime_layer: format!(
                        "aws_lambda_layer_version.asml_{}_runtime.arn",
                        service.clone()
                    ),
                    service_layer: match iomod_names.len() {
                        0 => None,
                        _ => Some(format!(
                            "aws_lambda_layer_version.asml_{}_service.arn",
                            service.clone()
                        )),
                    },
                    project_name: ctx.project.name.clone(),
                    size: function.size,
                    timeout: function.timeout,
                    http: match &function.http {
                        Some(http) => Some(HttpData {
                            verb: http.verb.clone(),
                            path: http.path.clone(),
                        }),
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
            None => Err(CastError(format!(
                "unable to find function {} in context",
                name.clone()
            ))),
        }
    }
}

#[derive(Serialize)]
pub struct ServiceData {
    pub name: String,
    pub aws_region: String,
    pub hcl_provider: String,
    pub layer_name: String,
    pub has_service_layer: bool,
    pub use_apigw: bool,
    pub authorizers: Vec<ServiceAuthData>,
}

#[derive(Serialize)]
pub struct FunctionData {
    pub name: String,
    pub service: String,
    pub runtime_layer: String,
    pub service_layer: Option<String>,
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

static SERVICE_TEMPLATE: &str = r#"provider "aws" {
    alias  = "{{name}}"
    region = "{{aws_region}}"
}

resource "aws_lambda_layer_version" "asml_{{name}}_runtime" {
    provider = aws.{{name}}

    filename   = "${local.project_path}/.asml/runtime/bootstrap.zip"
    layer_name = "{{layer_name}}"

    source_code_hash = filebase64sha256("${local.project_path}/.asml/runtime/bootstrap.zip")
}
{{#if has_service_layer}}
resource "aws_lambda_layer_version" "asml_{{name}}_service" {
    provider = aws.{{name}}
    
    filename   = "${local.project_path}/.asml/runtime/{{name}}.zip"
    layer_name = "asml-${local.project_name}-{{name}}-service"

    source_code_hash = filebase64sha256("${local.project_path}/.asml/runtime/{{name}}.zip")
}
{{/if}}
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

static FUNCTION_TEMPLATE: &str = r#"resource "aws_lambda_function" "asml_{{service}}_{{name}}" {
    provider = aws.{{service}}

    function_name = "asml-{{project_name}}-{{service}}-{{name}}"
    role          = aws_iam_role.{{service}}_{{name}}_lambda_iam_role.arn
    runtime       = "provided"
    handler       = "{{name}}.wasi._start"
    filename      = "${local.project_path}/net/services/{{service}}/{{name}}/{{name}}.zip"
    timeout       = {{timeout}}
    memory_size   = {{size}}

    layers = [{{runtime_layer}}{{#if service_layer}}, {{service_layer}}{{/if}}]

    source_code_hash = filebase64sha256("${local.project_path}/net/services/{{service}}/{{name}}/{{name}}.zip")
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
  
    source_arn = "${aws_apigatewayv2_api.{{service}}_http_api.execution_arn}/*"
}
{{/if}}
"#;
