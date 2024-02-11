use std::path::PathBuf;

use anyhow::{anyhow, Result};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

use crate::{context::Service, snake_case, CastResult, ContentType, Fragment, Options, CastError};

use super::{
    GatewayProvider, ContainerRegistryProvider, DnsProvider, FunctionProvider, Provider,
    ServiceProvider, Platform,
};

pub fn provider_name() -> String {
    "ecr".into()
}

#[derive(Serialize, Deserialize)]
pub struct EcrProvider {
    #[serde(default = "provider_name")]
    name: String,
    options: Options,
    platform: Option<Platform>,
}

impl EcrProvider {
    pub fn new(options: Options, platform: Option<Platform>) -> Box<Self> {
        Box::new(Self {
            name: provider_name(),
            options,
            platform,
        })
    }
}

#[typetag::serde]
impl Provider for EcrProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn platform(&self) -> Option<Platform> {
        self.platform.clone()
    }

    fn compatible_platforms(&self) -> Vec<String> {
        vec!["aws".into()]
    }

    fn options(&self) -> Options {
        self.options.clone()
    }

    fn set_option(&mut self, key: &str, value: &str) {
        self.options.insert(key.into(), value.into()).unwrap();
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

    fn as_gateway_provider(&self) -> Result<&dyn GatewayProvider> {
        Err(anyhow!("{} is not a GatewayProvider", self.name()))
    }

    fn as_dns_provider(&self) -> Result<&dyn DnsProvider> {
        Err(anyhow!("{} is not a DnsProvider", self.name()))
    }

    fn as_container_registry_provider(&self) -> Result<&dyn ContainerRegistryProvider> {
        Ok(self)
    }
}

impl ContainerRegistryProvider for EcrProvider {
    fn cast_service(&self, service: &Service) -> CastResult<Vec<Fragment>> {
        if self.platform().is_none() {
            return Err(CastError(format!("ContainerRegistryProvider `{}` requires a Platform", self.name())));
        }
        
        let mut hbs = Handlebars::new();
        hbs.register_helper("snake_case", Box::new(snake_case));
        hbs.register_template_string("service", include_str!("templates/ecr_impl.tf.handlebars"))
            .unwrap();

        let service_fragment = Fragment {
            content_type: ContentType::HCL,
            content: hbs.render("service", &service.as_json().unwrap()).unwrap(),
            write_path: PathBuf::from(format!(
                "net/services/{}/infra/{}/containers.tf",
                service.name,
                self.name(),
            )),
        };

        Ok(vec![service_fragment])
    }
}
