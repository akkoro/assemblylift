use std::path::PathBuf;

use anyhow::{anyhow, Result};
use assemblylift_tools::cmctl::CmCtl;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

use crate::{
    concat_cast,
    context::{Function, Service},
    snake_case, CastError, CastResult, ContentType, Fragment, Options,
};

use super::{
    GatewayProvider, ContainerRegistryProvider, DnsProvider, FunctionProvider, Provider,
    ServiceProvider, Platform,
};

pub fn provider_name() -> String {
    "k8s".into()
}

#[derive(Serialize, Deserialize)]
pub struct KubernetesProvider {
    #[serde(default = "provider_name")]
    name: String,
    options: Options,
    platform: Option<Platform>,
}

impl KubernetesProvider {
    pub fn new(options: Options, platform: Option<Platform>) -> Box<Self> {
        Box::new(Self {
            name: provider_name(),
            options,
            platform,
        })
    }
}

#[typetag::serde]
impl Provider for KubernetesProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn platform(&self) -> Option<Platform> {
        self.platform.clone()
    }

    fn compatible_platforms(&self) -> Vec<String> {
        vec!["kubernetes".into()]
    }

    fn options(&self) -> Options {
        self.options.clone()
    }

    fn set_option(&mut self, key: &str, value: &str) {
        self.options.insert(key.into(), value.into()).unwrap();
    }

    fn boot(&self) -> Result<()> {
        // TODO this is only needed by the DNS provider and only if K8s is in use
        //      could have boot take a &Context and do it that way
        let kubeconfig = self.platform.as_ref().unwrap().options.get("config_path").unwrap();
        CmCtl::default_with_config(kubeconfig.into()).install();
        Ok(())
    }

    fn is_booted(&self) -> bool {
        false // TODO
    }

    fn as_service_provider(&self) -> Result<&dyn ServiceProvider> {
        Ok(self)
    }

    fn as_function_provider(&self) -> Result<&dyn FunctionProvider> {
        Ok(self)
    }

    fn as_gateway_provider(&self) -> Result<&dyn GatewayProvider> {
        Err(anyhow!("{} is not a GatewayProvider", self.name()))
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

impl ServiceProvider for KubernetesProvider {
    fn cast_service(&self, service: &Service) -> CastResult<Vec<Fragment>> {
        if service.container_registry.is_none() {
            return Err(CastError(format!(
                "service `{}` requires registry_id to be set",
                service.name.clone()
            )));
        }
        
        let mut fragments: Vec<Fragment> = Vec::new();

        let mut function_fragments = service
            .functions
            .iter()
            .map(|function| self.as_function_provider().unwrap().cast_function(function))
            .reduce(concat_cast)
            .unwrap()?;

        let mut hbs = Handlebars::new();
        hbs.register_helper("snake_case", Box::new(snake_case));
        hbs.register_template_string("root", include_str!("templates/service_impl.tf.handlebars"))
            .unwrap();

        let service_fragment = Fragment {
            content_type: ContentType::HCL,
            content: hbs.render("root", &service.as_json().unwrap()).unwrap(),
            write_path: PathBuf::from(format!(
                "net/services/{}/infra/{}/service.tf",
                service.name,
                self.name(),
            )),
        };

        fragments.append(&mut vec![service_fragment]);
        fragments.append(&mut function_fragments);

        Ok(fragments)
    }
}

impl FunctionProvider for KubernetesProvider {
    fn cast_function(&self, function: &Function) -> CastResult<Vec<Fragment>> {
        let mut hbs = Handlebars::new();
        hbs.register_helper("snake_case", Box::new(snake_case));
        hbs.register_template_string(
            "root",
            include_str!("templates/function_impl.tf.handlebars"),
        )
        .unwrap();
        hbs.register_template_string(
            "dockerfile",
            include_str!("templates/function.dockerfile.handlebars"),
        )
        .unwrap();

        let tf_fragment = Fragment {
            content_type: ContentType::HCL,
            content: hbs.render("root", &function.as_json().unwrap()).unwrap(),
            write_path: PathBuf::from(format!(
                "net/services/{}/infra/{}/functions/{}/infra/function.tf",
                function.service_name,
                self.name(),
                function.name,
            )),
        };

        let dockerfile_fragment = Fragment {
            content_type: ContentType::Dockerfile,
            content: hbs
                .render("dockerfile", &function.as_json().unwrap())
                .unwrap(),
            write_path: PathBuf::from(format!(
                "net/services/{}/functions/{}/Dockerfile",
                function.service_name, function.name,
            )),
        };

        Ok(vec![tf_fragment, dockerfile_fragment])
    }
}
