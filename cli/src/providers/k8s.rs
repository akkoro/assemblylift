use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use clap::crate_version;
use handlebars::Handlebars;
use itertools::Itertools;
use once_cell::sync::Lazy;
use serde::Serialize;

use crate::providers::{DNS_PROVIDERS, flatten, gloo, KUBERNETES_PROVIDER_NAME, LockBox, Options, Provider, ProviderError, ProviderMap};
use crate::tools::glooctl::GlooCtl;
use crate::transpiler::{
    Artifact, Bindable, Bootable, Castable, CastError, ContentType, context, Template,
};
use crate::transpiler::context::Context;

fn to_container_registry(r: &context::Registry) -> ContainerRegistry {
    ContainerRegistry {
        is_dockerhub: r.host.eq_ignore_ascii_case("dockerhub"),
        is_ecr: r.host.eq_ignore_ascii_case("ecr"),
        options: r.options.clone(),
    }
}

pub struct KubernetesProvider {
    api_provider: Arc<gloo::ApiProvider>,
    service_subprovider: KubernetesService,
    options: Arc<Options>,
}

impl KubernetesProvider {
    pub fn new() -> Self {
        let api_provider = Arc::new(gloo::ApiProvider::new());
        Self {
            api_provider: api_provider.clone(),
            service_subprovider: KubernetesService {
                api_provider: api_provider.clone(),
                options: Arc::new(Options::new()),
            },
            options: Arc::new(Options::new()),
        }
    }
}

impl Provider for KubernetesProvider {
    fn name(&self) -> String {
        String::from(KUBERNETES_PROVIDER_NAME)
    }

    fn options(&self) -> Arc<Options> {
        self.options.clone()
    }

    fn set_options(&mut self, opts: Arc<Options>) -> Result<(), ProviderError> {
        self.options = opts.clone();
        self.service_subprovider.options = opts.clone();
        Ok(())
    }
}

// TODO kube context name as provider option
impl Castable for KubernetesProvider {
    fn cast(&self, ctx: Rc<Context>, _selector: Option<&str>) -> Result<Vec<Artifact>, CastError> {
        GlooCtl::default().install_gateway();
        let registries = ctx.registries.iter().map(to_container_registry).collect();

        let mut service_artifacts = ctx
            .services
            .iter()
            .filter(|&s| s.provider.name == self.name())
            .map(|s| {
                self.service_subprovider
                    .cast(ctx.clone(), Some(&s.name))
                    .unwrap()
            })
            .reduce(flatten)
            .unwrap();

        let base_tmpl = KubernetesBaseTemplate {
            project_name: ctx.project.name.clone(),
            docker_config_path: self
                .options
                .get("docker_config_path")
                .unwrap_or(&"~/.docker/config.json".to_string())
                .clone(),
            registries,
        };
        let hcl = Artifact {
            content_type: ContentType::HCL("HCL"),
            content: base_tmpl.render(),
            write_path: "net/plan.tf".to_string(),
        };

        let mut out = vec![hcl];

        out.append(&mut service_artifacts);
        Ok(out)
    }
}

impl Bindable for KubernetesProvider {
    fn bind(&self, ctx: Rc<Context>) -> Result<(), CastError> {
        ctx.services
            .iter()
            .filter(|&s| s.provider.name == self.name())
            .map(|s| self.service_subprovider.bind(ctx.clone()))
            .collect_vec();
        Ok(())
    }
}

impl Bootable for KubernetesProvider {
    fn boot(&self, ctx: Rc<Context>) -> Result<(), CastError> {
        self.api_provider.boot(ctx)
    }

    fn is_booted(&self, ctx: Rc<Context>) -> bool {
        self.api_provider.is_booted(ctx)
    }
}

struct KubernetesService {
    api_provider: Arc<gloo::ApiProvider>,
    options: Arc<Options>,
}

impl Castable for KubernetesService {
    fn cast(&self, ctx: Rc<Context>, selector: Option<&str>) -> Result<Vec<Artifact>, CastError> {
        let name = selector
            .expect("selector must be a service name")
            .to_string();

        let registries = ctx.registries.iter().map(to_container_registry).collect();

        let hcl_content = ServiceTemplate {
            project_name: ctx.project.name.clone(),
            service_name: name.clone(),
            kube_config_path: self
                .options
                .get("kube_config_path")
                .unwrap_or(&"~/.kube/config".to_string())
                .clone(),
            registries,
        }
        .render();

        // TODO in future we want to support other API Gateway providers -- for now, just Gloo :)
        let mut api_artifacts = self.api_provider.cast(ctx.clone(), Some(&*name)).unwrap();

        let function_subprovider = KubernetesFunction {
            service_name: name.clone(),
            options: self.options.clone(),
        };
        let function_artifacts = ctx
            .functions
            .iter()
            .filter(|f| f.service_name == name)
            .map(|f| {
                function_subprovider
                    .cast(ctx.clone(), Some(&f.name))
                    .unwrap()
            })
            .reduce(flatten)
            .unwrap();
        let function_hcl = function_artifacts
            .iter()
            .filter(|a| a.content_type == ContentType::HCL("HCL"))
            .map(|artifact| artifact.content.clone())
            .reduce(|accum, s| format!("{}{}", &accum, &s))
            .unwrap();

        let hcl = Artifact {
            content_type: ContentType::HCL("HCL"),
            content: format!("{}{}", &hcl_content, &function_hcl),
            write_path: "net/plan.tf".into(),
        };
        let mut out = vec![hcl];
        out.append(&mut api_artifacts);
        out.append(
            &mut function_artifacts
                .iter()
                .filter(|a| a.content_type == ContentType::Dockerfile("Dockerfile"))
                .map(|a| a.clone())
                .collect::<Vec<Artifact>>(),
        );
        Ok(out)
    }
}

impl Bindable for KubernetesService {
    fn bind(&self, ctx: Rc<Context>) -> Result<(), CastError> {
        self.api_provider.bind(ctx)
    }
}

struct KubernetesFunction {
    service_name: String,
    options: Arc<Options>,
}

impl Castable for KubernetesFunction {
    fn cast(&self, ctx: Rc<Context>, selector: Option<&str>) -> Result<Vec<Artifact>, CastError> {
        let name = selector
            .expect("selector must be a function name")
            .to_string();
        match ctx
            .functions
            .iter()
            .filter(|&f| f.service_name == self.service_name)
            .find(|&f| f.name == name)
        {
            Some(function) => {
                let service = function.service_name.clone();

                let iomods: Vec<IomodContainer> = ctx
                    .iomods
                    .iter()
                    .filter(|i| i.service_name == service.clone())
                    .map(|i| {
                        let coords: Vec<&str> = i.coordinates.split(".").collect();
                        IomodContainer {
                            // TODO eventually we'll allow overriding which service/mirror the IOmods come from
                            image: format!(
                                "public.ecr.aws/{}/iomod/{}/{}:{}",
                                coords[0], coords[1], coords[2], i.version
                            ),
                            name: i.coordinates.clone().replacen('.', "-", 2),
                        }
                    })
                    .collect();

                let registries: Vec<ContainerRegistry> =
                    ctx.registries.iter().map(to_container_registry).collect();

                let hcl_tmpl = FunctionTemplate {
                    base_image_version: crate_version!().to_string(),
                    project_name: ctx.project.name.clone(),
                    function_name: function.name.clone(),
                    service_name: service.clone(),
                    handler_name: match function.language.as_str() {
                        "rust" => format!("{}.wasmu", function.name.clone()),
                        "ruby" => "ruby.wasmu".into(),
                        _ => "handler".into(),
                    },
                    iomods: iomods.clone(),
                    has_iomods: iomods.len() > 0,
                    registry: ContainerRegistry {
                        is_dockerhub: function.registry.eq_ignore_ascii_case("dockerhub"),
                        is_ecr: function.registry.eq_ignore_ascii_case("ecr"),
                        options: registries
                            .iter()
                            .find(|r| {
                                (r.is_ecr && function.registry.eq_ignore_ascii_case("ecr"))
                                    || (r.is_dockerhub
                                        && function.registry.eq_ignore_ascii_case("dockerhub"))
                            })
                            .expect(&*format!(
                                "no registry configured for `{}` for function `{}`",
                                &function.registry, &function.name
                            ))
                            .options
                            .clone(),
                    },
                    is_ruby: function.language == "ruby".to_string(),
                };
                let hcl_content = hcl_tmpl.render();

                let dockerfile_content = DockerfileTemplate {
                    base_image_version: hcl_tmpl.base_image_version,
                    service_name: hcl_tmpl.service_name,
                    function_name: hcl_tmpl.function_name,
                    handler_name: hcl_tmpl.handler_name,
                    is_ruby: hcl_tmpl.is_ruby,
                }
                .render();

                let hcl = Artifact {
                    content_type: ContentType::HCL("HCL"),
                    content: hcl_content,
                    write_path: "net/plan.tf".into(),
                };
                let dockerfile = Artifact {
                    content_type: ContentType::Dockerfile("Dockerfile"),
                    content: dockerfile_content,
                    write_path: format!(
                        "net/services/{}/{}/Dockerfile",
                        service.clone(),
                        function.name.clone()
                    )
                    .into(),
                };
                Ok(vec![hcl, dockerfile])
            }
            None => Err(CastError(format!(
                "unable to find function {} in context",
                name.clone()
            ))),
        }
    }
}

#[derive(Serialize, Debug)]
struct KubernetesBaseTemplate {
    project_name: String,
    docker_config_path: String,
    registries: Vec<ContainerRegistry>,
}

impl Template for KubernetesBaseTemplate {
    fn render(&self) -> String {
        let mut reg = Box::new(Handlebars::new());
        reg.register_template_string("hcl_template", Self::tmpl())
            .unwrap();
        reg.render("hcl_template", &self).unwrap()
    }

    fn tmpl() -> &'static str {
        r#"# AssemblyLift K8S Provider Begin

// provider kubernetes {
//     alias       = "{{project_name}}-k8s"
//     config_path = pathexpand("~/.kube/config")
// }

{{#each registries}}{{#if this.is_ecr}}provider aws {
    alias  = "{{../project_name}}-k8s"
    region = "{{this.options.aws_region}}"
}

data aws_ecr_authorization_token token {
    provider = aws.{{../project_name}}-k8s
}{{/if}}{{/each}}

provider docker {
    alias = "{{project_name}}-k8s"
    {{#each registries}}{{#if this.is_dockerhub}}registry_auth {
        address     = "registry-1.docker.io"
        config_file = pathexpand("{{../docker_config_path}}")
    }{{/if}}
    {{#if this.is_ecr}}registry_auth {
        address  = data.aws_ecr_authorization_token.token.proxy_endpoint
        password = data.aws_ecr_authorization_token.token.password
        username = data.aws_ecr_authorization_token.token.user_name
    }{{/if}}
    {{/each}}
}

"#
    }
}

#[derive(Serialize)]
pub struct ServiceTemplate {
    pub project_name: String,
    pub service_name: String,
    pub kube_config_path: String,
    pub registries: Vec<ContainerRegistry>,
}

impl Template for ServiceTemplate {
    fn render(&self) -> String {
        let mut reg = Box::new(Handlebars::new());
        reg.register_template_string("hcl_template", Self::tmpl())
            .unwrap();
        reg.render("hcl_template", &self).unwrap()
    }

    fn tmpl() -> &'static str {
        r#"# Begin service `{{service_name}}`

resource kubernetes_namespace {{service_name}} {
    provider = kubernetes.{{project_name}}-k8s
    metadata {
        name = "asml-${local.project_name}-{{service_name}}"
    }
}

{{#each registries}}{{#if is_ecr}}resource kubernetes_secret dockerconfig_{{../service_name}}_ecr {
  provider = kubernetes.{{../project_name}}-k8s
  metadata {
      name      = "regcred-ecr"
      namespace = "asml-${local.project_name}-{{../service_name}}"
  }
  data = {
      ".dockerconfigjson" = jsonencode({
          auths = {
              (data.aws_ecr_authorization_token.token.proxy_endpoint) = {
                "username" = data.aws_ecr_authorization_token.token.user_name
                "password" = data.aws_ecr_authorization_token.token.password
                "email"    = "assemblylift@akkoro.io"
                "auth"     = data.aws_ecr_authorization_token.token.authorization_token
              }
          }
      })
  }
  type = "kubernetes.io/dockerconfigjson"
}{{/if}}
{{#if is_dockerhub}}resource kubernetes_secret dockerconfig_{{../service_name}}_dockerhub {
  provider = kubernetes.{{../project_name}}-k8s
  metadata {
      name      = "regcred-dockerhub"
      namespace = "asml-${local.project_name}-{{../service_name}}"
  }
  data = {
      ".dockerconfigjson" = jsonencode({
          auths = {
              "registry-1.docker.io" = {
                "username" = "{{this.options.username}}"
                "password" = "{{this.options.password}}"
                "email"    = "assemblylift@akkoro.io"
                "auth"     = base64encode("{{this.options.username}}:{{this.options.password}}")
              }
          }
      })
  }
  type = "kubernetes.io/dockerconfigjson"
}{{/if}}{{/each}}

"#
    }
}

#[derive(Serialize)]
pub struct FunctionTemplate {
    pub base_image_version: String,
    pub project_name: String,
    pub service_name: String,
    pub function_name: String,
    pub handler_name: String,
    pub has_iomods: bool,
    pub iomods: Vec<IomodContainer>,
    pub registry: ContainerRegistry,

    pub is_ruby: bool,
}

impl Template for FunctionTemplate {
    fn render(&self) -> String {
        let mut reg = Box::new(Handlebars::new());
        reg.register_template_string("hcl_template", Self::tmpl())
            .unwrap();
        reg.render("hcl_template", &self).unwrap()
    }

    fn tmpl() -> &'static str {
        r#"# Begin function `{{function_name}}` (in `{{service_name}}`)

locals {
    {{service_name}}_{{function_name}}_image_name = "asml-${local.project_name}-{{service_name}}-{{function_name}}"
}

{{#if registry.is_ecr}}resource aws_ecr_repository {{service_name}}_{{function_name}} {
    provider = aws.{{project_name}}-k8s
    name     = "asml/${local.project_name}/{{service_name}}/{{function_name}}"
}{{/if}}

{{#if is_ruby}}data archive_file {{service_name}}_{{function_name}}_rubysrc {
    type        = "zip"
    source_dir  = "${path.module}/services/{{service_name}}/{{function_name}}/rubysrc"
    output_path = "${path.module}/services/{{service_name}}/{{function_name}}/rubysrc.zip"
}{{/if}}

resource random_id {{service_name}}_{{function_name}}_image {
    byte_length = 8
    keepers = {
        dockerfile_hash = filebase64sha256("${path.module}/services/{{service_name}}/{{function_name}}/Dockerfile")
        wasm_hash       = filebase64sha256("${path.module}/services/{{service_name}}/{{function_name}}/{{handler_name}}")
        {{#if is_ruby}}rubysrc_hash    = data.archive_file.{{service_name}}_{{function_name}}_rubysrc.output_sha{{/if}}
    }
}

resource docker_registry_image {{service_name}}_{{function_name}} {
    provider = docker.{{project_name}}-k8s
    {{#if registry.is_dockerhub}}name = "{{registry.options.registry_name}}/${local.{{service_name}}_{{function_name}}_image_name}:${random_id.{{service_name}}_{{function_name}}_image.hex}"{{/if}}
    {{#if registry.is_ecr}}name = "${aws_ecr_repository.{{service_name}}_{{function_name}}.repository_url}:${random_id.{{service_name}}_{{function_name}}_image.hex}"{{/if}}

    build {
        context      = "${path.module}/services/{{service_name}}"
        dockerfile   = "{{function_name}}/Dockerfile"
        pull_parent  = true
        force_remove = true
    }
}

resource kubernetes_deployment {{function_name}} {
    provider   = kubernetes.{{project_name}}-k8s
    depends_on = [docker_registry_image.{{service_name}}_{{function_name}}, kubernetes_namespace.{{service_name}}]
    metadata {
        name      = "{{function_name}}"
        namespace = "asml-${local.project_name}-{{service_name}}"
        labels = {
            asml_function = "{{function_name}}"
            asml_service  = "{{service_name}}"
        }
    }

    spec {
        replicas = 1

        selector {
            match_labels = {
                asml_function = "{{function_name}}"
                asml_service  = "{{service_name}}"
            }
        }

        template {
            metadata {
                labels = {
                    asml_function = "{{function_name}}"
                    asml_service  = "{{service_name}}"
                }
            }

            spec {
                image_pull_secrets {
                    name = "regcred-{{#if registry.is_ecr}}ecr{{/if}}{{#if registry.is_dockerhub}}dockerhub{{/if}}"
                }
                container {
                    image = docker_registry_image.{{service_name}}_{{function_name}}.name
                    name  = "asml-{{service_name}}-{{function_name}}"
                    port {
                        container_port = 5543
                    }
                    port {
                        container_port = 13555
                    }
                }
                {{#each iomods}}
                container {
                    image = "{{this.image}}"
                    name  = "{{this.name}}"
                }
                {{/each}}
            }
        }
    }
}

resource kubernetes_service {{service_name}}_{{function_name}} {
    provider   = kubernetes.{{project_name}}-k8s
    depends_on = [kubernetes_namespace.{{service_name}}]

    metadata {
        name      = "asml-{{service_name}}-{{function_name}}"
        namespace = "asml-${local.project_name}-{{service_name}}"
    }

    spec {
        selector = {
            asml_function = "{{function_name}}"
            asml_service  = "{{service_name}}"
        }
        type = "ClusterIP"
        port {
            port        = 5543
            target_port = 5543
        }
    }
}

"#
    }
}

#[derive(Serialize)]
pub struct DockerfileTemplate {
    pub base_image_version: String,
    pub service_name: String,
    pub function_name: String,
    pub handler_name: String,
    pub is_ruby: bool,
}

impl Template for DockerfileTemplate {
    fn render(&self) -> String {
        let mut reg = Box::new(Handlebars::new());
        reg.register_template_string("dockerfile_template", Self::tmpl())
            .unwrap();
        reg.render("dockerfile_template", &self).unwrap()
    }

    fn tmpl() -> &'static str {
        r#"FROM public.ecr.aws/akkoro/assemblylift/hyper-alpine:{{base_image_version}}
ENV ASML_WASM_MODULE_NAME {{handler_name}}
{{#if is_ruby}}ENV ASML_FUNCTION_ENV ruby-docker{{/if}}
ADD ./{{function_name}}/{{handler_name}} /opt/assemblylift/{{handler_name}}
{{#if is_ruby}}COPY ./ruby-wasm32-wasi /usr/bin/ruby-wasm32-wasi
COPY ./{{function_name}}/rubysrc/* /usr/bin/ruby-wasm32-wasi/src/{{/if}}
"#
    }
}

#[derive(Serialize, Debug)]
pub struct ContainerRegistry {
    pub is_dockerhub: bool,
    pub is_ecr: bool,
    pub options: Options,
}

#[derive(Serialize, Clone, Debug)]
pub struct IomodContainer {
    pub image: String,
    pub name: String,
}
