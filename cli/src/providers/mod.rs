pub mod aws_lambda;

use std::rc::Rc;

use handlebars::{to_json, Handlebars};
use once_cell::sync::Lazy;
use serde::Serialize;

use crate::materials::{asml, toml, hcl};
use crate::materials::{StringMap, Artifact, ArtifactError, ContentType};

pub type ServiceList = Vec<Box<hcl::service::Module>>;
pub type ProviderMap = StringMap<Box<dyn Provider + Send + Sync>>;

// TODO where do these get initialized?
pub static SERVICE_PROVIDERS: Lazy<ProviderMap> = Lazy::new(|| ProviderMap::new());
pub static FUNCTION_PROVIDERS: Lazy<ProviderMap> = Lazy::new(|| ProviderMap::new());
pub static ROOT_PROVIDERS: Lazy<ProviderMap> = Lazy::new(|| ProviderMap::new());

pub type Options = StringMap<String>;

pub trait Transformable {
    const TYPE: &'static str;
}

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
    pub user_inject: bool,
}

pub struct RootProvider<'a> {
    services: ServiceList,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl RootProvider<'_> {
    pub fn new(services: ServiceList) -> Self {
        Self {
            services,
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
{{#if user_inject}}
module "usermod" {
  source = "../user_tf"
}
{{/if}}
"#;
