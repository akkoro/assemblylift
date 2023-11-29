use std::path::PathBuf;

use anyhow::{anyhow, Result};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

use crate::{providers::aws_lambda, snake_case, ContentType, Fragment, Options};

use super::{ApiProvider, DnsProvider, FunctionProvider, Provider, ServiceProvider};

pub fn provider_name() -> String {
    "aws-apigw".into()
}

#[derive(Serialize, Deserialize)]
pub struct ApiGatewayProvider {
    #[serde(default = "provider_name")]
    name: String,
    options: Options,
}

impl ApiGatewayProvider {
    pub fn new(options: Options) -> Box<Self> {
        Box::new(Self {
            name: provider_name(),
            options,
        })
    }
}

#[typetag::serde]
impl Provider for ApiGatewayProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn platform(&self) -> String {
        "aws".into()
    }

    fn options(&self) -> Options {
        self.options.clone()
    }

    fn boot(&self) -> Option<Result<()>> {
        None
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
        Ok(self)
    }

    fn as_dns_provider(&self) -> Result<&dyn DnsProvider> {
        Err(anyhow!("{} is not a DnsProvider", self.name()))
    }
}

impl ApiProvider for ApiGatewayProvider {
    fn cast_service(
        &self,
        service: &crate::context::Service,
    ) -> crate::CastResult<Vec<crate::Fragment>> {
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
        println!("api frag:\n{}", api_fragment.content);

        fragments.append(&mut vec![api_fragment]);

        Ok(fragments)
    }

    fn supported_service_providers(&self) -> Vec<String> {
        vec![aws_lambda::provider_name()]
    }
}
