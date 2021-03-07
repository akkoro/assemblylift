pub mod aws_lambda;

use once_cell::unsync::Lazy;

use crate::materials::{models, toml, hcl};
use crate::materials::{StringMap, Artifact};

pub type ServiceList = Vec<Box<hcl::service::Module>>;
pub type ProviderMap<T> = StringMap<Box<dyn Provider<T>>>;

// TODO statics for each type of provider (service, api, auth, etc)
pub static SERVICE_PROVIDERS: Lazy<ProviderMap<models::Service>> = Lazy::new(|| ProviderMap::new());
pub static FUNCTION_PROVIDERS: Lazy<ProviderMap<models::Function>> = Lazy::new(|| ProviderMap::new());
pub static ROOT_PROVIDERS: Lazy<ProviderMap<toml::asml::Manifest>> = Lazy::new(|| ProviderMap::new());

pub type Options = StringMap<String>;

pub trait Transformable {
    const TYPE: &'static str;
}

pub trait Provider<T: Transformable> {
    fn name(&self) -> String;

    fn transform(&self, source: T) -> Result<Box<dyn Artifact>, ProviderError>;

    fn options(&self) -> Options;
    fn set_options(&mut self, opts: Options) -> Result<(), ProviderError>;
}

#[derive(Debug)]
pub enum ProviderError {
    TransformationError(String),
    UnknownError(String),
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

impl<'a> Provider<toml::asml::Manifest> for RootProvider<'a> {
    fn name(&self) -> String {
        String::from("root")
    }

    fn transform(&self, root: toml::asml::Manifest) -> Result<Box<dyn Artifact>, ProviderError> {
        //
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
