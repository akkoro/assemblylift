use clap::ArgMatches;

use crate::projectfs::{locate_asml_manifest, Project};
use crate::transpiler::toml::{asml, service};

pub fn command(matches: Option<&ArgMatches>) {
    let matches = match matches {
        Some(matches) => matches,
        _ => panic!("could not get matches for make command"),
    };

    let manifest = match locate_asml_manifest() {
        Some(manifest) => manifest,
        None => panic!("could not find assemblylift.toml in tree"),
    };
    let mut manifest_dir = manifest.1.clone();
    manifest_dir.pop();

    let project = Project::new(manifest.0.project.name.clone(), Some(manifest_dir.clone()));

    let mut resource_type: Option<&str> = None;
    let mut resource_from: Option<&str> = None;
    let mut resource_to: Option<&str> = None;
    for el in matches.values_of("resource").unwrap() {
        if resource_type.is_none() {
            resource_type = Some(el);
            continue;
        }
        if resource_from.is_none() {
            resource_from = Some(el);
            continue;
        }
        if resource_to.is_none() {
            resource_to = Some(el);
            continue;
        }
    }

    match resource_type {
        Some("service") => {
            let old_name = resource_from.unwrap().to_string();
            let old_dir = &*project.service_dir(old_name.clone()).dir().clone();
            let new_name = resource_to.unwrap().to_string();
            let mut new_dir = old_dir.clone();
            new_dir.pop();
            new_dir.push(new_name.clone());
            std::fs::rename(old_dir, new_dir).unwrap();

            let mut manifest: asml::Manifest = manifest.0.clone();
            manifest.rename_service(&*old_name.clone(), &*new_name.clone());
            manifest
                .write(manifest_dir.clone())
                .expect("could not write assemblylift.toml");

            let service_dir = &*project.service_dir(new_name.clone()).dir().clone();
            let mut service_manifest_file = service_dir.clone();
            service_manifest_file.push("service.toml");
            let mut service_manifest = service::Manifest::read(&service_manifest_file).unwrap();
            service_manifest.rename(&*new_name.clone());
            service_manifest.write(service_dir.clone()).unwrap();
        }
        Some("function") => unimplemented!("function move is not yet implemented"),

        Some(_) => panic!("must specify either 'service' or 'function' as an argument to move"),
        None => panic!("must specify either 'service' or 'function' as an argument to move"),
    }
}