pub mod asml {
    use std::io;
    use std::path::PathBuf;
    use std::rc::Rc;

    use serde::Deserialize;
    use crate::materials::StringMap;

    #[derive(Deserialize)]
    pub struct Manifest {
        pub project: Project,
        pub services: Rc<StringMap<Rc<ServiceRef>>>, // map service_id -> service
    }

    #[derive(Deserialize)]
    pub struct Project {
        pub name: String,
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
    use serde::Deserialize;
    use crate::materials::{ContentType, StringMap};
    
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
        
        pub fn content_type(&self) -> ContentType {
            ContentType::TOML("TOML")
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

    fn default_provider() -> String {
        String::from("aws-lambda")
    }

    #[derive(Deserialize)]
    pub struct Service {
        pub name: String,
        #[serde(default = "default_provider")]
        pub provider: String,
    }

    #[derive(Deserialize)]
    pub struct Api {
        pub functions: Rc<StringMap<Function>>, // map function_id -> function
        pub authorizers: Rc<Option<StringMap<HttpAuth>>> // map auth_id -> authorizer
    }

    #[derive(Deserialize)]
    pub struct HttpAuth {
        pub auth_type: String,
        pub audience: Rc<Option<Vec<String>>>, // TODO do these actually need to be Rc?
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
        #[serde(default = "default_provider")]
        pub provider: String,
        pub handler_name: Option<String>,

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
        pub from: String,
        pub version: String,
        #[serde(alias = "type")]
        pub dependency_type: String,
    }
}
