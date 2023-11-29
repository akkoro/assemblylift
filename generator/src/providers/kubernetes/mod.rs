use anyhow::{anyhow, Result};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

use crate::{Options, context::{Service, Function}, CastResult, Fragment};

use super::{ApiProvider, DnsProvider, FunctionProvider, Provider, ServiceProvider};

pub fn provider_name() -> String {
    "k8s".into()
}

pub fn platform_name() -> String {
    "k8s".into()
}

#[derive(Serialize, Deserialize)]
pub struct KubernetesProvider {
    #[serde(default = "provider_name")]
    name: String,
    #[serde(default = "platform_name")]
    platform: String,
    options: Options,
}

impl KubernetesProvider {
    pub fn new(options: Options) -> Box<Self> {
        Box::new(Self {
            name: provider_name(),
            platform: platform_name(),
            options,
        })
    }
}

#[typetag::serde]
impl Provider for KubernetesProvider {
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
        Ok(self)
    }

    fn as_function_provider(&self) -> Result<&dyn FunctionProvider> {
        Ok(self)
    }

    fn as_api_provider(&self) -> Result<&dyn ApiProvider> {
        Err(anyhow!("{} is not a ApiProvider", self.name()))
    }

    fn as_dns_provider(&self) -> Result<&dyn DnsProvider> {
        Err(anyhow!("{} is not a DnsProvider", self.name()))
    }
}

impl ServiceProvider for KubernetesProvider {
    fn cast_service(&self, service: &Service) -> CastResult<Vec<Fragment>> {
        todo!()
    }
}

impl FunctionProvider for KubernetesProvider {
    fn cast_function(&self, function: &Function, service_name: &str) -> CastResult<Vec<Fragment>> {
        todo!()
    }
}
