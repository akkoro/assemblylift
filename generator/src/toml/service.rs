use std::io;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::Provider;
use super::StringMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct Manifest {
    pub gateway: Gateway,
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Default::default")]
    pub functions: Functions,
    pub iomod: Option<Iomod>,
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

    // pub fn service(&self) -> Service {
    //     self.service.clone()
    // }

    // pub fn functions(&self) -> Functions {
    //     self.functions.clone()
    // }

    pub fn iomods(&self) -> Iomods {
        match &self.iomod {
            Some(iomod) => iomod.dependencies.clone(),
            None => Iomods::new(),
        }
    }

    // pub fn rename(&mut self, new_name: &str) {
    //     let svc = self.service.clone();
    //     let new_svc = Service {
    //         name: new_name.to_string(),
    //         registry_id: svc.registry_id.clone(),
    //         provider: svc.provider.clone(),
    //     };
    //     self.service = new_svc;
    // }

    pub fn add_function(&mut self, resource_name: &str, language: &str) {
        let mut functions = Vec::new();
        for fun in &self.functions {
            functions.push(fun.clone());
        }
        let fun = Function {
            name: resource_name.to_string(),
            registry: None,
            language: Some(language.into()),
            http: None,
            authorizer_id: None,
            timeout_seconds: None,
            size_mb: None,
            cpu_compat_mode: None,
            precompile: None,
            environment: None,
        };
        functions.push(fun);
        self.functions = functions;
    }

    pub fn remove_function(&mut self, resource_name: &str) {
        let mut functions = Vec::new();
        for svc in self.functions.iter().filter(|f| f.name != resource_name) {
            functions.push(svc.clone());
        }
        self.functions = functions;
    }

    pub fn rename_function(&mut self, old_name: &str, new_name: &str) {
        let mut to_rename = self
            .functions
            .iter()
            .find(|f| f.name == old_name)
            .unwrap()
            .clone();
        to_rename.name = new_name.into();
        self.remove_function(old_name);
        let mut functions = Vec::new();
        for fun in &self.functions {
            functions.push(fun.clone());
        }
        functions.push(to_rename);
        self.functions = functions;
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Gateway {
    pub provider: Provider,
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
    pub cpu_compat_mode: Option<String>,
    pub precompile: Option<bool>,
    pub http: Option<HttpFunction>,
    pub environment: Option<StringMap<String>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Iomod {
    pub dependencies: Vec<Dependency>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Dependency {
    pub version: String,
    pub coordinates: String,
}
