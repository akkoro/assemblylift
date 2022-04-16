use std::rc::Rc;
use std::sync::Arc;

use clap::crate_version;
use handlebars::{to_json, Handlebars};
use serde::Serialize;

use crate::transpiler::{asml, Artifact};
use crate::providers::{render_string_list, Options, Provider, ProviderArtifact, ProviderError};

pub struct ServiceProvider {
    options: Arc<Options>,
}

impl ServiceProvider {
    pub fn new() -> Self {
        Self { options: Arc::new(Options::new()) }
    }
}

impl Provider for ServiceProvider {
    fn name(&self) -> String {
        String::from("k8s-generic-alpine")
    }

    fn init(&self, _ctx: Rc<asml::Context>, _name: String) -> Result<(), ProviderError> {
        Ok(())
    }

    fn transform(&self, ctx: Rc<asml::Context>, name: String) -> Result<Box<dyn Artifact>, ProviderError> {
        let mut reg = Box::new(Handlebars::new());
        reg.register_template_string("service", SERVICE_TEMPLATE)
            .unwrap();


        let data = ServiceData {
            project_name: ctx.project.name.clone(),
            service_name: name.clone(),
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
        Self { options: Arc::new(Options::new()) }
    }
}

impl Provider for FunctionProvider {
    fn name(&self) -> String {
        String::from("aws-lambda")
    }

    fn init(&self, _ctx: Rc<asml::Context>, _name: String) -> Result<(), ProviderError> {
        Ok(())
    }

    fn transform(&self, ctx: Rc<asml::Context>, name: String) -> Result<Box<dyn Artifact>, ProviderError> {
        use std::io::Write;

        let mut reg = Box::new(Handlebars::new());
        reg.register_template_string("dockerfile", DOCKERFILE_TEMPLATE)
            .unwrap();
        reg.register_template_string("function", FUNCTION_TEMPLATE)
            .unwrap();

        match ctx.functions.iter().find(|&f| *f.name == name.clone()) {
            Some(function) => {
                let service = function.service_name.clone();

                let data = FunctionData {
                    base_image_version: "0.4.0-alpha0".to_string(), // TODO crate_version!()
                    function_name: function.name.clone(),
                    handler_name: function.handler_name.clone(),
                    service_name: service.clone(),
                    project_name: ctx.project.name.clone(),
                };
                let data = to_json(data);

                let rendered_hcl = reg.render("function", &data).unwrap();
                let rendered_dockerfile = reg.render("dockerfile", &data).unwrap();

                let mut file = std::fs::File::create(format!("./net/services/{}/{}/Dockerfile", service.clone(), function.name.clone()))
                    .expect("could not create runtime Dockerfile");
                file.write_all(rendered_dockerfile.as_bytes()).expect("could not write runtime Dockerfile");

                Ok(Box::new(ProviderArtifact::new(rendered_hcl)))
            }
            None => Err(ProviderError::TransformationError(format!("unable to find function {} in context", name.clone()))),
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
    pub project_name: String,
    pub service_name: String,
}

#[derive(Serialize)]
pub struct FunctionData {
    pub base_image_version: String,
    pub project_name: String,
    pub service_name: String,
    pub function_name: String,
    pub handler_name: String,
}

static DOCKERFILE_TEMPLATE: &str =
r#"FROM public.ecr.aws/akkoro/assemblylift/asml-generic-alpine:{{base_image_version}}
ADD ./iomods/{{service_name}} /opt/assemblylift/iomod/
ADD ./{{function_name}}/handler.wasm.bin /opt/assemblylift/handler.wasm.bin
"#;

static SERVICE_TEMPLATE: &str =
r#"locals {
    // ecr = "{{aws_account_id}}.dkr.ecr.{{aws_region}}.amazonaws.com"
    image_name = "asml-{{project_name}}-{{service_name}}-{{function_name}}"
}

provider kubernetes {
    config_path = "~/.kube/config"
}

provider docker {
    alias   = "{{service_name}}"
    registry_auth {
        address  = "docker.io"
        config_file = pathexpand("~/.docker/config.json")
    }
}

resource kubernetes_namespace {{service_name}} {
    metadata {
        name = "asml-${local.project_name}-{{service_name}}"
    }
}

"#;

static FUNCTION_TEMPLATE: &str =
r#"data "archive_file" "{{service_name}}_{{function_name}}_iomods" {
    type        = "zip"
    source_dir  = "${path.module}/services/{{service_name}}/iomods"
    output_path = "${path.module}/services/{{service_name}}/iomods.zip"
}

resource "random_id" "{{service_name}}_{{function_name}}_image" {
    byte_length = 8
    keepers = {
        dockerfile_hash = filebase64sha256("${path.module}/services/{{service_name}}/{{function_name}}/Dockerfile")
        wasm_hash       = filebase64sha256("${path.module}/services/{{service_name}}/{{function_name}}/{{function_name}}.wasm.bin")
        iomods_hash     = data.archive_file.{{service_name}}_{{function_name}}_iomods.output_sha
    }
}

resource "docker_registry_image" "{{service_name}}_{{function_name}}" {
    provider = docker.{{service_name}}
    name = "${local.image_name}:${random_id.{{service_name}}_{{function_name}}_image.hex}"

    build {
        context      = "${path.module}/services/{{service_name}}"
        dockerfile   = "{{function_name}}/Dockerfile"
        pull_parent  = true
        force_remove = true
    }
}

resource kubernetes_deployment {{function_name}} {
    metadata {
        name = "{{function_name}}"
        labels = {
            asml_function = "{{function_name}}"
            asml_service = "{{service_name}}"
        }
    }

    spec {
        replicas = 1

        selector {
            match_labels = {
                asml_function = "{{function_name}}"
                asml_service = "{{service_name}}"
            }
        }

        template {
            metadata {
                labels = {
                    asml_function = "{{function_name}}"
                    asml_service = "{{service_name}}"
                }
            }

            spec {
                container {
                    image = "${local.image_name}:${random_id.{{service_name}}_{{function_name}}_image.hex}"
                    name  = "asml-{{service_name}}-{{function_name}}"
                    port {
                        container_port = 5543
                    }
                }
            }
        }
    }
}
"#;
