use std::fs;
use std::process;
use std::collections::HashMap;

use clap::ArgMatches;
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct AssemblyLiftConfig {
    services: HashMap<String, AssemblyLiftConfigServices> // map service_id -> service
}

#[derive(Deserialize)]
struct AssemblyLiftConfigServices {
    name: String
}

#[derive(Deserialize)]
struct AssemblyLiftServiceConfig {
    service: AssemblyLiftServiceConfigService
}

#[derive(Deserialize)]
struct AssemblyLiftServiceConfigService {
    name: String
}

pub fn compile(matches: Option<&ArgMatches>) {
    let matches = match matches {
        Some(matches) => matches,
        _ => panic!("could not get matches for compile")
    };

    // TODO 1) read assemblylift.toml to find functions ✅ 
    //      2) run appropriate build command for each function
    //      3) aggregate build output for deployment

    let asml_config_contents = fs::read_to_string("./assemblylift.toml").unwrap();
    let asml_config: AssemblyLiftConfig = toml::from_str(&asml_config_contents).unwrap();
    for (_k, v) in asml_config.services {
        let service_path = format!("./services/{}/service.toml", v.name);
        let service_config_contents = fs::read_to_string(service_path).unwrap();
        let service_config: AssemblyLiftServiceConfig = toml::from_str(&service_config_contents).unwrap();

        // TODO is this necessary? seems better to err to safety, I'm not sure what happens if these don't match
        if v.name != service_config.service.name {
            panic!("incorrect config; service names {}, {} do not match", v.name, service_config.service.name)
        }


    }
}
