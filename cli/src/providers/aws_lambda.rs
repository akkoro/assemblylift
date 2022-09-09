use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use handlebars::{Handlebars, to_json};
use itertools::Itertools;
use once_cell::sync::Lazy;
use registry_common::models::GetIomodAtResponse;
use serde::Serialize;

use crate::archive;
use crate::providers::{AWS_LAMBDA_PROVIDER_NAME, DNS_PROVIDERS, flatten, LockBox, Options, Provider, ProviderError, ProviderMap, render_string_list};
use crate::transpiler::{
    Artifact, Bindable, Bootable, Castable, CastError, ContentType, context, Template,
};
use crate::transpiler::context::{Context, Function};

pub struct AwsLambdaProvider {
    options: Arc<Options>,
}

impl AwsLambdaProvider {
    pub fn new() -> Self {
        let runtime_url = &*format!(
            "http://public.assemblylift.akkoro.io/runtime/{}/aws-lambda/bootstrap.zip",
            clap::crate_version!(),
        );
        let mut response =
            reqwest::blocking::get(runtime_url).expect("could not download bootstrap.zip");
        if !response.status().is_success() {
            panic!("unable to fetch asml runtime from {}", runtime_url);
        }
        let mut response_buffer = Vec::new();
        response.read_to_end(&mut response_buffer).unwrap();

        fs::create_dir_all("./.asml/runtime").unwrap();
        fs::write("./.asml/runtime/bootstrap.zip", response_buffer).unwrap();

        Self {
            options: Arc::new(Options::new()),
        }
    }

    pub fn cast_iomods(ctx: Rc<Context>, service_name: &str) -> Result<(), CastError> {
        let project_path = ctx.project.path.clone();
        let iomod_path = format!("{}/net/services/{}/iomods", project_path, service_name);
        if Path::new(&iomod_path.clone()).exists() {
            fs::remove_dir_all(iomod_path.clone()).expect("could not rm iomod directory");
        }
        fs::create_dir_all(iomod_path.clone()).expect("could not create iomod directory");

        let mut dependencies: Vec<PathBuf> = Vec::new();
        // println!("DEBUG service_name={:?}", service_name);
        // println!("DEBUG iomods={:?}", ctx.iomods);
        let service_iomods: Vec<&context::Iomod> = ctx
            .iomods
            .iter()
            .filter(|m| m.service_name == service_name.to_string())
            .collect();
        // println!("DEBUG iomods={:?}", service_iomods);
        for iomod in service_iomods {
            // let dependency_coords: Vec<&str> = iomod.coordinates.split('.').collect();
            // let dependency_name = dependency_coords.get(2).unwrap().to_string();

            // TODO try from file
            // if _ {
            //     let dependency_path = format!("{}/net/services/{}/iomods/{}", project_path, service_name, dependency_name);
            //     let dependency_from = dependency.from.as_ref()
            //         .expect("`from` must be defined when dependency type is `file`");
            //     match fs::metadata(dependency_from.clone()) {
            //         Ok(_) => {
            //             fs::copy(dependency_from.clone(), &dependency_path).unwrap();
            //             ()
            //         }
            //         Err(_) => panic!("ERROR: could not find file-type dependency named {} (check path)", dependency_name),
            //     }
            //
            //     dependencies.push(dependency_path);
            // } else {
            let dependency_path = format!(
                "{}/{}@{}.iomod",
                iomod_path, iomod.coordinates, iomod.version,
            );
            let client = reqwest::blocking::ClientBuilder::new()
                .build()
                .expect("could not build blocking HTTP client");
            let registry_url = format!(
                "https://registry.assemblylift.akkoro.io/iomod/{}/{}",
                iomod.coordinates, iomod.version
            );
            let res: GetIomodAtResponse = client.get(registry_url).send().unwrap().json().unwrap();
            let bytes = client.get(res.url).send().unwrap().bytes().unwrap();
            fs::write(&dependency_path, &*bytes).expect("could not write iomod package");
            dependencies.push(PathBuf::from(dependency_path));
        }

        archive::zip_dirs(
            dependencies,
            format!("./.asml/runtime/{}-iomods.zip", &service_name),
            Vec::new(),
        )
        .map_err(|_| CastError("unable to zip IOmods".into()))
    }

    pub fn cast_ruby(ctx: Rc<Context>, service_name: &str) -> Result<(), CastError> {
        let project_path = ctx.project.path.clone();
        let ruby_dir = format!(
            "{}/net/services/{}/ruby-wasm32-wasi",
            project_path, service_name
        );
        archive::zip_dirs(
            vec![ruby_dir.into()],
            format!("./.asml/runtime/{}-ruby.zip", &service_name),
            vec!["ruby.wasmu", "ruby.wasm", "ruby"],
        )
        .map_err(|_| CastError("could not zip ruby env directory".into()))
    }

    pub fn is_function_large(ctx: Rc<Context>, f: &Function) -> bool {
        let project_path = ctx.project.path.clone();
        let artifact_path = format!(
            "{}/net/services/{}/{}/{}.zip",
            &project_path,
            f.service_name.clone(),
            f.name.clone(),
            f.name.clone()
        );
        File::open(artifact_path)
            .unwrap()
            .metadata()
            .unwrap()
            .size()
            > (50 * 1000 * 1000)
    }
}

impl Castable for AwsLambdaProvider {
    fn cast(&self, ctx: Rc<Context>, _selector: Option<&str>) -> Result<Vec<Artifact>, CastError> {
        let service_subprovider = LambdaService {
            options: self.options.clone(),
        };

        let mut service_artifacts = ctx
            .services
            .iter()
            .filter(|&s| s.provider.name == self.name())
            .map(|s| {
                service_subprovider
                    .cast(ctx.clone(), Some(&s.name))
                    .unwrap()
            })
            .reduce(flatten)
            .unwrap();

        let base_tmpl = LambdaBaseTemplate {
            project_name: ctx.project.name.clone(),
            options: self.options.clone(),
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

impl Bindable for AwsLambdaProvider {
    fn bind(&self, _ctx: Rc<Context>) -> Result<(), CastError> {
        Ok(())
    }
}

impl Bootable for AwsLambdaProvider {
    fn boot(&self, _ctx: Rc<Context>) -> Result<(), CastError> {
        Ok(())
    }

    fn is_booted(&self, _ctx: Rc<Context>) -> bool {
        true
    }
}

impl Provider for AwsLambdaProvider {
    fn name(&self) -> String {
        String::from(AWS_LAMBDA_PROVIDER_NAME)
    }

    fn options(&self) -> Arc<Options> {
        self.options.clone()
    }

    fn set_options(&mut self, opts: Arc<Options>) -> Result<(), ProviderError> {
        self.options = opts;
        Ok(())
    }
}

struct LambdaService {
    options: Arc<Options>,
}

impl Castable for LambdaService {
    fn cast(&self, ctx: Rc<Context>, selector: Option<&str>) -> Result<Vec<Artifact>, CastError> {
        let name = selector
            .expect("selector must be a service name")
            .to_string();
        let project_name = ctx.project.name.clone();
        let layer_name = format!(
            "asml-{}-{}-lambda-runtime",
            ctx.project.name.clone(),
            name.clone(),
        );
        let service = ctx.service(&name).unwrap();

        AwsLambdaProvider::cast_iomods(ctx.clone(), &name).unwrap();
        let mut has_ruby_layer = false;
        if ctx
            .functions
            .iter()
            .filter(|&f| f.service_name == name.clone())
            .find(|f| f.language == "ruby")
            .is_some()
        {
            AwsLambdaProvider::cast_ruby(ctx.clone(), &name)?;
            has_ruby_layer = true;
        }
        let has_iomods_layer = ctx
            .iomods
            .iter()
            .filter(|&m| m.service_name == name.clone())
            .collect_vec()
            .len()
            > 0;
        let has_large_payloads = ctx
            .functions
            .iter()
            .filter(|&f| f.service_name == name.clone())
            .find(|f| AwsLambdaProvider::is_function_large(ctx.clone(), f))
            .is_some();
        let use_apigw = ctx.functions.iter().find(|f| f.http.is_some()).is_some();
        let has_domain_name = service.domain_name.is_some();

        let authorizers: Vec<ServiceAuthData> = ctx
            .authorizers
            .iter()
            .filter(|&a| a.service_name == name.clone())
            .filter(|&a| a.r#type.to_lowercase() != "aws_iam")
            .map(|a| ServiceAuthData {
                id: a.id.clone(),
                r#type: a.r#type.clone(),
                jwt_config: match &a.jwt_config {
                    Some(jwt) => {
                        let audience = render_string_list(jwt.audience.clone());

                        Some(ServiceAuthDataJwtConfig {
                            audience,
                            issuer: jwt.issuer.clone(),
                        })
                    }
                    None => None,
                },
            })
            .collect();

        let hcl_content = ServiceTemplate {
            project_name: ctx.project.name.clone(),
            service_name: name.clone(),
            domain_name: String::from(
                service
                    .domain_name
                    .as_ref()
                    .unwrap_or(&format!("{}.com", &project_name)),
            ),
            layer_name,
            use_apigw,
            has_iomods_layer,
            has_ruby_layer,
            has_large_payloads,
            has_domain_name,
            authorizers,
            options: self.options.clone(),
        }
        .render();

        let function_subprovider = LambdaFunction {
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
        Ok(vec![hcl])
    }
}

struct LambdaFunction {
    service_name: String,
    options: Arc<Options>,
}

impl Castable for LambdaFunction {
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

                // find dependencies for service
                let iomod_names: Vec<String> = ctx
                    .iomods
                    .iter()
                    .filter(|&m| *m.service_name == service.clone())
                    .map(|m| m.name.clone())
                    .collect();

                // find authorizers for service
                let auth = match &function.authorizer_id {
                    Some(id) => {
                        let authorizer = ctx
                            .authorizers
                            .iter()
                            .filter(|a| a.service_name == service.clone())
                            .find(|a| a.id == id.clone())
                            .expect(&format!(
                                "could not find authorizer by id \"{}\" in context",
                                id.clone()
                            ));
                        let auth_type = authorizer.r#type.clone();
                        Some(FunctionAuthData {
                            id: match auth_type.to_lowercase().as_str() {
                                "aws_iam" => None,
                                _ => Some(format!(
                                    "aws_apigatewayv2_authorizer.{}_{}.id",
                                    service.clone(),
                                    id
                                )),
                            },
                            r#type: auth_type.clone(),
                            scopes: match auth_type.to_lowercase().as_str() {
                                "aws_iam" => None,
                                _ => Some(render_string_list(authorizer.scopes.clone())),
                            },
                        })
                    }
                    None => None,
                };

                let tmpl = FunctionTemplate {
                    project_name: ctx.project.name.clone(),
                    service_name: service.clone(),
                    function_name: function.name.clone(),
                    handler_name: match function.language.as_str() {
                        "rust" => format!("{}.wasmu", function.name.clone()),
                        "ruby" => "ruby.wasmu".into(),
                        _ => "handler".into(),
                    },
                    runtime_layer: format!(
                        "aws_lambda_layer_version.asml_{}_runtime.arn",
                        service.clone()
                    ),
                    iomods_layer: match iomod_names.len() {
                        0 => None,
                        _ => Some(format!(
                            "aws_lambda_layer_version.asml_{}_iomods.arn",
                            service.clone()
                        )),
                    },
                    ruby_layer: match &*function.language {
                        "ruby" => Some(format!(
                            "aws_lambda_layer_version.asml_{}_ruby.arn",
                            service.clone()
                        )),
                        _ => None,
                    },
                    large_payload: AwsLambdaProvider::is_function_large(ctx.clone(), function),
                    size: function.size,
                    timeout: function.timeout,
                    http: match &function.http {
                        Some(http) => Some(HttpData {
                            verb: http.verb.clone(),
                            path: http.path.clone(),
                        }),
                        None => None,
                    },
                    auth,
                };

                let hcl = Artifact {
                    content_type: ContentType::HCL("HCL"),
                    content: tmpl.render(),
                    write_path: "net/plan.tf".into(),
                };
                Ok(vec![hcl])
            }
            None => Err(CastError(format!(
                "unable to find function {} in context",
                name.clone()
            ))),
        }
    }
}

#[derive(Serialize)]
struct LambdaBaseTemplate {
    project_name: String,
    options: Arc<Options>,
}

impl Template for LambdaBaseTemplate {
    fn render(&self) -> String {
        let mut reg = Box::new(Handlebars::new());
        reg.register_template_string("hcl_template", Self::tmpl())
            .unwrap();
        reg.render("hcl_template", &self).unwrap()
    }

    fn tmpl() -> &'static str {
        r#"# AssemblyLift AWS Lambda Provider Begin

provider aws {
    alias  = "{{project_name}}-aws-lambda"
    region = "{{options.aws_region}}"
}

"#
    }
}

#[derive(Serialize)]
struct ServiceTemplate {
    project_name: String,
    service_name: String,
    layer_name: String,
    domain_name: String,
    has_iomods_layer: bool,
    has_ruby_layer: bool,
    has_large_payloads: bool,
    use_apigw: bool,
    has_domain_name: bool,
    authorizers: Vec<ServiceAuthData>,
    options: Arc<Options>,
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

resource aws_lambda_layer_version asml_{{service_name}}_runtime {
    provider = aws.{{project_name}}-aws-lambda

    filename   = "${local.project_path}/.asml/runtime/bootstrap.zip"
    layer_name = "{{layer_name}}"

    source_code_hash = filebase64sha256("${local.project_path}/.asml/runtime/bootstrap.zip")
}

{{#if has_iomods_layer}}resource aws_lambda_layer_version asml_{{service_name}}_iomods {
    provider = aws.{{project_name}}-aws-lambda

    filename   = "${local.project_path}/.asml/runtime/{{service_name}}-iomods.zip"
    layer_name = "asml-${local.project_name}-{{service_name}}-iomods"

    source_code_hash = filebase64sha256("${local.project_path}/.asml/runtime/{{service_name}}-iomods.zip")
}{{/if}}

{{#if has_ruby_layer}}resource aws_lambda_layer_version asml_{{service_name}}_ruby {
    provider = aws.{{project_name}}-aws-lambda

    filename   = "${local.project_path}/.asml/runtime/{{service_name}}-ruby.zip"
    layer_name = "asml-${local.project_name}-{{service_name}}-ruby"

    source_code_hash = filebase64sha256("${local.project_path}/.asml/runtime/{{service_name}}-ruby.zip")
}{{/if}}

{{#if use_apigw}}resource aws_apigatewayv2_api {{service_name}}_http_api {
    provider      = aws.{{project_name}}-aws-lambda
    name          = "asml-${local.project_name}-{{service_name}}"
    protocol_type = "HTTP"
}

resource aws_apigatewayv2_stage {{service_name}}_default_stage {
    provider    = aws.{{project_name}}-aws-lambda
    api_id      = aws_apigatewayv2_api.{{service_name}}_http_api.id
    name        = "$default"
    auto_deploy = true
}
{{/if}}

{{#each authorizers}}resource aws_apigatewayv2_authorizer {{../service_name}}_{{this.id}} {
    provider    = aws.{{../project_name}}-aws-lambda

    api_id           = aws_apigatewayv2_api.{{../service_name}}_http_api.id
    authorizer_type  = "{{this.type}}"
    identity_sources = ["$request.header.Authorization"]
    name             = "{{../service_name}}-{{this.id}}"

    {{#if this.jwt_config}}jwt_configuration {
        audience = {{{this.jwt_config.audience}}}
        issuer   = "{{this.jwt_config.issuer}}"
    }{{/if}}
}{{/each}}

{{#if has_large_payloads}}resource aws_s3_bucket asml_{{service_name}}_functions {
    provider = aws.{{project_name}}-aws-lambda
    bucket   = "asml-${local.project_name}-{{service_name}}-functions"
}
resource aws_s3_bucket_acl functions {
    provider = aws.{{project_name}}-aws-lambda
    bucket   = aws_s3_bucket.asml_{{service_name}}_functions.id
    acl      = "private"
}{{/if}}
"#
    }
}

#[derive(Serialize)]
pub struct FunctionTemplate {
    pub service_name: String,
    pub function_name: String,
    pub handler_name: String,
    pub runtime_layer: String,
    pub iomods_layer: Option<String>,
    pub ruby_layer: Option<String>,
    pub large_payload: bool,
    pub http: Option<HttpData>,
    pub auth: Option<FunctionAuthData>,
    pub size: u16,
    pub timeout: u16,
    pub project_name: String,
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

{{#if large_payload}}resource aws_s3_object asml_{{service_name}}_{{function_name}} {
    key    = "{{function_name}}.zip"
    bucket = aws_s3_bucket.asml_{{service_name}}_functions.id
    source = "${local.project_path}/net/services/{{service_name}}/{{function_name}}/{{function_name}}.zip"
    etag   = filemd5("${local.project_path}/net/services/{{service_name}}/{{function_name}}/{{function_name}}.zip")
}{{/if}}

resource aws_lambda_function asml_{{service_name}}_{{function_name}} {
    provider = aws.{{project_name}}-aws-lambda

    function_name = "asml-{{project_name}}-{{service_name}}-{{function_name}}"
    role          = aws_iam_role.{{service_name}}_{{function_name}}_lambda_iam_role.arn
    runtime       = "provided"
    handler       = "{{handler_name}}"
    timeout       = {{timeout}}
    memory_size   = {{size}}

    {{#if large_payload}}
    s3_key    = "{{../function_name}}.zip"
    s3_bucket = aws_s3_bucket.asml_{{../service_name}}_functions.id
    {{else}}
    filename  = "${local.project_path}/net/services/{{../service_name}}/{{../function_name}}/{{../function_name}}.zip"
    {{/if}}

    {{#if ruby_layer}}environment {
      variables = {
        ASML_FUNCTION_ENV = "ruby-lambda"
      }
    }{{/if}}

    layers = [{{runtime_layer}}{{#if iomods_layer}}, {{iomods_layer}}{{/if}}{{#if ruby_layer}}, {{ruby_layer}}{{/if}}]

    source_code_hash = filebase64sha256("${local.project_path}/net/services/{{service_name}}/{{function_name}}/{{function_name}}.zip")
}

resource aws_iam_role {{service_name}}_{{function_name}}_lambda_iam_role {
    provider = aws.{{project_name}}-aws-lambda
    name     = "asml-{{project_name}}-{{service_name}}-{{function_name}}"

    assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      },
      "Effect": "Allow",
      "Sid": ""
    }
  ]
}
EOF

    inline_policy {
        name = "allow-cloudwatch-logging"
        policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": [
        "logs:CreateLogGroup",
        "logs:CreateLogStream",
        "logs:PutLogEvents"
      ],
      "Resource": "arn:aws:logs:*:*:*",
      "Effect": "Allow"
    }
  ]
}
EOF
    }
}
{{#if http}}
resource aws_apigatewayv2_route asml_{{service_name}}_{{function_name}} {
    provider = aws.{{project_name}}-aws-lambda

    api_id    = aws_apigatewayv2_api.{{service_name}}_http_api.id
    route_key = "{{http.verb}} {{http.path}}"
    target    = "integrations/${aws_apigatewayv2_integration.asml_{{service_name}}_{{function_name}}.id}"
{{#if auth}}
    authorization_type   = "{{auth.type}}"
    {{#if auth.id}}authorizer_id        = {{auth.id}}{{/if}}
    {{#if auth.scopes}}authorization_scopes = {{{auth.scopes}}}{{/if}}
{{else}}
    authorization_type = "NONE"
{{/if}}
}

resource aws_apigatewayv2_integration asml_{{service_name}}_{{function_name}} {
    provider = aws.{{project_name}}-aws-lambda

    api_id                 = aws_apigatewayv2_api.{{service_name}}_http_api.id
    integration_type       = "AWS_PROXY"
    payload_format_version = "2.0"

    connection_type    = "INTERNET"
    integration_method = "POST"
    integration_uri    = aws_lambda_function.asml_{{service_name}}_{{function_name}}.invoke_arn
}

resource aws_lambda_permission asml_{{service_name}}_{{function_name}} {
    provider = aws.{{project_name}}-aws-lambda

    action        = "lambda:InvokeFunction"
    function_name = "asml-{{project_name}}-{{service_name}}-{{function_name}}"
    principal     = "apigateway.amazonaws.com"

    source_arn = "${aws_apigatewayv2_api.{{service_name}}_http_api.execution_arn}/*"
}
{{/if}}
"#
    }
}

#[derive(Serialize)]
pub struct FunctionAuthData {
    pub id: Option<String>,
    pub r#type: String,
    pub scopes: Option<String>,
}

#[derive(Serialize)]
pub struct ServiceAuthData {
    pub id: String,
    pub r#type: String,
    pub jwt_config: Option<ServiceAuthDataJwtConfig>,
}

#[derive(Serialize)]
pub struct ServiceAuthDataJwtConfig {
    pub audience: String,
    pub issuer: String,
}

#[derive(Serialize)]
pub struct HttpData {
    pub verb: String,
    pub path: String,
}
