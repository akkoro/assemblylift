use std::path::PathBuf;

use anyhow::{anyhow, Result};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

use crate::{
    context::Service, providers::aws_lambda, snake_case, CastResult, ContentType, Fragment, Options,
};

use super::{
    GatewayProvider, ContainerRegistryProvider, DnsProvider, FunctionProvider, Provider,
    ServiceProvider, Platform,
};

pub fn provider_name() -> String {
    "aws-apigw".into()
}

#[derive(Serialize, Deserialize)]
pub struct ApiGatewayProvider {
    #[serde(default = "provider_name")]
    name: String,
    options: Options,
    platform: Option<Platform>,
}

impl ApiGatewayProvider {
    pub fn new(options: Options, platform: Option<Platform>) -> Box<Self> {
        Box::new(Self {
            name: provider_name(),
            options,
            platform,
        })
    }
}

#[typetag::serde]
impl Provider for ApiGatewayProvider {
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
        Ok(self)
    }

    fn as_dns_provider(&self) -> Result<&dyn DnsProvider> {
        Err(anyhow!("{} is not a DnsProvider", self.name()))
    }

    fn as_container_registry_provider(&self) -> Result<&dyn ContainerRegistryProvider> {
        Err(anyhow!(
            "{} is not a ContainerRegistryProvider",
            self.name()
        ))
    }
}

impl GatewayProvider for ApiGatewayProvider {
    fn cast_service(&self, service: &Service) -> CastResult<Vec<Fragment>> {
        let mut fragments: Vec<Fragment> = Vec::new();

        let mut hbs = Handlebars::new();
        hbs.register_helper("snake_case", Box::new(snake_case));
        hbs.register_template_string("root", include_str!("templates/api_impl.tf.handlebars"))
            .unwrap();

        let api_fragment = Fragment {
            content_type: ContentType::HCL,
            content: hbs.render("root", &service.as_json().unwrap()).unwrap(),
            write_path: PathBuf::from(format!(
                "net/services/{}/infra/{}/api.tf",
                service.name,
                self.name(),
            )),
        };

        fragments.append(&mut vec![api_fragment]);

        Ok(fragments)
    }

    fn compatible_service_providers(&self) -> Vec<String> {
        vec![aws_lambda::provider_name()]
    }
}
