use std::rc::Rc;
use std::sync::Arc;

use crate::transpiler::{toml, StringMap};
use crate::projectfs::Project as ProjectFs;

pub struct Context {
    pub project: Project,
    pub terraform: Option<Terraform>,
    pub services: Vec<Service>,
    pub functions: Vec<Function>,
    pub authorizers: Vec<Authorizer>,
    pub iomods: Vec<Iomod>,
}

impl Context {
    pub fn from_project(project: Rc<ProjectFs>, manifest: toml::asml::Manifest) -> Result<Self, String> {
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
                    handler_name: match &function.clone().handler_name {
                        Some(name) => name.clone(),
                        None => String::from("handler"),
                    },
                    size: function.size_mb.unwrap_or(1024u16),
                    timeout: function.timeout_seconds.unwrap_or(5u16),
                    http: match &function.clone().http.as_ref() {
                        Some(http) => {
                            Some(Http {
                                verb: http.verb.clone(),
                                path: http.path.clone(),
                            })
                        },
                        None => None,
                    },
                    authorizer_id: function.authorizer_id.clone(),
                });
            }

            for (id, _iomod) in iomods.as_ref() {
                ctx_iomods.push(Iomod {
                    name: id.clone(),
                    service_name: service.name.clone(),
                });
            }

            for (id, authorizer) in authorizers.as_ref() {
                ctx_authorizers.push(Authorizer {
                    id: id.clone(),
                    service_name: service.name.clone(),
                    r#type: authorizer.auth_type.clone(),
                    scopes: authorizer
                        .scopes.clone()
                        .as_ref().as_ref()
                        .unwrap_or(&Rc::new(Vec::<String>::new()))
                        .clone(),
                    jwt_config: match authorizer.auth_type.clone().to_lowercase().as_str() {
                        "jwt" => {
                            Some(AuthorizerJwt {
                                audience: authorizer
                                    .audience.clone()
                                    .as_ref().as_ref()
                                    .expect("JWT authorizer requires audience field")
                                    .clone(),
                                issuer: authorizer
                                    .issuer.clone()
                                    .as_ref().as_ref()
                                    .expect("JWT authorizer requires issuer field")
                                    .clone(),
                            })
                        }
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

pub struct Provider {
    pub name: String,
    pub options: Arc<StringMap<String>>,
}

pub struct Function {
    pub name: String,
    pub provider: Rc<Provider>,
    pub service_name: String,

    pub http: Option<Http>,
    pub authorizer_id: Option<String>,

    pub handler_name: String,
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
}
