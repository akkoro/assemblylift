use std::os::linux::raw::stat;
use std::rc::Rc;
use std::sync::Arc;

use handlebars::{to_json, Handlebars};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::providers::{Options, Provider, ProviderError};
use crate::tools::cmctl::CmCtl;
use crate::tools::glooctl::GlooCtl;
use crate::tools::kubectl::KubeCtl;
use crate::transpiler::context::{Context, Function};
use crate::transpiler::{Artifact, Bindable, Bootable, CastError, Castable, ContentType, Template};

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
        let service_name = selector.expect("selector must be a service name");
        let project_name = ctx.project.name.clone();

        let http_fns: Vec<&Function> = ctx
            .functions
            .iter()
            .filter(|&f| f.http.is_some() && &f.service_name == &service_name)
            .map(|f| f)
            .collect();

        let rendered_hcl = VirtualServiceTemplate {
            project_name: project_name.clone(),
            service_name: service_name.into(),
            domain_name: format!(
                "{}.{}",
                &service_name,
                ctx.service(&service_name)
                    .unwrap()
                    .domain_name
                    .as_ref()
                    .unwrap_or(&format!("{}.com", &project_name))
            ),
            has_routes: http_fns.len() > 0,
            has_domain: ctx.service(&service_name).unwrap().domain_name.is_some(),
            routes: http_fns
                .iter()
                .filter(|&f| f.service_name == service_name)
                .map(|f| RouteData {
                    path: f.http.as_ref().unwrap().path.clone(),
                    to_function_name: f.name.clone(),
                })
                .collect(),
        }
        .render();
        let out = Artifact {
            content_type: ContentType::HCL("HCL"),
            content: rendered_hcl,
            write_path: "net/plan.tf".into(),
        };
        Ok(vec![out])
    }
}

impl Bindable for ApiProvider {
    fn bind(&self, _ctx: Rc<Context>) -> Result<(), CastError> {
        CmCtl::default().install();
        Ok(())
    }
}

impl Bootable for ApiProvider {
    fn boot(&self, ctx: Rc<Context>) -> Result<(), CastError> {
        let kubectl = KubeCtl::default();
        let project_name = ctx.project.name.clone();

        let issuer_yaml = CertIssuerTemplate {
            project_name: project_name.clone(),
            domain_names: ctx
                .services
                .clone()
                .into_iter()
                .map(|s| {
                    s.domain_name
                        .clone()
                        .unwrap_or(format!("{}.com", &project_name))
                })
                .collect_vec(),
        }
        .render();
        kubectl
            .apply_from_str(&issuer_yaml)
            .expect("could not apply issuer yaml");

        for service in ctx
            .services
            .iter()
            .filter(|&s| &s.provider.name == "k8s")
            .collect_vec()
        {
            if let Some(domain_name) = service.domain_name.clone() {
                let certificate_yaml = CertificateTemplate {
                    project_name: project_name.clone(),
                    service_name: service.name.clone(),
                    domain_name,
                }
                .render();
                kubectl
                    .apply_from_str(&certificate_yaml)
                    .expect("could not apply certificate yaml");

                // TODO get order token & create virtual service
                let orders = kubectl
                    .get_in_namespace(
                        "orders.acme.cert-manager.io",
                        &format!("asml-{}-{}", project_name, service.name.clone()),
                    )
                    .expect("kubectl could not get acme orders");
                let token = orders
                    .get("items")
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .iter()
                    .find_map(|&item| {
                        let status = serde_json::from_value::<Status>(item.get("status").unwrap().clone())
                            .unwrap();
                        if status.authorizations.len() == 0 {
                            None
                        }
                        match status.authorizations[0]
                            .challenges
                            .iter()
                            .find(|&c| &c.r#type == "http-01")
                        {
                            Some(c) => Some(c.token.clone()),
                            None => None,
                        }
                    })
                    .unwrap();
                println!("DEBUG token={:?}", token);
            }
        }

        Ok(())
    }

    fn is_booted(&self, _ctx: Rc<Context>) -> bool {
        let kubectl = KubeCtl::default();
        let issuers = kubectl
            .get("clusterissuers")
            .expect("kubectl could not get clusterissuers");
        return if let Some(issuers) = issuers.get("items").unwrap().as_array() {
            issuers
                .iter()
                .find(|&v| {
                    v.get("metadata")
                        .unwrap()
                        .get("name")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        == "asml-letsencrypt-staging-http01"
                })
                .is_some()
        } else {
            false
        };
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
struct Status {
    authorizations: Vec<Authorization>,
}

#[derive(Deserialize, Clone, Debug)]
struct Authorization {
    challenges: Vec<Challenge>,
}

#[derive(Deserialize, Clone, Debug)]
struct Challenge {
    r#type: String,
    token: String,
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
    domain_name: String,
    has_routes: bool,
    has_domain: bool,
    routes: Vec<RouteData>,
}

impl Template for VirtualServiceTemplate {
    fn render(&self) -> String {
        let mut reg = Box::new(Handlebars::new());
        reg.register_template_string("tmpl", Self::tmpl()).unwrap();
        reg.render("tmpl", &self).unwrap()
    }

    fn tmpl() -> &'static str {
        r#"# Begin Gloo VirtualService
resource kubernetes_manifest gloo_virtualservice_{{service_name}} {
  provider = kubernetes.{{project_name}}-k8s
  manifest = {
    apiVersion = "gateway.solo.io/v1"
    kind       = "VirtualService"

    metadata = {
      name      = "{{service_name}}"
      namespace = "asml-{{project_name}}-{{service_name}}"
    }

    spec = {
      virtualHost = {
        domains = ["{{domain_name}}"]
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
struct AcmeChallengeServiceTemplate {
    project_name: String,
    service_name: String,
    domain_names: Vec<String>,
    token: String,
    upstream_name: String,
}

impl Template for AcmeChallengeServiceTemplate {
    fn render(&self) -> String {
        let mut reg = Box::new(Handlebars::new());
        reg.register_template_string("tmpl", Self::tmpl()).unwrap();
        reg.render("tmpl", &self).unwrap()
    }

    // TODO maybe not namespace in gloo-system
    fn tmpl() -> &'static str {
        r#"# Begin ACME Challenge VirtualService for `{{service_name}}`
apiVersion: gateway.solo.io/v1
kind: VirtualService
metadata:
  name: asml-{{project_name}}-{{service_name}}-letsencrypt
  namespace: gloo-system
spec:
  virtualHost:
    domains:
    {{#each domain_names}}- {{this}}
    {{/each}}
    routes:
    - matchers:
      - exact: /.well-known/acme-challenge/{{token}}
      routeAction:
        single:
          upstream:
            name: {{upstream_name}}
            namespace: gloo-system
"#
    }
}

#[derive(Serialize)]
struct CertificateTemplate {
    project_name: String,
    service_name: String,
    domain_name: String,
}

impl Template for CertificateTemplate {
    fn render(&self) -> String {
        let mut reg = Box::new(Handlebars::new());
        reg.register_template_string("tmpl", Self::tmpl()).unwrap();
        reg.render("tmpl", &self).unwrap()
    }

    fn tmpl() -> &'static str {
        r#"# Begin cert-manager Certificate
apiVersion: cert-manager.io/v1
kind: Certificate
metadata:
  name: asml-{{project_name}}-{{service_name}}-certificate
  namespace: asml-{{project_name}}-{{service_name}}
spec:
  secretName: asml-{{project_name}}-{{service_name}}-tls
  issuerRef:
    kind: ClusterIssuer
    name: asml-letsencrypt-staging-http01
  commonName: {{domain_name}}
  dnsNames:
  - {{domain_name}}
"#
    }
}

#[derive(Serialize)]
struct CertIssuerTemplate {
    project_name: String,
    domain_names: Vec<String>,
}

impl Template for CertIssuerTemplate {
    fn render(&self) -> String {
        let mut reg = Box::new(Handlebars::new());
        reg.register_template_string("tmpl", Self::tmpl()).unwrap();
        reg.render("tmpl", &self).unwrap()
    }

    fn tmpl() -> &'static str {
        r#"# Begin cert-manager ClusterIssuer
apiVersion: cert-manager.io/v1
kind: ClusterIssuer
metadata:
  name: asml-letsencrypt-staging-http01
spec:
  acme:
    server: https://acme-staging-v02.api.letsencrypt.org/directory
    email: assemblylift@akkoro.io
    privateKeySecretRef:
      name: asml-letsencrypt-staging-http01
    solvers:
    - http01:
        ingress:
          serviceType: ClusterIP
      selector:
        dnsNames:
        {{#each domain_names}}- {{this}}
        {{/each}}
"#
    }
}

#[derive(Serialize)]
struct RouteData {
    path: String,
    to_function_name: String,
}
