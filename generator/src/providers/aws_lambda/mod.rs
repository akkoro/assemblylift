use std::path::PathBuf;

use anyhow::{anyhow, Result};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

use crate::{
    concat_cast,
    context::{Function, Service},
    snake_case, CastResult, ContentType, Fragment, Options,
};

use super::{
    GatewayProvider, ContainerRegistryProvider, DnsProvider, FunctionProvider, Platform, Provider,
    ServiceProvider,
};

pub fn provider_name() -> String {
    "aws-lambda".into()
}

#[derive(Serialize, Deserialize)]
pub struct AwsLambdaProvider {
    #[serde(default = "provider_name")]
    name: String,
    // #[serde(default = "platform_name")]
    // platform: String,
    options: Options,
    platform: Option<Platform>,
}

impl AwsLambdaProvider {
    pub fn new(options: Options, platform: Option<Platform>) -> Box<Self> {
        Box::new(Self {
            name: provider_name(),
            options,
            platform,
        })
    }
}

#[typetag::serde]
impl Provider for AwsLambdaProvider {
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
        let runtime_url = &*format!(
            "http://public.assemblylift.akkoro.io/runtime/{}/aws-lambda/bootstrap.zip",
            // clap::crate_version!(),
            "0.4.0-beta.0"
        );

        std::fs::create_dir_all("./.asml/runtime").unwrap();
        assemblylift_tools::download_to_path(runtime_url, "./.asml/runtime/bootstrap.zip")?;

        // FIXME handle errors
        Ok(())
    }

    fn is_booted(&self) -> bool {
        std::path::Path::new("./.asml/runtime/bootstrap.zip").exists()
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

impl ServiceProvider for AwsLambdaProvider {
    fn cast_service(&self, service: &Service) -> CastResult<Vec<Fragment>> {
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

impl FunctionProvider for AwsLambdaProvider {
    fn cast_function(&self, function: &Function) -> CastResult<Vec<Fragment>> {
        let mut fragments: Vec<Fragment> = Vec::new();

        let mut hbs = Handlebars::new();
        hbs.register_template_string(
            "root",
            include_str!("templates/function_impl.tf.handlebars"),
        )
        .unwrap();

        let function_fragment = Fragment {
            content_type: ContentType::HCL,
            content: hbs.render("root", &function.as_json().unwrap()).unwrap(),
            write_path: PathBuf::from(format!(
                "net/services/{}/infra/{}/functions/{}/infra/function.tf",
                function.service_name,
                self.name(),
                function.name,
            )),
        };

        fragments.append(&mut vec![function_fragment]);

        Ok(fragments)
    }
}
