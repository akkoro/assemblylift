use assemblylift_generator::{
    projectfs::{self, Project},
    toml::{asml, service},
};
use clap::ArgMatches;
use dialoguer::Confirm;

pub fn command(matches: Option<&ArgMatches>) {
    let matches = match matches {
        Some(matches) => matches,
        _ => panic!("could not get matches for make command"),
    };

    let manifest = match projectfs::locate_asml_manifest() {
        Some(manifest) => manifest,
        None => panic!("could not find assemblylift.toml in tree"),
    };
    let mut manifest_dir = manifest.1.clone();
    manifest_dir.pop();

    let project = Project::new(manifest.0.project.name.clone(), Some(manifest_dir.clone()));

    let mut resource_type: Option<&str> = None;
    let mut resource_name: Option<&str> = None;
    for el in matches
        .values_of("resource")
        .expect("must specify either 'service', 'function' as an argument to burn")
    {
        if resource_type.is_none() {
            resource_type = Some(el);
            continue;
        }
        if resource_name.is_none() {
            resource_name = Some(el);
            continue;
        }
    }

    match resource_type {
        Some("service") => {
            let service_name = resource_name.unwrap();
            if Confirm::new()
                .with_prompt(format!(
                    "Are you sure you want to destroy service \"{}\"?\nThis is PERMANENT!",
                    service_name
                ))
                .interact()
                .unwrap()
            {
                let mut manifest: asml::Manifest = manifest.0.clone();
                manifest.remove_service(service_name);
                manifest
                    .write(manifest_dir.clone())
                    .expect("could not write assemblylift.toml");

                let mut service_dir = manifest_dir.clone();
                service_dir.push("services");
                service_dir.push(service_name);
                std::fs::remove_dir_all(service_dir).expect("could not remove service directory");
            }
        }

        Some("function") => {
            let resource_name = resource_name.unwrap().to_string();
            let function_name: Vec<&str> = resource_name.split(".").collect();
            if function_name.len() != 2 {
                panic!("syntax is `burn function <service>.<function>`")
            }

            if Confirm::new()
                .with_prompt(format!(
                    "Are you sure you want to destroy function \"{}\"\nThis is PERMANENT!",
                    resource_name
                ))
                .interact()
                .unwrap()
            {
                let service_dir = project.service_dir(function_name[0].into()).dir().clone();
                let mut manifest_file = service_dir.clone();
                manifest_file.push("service.toml");
                let mut service_manifest = service::Manifest::read(&manifest_file).unwrap();
                service_manifest.remove_function(function_name[1]);
                service_manifest.write(service_dir.clone()).unwrap();
                std::fs::remove_dir_all(
                    &*project
                        .service_dir(function_name[0].into())
                        .function_dir(function_name[1].into()),
                )
                .expect("could not remove service directory");
            }
        }

        Some(_) => {
            panic!("must specify either 'service', or 'function', as an argument to burn")
        }
        None => panic!("must specify either 'service', or 'function' as an argument to burn"),
    }
}
