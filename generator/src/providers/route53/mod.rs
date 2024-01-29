use std::path::PathBuf;

use anyhow::{anyhow, Result};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

use crate::{
    context::{Domain, Service},
    providers::{api_gateway, gloo, Provider},
    snake_case, CastResult, ContentType, Fragment, Options,
};

use super::{
    GatewayProvider, ContainerRegistryProvider, DnsProvider, FunctionProvider, ServiceProvider, Platform,
};

pub fn provider_name() -> String {
    "route53".into()
}

#[derive(Serialize, Deserialize)]
pub struct Route53Provider {
    #[serde(default = "provider_name")]
    name: String,
    options: Options,
    platform: Option<Platform>,
}

impl Route53Provider {
    pub fn new(options: Options, platform: Option<Platform>) -> Box<Self> {
        Box::new(Self {
            name: provider_name(),
            options,
            platform,
        })
    }
}

#[typetag::serde]
impl Provider for Route53Provider {
    fn name(&self) -> String {
        provider_name()
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
        Ok(self)
    }

    fn as_container_registry_provider(&self) -> Result<&dyn ContainerRegistryProvider> {
        Err(anyhow!(
            "{} is not a ContainerRegistryProvider",
            self.name()
        ))
    }
}

impl DnsProvider for Route53Provider {
    fn cast_domain(&self, domain: &Domain) -> CastResult<Vec<Fragment>> {
        let mut hbs = Handlebars::new();
        hbs.register_helper("snake_case", Box::new(snake_case));
        hbs.register_template_string(
            "root",
            include_str!("templates/dns_impl_root.tf.handlebars"),
        )
        .unwrap();

        let root_fragment = Fragment {
            content_type: ContentType::HCL,
            content: hbs.render("root", &domain.as_json().unwrap()).unwrap(),
            write_path: PathBuf::from(format!(
                "net/infra/{}/{}/dns.tf",
                self.name(),
                domain.dns_name
            )),
        };

        Ok(vec![root_fragment])
    }

    fn cast_service(&self, service: &Service) -> CastResult<Vec<Fragment>> {
        let mut hbs = Handlebars::new();
        hbs.register_helper("snake_case", Box::new(snake_case));
        hbs.register_template_string("service", include_str!("templates/dns_impl.tf.handlebars"))
            .unwrap();
        hbs.register_template_string(
            &api_gateway::provider_name(),
            include_str!("templates/dns_impl_apigw.tf.handlebars"),
        )
        .unwrap();
        hbs.register_template_string(
            &gloo::provider_name(),
            include_str!("templates/dns_impl_gloo.tf.handlebars"),
        )
        .unwrap();

        let service_fragment = Fragment {
            content_type: ContentType::HCL,
            content: hbs.render("service", &service.as_json().unwrap()).unwrap(),
            write_path: PathBuf::from(format!(
                "net/services/{}/infra/{}/dns.tf",
                service.name,
                self.name(),
            )),
        };

        Ok(vec![service_fragment])
    }

    fn compatible_gateway_providers(&self) -> Vec<String> {
        vec![api_gateway::provider_name(), gloo::provider_name()]
    }
}
