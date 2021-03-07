pub mod asml {
    use std::io;
    use std::path::PathBuf;
    use serde::Deserialize;
    use crate::materials::StringMap;
    use crate::providers::Transformable;

    #[derive(Deserialize)]
    pub struct Manifest {
        pub project: Project,
        pub services: StringMap<ServiceRef>, // map service_id -> service
    }

    impl Transformable for Manifest {
        const TYPE: &'static str = "root";
    }

    // TODO move Project to models.rs
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
    use crate::materials::{models, ContentType};
    
    #[derive(Deserialize)]
    pub struct Manifest {
        service: models::Service,
        api: models::Api,
        iomod: Rc<Option<models::Iomod>>,
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

        pub fn service(&self) -> models::Service {
            self.service
        }
        
        pub fn functions(&self) -> models::Functions {
            *self.api.functions
        }
        
        pub fn iomods(&self) -> Option<models::Iomod> {
            *self.iomod
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
