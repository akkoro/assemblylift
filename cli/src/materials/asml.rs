use std::rc::Rc;

use crate::materials::toml;
use crate::projectfs::Project as ProjectFs;

pub struct Context {
    pub project: Project,
    pub services: Vec<Service>,
    pub functions: Vec<Function>,
    pub authorizers: Vec<Authorizer>,
}

impl Context {
    pub fn from_project(project: Rc<ProjectFs>, manifest: toml::asml::Manifest) -> Result<Self, String> {
        let mut ctx_services: Vec<Service> = Vec::new();
        let mut ctx_functions: Vec<Function> = Vec::new();
        let mut ctx_authorizers: Vec<Authorizer> = Vec::new();

        for (_id, service_ref) in &*manifest.services {
            let service_manifest = toml::service::Manifest::read(&*project.service_dir(service_ref.name.clone()).dir()).unwrap();

            let service = service_manifest.service();
            let functions = service_manifest.functions();

            let service_provider = service.provider.clone();

            ctx_services.push(Service {
                name: service.name.clone(),
                provider: service_provider.clone(),
                project_name: project.name.clone(),
            });


            for (_id, function) in functions.as_ref() {
                ctx_functions.push(Function {
                    name: function.name.clone(),
                    provider: service_provider.clone(),
                    service_name: service.name.clone(),
                });
            }
        }

        Ok(Context {
            project: Project {
                name: manifest.project.name.clone(),
            },
            services: ctx_services,
            functions: ctx_functions,
            authorizers: ctx_authorizers,
        })
    }
}

pub struct Project {
    pub name: String,
//    pub version: String,
}

pub struct Service {
    pub name: String,
    pub provider: String,
    pub project_name: String,
}

pub struct Function {
    pub name: String,
    pub provider: String,
    pub service_name: String,
}

pub struct Authorizer {
    pub name: String,
    pub service_name: String,
}
