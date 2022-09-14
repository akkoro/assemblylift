use std::rc::Rc;
use std::sync::Arc;

use handlebars::{Handlebars, to_json};
use itertools::Itertools;
use jsonpath_lib::Selector;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::providers::{
    KUBERNETES_PROVIDER_NAME, Options, Provider, ProviderError, ROUTE53_PROVIDER_NAME,
};
use crate::tools::cmctl::CmCtl;
use crate::tools::glooctl::GlooCtl;
use crate::tools::kubectl::KubeCtl;
use crate::transpiler::{
    Artifact, Bindable, Bootable, Castable, CastError, ContentType, StringMap, Template,
};
use crate::transpiler::context::{Context, Domain, Function};

pub struct ApiProvider {
    options: Arc<Options>,
}

impl ApiProvider {
    pub fn new() -> Self {
        Self {
            options: Arc::new(Options::new()),
        }
    }

    fn is_installed() -> bool {
        let kubectl = KubeCtl::default();
        let namespaces = kubectl.get_namespaces().unwrap();
        let items = namespaces.get("items").unwrap().as_array().unwrap();
        // TODO use jsonpath
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

    fn domain_for_service(&self, ctx: Rc<Context>, name: &str) -> String {
        let project_name = ctx.project.name.clone();
        let domain_name = ctx
            .service(&name)
            .unwrap()
            .domain_name
            .as_ref()
            .unwrap_or(&"local".to_string())
            .clone();
        let domain = ctx
            .domains
            .iter()
            .find(|&d| &d.dns_name == &domain_name);
        match domain {
            Some (domain) => {
                match domain.map_to_root {
                    true => format!("{}.{}", name, domain_name),
                    false => format!("{}.{}.{}", name, project_name, domain_name),
                }
            }
            None => format!("{}.{}.{}", name, project_name, domain_name),
        }
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
            domain_name: self.domain_for_service(ctx.clone(), service_name),
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
        Ok(())
    }
}

impl Bootable for ApiProvider {
    fn boot(&self, ctx: Rc<Context>) -> Result<(), CastError> {
        // TODO prompt before install & handle errors
        CmCtl::default().install();

        let kubectl = KubeCtl::default();
        let project_name = ctx.project.name.clone();

        let route53_provider = ctx
            .domains
            .iter()
            .find(|&d| d.provider.name == ROUTE53_PROVIDER_NAME);

        let issuer_yaml = CertIssuerTemplate {
            project_name: project_name.clone(),
            route53_options: match route53_provider {
                Some(r53) => {
                    let opts = r53.provider.options.clone();
                    println!("DEBUG opts={:?}", opts);
                    match opts.get("cert_manager_aws_credentials_secret_name") {
                        Some(secret) => {
                            let secrets = kubectl
                                .get_in_namespace("secrets", "cert-manager", None)
                                .unwrap();
                            // println!("DEBUG secrets={:?}", secrets);
                            let mut selector = Selector::new();
                            let results = selector
                                .str_path(&*format!(
                                    "$.items[?(@.metadata.name == '{}')].data",
                                    secret
                                ))
                                .unwrap()
                                .value(&secrets)
                                .select()
                                .unwrap();
                            let data = results[0];
                            let mut opts = StringMap::new();
                            opts.insert(
                                "cert_manager_aws_credentials_secret_name".to_string(),
                                secret.clone(),
                            );
                            opts.insert(
                                "aws_region".to_string(),
                                std::str::from_utf8(
                                    &*base64::decode(
                                        data.get("aws_region")
                                            .unwrap()
                                            .as_str()
                                            .unwrap()
                                            .to_string(),
                                    )
                                    .unwrap(),
                                )
                                .unwrap()
                                .to_string(),
                            );
                            opts.insert(
                                "aws_access_key_id".to_string(),
                                std::str::from_utf8(
                                    &*base64::decode(
                                        data.get("aws_access_key_id")
                                            .unwrap()
                                            .as_str()
                                            .unwrap()
                                            .to_string(),
                                    )
                                    .unwrap(),
                                )
                                .unwrap()
                                .to_string(),
                            );
                            opts.insert(
                                "aws_secret_access_key".to_string(),
                                std::str::from_utf8(
                                    &*base64::decode(
                                        data.get("aws_secret_access_key")
                                            .unwrap()
                                            .as_str()
                                            .unwrap()
                                            .to_string(),
                                    )
                                    .unwrap(),
                                )
                                .unwrap()
                                .to_string(),
                            );
                            Arc::new(opts)
                        }
                        None => Default::default(),
                    }
                }
                _ => Default::default(),
            },
            has_route53: route53_provider.is_some(),
        }
        .render();
        kubectl
            .apply_from_str(&issuer_yaml)
            .expect("could not apply issuer yaml");

        for service in ctx
            .services
            .iter()
            .filter(|&s| &s.provider.name == KUBERNETES_PROVIDER_NAME)
            .collect_vec()
        {
            if let Some(_) = service.domain_name.clone() {
                let certificate_yaml = CertificateTemplate {
                    project_name: project_name.clone(),
                    service_name: service.name.clone(),
                    domain_name: self.domain_for_service(ctx.clone(), &service.name.clone()),
                }
                .render();
                kubectl
                    .apply_from_str(&certificate_yaml)
                    .expect("could not apply certificate yaml");
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
            // TODO use jsonpath
            issuers
                .iter()
                .find(|&v| {
                    v.get("metadata")
                        .unwrap()
                        .get("name")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        == "asml-letsencrypt-staging"
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
    metadata: Metadata,
}

#[derive(Deserialize, Clone, Debug)]
struct Metadata {
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
      {{#if has_domain}}sslConfig = {
        secretRef = {
          name      = "asml-{{project_name}}-{{service_name}}-tls"
          namespace = "asml-{{project_name}}-{{service_name}}"
        }
      }{{/if}}
    }
  }
}
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
    name: asml-letsencrypt-staging
  dnsNames:
  - {{domain_name}}
"#
    }
}

#[derive(Serialize)]
struct CertIssuerTemplate {
    project_name: String,
    route53_options: Arc<Options>,
    has_route53: bool,
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
  name: asml-letsencrypt-staging
spec:
  acme:
    server: https://acme-staging-v02.api.letsencrypt.org/directory
    email: assemblylift@akkoro.io
    privateKeySecretRef:
      name: asml-letsencrypt-staging
    solvers:
    {{#if has_route53}}- dns01:
        route53:
          region: {{route53_options.aws_region}}
          accessKeyID: {{route53_options.aws_access_key_id}}
          secretAccessKeySecretRef:
            name: {{route53_options.cert_manager_aws_credentials_secret_name}}
            key: aws_secret_access_key
    {{/if}}
"#
    }
}

#[derive(Serialize)]
struct RouteData {
    path: String,
    to_function_name: String,
}
