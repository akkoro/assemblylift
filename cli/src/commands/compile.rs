use std::fs;
use std::path;
use std::process;
use std::process::Stdio;
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
    service: AssemblyLiftServiceConfigService,
    api: AssemblyLiftServiceConfigApi
}

#[derive(Deserialize)]
struct AssemblyLiftServiceConfigService {
    name: String
}

#[derive(Deserialize)]
struct AssemblyLiftServiceConfigApi {
    name: String,
    functions: HashMap<String, AssemblyLiftServiceConfigApiFunction>
}

#[derive(Deserialize)]
struct AssemblyLiftServiceConfigApiFunction {
    name: String
}

pub fn compile(matches: Option<&ArgMatches>) {
    let matches = match matches {
        Some(matches) => matches,
        _ => panic!("could not get matches for compile")
    };

    // TODO 1) read assemblylift.toml to find functions ✅ 
    //      2) run appropriate build command for each function ✅ 
    //      3) aggregate build output for deployment

    let asml_config_contents = fs::read_to_string("./assemblylift.toml").unwrap();
    let asml_config: AssemblyLiftConfig = toml::from_str(&asml_config_contents).unwrap();
    for (_sid, service) in asml_config.services {
        let service_path = format!("./services/{}/service.toml", service.name);
        let service_config_contents = fs::read_to_string(service_path).unwrap();
        let service_config: AssemblyLiftServiceConfig = toml::from_str(&service_config_contents).unwrap();

        // TODO is this necessary? seems better to err to safety, I'm not sure what happens if these don't match
        if service.name != service_config.service.name {
            panic!("incorrect config; service names {}, {} do not match", service.name, service_config.service.name)
        }

        for (_fid, function) in service_config.api.functions {
            let function_path = format!("./services/{}/{}", service.name, function.name);
            let canonical_function_path = &fs::canonicalize(
                path::Path::new(&format!("{}/Cargo.toml", function_path))).unwrap();

            let mode = "release"; // TODO should this really be the default?

            let mut cargo_build = process::Command::new("cargo")
                .arg("build")
                .arg(format!("--{}", mode)) 
                .arg("--manifest-path")
                .arg(canonical_function_path)
                .arg("--target")
                .arg("wasm32-unknown-unknown")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .unwrap();

            match cargo_build.wait() {
                Ok(_) => {},
                Err(_) => {}
            }

            let function_name_snaked = function.name.replace("-", "_");
            let copy_result = fs::copy(
                format!("{}/target/wasm32-unknown-unknown/{}/{}.wasm", function_path, mode, function_name_snaked), 
                format!("{}/{}.wasm", function_path, function_name_snaked)
            );

            if copy_result.is_err() {
                println!("{:?}", copy_result.err());
            }
        }
    }
}
