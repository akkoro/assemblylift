use std::io;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::Provider;
use crate::Options;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Manifest {
    pub project: Project,
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Default::default")]
    pub platforms: Vec<Platform>,
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Default::default")]
    pub services: Vec<ServiceRef>,
    authorizers: Option<Vec<HttpAuth>>,
    domains: Option<Vec<Domain>>,
    registries: Option<Vec<Registry>>,
    pub terraform: Option<Terraform>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Platform {
    pub id: String,
    pub name: String,
    pub options: Options,
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
    pub provider: Provider,
    pub registry_id: Option<String>,
    pub domain_name: Option<String>,
    pub is_root: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Registry {
    pub id: String,
    pub provider: Provider,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Domain {
    pub dns_name: String,
    #[serde(default)]
    pub map_to_root: bool,
    pub provider: Provider,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HttpAuth {
    pub id: String,
    pub auth_type: String,
    pub issuer: Option<String>,
    pub audience: Option<Vec<String>>,
    pub scopes: Option<Vec<String>>,
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
        for svc in self.services.clone() {
            services.push(svc);
        }
        services.push(ServiceRef {
            name: resource_name.into(),
            provider: Provider {
                name: "my-provider".into(),
                options: Default::default(),
                platform_id: None
            },
            registry_id: None,
            domain_name: None,
            is_root: None,
        });
        self.services = services;
    }

    pub fn remove_service(&mut self, resource_name: &str) {
        let mut services = Vec::new();
        for svc in self.services.iter().filter(|s| s.name != resource_name) {
            services.push(svc.clone());
        }
        self.services = services;
    }

    pub fn rename_service(&mut self, old_name: &str, new_name: &str) {
        let mut services = Vec::new();
        for svc in self.services.clone() {
            if svc.name == old_name {
                services.push(ServiceRef {
                    name: new_name.into(),
                    provider: svc.provider,
                    registry_id: svc.registry_id,
                    domain_name: svc.domain_name,
                    is_root: svc.is_root,

                });
            } else {
                services.push(svc);
            }
        }
        self.services = services;
    }

    pub fn authorizers(&self) -> Vec<HttpAuth> {
        match self.authorizers.as_ref() {
            Some(auth) => auth.clone(),
            None => Vec::new(),
        }
    }

    pub fn domains(&self) -> Vec<Domain> {
        match self.domains.as_ref() {
            Some(domains) => domains.clone(),
            None => Vec::new(),
        }
    }

    pub fn registries(&self) -> Vec<Registry> {
        match self.registries.as_ref() {
            Some(registries) => registries.clone(),
            None => Vec::new(),
        }
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
