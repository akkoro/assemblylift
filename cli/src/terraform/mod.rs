use std::fs;
use std::io;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path;
use std::path::{Path, PathBuf};

use clap::crate_version;
use handlebars::{to_json, Handlebars};
use serde_json::value::{Map, Value as Json};

use crate::artifact;
use crate::terraform::function::TerraformFunction;
use crate::terraform::service::TerraformService;

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

pub fn write(
    project_path: &PathBuf,
    project_name: String,
    functions: Vec<TerraformFunction>,
    services: Vec<TerraformService>,
) -> Result<(), io::Error> {
    let file_name = "main.tf";

    let mut reg = Handlebars::new();
    reg.register_template_string(file_name, TERRAFORM_ROOT)
        .unwrap();

    let mut data = Map::<String, Json>::new();
    data.insert("asml_version".to_string(), to_json(crate_version!()));
    data.insert("aws_region".to_string(), to_json("us-east-1"));
    data.insert("project_name".to_string(), to_json(project_name));
    data.insert("functions".to_string(), to_json(functions));
    data.insert("services".to_string(), to_json(services));

    let mut usermod_path = PathBuf::from(project_path);
    usermod_path.push("user_tf/");
    let usermod: bool = fs::metadata(usermod_path).is_ok();
    data.insert("user_inject".to_string(), to_json(usermod));

    let render = reg.render(file_name, &data).unwrap();

    let path_str = &format!(
        "{}/net/{}",
        project_path.clone().into_os_string().into_string().unwrap(),
        file_name
    );
    let path = path::Path::new(path_str);

    write_to_file(&path, render)
}

fn write_to_file(path: &path::Path, contents: String) -> Result<(), io::Error> {
    let mut file = match fs::File::create(path) {
        Err(why) => panic!(
            "couldn't create file {}: {}",
            path.display(),
            why.to_string()
        ),
        Ok(file) => file,
    };

    println!("📄 > Wrote {}", path.display());
    file.write_all(contents.as_bytes())
}
