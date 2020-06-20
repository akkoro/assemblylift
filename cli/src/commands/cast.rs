use std::fs;
use std::path;
use std::process;
use std::process::Stdio;
use std::collections::HashMap;

use clap::ArgMatches;
use serde_derive::Deserialize;

use crate::artifact;
use crate::terraform;
use crate::terraform::TerraformFunction;

// TODO these config structs belong in their own module
// TODO they also need to be less confusingly named, wtf am I doing?

// assemblylift.toml

#[derive(Deserialize)]
struct AssemblyLiftConfig {
    project: AssemblyLiftConfigProject,
    services: HashMap<String, AssemblyLiftConfigServices> // map service_id -> service
}

#[derive(Deserialize)]
struct AssemblyLiftConfigProject {
    name: String
}

#[derive(Deserialize)]
struct AssemblyLiftConfigServices {
    name: String
}

// service.toml

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
    functions: HashMap<String, AssemblyLiftServiceConfigApiFunction> // map function_id -> function
}

#[derive(Deserialize)]
struct AssemblyLiftServiceConfigApiFunction {
    name: String,
    handler_name: String
}

pub fn command(matches: Option<&ArgMatches>) {
    let matches = match matches {
        Some(matches) => matches,
        _ => panic!("could not get matches for cast command")
    };

    

    // Compile function source
    // This currently assumes the language is Rust

    let asml_config_contents = match fs::read_to_string("./assemblylift.toml") {
        Ok(contents) => contents,
        Err(why) => panic!("could not read assemblylift.toml: {}", why.to_string())
    };
    
    let asml_config: AssemblyLiftConfig = match toml::from_str(&asml_config_contents) {
        Ok(config) => config,
        Err(why) => panic!("could not parse assemblylift.toml: {}", why.to_string())
    };

    let canonical_project_path = match fs::canonicalize(path::Path::new("./")) {
        Ok(path) => path,
        Err(why) => panic!("unable to build canonical project path: {}", why.to_string())
    };

    terraform::write_root_terraform(&canonical_project_path);

    for (_sid, service) in asml_config.services {
        let service_path = format!("./services/{}/service.toml", service.name);
        let service_config_contents = fs::read_to_string(service_path).unwrap();
        let service_config: AssemblyLiftServiceConfig = toml::from_str(&service_config_contents).unwrap();

        // TODO is this necessary? seems better to err to safety, I'm not sure what happens if these don't match
        if service.name != service_config.service.name {
            panic!("incorrect config; service names {}, {} do not match", service.name, service_config.service.name)
        }

        for (_fid, function) in service_config.api.functions {
            let function_artifact_path = format!("./net/services/{}/{}", &service.name, function.name);
            if let Err(err) = fs::create_dir_all(path::Path::new(&function_artifact_path)) {
                panic!(err)
            }
            
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
                format!("{}/{}.wasm", function_artifact_path, &function.name)
            );

            if copy_result.is_err() {
                println!("{:?}", copy_result.err());
            }

            artifact::zip_files(vec![path::Path::new(&format!("{}/{}.wasm", function_artifact_path, &function.name))], 
                                &path::Path::new(&format!("{}/{}.zip", function_artifact_path, &function.name)));

            terraform::write_function_terraform(&canonical_project_path, &service.name, &TerraformFunction {
                name: function.name.clone(),
                handler_name: function.handler_name
            });

            terraform::run_terraform_init();
            terraform::run_terraform_plan();
        }
    }
}
