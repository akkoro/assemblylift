use anyhow::{anyhow, Result};

pub mod api_gateway;
pub mod aws_lambda;
pub mod ecr;
pub mod gloo;
pub mod kubernetes;
pub mod route53;

use crate::{
    context::{Domain, Function, Service},
    CastResult, Fragment, Options,
};

use self::{
    api_gateway::ApiGatewayProvider, aws_lambda::AwsLambdaProvider, ecr::EcrProvider,
    gloo::GlooProvider, kubernetes::KubernetesProvider, route53::Route53Provider,
};

#[typetag::serde(tag = "provider")]
pub trait Provider {
    fn name(&self) -> String;
    fn platform(&self) -> String;
    fn options(&self) -> Options;
    fn set_option(&mut self, key: &str, value: &str);
    fn boot(&self) -> Result<()>;
    fn is_booted(&self) -> bool;
    fn as_service_provider(&self) -> Result<&dyn ServiceProvider>;
    fn as_function_provider(&self) -> Result<&dyn FunctionProvider>;
    fn as_api_provider(&self) -> Result<&dyn ApiProvider>;
    fn as_dns_provider(&self) -> Result<&dyn DnsProvider>;
    fn as_container_registry_provider(&self) -> Result<&dyn ContainerRegistryProvider>;
}

pub trait ServiceProvider: Provider {
    fn cast_service(&self, service: &Service) -> CastResult<Vec<Fragment>>;
}

pub trait FunctionProvider: Provider {
    fn cast_function(&self, function: &Function) -> CastResult<Vec<Fragment>>;
}

pub trait ApiProvider: Provider {
    fn cast_service(&self, service: &Service) -> CastResult<Vec<Fragment>>;
    fn supported_service_providers(&self) -> Vec<String>;
}

pub trait DnsProvider: Provider {
    fn cast_domain(&self, domain: &Domain) -> CastResult<Vec<Fragment>>;
    fn cast_service(&self, service: &Service) -> CastResult<Vec<Fragment>>;
    fn supported_api_providers(&self) -> Vec<String>;
}

pub trait ContainerRegistryProvider: Provider {
    fn cast_service(&self, service: &Service) -> CastResult<Vec<Fragment>>;
}

pub struct ProviderFactory;

impl ProviderFactory {
    pub fn new_provider(name: &str, options: Options) -> Result<Box<dyn Provider>> {
        match name {
            _ if name == api_gateway::provider_name() => Ok(ApiGatewayProvider::new(options)),
            _ if name == aws_lambda::provider_name() => Ok(AwsLambdaProvider::new(options)),
            _ if name == ecr::provider_name() => Ok(EcrProvider::new(options)),
            _ if name == gloo::provider_name() => Ok(GlooProvider::new(options)),
            _ if name == kubernetes::provider_name() => Ok(KubernetesProvider::new(options)),
            _ if name == route53::provider_name() => Ok(Route53Provider::new(options)),
            _ => Err(anyhow!("unrecognized provider named {}", name)),
        }
    }
}
