use anyhow::{anyhow, Result};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

use crate::{Options, context::{Service, Function, Registry}, CastResult, Fragment};

use super::{ApiProvider, ContainerRegistryProvider, DnsProvider, FunctionProvider, Provider, ServiceProvider};

pub fn provider_name() -> String {
    "ecr".into()
}

pub fn platform_name() -> String {
    "*".into()
}

#[derive(Serialize, Deserialize)]
pub struct EcrProvider {
    #[serde(default = "provider_name")]
    name: String,
    #[serde(default = "platform_name")]
    platform: String,
    options: Options,
}

impl EcrProvider {
    pub fn new(options: Options) -> Box<Self> {
        Box::new(Self {
            name: provider_name(),
            platform: platform_name(),
            options,
        })
    }
}

#[typetag::serde]
impl Provider for EcrProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn platform(&self) -> String {
        self.platform.clone()
    }

    fn options(&self) -> Options {
        self.options.clone()
    }

    fn boot(&self) -> Result<()> {
        Ok(())
    }

    fn is_booted(&self) -> bool {
        true
    }

    fn as_service_provider(&self) -> Result<&dyn ServiceProvider> {
        Err(anyhow!("{} is not a ServiceProvider", self.name()))
    }

    fn as_function_provider(&self) -> Result<&dyn FunctionProvider> {
        Err(anyhow!("{} is not a FunctionProvider", self.name()))
    }

    fn as_api_provider(&self) -> Result<&dyn ApiProvider> {
        Err(anyhow!("{} is not a ApiProvider", self.name()))
    }

    fn as_dns_provider(&self) -> Result<&dyn DnsProvider> {
        Err(anyhow!("{} is not a DnsProvider", self.name()))
    }

    fn as_container_registry_provider(&self) -> Result<&dyn ContainerRegistryProvider> {
        Ok(self)
    }
}

impl ContainerRegistryProvider for EcrProvider {
    fn cast_registry(&self, registry: &Registry) -> CastResult<Vec<Fragment>> {
        todo!()
    }

    fn cast_service(&self, service: &Service) -> CastResult<Vec<Fragment>> {
        todo!()
    }
}
