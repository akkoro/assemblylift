pub mod aws_lambda;
pub mod aws_lambda_alpine;

use std::rc::Rc;
use std::sync::{Arc, Mutex};

use handlebars::{to_json, Handlebars};
use once_cell::sync::Lazy;
use serde::Serialize;

use crate::transpiler::asml;
use crate::transpiler::{StringMap, Artifact, ArtifactError, ContentType};

pub type ProviderMap = StringMap<Mutex<Box<dyn Provider + Send + Sync>>>;

pub static SERVICE_PROVIDERS: Lazy<ProviderMap> = Lazy::new(|| {
    let mut map = ProviderMap::new();
    map.insert(String::from("aws-lambda"), Mutex::new(Box::new(aws_lambda::ServiceProvider)));
    map.insert(String::from("aws-lambda-alpine"), Mutex::new(Box::new(aws_lambda_alpine::ServiceProvider::new())));
    map
});
pub static FUNCTION_PROVIDERS: Lazy<ProviderMap> = Lazy::new(|| {
    let mut map = ProviderMap::new();
    map.insert(String::from("aws-lambda"), Mutex::new(Box::new(aws_lambda::FunctionProvider)));
    map.insert(String::from("aws-lambda-alpine"), Mutex::new(Box::new(aws_lambda_alpine::FunctionProvider::new())));
    map
});
pub static ROOT_PROVIDERS: Lazy<ProviderMap> = Lazy::new(|| {
    let mut map = ProviderMap::new();
    map.insert(String::from("root"), Mutex::new(Box::new(RootProvider::new())));
    map
});

pub type Options = StringMap<String>;

pub trait Provider {
    fn name(&self) -> String;

    fn init(&self, ctx: Rc<asml::Context>, name: String) -> Result<(), ProviderError>;
    fn transform(&self, ctx: Rc<asml::Context>, name: String) -> Result<Box<dyn Artifact>, ProviderError>;

    fn options(&self) -> Arc<Options>;
    fn set_options(&mut self, opts: Arc<Options>) -> Result<(), ProviderError>;
}

#[derive(Debug)]
pub enum ProviderError {
    TransformationError(String),
}

pub fn render_string_list(list: Rc<Vec<String>>) -> String {
    let mut out = String::from("[");
    for (i, l) in list.iter().enumerate() {
        out.push_str(&format!("\"{}\"", l));
        if i < list.len() - 1 {
            out.push_str(",");
        }
    }
    out.push_str("]");
    out
}

pub struct ProviderArtifact {
    content: Rc<Option<String>>,
}

impl ProviderArtifact {
    pub fn new(content: String) -> Self {
        ProviderArtifact { content: Rc::new(Some(content)) }
    }
}

impl Artifact for ProviderArtifact {
    fn content_type(&self) -> ContentType {
        ContentType::HCL("HCL")
    }
    
    fn content(&self) -> Rc<Option<String>> {
        self.content.clone()
    }

    fn cast(&mut self) -> Result<String, ArtifactError> {
        let content = self.content().as_ref().as_ref().unwrap().clone();
        Ok(content)
    }
}

#[derive(Serialize)]
pub struct RootData {
    pub project_name: String,
    pub project_path: String,
    pub user_inject: bool,
}

pub struct RootProvider<'a> {
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl RootProvider<'_> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a> Provider for RootProvider<'a> {
    fn name(&self) -> String {
        String::from("root")
    }

    fn init(&self, _ctx: Rc<asml::Context>, _name: String) -> Result<(), ProviderError> {
        Ok(())
    }

    fn transform(&self, ctx: Rc<asml::Context>, _name: String) -> Result<Box<dyn Artifact>, ProviderError> {
        use std::path::PathBuf;
        use std::fs;

        let mut usermod_path = PathBuf::from(
            crate::projectfs::locate_asml_manifest().expect("could not locate assemblylift.toml").1
        );
        usermod_path.pop();
        usermod_path.push("user_tf/");
        let user_inject: bool = fs::metadata(usermod_path.clone()).is_ok();
        
        let data = RootData { 
            project_name: ctx.project.name.clone(),
            project_path: ctx.project.path.clone(),
            user_inject,
        };
        let data = to_json(data);
        
        let mut reg = Box::new(Handlebars::new()); 
        reg.register_template_string("template", ROOT_TEMPLATE)
            .unwrap();
        let rendered = reg.render("template", &data).unwrap();

        Ok(Box::new(ProviderArtifact::new(rendered)))
    }
    
    fn options(&self) -> Arc<Options> {
        Arc::new(Options::new())
    }

    fn set_options(&mut self, _opts: Arc<Options>) -> Result<(), ProviderError> {
        Ok(())
    }
}

static ROOT_TEMPLATE: &str = 
r#"terraform {
    required_providers {
        docker = {
            source  = "kreuzwerker/docker"
            version = "2.11.0"
        }
    }
}

locals {
    project_name = "{{project_name}}"
    project_path = "{{project_path}}"
}
{{#if user_inject}}
module "usermod" {
  source = "../user_tf"
}
{{/if}}
"#;
