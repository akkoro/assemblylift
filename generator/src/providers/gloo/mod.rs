use std::{collections::HashMap, path::PathBuf};

use anyhow::{anyhow, Result};
use handlebars::Handlebars;
use jsonpath_lib::Selector;
use serde::{Deserialize, Serialize};

use assemblylift_tools::{glooctl::GlooCtl, kubectl::KubeCtl};

use crate::{
    context::Service, providers::kubernetes, snake_case, CastError, CastResult, ContentType,
    Fragment, Options,
};

use super::{
    ApiProvider, ContainerRegistryProvider, DnsProvider, FunctionProvider, Provider,
    ServiceProvider,
};

pub fn provider_name() -> String {
    "gloo".into()
}

#[derive(Serialize, Deserialize)]
pub struct GlooProvider {
    #[serde(default = "provider_name")]
    name: String,
    options: Options,
}

impl GlooProvider {
    pub fn new(options: Options) -> Box<Self> {
        Box::new(Self {
            name: provider_name(),
            options,
        })
    }

    pub fn gloo_proxy_ip(&self) -> Option<String> {
        let mut labels = HashMap::new();
        labels.insert("gloo".to_string(), "gateway-proxy".to_string());
        let kubectl = KubeCtl::default();
        let gateways = kubectl
            .get_in_namespace("services", "gloo-system", Some(labels))
            .unwrap();
        let mut selector = Selector::new();
        let v: Vec<String> = selector
            .str_path("$.items[0].status.loadBalancer.ingress[0].ip")
            .unwrap()
            .value(&gateways)
            .select_as()
            .unwrap();
        match v.len() > 0 {
            true => Some(v[0].clone()),
            false => None,
        }
    }
}

#[typetag::serde]
impl Provider for GlooProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn platform(&self) -> String {
        "kubernetes".into()
    }

    fn options(&self) -> Options {
        self.options.clone()
    }

    fn set_option(&mut self, key: &str, value: &str) {
        self.options.insert(key.into(), value.into());
    }

    fn boot(&self) -> Result<()> {
        Ok(GlooCtl::default().install_gateway())
    }

    fn is_booted(&self) -> bool {
        self.gloo_proxy_ip().is_some()
    }

    fn as_service_provider(&self) -> Result<&dyn ServiceProvider> {
        Err(anyhow!("{} is not a ServiceProvider", self.name()))
    }

    fn as_function_provider(&self) -> Result<&dyn FunctionProvider> {
        Err(anyhow!("{} is not a FunctionProvider", self.name()))
    }

    fn as_api_provider(&self) -> Result<&dyn ApiProvider> {
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

impl ApiProvider for GlooProvider {
    fn cast_service(&self, service: &Service) -> CastResult<Vec<Fragment>> {
        let mut svc: Service = service.into();
        svc.api.provider.set_option(
            "__cluster_ip",
            &self
                .gloo_proxy_ip()
                .ok_or(CastError("no IP found for gloo gateway".into()))?,
        );

        let mut fragments: Vec<Fragment> = Vec::new();

        let mut hbs = Handlebars::new();
        hbs.register_helper("snake_case", Box::new(snake_case));
        hbs.register_template_string("root", include_str!("templates/api_impl.tf.handlebars"))
            .unwrap();

        let api_fragment = Fragment {
            content_type: ContentType::HCL,
            content: hbs.render("root", &svc.as_json().unwrap()).unwrap(),
            write_path: PathBuf::from(format!(
                "net/services/{}/infra/{}/api.tf",
                service.name,
                self.name(),
            )),
        };

        fragments.append(&mut vec![api_fragment]);

        Ok(fragments)
    }

    fn supported_service_providers(&self) -> Vec<String> {
        vec![kubernetes::provider_name()]
    }
}
