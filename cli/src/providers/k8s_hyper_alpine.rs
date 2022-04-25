use std::rc::Rc;
use std::sync::Arc;

use clap::crate_version;
use handlebars::{to_json, Handlebars};
use serde::Serialize;

use crate::providers::{render_string_list, Options, Provider, ProviderArtifact, ProviderError};
use crate::transpiler::{asml, Artifact, Map};

pub struct ServiceProvider {
    options: Arc<Options>,
}

impl ServiceProvider {
    pub fn new() -> Self {
        Self {
            options: Arc::new(Options::new()),
        }
    }
}

impl Provider for ServiceProvider {
    fn name(&self) -> String {
        String::from("k8s-hyper-alpine")
    }

    fn init(&self, _ctx: Rc<asml::Context>, _name: String) -> Result<(), ProviderError> {
        Ok(())
    }

    fn transform(
        &self,
        ctx: Rc<asml::Context>,
        name: String,
    ) -> Result<Box<dyn Artifact>, ProviderError> {
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
                return Err(ProviderError::TransformationError(format!(
                    "ecr registry type requires aws_account_id"
                )));
            }
            // TODO warn if defaulting aws_region
        }
        if registry_type == "dockerhub" {
            if self.options.get("registry_name").is_none() {
                return Err(ProviderError::TransformationError(format!(
                    "dockerhub registry type requires registry_name"
                )));
            }
        }

        let data = ServiceData {
            service_name: name.clone(),
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
        };
        let data = to_json(data);

        let rendered = reg.render("service", &data).unwrap();

        Ok(Box::new(ProviderArtifact::new(rendered)))
    }

    fn options(&self) -> Arc<Options> {
        self.options.clone()
    }

    fn set_options(&mut self, opts: Arc<Options>) -> Result<(), ProviderError> {
        self.options = opts;
        Ok(())
    }
}

pub struct FunctionProvider {
    options: Arc<Options>,
}

impl FunctionProvider {
    pub fn new() -> Self {
        Self {
            options: Arc::new(Options::new()),
        }
    }
}

impl Provider for FunctionProvider {
    fn name(&self) -> String {
        String::from("k8s-hyper-alpine")
    }

    fn init(&self, _ctx: Rc<asml::Context>, _name: String) -> Result<(), ProviderError> {
        Ok(())
    }

    fn transform(
        &self,
        ctx: Rc<asml::Context>,
        name: String,
    ) -> Result<Box<dyn Artifact>, ProviderError> {
        use std::io::Write;

        let mut reg = Box::new(Handlebars::new());
        reg.register_template_string("dockerfile", DOCKERFILE_TEMPLATE)
            .unwrap();
        reg.register_template_string("function", FUNCTION_TEMPLATE)
            .unwrap();

        match ctx.functions.iter().find(|&f| *f.name == name.clone()) {
            Some(function) => {
                let service = function.service_name.clone();
                let registry_type = self
                    .options
                    .get("registry_type")
                    .unwrap_or(
                        ctx.service(service.clone())
                            .unwrap()
                            .option("registry_type")
                            .unwrap()
                    )
                    .clone();

                let data = FunctionData {
                    base_image_version: "0.4.0-alpha.0".to_string(), // TODO crate_version!()
                    function_name: function.name.clone(),
                    handler_name: function.handler_name.clone(),
                    service_name: service.clone(),
                    has_iomods: ctx
                        .iomods
                        .iter()
                        .filter(|i| i.service_name == service.clone())
                        .count()
                        > 0,
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
                                    .unwrap_or(&"".to_string())
                            )
                            .clone(),
                        aws_account_id: self
                            .options
                            .get("aws_account_id")
                            .unwrap_or(
                                ctx.service(service.clone())
                                    .unwrap()
                                    .option("aws_account_id")
                                    .unwrap()
                            )
                            .clone(),
                        aws_region: self
                            .options
                            .get("aws_region")
                            .unwrap_or(&"us-east-1".to_string())
                            .clone(),
                    },
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

                Ok(Box::new(ProviderArtifact::new(rendered_hcl)))
            }
            None => Err(ProviderError::TransformationError(format!(
                "unable to find function {} in context",
                name.clone()
            ))),
        }
    }

    fn options(&self) -> Arc<Options> {
        self.options.clone()
    }

    fn set_options(&mut self, opts: Arc<Options>) -> Result<(), ProviderError> {
        self.options = opts;
        Ok(())
    }
}

#[derive(Serialize)]
pub struct ServiceData {
    pub service_name: String,
    pub container_registry: ContainerRegistryData,
}

#[derive(Serialize)]
pub struct FunctionData {
    pub base_image_version: String,
    pub service_name: String,
    pub function_name: String,
    pub handler_name: String,
    pub has_iomods: bool,
    pub container_registry: ContainerRegistryData,
}

#[derive(Serialize)]
pub struct ContainerRegistryData {
    pub is_dockerhub: bool,
    pub is_ecr: bool,
    pub registry_name: String,
    pub aws_account_id: String,
    pub aws_region: String,
}

static DOCKERFILE_TEMPLATE: &str = r#"FROM public.ecr.aws/akkoro/assemblylift/hyper-alpine:{{base_image_version}}
{{#if has_iomods}}ADD ./iomods/{{service_name}} /opt/assemblylift/iomod/{{/if}}
ADD ./{{function_name}}/{{function_name}}.wasm.bin /opt/assemblylift/handler.wasm.bin
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
    config_path = pathexpand("~/.kube/config")
}

provider docker {
    alias = "{{service_name}}"
    registry_auth {
        address     = "registry-1.docker.io"
        config_file = pathexpand("~/.docker/config.json")
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

resource random_id {{service_name}}_{{function_name}}_image {
    byte_length = 8
    keepers = {
        dockerfile_hash = filebase64sha256("${path.module}/services/{{service_name}}/{{function_name}}/Dockerfile")
        wasm_hash       = filebase64sha256("${path.module}/services/{{service_name}}/{{function_name}}/{{function_name}}.wasm.bin")
        iomods_hash     = data.archive_file.{{service_name}}_{{function_name}}_iomods.output_sha
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
                container {
                    image = "xlem/assemblylift-iomod-std-http:latest"
                    name  = "assemblylift-iomod-std-http"
                }
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
