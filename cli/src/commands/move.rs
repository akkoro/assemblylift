use assemblylift_generator::{
    projectfs::{self, Project},
    toml::{asml, service},
};
use clap::ArgMatches;

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
            let old_dir = project.service_dir(old_name.clone()).dir().clone();
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

            // TODO this (find & read a service toml) is duped a few times;
            //      could be moved to a fn in ProjectFs
            let service_dir = project.service_dir(new_name.clone()).dir().clone();
            let mut service_manifest_file = service_dir.clone();
            service_manifest_file.push("service.toml");
            let mut service_manifest = service::Manifest::read(&service_manifest_file).unwrap();
            // service_manifest.rename(&*new_name.clone());
            service_manifest.write(service_dir.clone()).unwrap();
        }

        Some("function") => {
            let old_name = resource_from.unwrap().to_string();
            let old_name: Vec<&str> = old_name.split(".").collect();
            let old_service = old_name[0];
            let old_function = old_name[1];
            let old_dir = &project
                .service_dir(old_service.into())
                .function_dir(old_function.into());

            let new_name = resource_to.unwrap().to_string();
            let new_name: Vec<&str> = new_name.split(".").collect();
            let new_service = new_name[0];
            let new_function = new_name[1];
            let mut new_dir = old_dir.clone();
            new_dir.pop();
            new_dir.pop();
            new_dir.push(String::from(new_service));
            new_dir.push(String::from(new_function));

            std::fs::rename(old_dir, new_dir).unwrap();

            // rename function,
            // if old_service != new_service then remove_function from old and add_function to new
            let service_dir = project.service_dir(String::from(old_service)).dir().clone();
            let mut service_manifest_file = service_dir.clone();
            service_manifest_file.push("service.toml");
            let mut service_manifest = service::Manifest::read(&service_manifest_file).unwrap();
            service_manifest.rename_function(old_function, new_function);
            if old_service != new_service {
                let to_move = service_manifest
                    .functions
                    .clone()
                    .iter()
                    .find(|f| f.name == new_function)
                    .unwrap()
                    .clone();
                service_manifest.remove_function(new_function);

                let new_service_dir =
                    project.service_dir(String::from(new_service)).dir().clone();
                let mut new_service_manifest_file = new_service_dir.clone();
                new_service_manifest_file.push("service.toml");
                let mut new_service_manifest =
                    service::Manifest::read(&new_service_manifest_file).unwrap();
                let mut functions = Vec::new();
                functions.extend(new_service_manifest.functions.clone());
                functions.push(to_move);
                new_service_manifest.functions = functions;

                new_service_manifest.write(new_service_dir.clone()).unwrap()
            }
            service_manifest.write(service_dir.clone()).unwrap();
        }

        Some(_) => panic!("must specify either 'service' or 'function' as an argument to move"),
        None => panic!("must specify either 'service' or 'function' as an argument to move"),
    }
}
