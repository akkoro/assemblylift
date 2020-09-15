use std::collections::HashMap;
use std::fs;

use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Manifest {
    pub service: Service,
    pub api: Api,
    pub iomod: Iomod,
}

#[derive(Deserialize)]
pub struct Service {
    pub name: String,
}

#[derive(Deserialize)]
pub struct Api {
    pub name: String,
    pub functions: HashMap<String, Function>, // map function_id -> function
}

#[derive(Deserialize)]
pub struct Function {
    pub name: String,
    pub handler_name: String,
}

#[derive(Deserialize)]
pub struct Iomod {
    pub dependencies: HashMap<String, Dependency>, // map dependency_id -> dependency
}

#[derive(Clone, Deserialize)]
pub struct Dependency {
    pub from: String,
    pub version: String,
    #[serde(alias = "type")]
    pub dependency_type: String,
}

pub fn read(name: &str) -> Manifest {
    let service_path = format!("./services/{}/service.toml", name);
    let service_config_contents = fs::read_to_string(service_path).unwrap();
    let service_config: Manifest = toml::from_str(&service_config_contents).unwrap();

    // TODO is this necessary? seems better to err to safety, I'm not sure what happens if these don't match
    if name != service_config.service.name {
        panic!(
            "incorrect config; service names {}, {} do not match",
            name, service_config.service.name
        )
    }

    service_config
}
