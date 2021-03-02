use handlebars::{to_json, Handlebars};
use serde::Serialize;

use crate::materials::{Artifact, ContentType};
use crate::materials::models;
use crate::providers::{Options, Provider, ProviderError};

#[derive(Serialize)]
pub struct ServiceData {
    pub layer_name: String,
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
    
    fn content(&self) -> String {
        self.content
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
        
        Self { reg }
    }
}

impl<'a> Provider<models::Service> for ServiceProvider<'a> {
    fn name(&self) -> String {
        String::from("aws-lambda")
    }
    
    fn transform(&self, service: models::Service) -> Result<Box<dyn Artifact>, ProviderError> {
        let data = ServiceData { layer_name: format!("asml-{}-runtime", service.name) };
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
        //
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
