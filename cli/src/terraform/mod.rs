use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use crate::artifact;

pub mod commands;
pub mod function;
pub mod service;

static TERRAFORM_ROOT: &str = r#"# Generated with assemblylift-cli {{asml_version}}

provider "aws" {
    region = "{{aws_region}}"
}

resource "aws_lambda_layer_version" "asml_runtime_layer" {
  filename   = "${path.module}/../.asml/runtime/bootstrap.zip"
  layer_name = "asml-{{project_name}}-runtime"

  source_code_hash = filebase64sha256("${path.module}/../.asml/runtime/bootstrap.zip")
}

{{#if user_inject}}
module "usermod" {
  source = "../user_tf"
}
{{/if}}

{{#each services}}
module "{{this.name}}" {
  source = "./services/{{this.name}}"
}
{{/each}}

{{#each functions}}
module "{{this.name}}" {
  source = "./services/{{this.service}}/{{this.name}}"

  runtime_layer_arn = aws_lambda_layer_version.asml_runtime_layer.arn
  {{#if this.service_has_layer}}
  service_layer_arn = module.{{this.service}}.service_layer_arn
  {{/if}}

  {{#if this.service_has_http_api}}
  service_http_api_id    = module.{{this.service}}.http_api_id
  http_api_execution_arn = module.{{this.service}}.http_api_execution_arn
  http_verb = "{{this.http_verb}}"
  http_path = "{{this.http_path}}"
  {{#if this.auth_has_id}}
  http_authorizer_id = module.{{this.service}}.{{this.auth_name}}_authorizer_id
  {{/if}}
  {{/if}}
}

{{/each}}

"#;

pub fn relative_binary_path() -> &'static str {
    ".asml/bin/terraform"
}

pub fn fetch(project_path: &PathBuf) {
    use std::io::Read;

    let terraform_path = format!(
        "{}/{}",
        project_path.clone().into_os_string().into_string().unwrap(),
        relative_binary_path()
    );

    if Path::new(&terraform_path).exists() {
        println!(
            "Found terraform at {}, skipping download...",
            terraform_path
        );
        return;
    }

    println!("Extracting terraform to {}", terraform_path);

    let mut terraform_zip = Vec::new();

    #[cfg(target_os = "linux")]
    let mut response = reqwest::blocking::get(
        "https://releases.hashicorp.com/terraform/0.12.26/terraform_0.12.26_linux_amd64.zip",
    )
    .unwrap();
    #[cfg(target_os = "macos")]
    let mut response = reqwest::blocking::get(
        "https://releases.hashicorp.com/terraform/0.12.26/terraform_0.12.26_darwin_amd64.zip",
    )
    .unwrap();

    response.read_to_end(&mut terraform_zip).unwrap();

    if let Err(_) = fs::create_dir_all(terraform_path.replace("/terraform", "")) {
        panic!("could not create directory ./.asml/bin")
    }

    artifact::unzip_to(terraform_zip, &terraform_path).unwrap();

    let mut perms = fs::metadata(&terraform_path).unwrap().permissions();
    perms.set_mode(0o755);
    if let Err(_) = fs::set_permissions(&terraform_path, perms) {
        panic!("could not set terraform binary executable (octal 755) permissions")
    }
}
