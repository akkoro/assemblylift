use std::path::PathBuf;

use anyhow::{anyhow, Result};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

use crate::{
    concat_cast,
    context::{Function, Service},
    snake_case, CastError, CastResult, ContentType, Fragment, Options,
};

use super::{ApiProvider, DnsProvider, FunctionProvider, Provider, ServiceProvider};

pub fn provider_name() -> String {
    "aws-lambda".into()
}

pub fn platform_name() -> String {
    "aws".into()
}

#[derive(Serialize, Deserialize)]
pub struct AwsLambdaProvider {
    #[serde(default = "provider_name")]
    name: String,
    #[serde(default = "platform_name")]
    platform: String,
    options: Options,
}

impl AwsLambdaProvider {
    pub fn new(options: Options) -> Box<Self> {
        Box::new(Self {
            name: provider_name(),
            platform: platform_name(),
            options,
        })
    }
}

#[typetag::serde]
impl Provider for AwsLambdaProvider {
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

    fn as_api_provider(&self) -> Result<&dyn ApiProvider> {
        Err(anyhow!("{} is not a ApiProvider", self.name()))
    }

    fn as_dns_provider(&self) -> Result<&dyn DnsProvider> {
        Err(anyhow!("{} is not a DnsProvider", self.name()))
    }
}

impl ServiceProvider for AwsLambdaProvider {
    fn cast_service(&self, service: &Service) -> CastResult<Vec<Fragment>> {
        let mut fragments: Vec<Fragment> = Vec::new();

        let mut function_fragments = service
            .functions
            .iter()
            .map(|function| {
                self.as_function_provider()
                    .unwrap()
                    .cast_function(function, &service.name)
            })
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
    fn cast_function(&self, function: &Function, service_name: &str) -> CastResult<Vec<Fragment>> {
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
                service_name,
                self.name(),
                function.name,
            )),
        };

        fragments.append(&mut vec![function_fragment]);

        Ok(fragments)
    }
}
