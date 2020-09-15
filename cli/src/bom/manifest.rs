use std::collections::HashMap;
use std::fs;

use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Manifest {
    pub project: Project,
    pub services: HashMap<String, Service>, // map service_id -> service
}

#[derive(Deserialize)]
pub struct Project {
    pub name: String,
}

#[derive(Deserialize)]
pub struct Service {
    pub name: String,
}

pub fn read() -> Manifest {
    let manifest_contents = match fs::read_to_string("./assemblylift.toml") {
        Ok(contents) => contents,
        Err(why) => panic!("could not read assemblylift.toml: {}", why.to_string()),
    };

    let manifest: Manifest = match toml::from_str(&manifest_contents) {
        Ok(config) => config,
        Err(why) => panic!("could not parse assemblylift.toml: {}", why.to_string()),
    };

    manifest
}
