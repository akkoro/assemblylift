use std::rc::Rc;
use std::sync::Arc;

use handlebars::{Handlebars, to_json};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::providers::{BoxedCastable, Options, Provider, ProviderError};
use crate::tools::glooctl::GlooCtl;
use crate::tools::kubectl::KubeCtl;
use crate::transpiler::{Castable, CastError, ContentType};
use crate::transpiler::context::{Context, Function, Http};

pub struct ApiProvider {
    gloo_installed: Option<bool>,
    options: Arc<Options>,
}

impl ApiProvider {
    pub fn new() -> Self {
        Self {
            gloo_installed: None,
            options: Arc::new(Options::new()),
        }
    }
}

impl Castable for ApiProvider {
    fn cast(&mut self, ctx: Rc<Context>, name: &str) -> Result<Vec<String>, CastError> {
        let project_name = ctx.project.name.clone();
        {
            // FIXME this should really happen at the beginning of the `bind` command,
            //       but we don't have a way to queue actions from `cast` (yet)
            if !self.gloo_installed.unwrap() {
                let glooctl = GlooCtl::default();
                glooctl.install_gateway(&project_name);
            }
        }

        let template_name = "yaml_template";
        let mut reg = Box::new(Handlebars::new());
        reg.register_template_string(template_name, VIRTUALSERVICE_TEMPLATE)
            .unwrap();

        let http_fns: Vec<&Function> = ctx
            .functions
            .iter()
            .filter(|f| f.http.is_some())
            .map(|f| f)
            .collect();

        let data = VirtualServiceData {
            project_name: project_name.clone(),
            has_routes: http_fns.len() > 0,
            routes: http_fns
                .iter()
                .map(|f| RouteData {
                    path: f.http.as_ref().unwrap().path.clone(),
                    to_service_name: f.service_name.clone(),
                    to_function_name: f.name.clone(),
                })
                .collect(),
        };
        let data = to_json(data);
        let rendered_yaml = reg
            .render(template_name, &data)
            .expect("couldn't render yaml template");
        Ok(vec![rendered_yaml])
    }

    fn content_type(&self) -> Vec<ContentType> {
        vec![ContentType::KubeYaml("kube-yaml")]
    }
}

impl Provider for ApiProvider {
    fn name(&self) -> String {
        String::from("k8s-gloo")
    }

    fn init(&mut self, ctx: Rc<Context>, name: &str) -> Result<(), ProviderError> {
        let kubectl = KubeCtl::default();
        let namespaces = kubectl.get_namespaces().unwrap();
        let items = namespaces.get("items").unwrap().as_array().unwrap();
        let is_installed = items
            .iter()
            .find(|i| {
                i.get("metadata")
                    .unwrap()
                    .as_object()
                    .unwrap()
                    .get("labels")
                    .unwrap()
                    .as_object()
                    .unwrap()
                    .get("kubernetes.io/metadata.name")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .contains("asml-gloo-")
            })
            .is_some();
        self.gloo_installed = Some(is_installed);

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

#[derive(Deserialize, Clone, Debug)]
struct VirtualService {
    #[serde(rename = "apiVersion")]
    api_version: String,
    kind: String,
    metadata: std::collections::HashMap<String, Value>,
    spec: VirtualServiceSpec,
}

#[derive(Deserialize, Clone, Debug)]
struct VirtualServiceSpec {
    #[serde(rename = "virtualHost")]
    virtual_host: VirtualHost,
}

#[derive(Deserialize, Clone, Debug)]
struct VirtualHost {
    domains: Vec<String>,
    routes: Vec<Route>,
}

#[derive(Deserialize, Clone, Debug)]
struct Route {
    matchers: Vec<std::collections::HashMap<String, String>>,
    #[serde(rename = "routeAction")]
    route_action: RouteAction,
}

#[derive(Deserialize, Clone, Debug)]
struct RouteAction {
    single: RouteActionSingle,
}

#[derive(Deserialize, Clone, Debug)]
struct RouteActionSingle {
    upstream: Upstream,
}

#[derive(Deserialize, Clone, Debug)]
struct Upstream {
    name: String,
    namespace: String,
}

#[derive(Serialize)]
struct VirtualServiceData {
    project_name: String,
    has_routes: bool,
    routes: Vec<RouteData>,
}

#[derive(Serialize)]
struct RouteData {
    path: String,
    to_service_name: String,
    to_function_name: String,
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
    {{#if has_routes}}routes:
    {{#each routes}}- matchers:
      - exact: {{this.path}}
      routeAction:
        single:
          upstream:
            name: asml-{{../project_name}}-{{to_service_name}}-asml-{{to_service_name}}-{{to_function_name}}-5543
            namespace: asml-gloo-{{../project_name}}
    {{/each}}
    {{/if}}
"#;
