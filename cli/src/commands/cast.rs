use std::fs;
use std::path;
use std::process;
use std::process::Stdio;

use clap::ArgMatches;

use crate::artifact;
use crate::bom;
use crate::terraform;
use crate::terraform::{TerraformFunction, TerraformService};

macro_rules! path {
    ($str:tt) => {
        path::Path::new(&$str)
    };
    ($str:expr) => {
        path::Path::new(&$str)
    };
}

pub fn command(matches: Option<&ArgMatches>) {
    use std::io::Read;

    let _matches = match matches {
        Some(matches) => matches,
        _ => panic!("could not get matches for cast command"),
    };

    // Download the latest runtime binary
    // TODO in the future we should check if we already have the same version
    // TODO argument to specify which version -- default to 'latest'
    let mut response = reqwest::blocking::get(
        "http://runtime.assemblylift.akkoro.io/aws-lambda/xlem/bootstrap.zip",
    )
    .unwrap();
    let mut response_buffer = Vec::new();
    response.read_to_end(&mut response_buffer);

    fs::create_dir_all("./.asml/runtime");
    fs::write("./.asml/runtime/bootstrap.zip", response_buffer);

    // Compile function source
    // This currently assumes the language is Rust

    let asml_manifest = bom::manifest::read();

    let canonical_project_path = match fs::canonicalize(path!("./")) {
        Ok(path) => path,
        Err(why) => panic!(
            "unable to build canonical project path: {}",
            why.to_string()
        ),
    };

    terraform::extract(&canonical_project_path);

    let mut functions: Vec<TerraformFunction> = Vec::new();
    let mut services: Vec<TerraformService> = Vec::new();

    for (_, service) in asml_manifest.services {
        let service_name = service.name.clone();
        let service_manifest = bom::service::read(&service_name);

        let tf_service = TerraformService {
            name: service_name.clone(),
        };
        services.push(tf_service.clone());
        terraform::write_service_terraform(&canonical_project_path, tf_service).unwrap();

        let mut dependencies: Vec<String> = Vec::new();
        for (name, dependency) in service_manifest.iomod.dependencies {
            match dependency.dependency_type.as_str() {
                "file" => {
                    // copy file & rename it to `name`

                    let dependency_name = name.clone();
                    let dependency_path = dependency.from.clone();

                    let runtime_path = format!("./.asml/runtime/{}", dependency_name);
                    fs::copy(dependency_path, &runtime_path).unwrap();

                    dependencies.push(runtime_path);
                }
                _ => unimplemented!("only type=file is available currently"),
            }
        }

        artifact::zip_files(
            dependencies,
            format!("./.asml/runtime/{}.zip", &service_name),
            Some("iomod/"),
            false,
        );

        for (_id, function) in service_manifest.api.functions {
            let function_artifact_path =
                format!("./net/services/{}/{}", &service_name, function.name);
            if let Err(err) = fs::create_dir_all(path!(function_artifact_path)) {
                panic!(err)
            }

            let function_path = format!("./services/{}/{}", service_name, function.name);
            let canonical_function_path =
                &fs::canonicalize(path!(format!("{}/Cargo.toml", function_path))).unwrap();

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
                Ok(_) => {}
                Err(_) => {}
            }

            let function_name_snaked = function.name.replace("-", "_");
            let copy_result = fs::copy(
                format!(
                    "{}/target/wasm32-unknown-unknown/{}/{}.wasm",
                    function_path, mode, function_name_snaked
                ),
                format!("{}/{}.wasm", function_artifact_path, &function.name),
            );

            if copy_result.is_err() {
                println!("{:?}", copy_result.err());
            }

            let wasm_path = format!("{}/{}.wasm", function_artifact_path, &function.name);

            artifact::zip_files(
                vec![wasm_path],
                format!("{}/{}.zip", function_artifact_path, &function.name),
                None,
                false,
            );

            let tf_function = TerraformFunction {
                name: function.name.clone(),
                handler_name: function.handler_name,
                service: service.name.clone(),
            };

            terraform::write_function_terraform(&canonical_project_path, &tf_function).unwrap();
            functions.push(tf_function.clone());
        }
    }

    terraform::write_root_terraform(&canonical_project_path, functions, services).unwrap();

    terraform::run_terraform_init();
    terraform::run_terraform_plan();
}
