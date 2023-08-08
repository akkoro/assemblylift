use handlebars::Handlebars;
use serde::Serialize;

use crate::providers::Options;
use crate::transpiler::Template;

#[derive(Serialize, Debug)]
pub struct KubernetesBaseTemplate {
    pub project_name: String,
    pub docker_config_path: String,
    pub registries: Vec<ContainerRegistry>,
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
    pub environment: Vec<ContainerEnv>,
    pub is_ruby: bool,
    pub cpu_compat_mode: String,
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
    provider      = docker.{{project_name}}-k8s
    name          = docker_image.{{service_name}}_{{function_name}}.name
    keep_remotely = true
}

resource docker_image {{service_name}}_{{function_name}} {
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
                    {{#each environment}}
                    env {
                        name  = "{{this.name}}"
                        value = "{{this.value}}"
                    }
                    {{/each}}
                    env {
                        name  = "ASML_CPU_COMPAT_MODE"
                        value = "{{this.cpu_compat_mode}}"
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
    pub project_name: String,
    pub service_name: String,
    pub function_name: String,
    pub handler_name: String,
    pub function_coordinates: String,
    pub function_precompiled: String,
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
ENV ASML_FUNCTION_COORDINATES {{function_coordinates}}
ENV ASML_FUNCTION_PRECOMPILED {{function_precompiled}}
{{#if is_ruby}}ENV ASML_FUNCTION_ENV ruby-docker{{/if}}
ADD ./{{function_name}}/{{handler_name}} /opt/assemblylift/projects/{{project_name}}/services/{{service_name}}/{{handler_name}}
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

#[derive(Serialize, Clone, Debug)]
pub struct ContainerEnv {
    pub name: String,
    pub value: String,
}
