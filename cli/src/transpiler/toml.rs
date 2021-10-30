pub mod asml {
    use std::io;
    use std::path::PathBuf;
    use std::rc::Rc;

    use serde::Deserialize;
    use crate::transpiler::StringMap;

    #[derive(Deserialize)]
    pub struct Manifest {
        pub project: Project,
        pub services: Rc<StringMap<Rc<ServiceRef>>>, // map service_id -> service
        pub terraform: Option<Terraform>,
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

    pub type Functions = StringMap<Function>;
    pub type Iomods = StringMap<Dependency>;
    pub type Authorizers = StringMap<HttpAuth>;

    fn default_api_provider() -> String {
        String::from("aws-apigw")
    }

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
        #[serde(default = "default_api_provider")]
        pub provider: String,
        pub functions: Rc<StringMap<Function>>, // map function_id -> function
        pub authorizers: Option<Rc<StringMap<HttpAuth>>> // map auth_id -> authorizer
    }

    #[derive(Deserialize)]
    pub struct HttpAuth {
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
        pub handler_name: Option<String>,
        #[serde(default)]
        pub enable_wasi: bool,

        pub http: Rc<Option<HttpFunction>>,
        pub authorizer_id: Option<String>,

        pub timeout_seconds: Option<u16>,
        pub size_mb: Option<u16>,
    }

    #[derive(Deserialize)]
    pub struct Iomod {
        pub dependencies: Rc<StringMap<Dependency>>, // map dependency_id -> dependency
    }

    #[derive(Clone, Deserialize)]
    pub struct Dependency {
        #[serde(alias = "type", default = "default_dependency_type")]
        pub dependency_type: Option<String>,
        pub from: Option<String>,
        pub version: String,
        pub coordinates: String,
    }

    fn default_dependency_type() -> Option<String> {
        Some("registry".to_string())
    }
}
