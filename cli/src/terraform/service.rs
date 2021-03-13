use std::path;
use std::path::PathBuf;
use std::{fs, io};

use serde::{Deserialize, Serialize};

use crate::templates;

static TERRAFORM_SERVICE: &str = r#"# Generated with assemblylift-cli {{asml_version}}
{{#if has_layer}}
resource "aws_lambda_layer_version" "asml_{{name}}_service_layer" {
  filename   = "${path.module}/../../../.asml/runtime/{{name}}.zip"
  layer_name = "asml-{{project_name}}-{{name}}-service"

  source_code_hash = filebase64sha256("${path.module}/../../../.asml/runtime/{{name}}.zip")
}

output "service_layer_arn" {
  value = aws_lambda_layer_version.asml_{{name}}_service_layer.arn
}
{{/if}}

{{#if has_http_api}}
resource "aws_apigatewayv2_api" "{{name}}_http_api" {
  name          = "asml-{{project_name}}-{{name}}"
  protocol_type = "HTTP"
}

resource "aws_apigatewayv2_stage" "{{name}}_default_stage" {
  api_id      = aws_apigatewayv2_api.{{name}}_http_api.id
  name        = "$default"
  auto_deploy = true
}

{{#each jwt_authorizers}}
resource "aws_apigatewayv2_authorizer" "{{this.name}}" {
  api_id           = aws_apigatewayv2_api.{{../name}}_http_api.id
  authorizer_type  = "JWT"
  identity_sources = ["$request.header.Authorization"]
  name             = "{{this.name}}"

  jwt_configuration {
    audience = [{{#each this.audience}}"{{this}}"{{#if this.has_next}},{{/if}}{{/each}}]
    issuer   = "{{this.issuer}}"
  }
}
output "{{this.name}}_authorizer_id" {
    value = aws_apigatewayv2_authorizer.{{this.name}}.id
}
{{/each}}

output "http_api_id" {
  value = aws_apigatewayv2_api.{{name}}_http_api.id
}

output "http_api_execution_arn" {
  value = aws_apigatewayv2_api.{{name}}_http_api.execution_arn
}
{{/if}}
"#;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TerraformService {
    pub name: String,
    pub has_layer: bool,
    pub has_http_api: bool,
    pub jwt_authorizers: Vec<JwtAuth>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct JwtAuth {
    pub name: String,
    pub audience: Vec<String>,
    pub issuer: String,
    pub has_next: bool,
}

//impl From<templates::service::Manifest> for TerraformService {
//    fn from(manifest: templates::service::Manifest) -> Self {
//        let service_name = manifest.service.name.clone();
//        let manifest_authorizers = manifest.api.authorizers.clone();
//        let authorizers = manifest_authorizers.as_ref();
//
//        Self {
//            name: service_name.clone(),
//            has_layer: manifest.iomod.is_some(),
//            has_http_api: manifest
//                .api
//                .functions
//                .values()
//                .any(|f| f.http.is_some()),
//            jwt_authorizers: match authorizers.as_ref() {
//                Some(auths) => auths.into_iter()
//                    .filter(|(_,v)| v.auth_type.eq("JWT"))
//                    .enumerate()
//                    .map(|(idx, (k,v))| {
//                        JwtAuth { 
//                            name: String::from(k), 
//                            audience: v.audience
//                                .clone()
//                                .as_ref()
//                                .as_ref()
//                                .unwrap()
//                                .to_vec(), 
//                            issuer: v.issuer
//                                .clone()
//                                .as_ref()
//                                .as_ref()
//                                .unwrap()
//                                .to_string(),
//                            has_next: (idx + 1) < auths.keys().len()
//                        }
//                    })
//                    .collect(),
//                None => Vec::new(),
//            },
//        }
//    }
//}
//
//pub fn write(project_path: &PathBuf, project_name: String, service_name: String, service: String) -> Result<(), io::Error> {
//    let file_name = "service.tf";
//
//    let path = &format!(
//        "{}/net/services/{}",
//        project_path.clone().into_os_string().into_string().unwrap(),
//        &service_name
//    );
//
//    fs::create_dir_all(path).expect(&*format!("unable to create path {:?}", path));
//
//    let file_path = &format!("{}/{}", path, file_name);
//    let file_path = path::Path::new(file_path);
//
//    write_to_file(&file_path, service)
//}
