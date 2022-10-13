use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use clap::crate_version;
use handlebars::Handlebars;
use itertools::Itertools;
use serde::Serialize;

use crate::projectfs::Project as ProjectFs;
use crate::providers::{DNS_PROVIDERS, PROVIDERS};
use crate::transpiler::{
    Artifact, Bindable, Castable, CastError, ContentType, StringMap, Template, toml,
};

/// `Context` is a state object, containing the configuration of a project as deserialized from the
/// project and service manifests (TOML). `Context` is `Castable` and is the entrypoint of the `cast`
/// operation.
///
/// See docs/cli-transpiler.md
pub struct Context {
    pub project: Project,
    pub terraform: Option<Terraform>,
    pub domains: Vec<Domain>,
    pub services: Vec<Service>,
    pub functions: Vec<Function>,
    pub authorizers: Vec<Authorizer>,
    pub iomods: Vec<Iomod>,
    pub registries: Vec<Registry>,
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
        let mut ctx_registries: Vec<Registry> = manifest
            .registries
            .unwrap_or(Vec::new())
            .iter()
            .map(|r| Registry {
                host: r.host.clone(),
                options: r.options.clone(),
            })
            .collect();
        let mut ctx_domains = manifest
            .domains
            .unwrap_or(Vec::new())
            .iter()
            .map(|d| Domain {
                dns_name: d.dns_name.clone(),
                map_to_root: d.map_to_root,
                provider: Rc::new(Provider {
                    name: d.provider.name.clone(),
                    options: d.provider.options.clone(),
                    tf_name: "nil".to_string(),
                }),
            })
            .collect();

        for service_ref in &*manifest.services {
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
                    tf_name: match &*service_provider.name {
                        "aws-lambda" => "aws",
                        "k8s" => "kubernetes",
                        _ => "nil"
                    }.to_string(),
                }),
                is_root: service_manifest.api.is_root,
                domain_name: service_manifest.api.domain_name,
                project_name: project.name.clone(),
            });

            for function in functions.as_ref() {
                ctx_functions.push(Function {
                    name: function.name.clone(),
                    registry: function.registry.clone().unwrap_or("ecr".to_string()),
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

            for iomod in iomods.as_ref() {
                let coords: Vec<&str> = iomod.coordinates.split('.').collect();
                let name = coords.get(2).unwrap();
                ctx_iomods.push(Iomod {
                    name: name.to_string(),
                    service_name: service.name.clone(),
                    coordinates: iomod.coordinates.clone(),
                    version: iomod.version.clone(),
                });
            }

            for authorizer in authorizers.as_ref() {
                ctx_authorizers.push(Authorizer {
                    id: authorizer.id.clone(),
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
            domains: ctx_domains,
            services: ctx_services,
            functions: ctx_functions,
            authorizers: ctx_authorizers,
            iomods: ctx_iomods,
            registries: ctx_registries,
        })
    }

    pub fn service(&self, name: &str) -> Option<&Service> {
        self.services.iter().find(|&s| &s.name == name)
    }
}

impl Castable for Context {
    fn cast(&self, ctx: Rc<Context>, _selector: Option<&str>) -> Result<Vec<Artifact>, CastError> {
        let mut hcl_content = format!("# Generated by assemblylift-cli {}\r\n\n", crate_version!());
        let mut kube_content =
            format!("# Generated by assemblylift-cli {}\r\n\n", crate_version!());

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

        let mut providers: Vec<Rc<Provider>> =
            ctx.services.iter().map(|s| s.provider.clone()).collect();
        providers.dedup_by(|a, b| a.name.eq_ignore_ascii_case(&*b.name));

        let tmpl = ContextTemplate {
            project_name: self.project.name.clone(),
            project_path: self.project.path.clone(),
            user_inject,
            remote_state,
            state_bucket_name,
            lock_table_name,
            providers: providers.clone(),
        };
        hcl_content.push_str(&*tmpl.render());

        // FIXME dedupe by name (there's only one provider possible rn)
        let mut dns_providers = ctx.domains.iter().map(|d| d.provider.clone()).collect_vec();
        for dns in dns_providers {
            let provider = DNS_PROVIDERS
                .get(&*dns.name.clone())
                .expect("could not find dns provider");
            provider
                .lock()
                .unwrap()
                .set_options(dns.options.clone())
                .expect("could not set dns provider options");
            let artifacts = provider
                .lock()
                .unwrap()
                .cast(ctx.clone(), None)
                .unwrap();
            for a in artifacts {
                if let ContentType::HCL(_) = a.content_type {
                    hcl_content.push_str(&*a.content.clone());
                }
            }
        }

        let mut out: Vec<Artifact> = Vec::new();
        for p in providers {
            // println!("DEBUG casting provider {}", p.name.clone());
            let provider = PROVIDERS
                .get(&*p.name.clone())
                .expect("could not find provider");
            provider
                .lock()
                .unwrap()
                .set_options(p.options.clone())
                .expect("could not set provider options");

            let artifacts = provider.lock().unwrap().cast(ctx.clone(), None).unwrap();
            for a in artifacts {
                match a.content_type {
                    ContentType::HCL(_) => hcl_content.push_str(&*a.content.clone()),
                    ContentType::KubeYaml(_) => kube_content.push_str(&*a.content.clone()),
                    ContentType::Dockerfile(_) => out.push(a.clone()),
                }
            }
        }

        let hcl = Artifact {
            content_type: ContentType::HCL("HCL"),
            content: hcl_content,
            write_path: "net/plan.tf".into(),
        };
        let yaml = Artifact {
            content_type: ContentType::KubeYaml("kube-yaml"),
            content: kube_content,
            write_path: "net/kube.yaml".into(),
        };
        out.append(&mut vec![hcl, yaml]);
        Ok(out)
    }
}

impl Bindable for Context {
    fn bind(&self, ctx: Rc<Context>) -> Result<(), CastError> {
        let mut providers: Vec<Rc<Provider>> =
            ctx.services.iter().map(|s| s.provider.clone()).collect();
        providers.dedup_by(|a, b| a.name.eq_ignore_ascii_case(&*b.name));
        for p in providers {
            let provider = PROVIDERS
                .get(&*p.name.clone())
                .expect("could not find provider");
            let mut provider_lock = provider.lock().unwrap();
            provider_lock
                .set_options(p.options.clone())
                .expect("could not set provider options");
            // if !provider_lock.is_booted(ctx.clone()) {
            println!("Booting provider {}...", provider_lock.name());
            provider_lock
                .boot(ctx.clone())
                .expect("could not bootstrap provider");
            // }
            provider_lock
                .bind(ctx.clone())
                .expect("could not run provider bind step")
        }

        Ok(())
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

pub struct Registry {
    pub host: String,
    pub options: StringMap<String>,
}

pub struct Domain {
    pub dns_name: String,
    pub map_to_root: bool,
    pub provider: Rc<Provider>,
}

#[derive(Clone)]
pub struct Service {
    pub name: String,
    pub provider: Rc<Provider>,
    pub is_root: Option<bool>,
    pub domain_name: Option<String>,
    pub project_name: String,
}

impl Service {
    pub fn option(&self, name: &str) -> Option<&String> {
        self.provider.options.get(name)
    }
}

#[derive(Serialize)]
pub struct Provider {
    pub name: String,
    pub options: Arc<StringMap<String>>,
    tf_name: String,
}

pub struct Function {
    pub name: String,
    pub registry: String,
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

#[derive(Debug, Clone)]
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
    pub providers: Vec<Rc<Provider>>,
}

impl Template for ContextTemplate {
    fn render(&self) -> String {
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
{{#each providers}}
provider {{tf_name}} {
    alias  = "{{../project_name}}-{{name}}"
    region = "{{options.aws_region}}"
}
{{/each}}
{{#if user_inject}}module "usermod" {
  source = "../user_tf"
  providers = {
  {{#each providers}}  {{this.tf_name}}.{{../project_name}}-{{this.name}} = {{this.tf_name}}.{{../project_name}}-{{this.name}}{{/each}}
  }
}{{/if}}
{{#if remote_state}}terraform {
  backend "s3" {
    encrypt = true
    bucket = "{{state_bucket_name}}"
    dynamodb_table = "{{lock_table_name}}"
    key    = "terraform.tfstate"
    region = "us-east-1"
  }
}{{/if}}
"#
    }
}
