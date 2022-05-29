use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use clap::crate_version;
use handlebars::{Handlebars, to_json};
use serde::Serialize;

use crate::projectfs::Project as ProjectFs;
use crate::providers::SERVICE_PROVIDERS;
use crate::transpiler::{Castable, CastError, ContentType, context, StringMap, Template, toml};

pub struct Context {
    pub project: Project,
    pub terraform: Option<Terraform>,
    pub services: Vec<Service>,
    pub functions: Vec<Function>,
    pub authorizers: Vec<Authorizer>,
    pub iomods: Vec<Iomod>,
}

impl Context {
    pub fn from_project(
        project: Rc<ProjectFs>,
        manifest: toml::asml::Manifest,
    ) -> Result<Self, String> {
        let mut ctx_services: Vec<Service> = Vec::new();
        let mut ctx_functions: Vec<Function> = Vec::new();
        let mut ctx_authorizers: Vec<Authorizer> = Vec::new();
        let mut ctx_iomods: Vec<Iomod> = Vec::new();

        for (_id, service_ref) in &*manifest.services {
            let mut service_path = project.service_dir(service_ref.name.clone()).dir();
            service_path.push("service.toml");
            let service_manifest = toml::service::Manifest::read(&service_path).unwrap();

            let service = service_manifest.service();
            let functions = service_manifest.functions();
            let iomods = service_manifest.iomods();
            let authorizers = service_manifest.authorizers();

            let service_provider = service.provider.clone();

            ctx_services.push(Service {
                name: service.name.clone(),
                provider: Rc::new(Provider {
                    name: service_provider.name.clone(),
                    options: service_provider.options.clone(),
                }),
                project_name: project.name.clone(),
            });

            for (_id, function) in functions.as_ref() {
                ctx_functions.push(Function {
                    name: function.name.clone(),
                    provider: Rc::new(Provider {
                        name: function.provider.name.clone(),
                        options: function.provider.options.clone(),
                    }),
                    service_name: service.name.clone(),
                    language: function.language.clone().unwrap_or("rust".to_string()),
                    size: function.size_mb.unwrap_or(1024u16),
                    timeout: function.timeout_seconds.unwrap_or(5u16),
                    http: match &function.clone().http.as_ref() {
                        Some(http) => Some(Http {
                            verb: http.verb.clone(),
                            path: http.path.clone(),
                        }),
                        None => None,
                    },
                    authorizer_id: function.authorizer_id.clone(),
                });
            }

            for (id, iomod) in iomods.as_ref() {
                ctx_iomods.push(Iomod {
                    name: id.clone(),
                    service_name: service.name.clone(),
                    coordinates: iomod.coordinates.clone(),
                    version: iomod.version.clone(),
                });
            }

            for (id, authorizer) in authorizers.as_ref() {
                ctx_authorizers.push(Authorizer {
                    id: id.clone(),
                    service_name: service.name.clone(),
                    r#type: authorizer.auth_type.clone(),
                    scopes: authorizer
                        .scopes
                        .clone()
                        .as_ref()
                        .as_ref()
                        .unwrap_or(&Rc::new(Vec::<String>::new()))
                        .clone(),
                    jwt_config: match authorizer.auth_type.clone().to_lowercase().as_str() {
                        "jwt" => Some(AuthorizerJwt {
                            audience: authorizer
                                .audience
                                .clone()
                                .as_ref()
                                .as_ref()
                                .expect("JWT authorizer requires audience field")
                                .clone(),
                            issuer: authorizer
                                .issuer
                                .clone()
                                .as_ref()
                                .as_ref()
                                .expect("JWT authorizer requires issuer field")
                                .clone(),
                        }),
                        _ => None,
                    },
                })
            }
        }

        Ok(Context {
            project: Project {
                name: manifest.project.name.clone(),
                path: (*project.dir()).into_os_string().into_string().unwrap(),
            },
            terraform: match manifest.terraform {
                Some(tf) => Some(Terraform {
                    state_bucket_name: tf.state_bucket_name,
                    lock_table_name: tf.lock_table_name,
                }),
                None => None,
            },
            services: ctx_services,
            functions: ctx_functions,
            authorizers: ctx_authorizers,
            iomods: ctx_iomods,
        })
    }

    pub fn service(&self, name: String) -> Option<&Service> {
        match self.services.binary_search_by(|s| s.name.cmp(&name)) {
            Ok(idx) => Some(self.services.get(idx).unwrap()),
            Err(_) => None,
        }
    }
}

impl Castable for Context {
    fn cast(&self, ctx: Rc<Context>, selector: Option<&str>) -> Result<Vec<String>, CastError> {
        let mut hcl_content = format!("# Generated by assemblylift-cli {}\r\n", crate_version!());

        let mut usermod_path = PathBuf::from(
            crate::projectfs::locate_asml_manifest()
                .expect("could not locate assemblylift.toml")
                .1,
        );
        usermod_path.pop();
        usermod_path.push("user_tf/");
        let user_inject: bool = fs::metadata(usermod_path.clone()).is_ok();
        let (remote_state, state_bucket_name, lock_table_name) = match &self.terraform {
            Some(tf) => (
                true,
                Some(tf.state_bucket_name.clone()),
                Some(tf.lock_table_name.clone()),
            ),
            None => (false, None, None),
        };

        let tmpl = ContextTemplate {
            project_name: self.project.name.clone(),
            project_path: self.project.path.clone(),
            user_inject,
            remote_state,
            state_bucket_name,
            lock_table_name,
        };
        hcl_content.push_str(&*tmpl.render());

        for service in &self.services {
            hcl_content
                .push_str(&service.cast(ctx.clone(), Some(&*service.name)).unwrap()[0].clone());
        }

        Ok(vec![hcl_content])
    }

    fn content_type(&self) -> Vec<ContentType> {
        todo!()
    }
}

pub struct Project {
    pub name: String,
    pub path: String,
    //    pub version: String,
}

pub struct Terraform {
    pub state_bucket_name: String,
    pub lock_table_name: String,
}

pub struct Service {
    pub name: String,
    pub provider: Rc<Provider>,
    pub project_name: String,
}

impl Service {
    pub fn option(&self, name: &str) -> Option<&String> {
        self.provider.options.get(name)
    }
}

impl Castable for Service {
    fn cast(&self, ctx: Rc<Context>, selector: Option<&str>) -> Result<Vec<String>, CastError> {
        let mut content = format!("# Begin service `{}`\n", &self.name);

        let provider_name = self.provider.name.clone();
        let mut service_provider = SERVICE_PROVIDERS
            .get(&provider_name)
            .expect(&format!(
                "could not find service provider named {}",
                provider_name
            ))
            .lock()
            .unwrap();

        service_provider
            .set_options(self.provider.options.clone())
            .expect("unable to set provider options");
        // service_provider
        //     .init(ctx.clone(), self.name.into())
        //     .expect("unable to initialize service provider");

        let service_artifact = service_provider
            .cast(ctx.clone(), selector)
            .expect("unexpected error transforming service");
        content.push_str(&service_artifact[0].clone()); // FIXME unchecked index

        Ok(vec![content])
    }

    fn content_type(&self) -> Vec<ContentType> {
        todo!()
    }
}

pub struct Provider {
    pub name: String,
    pub options: Arc<StringMap<String>>,
}

pub struct Function {
    pub name: String,
    pub provider: Rc<Provider>,
    pub language: String,
    pub service_name: String,

    pub http: Option<Http>,
    pub authorizer_id: Option<String>,

    pub size: u16,
    pub timeout: u16,
}

pub struct Http {
    pub verb: String,
    pub path: String,
}

pub struct Authorizer {
    pub id: String,
    pub service_name: String,
    pub r#type: String,
    pub scopes: Rc<Vec<String>>,
    pub jwt_config: Option<AuthorizerJwt>,
}

pub struct AuthorizerJwt {
    pub issuer: String,
    pub audience: Rc<Vec<String>>,
}

pub struct Iomod {
    pub name: String,
    pub service_name: String,
    pub coordinates: String,
    pub version: String,
}

#[derive(Serialize)]
pub struct ContextTemplate {
    pub project_name: String,
    pub project_path: String,
    pub user_inject: bool,
    pub remote_state: bool,
    pub state_bucket_name: Option<String>,
    pub lock_table_name: Option<String>,
}

impl Template for ContextTemplate {
    fn render(&self) -> String {
        // to_json(&self);
        let mut reg = Box::new(Handlebars::new());
        reg.register_template_string("hcl_template", Self::tmpl())
            .unwrap();
        reg.render("hcl_template", &self).unwrap()
    }

    fn tmpl() -> &'static str {
        r#"terraform {
    required_providers {
        docker = {
            source  = "kreuzwerker/docker"
            version = ">= 2.16.0"
        }
        kubernetes = {
          source  = "hashicorp/kubernetes"
          version = ">= 2.0.0"
        }
    }
}

locals {
    project_name = "{{project_name}}"
    project_path = "{{project_path}}"
}
{{#if user_inject}}
module "usermod" {
  source = "../user_tf"
}
{{/if}}
{{#if remote_state}}
terraform {
  backend "s3" {
    encrypt = true
    bucket = "{{state_bucket_name}}"
    dynamodb_table = "{{lock_table_name}}"
    key    = "terraform.tfstate"
    region = "us-east-1"
  }
}
{{/if}}
"#
    }
}
