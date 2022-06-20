use std::rc::Rc;
use std::sync::Arc;

use handlebars::{Handlebars, to_json};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::providers::{Options, Provider, ProviderError};
use crate::tools::glooctl::GlooCtl;
use crate::tools::kubectl::KubeCtl;
use crate::transpiler::{Artifact, Bindable, Castable, CastError, ContentType, Template};
use crate::transpiler::context::{Context, Function};

pub struct ApiProvider {
    gloo_installed: bool,
    options: Arc<Options>,
}

impl ApiProvider {
    pub fn new() -> Self {
        Self {
            gloo_installed: Self::is_installed(),
            options: Arc::new(Options::new()),
        }
    }

    fn is_installed() -> bool {
        let kubectl = KubeCtl::default();
        let namespaces = kubectl.get_namespaces().unwrap();
        let items = namespaces.get("items").unwrap().as_array().unwrap();
        items
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
                    .contains("gloo-system")
            })
            .is_some()
    }
}

impl Castable for ApiProvider {
    /// `selector` parameter is the name of service to deploy this API for
    fn cast(&self, ctx: Rc<Context>, selector: Option<&str>) -> Result<Vec<Artifact>, CastError> {
        let project_name = ctx.project.name.clone();
        let service_name = selector
            .expect("selector must be a service name")
            .to_string();

        let http_fns: Vec<&Function> = ctx
            .functions
            .iter()
            .filter(|f| f.http.is_some())
            .map(|f| f)
            .collect();

        let rendered_hcl = VirtualServiceTemplate {
            project_name: project_name.clone(),
            service_name: service_name.clone(),
            has_routes: http_fns.len() > 0,
            routes: http_fns
                .iter()
                .filter(|f| f.service_name == service_name)
                .map(|f| RouteData {
                    path: f.http.as_ref().unwrap().path.clone(),
                    to_function_name: f.name.clone(),
                })
                .collect(),
        }.render();
        let out = Artifact {
            content_type: ContentType::HCL("HCL"),
            content: rendered_hcl,
            write_path: "net/plan.tf".into(),
        };
        Ok(vec![out])
    }
}

impl Bindable for ApiProvider {
    fn bind(&self, ctx: Rc<Context>) -> Result<(), CastError> {
        todo!()
    }
}

impl Provider for ApiProvider {
    fn name(&self) -> String {
        String::from("k8s-gloo")
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
struct VirtualServiceTemplate {
    project_name: String,
    service_name: String,
    has_routes: bool,
    routes: Vec<RouteData>,
}

impl Template for VirtualServiceTemplate {
    fn render(&self) -> String {
        let mut reg = Box::new(Handlebars::new());
        reg.register_template_string("hcl_template", Self::tmpl())
            .unwrap();
        reg.render("hcl_template", &self).unwrap()
    }

    fn tmpl() -> &'static str {
        r#"# Begin Gloo VirtualService
resource kubernetes_manifest gloo_virtualservice_{{service_name}} {
  provider = kubernetes.{{project_name}}
  manifest = {
    apiVersion = "gateway.solo.io/v1"
    kind       = "VirtualService"

    metadata = {
      name      = "{{project_name}}"
      namespace = "asml-{{project_name}}-{{service_name}}"
    }

    spec = {
      virtualHost = {
        domains = ["*"] # TODO service domain
        {{#if has_routes}}routes = [
          {{#each routes}}{
            matchers = [
              {
                exact = "{{this.path}}"
              }
            ]
            routeAction = {
              single = {
                upstream = {
                  name      = "asml-{{../project_name}}-{{../service_name}}-asml-{{../service_name}}-{{to_function_name}}-5543"
                  namespace = "gloo-system"
                }
              }
            }
          },
        {{/each}}]{{/if}}
      }
    }
  }
}

"#
    }
}

#[derive(Serialize)]
struct RouteData {
    path: String,
    to_function_name: String,
}
