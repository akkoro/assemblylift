pub mod aws_lambda;

use std::rc::Rc;

use handlebars::{to_json, Handlebars};
use once_cell::sync::Lazy;
use serde::Serialize;

use crate::materials::asml;
use crate::materials::{StringMap, Artifact, ArtifactError, ContentType};

pub type ProviderMap = StringMap<Box<dyn Provider + Send + Sync>>;

pub static SERVICE_PROVIDERS: Lazy<ProviderMap> = Lazy::new(|| {
    let mut map = ProviderMap::new();
    map.insert(String::from("aws-lambda"), Box::new(aws_lambda::ServiceProvider));
    map
});
pub static FUNCTION_PROVIDERS: Lazy<ProviderMap> = Lazy::new(|| {
    let mut map = ProviderMap::new();
    map.insert(String::from("aws-lambda"), Box::new(aws_lambda::FunctionProvider));
    map
});
pub static ROOT_PROVIDERS: Lazy<ProviderMap> = Lazy::new(|| {
    let mut map = ProviderMap::new();
    map.insert(String::from("root"), Box::new(RootProvider::new()));
    map
});

pub type Options = StringMap<String>;

pub trait Provider {
    fn name(&self) -> String;

    fn transform(&self, ctx: Rc<asml::Context>, name: String) -> Result<Box<dyn Artifact>, ProviderError>;

    fn options(&self) -> Options;
    fn set_options(&mut self, opts: Options) -> Result<(), ProviderError>;
}

#[derive(Debug)]
pub enum ProviderError {
    TransformationError(String),
    UnknownError(String),
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

    fn transform(&self, ctx: Rc<asml::Context>, name: String) -> Result<Box<dyn Artifact>, ProviderError> {
        use std::path::PathBuf;
        use std::fs;

        let mut usermod_path = PathBuf::from(
            crate::projectfs::locate_asml_manifest().expect("could not locate assemblylift.toml").1
        );
        usermod_path.push("user_tf/");
        let user_inject: bool = fs::metadata(usermod_path).is_ok();
        
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
    
    fn options(&self) -> Options {
        Options::new()
    }

    fn set_options(&mut self, opts: Options) -> Result<(), ProviderError> {
        Ok(())
    }
}

static ROOT_TEMPLATE: &str = r#"
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
