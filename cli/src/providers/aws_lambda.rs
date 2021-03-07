use handlebars::{to_json, Handlebars};
use serde::Serialize;

use crate::materials::{Artifact, ArtifactError, ContentType};
use crate::materials::models;
use crate::providers::{Options, Provider, ProviderError};

#[derive(Serialize)]
pub struct ServiceData {
    pub aws_region: String,
    pub layer_name: String,
}

#[derive(Serialize)]
pub struct FunctionData {
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

    pub size: Option<u16>,
    pub timeout: Option<u16>,

    pub project_name: String,
}

pub struct ProviderArtifact {
    content: String,
}

impl ProviderArtifact {
    pub fn new(content: String) -> Self {
        ProviderArtifact { content }
    }
}

impl Artifact for ProviderArtifact {
    fn content_type(&self) -> ContentType {
        ContentType::HCL("HCL")
    }
    
    fn content(&self) -> Option<String> {
        Some(self.content)
    }

    fn cast(&self) -> Result<String, ArtifactError> {
        Ok(self.content)
    }
}

pub struct ServiceProvider<'a> {
    reg: Box<Handlebars<'a>>,
}

impl<'a> ServiceProvider<'a> {
    pub fn new() -> Self {
        let mut reg = Box::new(Handlebars::new()); 
        
        reg.register_template_string("service", SERVICE_TEMPLATE)
            .unwrap();
        reg.register_template_string("function", FUNCTION_TEMPLATE)
            .unwrap();
        
        Self { reg }
    }
}

impl<'a> Provider<models::Service> for ServiceProvider<'a> {
    fn name(&self) -> String {
        String::from("aws-lambda")
    }
    
    fn transform(&self, service: models::Service) -> Result<Box<dyn Artifact>, ProviderError> {
        let data = ServiceData { 
            aws_region: String::from("us-east-1"),
            layer_name: format!("asml-{}-runtime", service.name),
        };
        let data = to_json(data);
        
        let rendered = self.reg.render("service", &data).unwrap();

        Ok(Box::new(ProviderArtifact::new(rendered)))
    }

    fn options(&self) -> Options {
        Options::new()
    }

    fn set_options(&mut self, opts: Options) -> Result<(), ProviderError> {
        Ok(())
    }
}

impl<'a> Provider<models::Function> for ServiceProvider<'a> {
    fn name(&self) -> String {
        String::from("aws-lambda")
    }

    fn transform(&self, function: models::Function) -> Result<Box<dyn Artifact>, ProviderError> {
        let data = FunctionData {
            name: function.name.clone(),
            handler_name: match &function.handler_name {
                Some(name) => name.clone(),
                None => String::from("handler"),
            },
            service: service.name.clone(),
            service_has_layer: tf_function_service.has_layer,
            service_has_http_api: tf_function_service.has_http_api,
        };
        let data = to_json(data);
        
        let rendered = self.reg.render("function", &data).unwrap();

        Ok(Box::new(ProviderArtifact::new(rendered)))
    }
    
    fn options(&self) -> Options {
        Options::new()
    }

    fn set_options(&mut self, opts: Options) -> Result<(), ProviderError> {
        Ok(())
    }
}

static SERVICE_TEMPLATE: &str = 
r#"provider "aws" {
    region = "{{aws_region}}"
}

resource "aws_lambda_layer_version" "asml_runtime_layer" {
  filename   = "${path.module}/../.asml/runtime/bootstrap.zip"
  layer_name = "{{layer_name}}"

  source_code_hash = filebase64sha256("${path.module}/../.asml/runtime/bootstrap.zip")
}
"#;

static FUNCTION_TEMPLATE: &str =
r#"resource "aws_lambda_function" "asml_{{service}}_{{name}}_lambda" {
    function_name = "asml-{{project_name}}-{{service}}-{{name}}"
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
"#;
