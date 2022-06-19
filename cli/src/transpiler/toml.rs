pub mod asml {
    use std::io;
    use std::path::PathBuf;
    use std::rc::Rc;

    use serde::Deserialize;

    use crate::providers::Options;

    #[derive(Deserialize)]
    pub struct Manifest {
        pub project: Project,
        pub services: Rc<Vec<Rc<ServiceRef>>>,
        pub terraform: Option<Terraform>,
        pub registries: Option<Vec<Registry>>,
    }

    #[derive(Deserialize)]
    pub struct Project {
        pub name: String,
    }

    #[derive(Deserialize)]
    pub struct Terraform {
        pub state_bucket_name: String,
        pub lock_table_name: String,
    }

    /* Represents a reference by name to a service (toml::service::Manifest) */
    #[derive(Deserialize)]
    pub struct ServiceRef {
        pub name: String,
    }

    #[derive(Deserialize)]
    pub struct Registry {
        pub host: String,
        pub options: Options,
    }

    impl Manifest {
        pub fn read(path: &PathBuf) -> Result<Self, io::Error> {
            match std::fs::read_to_string(path) {
                Ok(contents) => Ok(Self::from(contents)),
                Err(why) => Err(io::Error::new(io::ErrorKind::Other, why.to_string())),
            }
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
}

pub mod service {
    use std::io;
    use std::path::PathBuf;
    use std::rc::Rc;
    use std::sync::Arc;

    use serde::Deserialize;

    use crate::transpiler::StringMap;

    #[derive(Deserialize)]
    pub struct Manifest {
        service: Rc<Service>,
        api: Api,
        iomod: Rc<Option<Iomod>>,
    }

    impl Manifest {
        pub fn read(path: &PathBuf) -> Result<Self, io::Error> {
            match std::fs::read_to_string(path) {
                Ok(contents) => Ok(Self::from(contents)),
                Err(why) => Err(io::Error::new(io::ErrorKind::Other, why.to_string())),
            }
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
    }

    impl From<String> for Manifest {
        fn from(string: String) -> Self {
            match toml::from_str(&string) {
                Ok(manifest) => manifest,
                Err(why) => panic!("error parsing ServiceManifest: {}", why.to_string()),
            }
        }
    }

    pub type Functions = Vec<Function>;
    pub type Iomods = Vec<Dependency>;
    pub type Authorizers = Vec<HttpAuth>;

    #[derive(Deserialize)]
    pub struct Provider {
        pub name: String,
        #[serde(default)]
        pub options: Arc<StringMap<String>>,
    }

    #[derive(Deserialize)]
    pub struct Service {
        pub name: String,
        #[serde(default)]
        pub provider: Rc<Provider>,
    }

    impl Default for Provider {
        fn default() -> Self {
            Provider {
                name: String::from("aws-lambda"),
                options: Default::default(),
            }
        }
    }

    #[derive(Deserialize)]
    pub struct Api {
        pub functions: Rc<Vec<Function>>,
        pub authorizers: Option<Rc<Vec<HttpAuth>>>,
    }

    #[derive(Deserialize)]
    pub struct HttpAuth {
        pub id: String,
        pub auth_type: String,
        pub audience: Rc<Option<Rc<Vec<String>>>>,
        pub scopes: Rc<Option<Rc<Vec<String>>>>,
        pub issuer: Rc<Option<String>>,
    }

    #[derive(Deserialize)]
    pub struct HttpFunction {
        pub verb: String,
        pub path: String,
    }

    #[derive(Deserialize)]
    pub struct Function {
        pub name: String,
        #[serde(default)]
        pub provider: Rc<Provider>,
        pub registry: Option<String>,
        pub language: Option<String>,

        pub http: Rc<Option<HttpFunction>>,
        pub authorizer_id: Option<String>,

        pub timeout_seconds: Option<u16>,
        pub size_mb: Option<u16>,
    }

    #[derive(Deserialize)]
    pub struct Iomod {
        pub dependencies: Rc<Vec<Dependency>>,
    }

    #[derive(Clone, Deserialize)]
    pub struct Dependency {
        pub version: String,
        pub coordinates: String,
    }
}
