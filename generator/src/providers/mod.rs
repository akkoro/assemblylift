use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

pub mod api_gateway;
pub mod aws_lambda;
pub mod ecr;
pub mod gloo;
pub mod kubernetes;
pub mod route53;

use crate::{
    context::{Domain, Function, Service},
    CastResult, Fragment, Options, StringMap,
};

use self::{
    api_gateway::ApiGatewayProvider, aws_lambda::AwsLambdaProvider, ecr::EcrProvider,
    gloo::GlooProvider, kubernetes::KubernetesProvider, route53::Route53Provider,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Platform {
    pub id: String,
    pub name: String,
    pub options: StringMap<String>,
}

impl From<&crate::toml::asml::Platform> for Platform {
    fn from(value: &crate::toml::asml::Platform) -> Self {
        Self {
            id: value.id.clone(),
            name: value.name.clone(),
            options: value.options.clone(),
        }
    }
}

#[typetag::serde(tag = "provider")]
pub trait Provider {
    fn name(&self) -> String;
    fn platform(&self) -> Option<Platform>;
    fn compatible_platforms(&self) -> Vec<String>;
    fn options(&self) -> Options;
    fn set_option(&mut self, key: &str, value: &str);
    fn boot(&self) -> Result<()>;
    fn is_booted(&self) -> bool;
    // fn validate(&self) -> bool;
    fn as_service_provider(&self) -> Result<&dyn ServiceProvider>;
    fn as_function_provider(&self) -> Result<&dyn FunctionProvider>;
    fn as_gateway_provider(&self) -> Result<&dyn GatewayProvider>;
    fn as_dns_provider(&self) -> Result<&dyn DnsProvider>;
    fn as_container_registry_provider(&self) -> Result<&dyn ContainerRegistryProvider>;
}

pub trait ServiceProvider: Provider {
    fn cast_service(&self, service: &Service) -> CastResult<Vec<Fragment>>;
}

pub trait FunctionProvider: Provider {
    fn cast_function(&self, function: &Function) -> CastResult<Vec<Fragment>>;
}

pub trait GatewayProvider: Provider {
    fn cast_service(&self, service: &Service) -> CastResult<Vec<Fragment>>;
    fn compatible_service_providers(&self) -> Vec<String>;
}

pub trait DnsProvider: Provider {
    fn cast_domain(&self, domain: &Domain) -> CastResult<Vec<Fragment>>;
    fn cast_service(&self, service: &Service) -> CastResult<Vec<Fragment>>;
    fn compatible_gateway_providers(&self) -> Vec<String>;
}

pub trait ContainerRegistryProvider: Provider {
    fn cast_service(&self, service: &Service) -> CastResult<Vec<Fragment>>;
}

pub struct ProviderFactory;

impl ProviderFactory {
    pub fn new_provider(name: &str, options: Options, platform: Option<Platform>) -> Result<Box<dyn Provider>> {
        match name {
            _ if name == api_gateway::provider_name() => Ok(ApiGatewayProvider::new(options, platform)),
            _ if name == aws_lambda::provider_name() => Ok(AwsLambdaProvider::new(options, platform)),
            _ if name == ecr::provider_name() => Ok(EcrProvider::new(options, platform)),
            _ if name == gloo::provider_name() => Ok(GlooProvider::new(options, platform)),
            _ if name == kubernetes::provider_name() => Ok(KubernetesProvider::new(options, platform)),
            _ if name == route53::provider_name() => Ok(Route53Provider::new(options, platform)),
            _ => Err(anyhow!("unrecognized provider named {}", name)),
        }
    }
}
