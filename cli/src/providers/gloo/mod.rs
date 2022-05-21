use std::rc::Rc;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::providers::{BoxedCastable, Options, Provider, ProviderError};
use crate::transpiler::{Castable, CastError, ContentType};
use crate::transpiler::asml::Context;

pub struct ApiProvider {
    options: Arc<Options>,
}

impl ApiProvider {
    pub fn new() -> Self {
        Self {
            options: Arc::new(Options::new()),
        }
    }
}

impl Castable for ApiProvider {
    fn cast(&mut self, ctx: Rc<Context>, name: &str) -> Result<Vec<String>, CastError> {
        todo!()
    }

    fn content_type(&self) -> Vec<ContentType> {
        todo!()
    }
}

impl Provider for ApiProvider {
    fn name(&self) -> String {
        String::from("k8s-gloo")
    }

    fn init(&self, ctx: Rc<Context>, name: String) -> Result<(), ProviderError> {
        Ok(())
    }

    fn options(&self) -> Arc<Options> {
        self.options.clone()
    }

    fn set_options(&mut self, opts: Arc<Options>) -> Result<(), ProviderError> {
        self.options = opts;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct VirtualService {
    #[serde(rename = "apiVersion")]
    api_version: String,
    kind: String,
    metadata: std::collections::HashMap<String, Value>,
    spec: VirtualServiceSpec,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct VirtualServiceSpec {
    #[serde(rename = "virtualHost")]
    virtual_host: VirtualHost,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct VirtualHost {
    domains: Vec<String>,
    routes: Vec<Route>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Route {
    matchers: Vec<std::collections::HashMap<String, String>>,
    #[serde(rename = "routeAction")]
    route_action: RouteAction,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct RouteAction {
    single: RouteActionSingle,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct RouteActionSingle {
    upstream: Upstream,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Upstream {
    name: String,
    namespace: String,
}

static VIRTUALSERVICE_TEMPLATE: &str = r#"apiVersion: gateway.solo.io/v1
kind: VirtualService
metadata:
  name: {{project_name}}
  namespace: asml-gloo-{{project_name}}
spec:
  virtualHost:
    domains:
    - '*'
    {{#if has_routes}}
    routes:
    {{#each routes}}- matchers:
    {{/each}}
    {{/if}}
"#;
