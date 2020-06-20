use std::fs;
use std::path;
use std::process;

use clap::ArgMatches;

use crate::projectfs;
use crate::terraform;

pub fn command(matches: Option<&ArgMatches>) {
    let matches = match matches {
        Some(matches) => matches,
        _ => panic!("could not get matches for init command")
    };

    let default_service_name = "my-service";
    let default_function_name = "my-function";
    let project_name = matches.value_of("project_name").unwrap();

    projectfs::initialize_project_directories(project_name,
                                              default_service_name,
                                              default_function_name).unwrap();

    let canonical_project_path =
        &fs::canonicalize(path::Path::new(&format!("./{}", project_name)))
            .unwrap();

    terraform::extract(canonical_project_path);

    projectfs::write_project_gitignore(canonical_project_path, project_name, default_service_name).unwrap();
    projectfs::write_project_manifest(canonical_project_path, project_name, default_service_name).unwrap();
    projectfs::write_service_manifest(canonical_project_path, default_service_name).unwrap();

    match matches.value_of("language") {
        Some("rust") => {
            assert_prereqs();

            projectfs::write_function_manifest(canonical_project_path,
                                               default_service_name,
                                               default_function_name).unwrap();
            projectfs::write_function_cargo_config(canonical_project_path,
                                                   default_service_name,
                                                   default_function_name).unwrap();
            projectfs::write_function_lib(canonical_project_path,
                                          default_service_name,
                                          default_function_name).unwrap();
            projectfs::write_function_gitignore(canonical_project_path, 
                                                default_service_name, 
                                                default_function_name).unwrap();
        }
        Some(unknown) => panic!("unsupported language: {}", unknown),
        _ => {}
    }

    println!("\r\n✅  Done! Your project root is: {}", canonical_project_path.display())
}

fn check_rust_prereqs() -> bool {
    let cargo_version = process::Command::new("cargo")
        .arg("--version")
        .output();

    match cargo_version {
        Ok(_version) => true,
        Err(_) => {
            println!("ERROR: missing Cargo!");
            false
        }
    }
}

fn assert_prereqs() {
    if !check_rust_prereqs() {
        panic!("missing system dependencies")
    }
}
