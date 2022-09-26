use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::transpiler::StringMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Provider {
    pub name: String,
    #[serde(skip_serializing_if = "StringMap::is_empty", default)]
    pub options: Arc<StringMap<String>>,
}

pub mod asml {
    use std::io;
    use std::path::PathBuf;
    use std::rc::Rc;

    use serde::{Deserialize, Serialize};

    use crate::providers::Options;
    use crate::transpiler::toml::Provider;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Manifest {
        pub project: Project,
        #[serde(skip_serializing_if = "Vec::is_empty", default = "Default::default")]
        pub services: Rc<Vec<Rc<ServiceRef>>>,
        pub domains: Option<Vec<Domain>>,
        pub terraform: Option<Terraform>,
        pub registries: Option<Vec<Registry>>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Project {
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Terraform {
        pub state_bucket_name: String,
        pub lock_table_name: String,
    }

    /* Represents a reference by name to a service (toml::service::Manifest) */
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct ServiceRef {
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Registry {
        pub host: String,
        pub options: Options,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Domain {
        pub dns_name: String,
        #[serde(default)]
        pub map_to_root: bool,
        pub provider: Provider,
    }

    impl Manifest {
        pub fn read(file: &PathBuf) -> Result<Self, io::Error> {
            match std::fs::read_to_string(file) {
                Ok(contents) => Ok(Self::from(contents)),
                Err(why) => Err(io::Error::new(io::ErrorKind::Other, why.to_string())),
            }
        }

        pub fn write(&self, mut path: PathBuf) -> Result<(), io::Error> {
            path.push("assemblylift.toml");
            let contents = Into::<String>::into(self.clone());
            std::fs::write(path, contents)
        }

        pub fn add_service(&mut self, resource_name: &str) {
            let mut services = Vec::new();
            for svc in self.services.as_ref() {
                services.push(svc.clone());
            }
            services.push(Rc::new(ServiceRef {
                name: resource_name.to_string(),
            }));
            self.services = Rc::new(services);
        }

        pub fn remove_service(&mut self, resource_name: &str) {
            let mut services = Vec::new();
            for svc in self
                .services
                .as_ref()
                .iter()
                .filter(|s| s.name != resource_name)
            {
                services.push(svc.clone());
            }
            self.services = Rc::new(services);
        }

        pub fn rename_service(&mut self, old_name: &str, new_name: &str) {
            let mut services = Vec::new();
            for svc in self.services.as_ref() {
                if svc.name == old_name {
                    services.push(Rc::new(ServiceRef {
                        name: new_name.into(),
                    }));
                } else {
                    services.push(svc.clone());
                }
            }
            self.services = Rc::new(services);
        }
    }

    impl From<String> for Manifest {
        fn from(string: String) -> Self {
            match toml::from_str(&string) {
                Ok(manifest) => manifest,
                Err(why) => panic!("error parsing Manifest: {}", why.to_string()),
            }
        }
    }

    impl Into<String> for Manifest {
        fn into(self) -> String {
            toml::to_string(&self).expect("unable to serialize TOML")
        }
    }
}

pub mod service {
    use std::io;
    use std::path::PathBuf;
    use std::rc::Rc;
    use std::sync::Arc;

    use serde::{Deserialize, Serialize};

    use crate::transpiler::StringMap;
    use crate::transpiler::toml::Provider;

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Manifest {
        service: Rc<Service>,
        #[serde(default)]
        pub api: Api,
        iomod: Rc<Option<Iomod>>,
    }

    impl Manifest {
        pub fn read(file: &PathBuf) -> Result<Self, io::Error> {
            match std::fs::read_to_string(file) {
                Ok(contents) => Ok(Self::from(contents)),
                Err(why) => Err(io::Error::new(io::ErrorKind::Other, why.to_string())),
            }
        }

        pub fn write(&self, mut path: PathBuf) -> Result<(), io::Error> {
            path.push("service.toml");
            std::fs::write(path, Into::<String>::into(self.clone()))
        }

        pub fn service(&self) -> Rc<Service> {
            self.service.clone()
        }

        pub fn functions(&self) -> Rc<Functions> {
            self.api.functions.clone()
        }

        pub fn iomods(&self) -> Rc<Iomods> {
            match self.iomod.as_ref() {
                Some(iomod) => iomod.dependencies.clone(),
                None => Rc::new(Iomods::new()),
            }
        }

        pub fn authorizers(&self) -> Rc<Authorizers> {
            match self.api.authorizers.as_ref() {
                Some(auth) => auth.clone(),
                None => Rc::new(Authorizers::new()),
            }
        }

        pub fn rename(&mut self, new_name: &str) {
            let svc = self.service.clone();
            let new_svc = Service {
                name: new_name.to_string(),
                provider: svc.provider.clone(),
            };
            self.service = Rc::new(new_svc);
        }

        pub fn add_function(&mut self, resource_name: &str, language: &str) {
            let mut functions = Vec::new();
            for fun in self.functions().as_ref() {
                functions.push(fun.clone());
            }
            let fun = Function {
                name: resource_name.to_string(),
                registry: None,
                language: Some(language.into()),
                http: Rc::new(None),
                authorizer_id: None,
                timeout_seconds: None,
                size_mb: None,
            };
            functions.push(fun);
            self.api.functions = Rc::new(functions);
        }

        pub fn remove_function(&mut self, resource_name: &str) {
            let mut functions = Vec::new();
            for svc in self
                .functions()
                .as_ref()
                .iter()
                .filter(|f| f.name != resource_name)
            {
                functions.push(svc.clone());
            }
            self.api.functions = Rc::new(functions);
        }

        pub fn rename_function(&mut self, old_name: &str, new_name: &str) {
            let mut to_rename = self
                .functions()
                .iter()
                .find(|f| f.name == old_name)
                .unwrap()
                .clone();
            to_rename.name = new_name.into();
            self.remove_function(old_name);
            let mut functions = Vec::new();
            for fun in self.functions().as_ref() {
                functions.push(fun.clone());
            }
            functions.push(to_rename);
            self.api.functions = Rc::new(functions);
        }
    }

    impl From<String> for Manifest {
        fn from(string: String) -> Self {
            match toml::from_str(&string) {
                Ok(manifest) => manifest,
                Err(why) => panic!("error parsing ServiceManifest: {}", why.to_string()),
            }
        }
    }

    impl Into<String> for Manifest {
        fn into(self) -> String {
            toml::to_string(&self).expect("unable to serialize TOML")
        }
    }

    pub type Functions = Vec<Function>;
    pub type Iomods = Vec<Dependency>;
    pub type Authorizers = Vec<HttpAuth>;

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Service {
        pub name: String,
        pub provider: Rc<Provider>,
    }

    #[derive(Serialize, Deserialize, Clone, Default)]
    pub struct Api {
        pub domain_name: Option<String>,
        pub is_root: Option<bool>,
        #[serde(skip_serializing_if = "Vec::is_empty", default = "Default::default")]
        pub functions: Rc<Vec<Function>>,
        pub authorizers: Option<Rc<Vec<HttpAuth>>>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct HttpAuth {
        pub id: String,
        pub auth_type: String,
        pub issuer: Rc<Option<String>>,
        pub audience: Rc<Option<Rc<Vec<String>>>>,
        pub scopes: Rc<Option<Rc<Vec<String>>>>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct HttpFunction {
        pub verb: String,
        pub path: String,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Function {
        pub name: String,
        pub registry: Option<String>,
        pub language: Option<String>,
        pub authorizer_id: Option<String>,
        pub timeout_seconds: Option<u16>,
        pub size_mb: Option<u16>,
        pub http: Rc<Option<HttpFunction>>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Iomod {
        pub dependencies: Rc<Vec<Dependency>>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Dependency {
        pub version: String,
        pub coordinates: String,
    }
}
