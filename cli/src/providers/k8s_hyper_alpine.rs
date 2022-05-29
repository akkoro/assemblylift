use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use clap::crate_version;
use handlebars::{Handlebars, to_json};
use serde::Serialize;

use crate::providers::{Options, Provider, ProviderError};
use crate::tools::glooctl::GlooCtl;
use crate::tools::kubectl::KubeCtl;
use crate::transpiler::{BoxedCastable, Castable, CastError, ContentType, context, Template};
use crate::transpiler::context::Context;

pub struct KubernetesProvider {
    options: Arc<Options>,
}

impl KubernetesProvider {
    pub fn new() -> Self {
        Self {
            options: Arc::new(Options::new()),
        }
    }
}

impl Castable for KubernetesProvider {
    fn cast(&self, ctx: Rc<Context>, selector: Option<&str>) -> Result<Vec<String>, CastError> {
        let mut reg = Box::new(Handlebars::new());
        reg.register_template_string("service", SERVICE_TEMPLATE)
            .unwrap();

        let registry_type = self
            .options
            .get("registry_type")
            .unwrap_or(&"dockerhub".to_string())
            .clone();

        // TODO this should happen in a validate() step
        if registry_type == "ecr" {
            if self.options.get("aws_account_id").is_none() {
                return Err(CastError(format!(
                    "ecr registry type requires aws_account_id"
                )));
            }
            // TODO warn if defaulting aws_region
        }
        if registry_type == "dockerhub" {
            if self.options.get("registry_name").is_none() {
                return Err(CastError(format!(
                    "dockerhub registry type requires registry_name"
                )));
            }
        }

        let data = ServiceTemplate {
            service_name: selector
                .expect("selector must be a service name")
                .to_string(),
            container_registry: ContainerRegistryData {
                is_dockerhub: registry_type == "dockerhub",
                is_ecr: registry_type == "ecr",
                registry_name: self
                    .options
                    .get("registry_name")
                    .unwrap_or(&"".to_string())
                    .clone(),
                aws_account_id: self
                    .options
                    .get("aws_account_id")
                    .unwrap_or(&"".to_string())
                    .clone(),
                aws_region: self
                    .options
                    .get("aws_region")
                    .unwrap_or(&"us-east-1".to_string())
                    .clone(),
            },
            kube_config_path: self
                .options
                .get("kube_config_path")
                .unwrap_or(&"~/.kube/config".to_string())
                .clone(),
            docker_config_path: self
                .options
                .get("docker_config_path")
                .unwrap_or(&"~/.docker/config.json".to_string())
                .clone(),
        };
        let data = to_json(data);

        let rendered = reg.render("service", &data).unwrap();

        // TODO render functions from template

        Ok(vec![rendered])
    }

    fn content_type(&self) -> Vec<ContentType> {
        todo!()
    }
}

impl Provider for KubernetesProvider {
    fn name(&self) -> String {
        String::from("k8s-hyper-alpine")
    }

    fn options(&self) -> Arc<Options> {
        self.options.clone()
    }

    fn set_options(&mut self, opts: Arc<Options>) -> Result<(), ProviderError> {
        self.options = opts;
        Ok(())
    }
}

struct KubernetesFunction {
    options: Arc<Options>,
}

impl KubernetesFunction {
    pub fn new() -> Self {
        Self {
            options: Arc::new(Options::new()),
        }
    }
}

impl Castable for KubernetesFunction {
    fn cast(&self, ctx: Rc<Context>, selector: Option<&str>) -> Result<Vec<String>, CastError> {
        use std::io::Write;

        let mut reg = Box::new(Handlebars::new());
        reg.register_template_string("dockerfile", DOCKERFILE_TEMPLATE)
            .unwrap();
        reg.register_template_string("function", FUNCTION_TEMPLATE)
            .unwrap();

        let name = selector
            .expect("selector must be a function name")
            .to_string();
        match ctx.functions.iter().find(|&f| f.name == name) {
            Some(function) => {
                let service = function.service_name.clone();
                let registry_type = self
                    .options
                    .get("registry_type")
                    .unwrap_or(
                        ctx.service(service.clone())
                            .unwrap()
                            .option("registry_type")
                            .unwrap(),
                    )
                    .clone();

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

                let data = FunctionData {
                    base_image_version: crate_version!().to_string(),
                    function_name: function.name.clone(),
                    service_name: service.clone(),
                    handler_name: match function.language.as_str() {
                        "rust" => format!("{}.wasm.bin", function.name.clone()),
                        "ruby" => "ruby.wasmu".into(),
                        _ => "handler".into(),
                    },
                    iomods: iomods.clone(),
                    has_iomods: iomods.len() > 0,
                    container_registry: ContainerRegistryData {
                        is_dockerhub: registry_type == "dockerhub",
                        is_ecr: registry_type == "ecr",
                        registry_name: self
                            .options
                            .get("registry_name")
                            .unwrap_or(
                                ctx.service(service.clone())
                                    .unwrap()
                                    .option("registry_name")
                                    .unwrap_or(&"".to_string()),
                            )
                            .clone(),
                        aws_account_id: self
                            .options
                            .get("aws_account_id")
                            .unwrap_or(
                                ctx.service(service.clone())
                                    .unwrap()
                                    .option("aws_account_id")
                                    .unwrap(),
                            )
                            .clone(),
                        aws_region: self
                            .options
                            .get("aws_region")
                            .unwrap_or(&"us-east-1".to_string())
                            .clone(),
                    },
                    is_ruby: function.language == "ruby".to_string(),
                };
                let data = to_json(data);

                let rendered_hcl = reg.render("function", &data).unwrap();
                let rendered_dockerfile = reg.render("dockerfile", &data).unwrap();

                let mut file = std::fs::File::create(format!(
                    "./net/services/{}/{}/Dockerfile",
                    service.clone(),
                    function.name.clone()
                ))
                .expect("could not create runtime Dockerfile");
                file.write_all(rendered_dockerfile.as_bytes())
                    .expect("could not write runtime Dockerfile");

                Ok(vec![rendered_hcl])
            }
            None => Err(CastError(format!(
                "unable to find function {} in context",
                name.clone()
            ))),
        }
    }

    fn content_type(&self) -> Vec<ContentType> {
        todo!()
    }
}

#[derive(Serialize)]
pub struct ServiceTemplate {
    pub service_name: String,
    pub container_registry: ContainerRegistryData,
    pub kube_config_path: String,
    pub docker_config_path: String,
}

impl Template for ServiceTemplate {
    fn render(&self) -> String {
        todo!()
    }

    fn tmpl() -> &'static str {
        todo!()
    }
}

#[derive(Serialize)]
pub struct FunctionData {
    pub base_image_version: String,
    pub service_name: String,
    pub function_name: String,
    pub handler_name: String,
    pub has_iomods: bool,
    pub container_registry: ContainerRegistryData,
    pub iomods: Vec<IomodContainer>,

    pub is_ruby: bool,
}

#[derive(Serialize)]
pub struct ContainerRegistryData {
    pub is_dockerhub: bool,
    pub is_ecr: bool,
    pub registry_name: String,
    pub aws_account_id: String,
    pub aws_region: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct IomodContainer {
    pub image: String,
    pub name: String,
}

static DOCKERFILE_TEMPLATE: &str = r#"FROM public.ecr.aws/akkoro/assemblylift/hyper-alpine:{{base_image_version}}
ENV ASML_WASM_MODULE_NAME {{handler_name}}
ADD ./{{function_name}}/{{handler_name}} /opt/assemblylift/{{handler_name}}
{{#if is_ruby}}COPY ./{{function_name}}/ruby-wasm32-wasi /usr/bin/ruby-wasm32-wasi
COPY ./{{function_name}}/rubysrc/* /usr/bin/ruby-wasm32-wasi/src/
ENV ASML_FUNCTION_ENV ruby{{/if}}
"#;

static SERVICE_TEMPLATE: &str = r#"locals {
    {{#if container_registry.is_ecr}}ecr_address = "{{container_registry.aws_account_id}}.dkr.ecr.{{container_registry.aws_region}}.amazonaws.com"{{/if}}
}

{{#if container_registry.is_ecr}}provider aws {
    alias  = "{{service_name}}"
    region = "{{container_registry.aws_region}}"
}{{/if}}

{{#if container_registry.is_ecr}}data aws_ecr_authorization_token {{service_name}}_token {
    provider = aws.{{service_name}}
}{{/if}}

provider kubernetes {
    alias       = "{{service_name}}"
    config_path = pathexpand("{{kube_config_path}}")
}

provider docker {
    alias = "{{service_name}}"
    registry_auth {
        address     = "registry-1.docker.io"
        config_file = pathexpand("{{docker_config_path}}")
    }
    registry_auth {
        address  = local.ecr_address
        password = data.aws_ecr_authorization_token.{{service_name}}_token.password
        username = data.aws_ecr_authorization_token.{{service_name}}_token.user_name
    }
}

resource kubernetes_namespace {{service_name}} {
    provider = kubernetes.{{service_name}}
    metadata {
        name = "asml-${local.project_name}-{{service_name}}"
    }
}

"#;

static FUNCTION_TEMPLATE: &str = r#"locals {
    {{service_name}}_{{function_name}}_image_name = "asml-${local.project_name}-{{service_name}}-{{function_name}}"
}

{{#if container_registry.is_ecr}}resource aws_ecr_repository {{service_name}}_{{function_name}} {
    provider = aws.{{service_name}}
    name     = "asml/${local.project_name}/{{service_name}}/{{function_name}}"
}{{/if}}

data archive_file {{service_name}}_{{function_name}}_iomods {
    type        = "zip"
    source_dir  = "${path.module}/services/{{service_name}}/iomods"
    output_path = "${path.module}/services/{{service_name}}/iomods.zip"
}

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
        iomods_hash     = data.archive_file.{{service_name}}_{{function_name}}_iomods.output_sha
        {{#if is_ruby}}rubysrc_hash    = data.archive_file.{{service_name}}_{{function_name}}_rubysrc.output_sha{{/if}}
    }
}

resource docker_registry_image {{service_name}}_{{function_name}} {
    provider = docker.{{service_name}}
    {{#if container_registry.is_dockerhub}}name = "{{container_registry.registry_name}}/${local.{{service_name}}_{{function_name}}_image_name}:${random_id.{{service_name}}_{{function_name}}_image.hex}"{{/if}}
    {{#if container_registry.is_ecr}}name = "${aws_ecr_repository.{{service_name}}_{{function_name}}.repository_url}:${random_id.{{service_name}}_{{function_name}}_image.hex}"{{/if}}

    build {
        context      = "${path.module}/services/{{service_name}}"
        dockerfile   = "{{function_name}}/Dockerfile"
        pull_parent  = true
        force_remove = true
    }
}

resource kubernetes_deployment {{function_name}} {
    provider   = kubernetes.{{service_name}}
    depends_on = [docker_registry_image.{{service_name}}_{{function_name}}]
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
    provider = kubernetes.{{service_name}}

    metadata {
        name      = "asml-{{service_name}}-{{function_name}}"
        namespace = "asml-${local.project_name}-{{service_name}}"
    }

    spec {
        selector = {
            asml_function = "{{function_name}}"
            asml_service  = "{{service_name}}"
        }
        type = "NodePort"
        port {
            port        = 5543
            target_port = 5543
        }
    }
}
"#;
