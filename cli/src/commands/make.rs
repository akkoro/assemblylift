use clap::ArgMatches;
use handlebars::to_json;
use serde_json::value::{Map, Value as Json};
use std::rc::Rc;

use crate::projectfs::{locate_asml_manifest, Project};
use crate::templates::project::{
    RUBY_FUNCTION_DOCUMENTS, RUST_FUNCTION_DOCUMENTS, SERVICE_DOCUMENTS,
};
use crate::templates::write_documents;
use crate::transpiler::toml::{asml, service};
use crate::transpiler::toml::asml::ServiceRef;

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
    let mut resource_name: Option<&str> = None;
    for el in matches.values_of("resource").unwrap() {
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
            let data = &mut Map::<String, Json>::new();
            data.insert(
                "service_name".to_string(),
                to_json(resource_name.unwrap().to_string()),
            );
            let path = project
                .service_dir(String::from(resource_name.unwrap()))
                .dir();
            write_documents(&path, (*SERVICE_DOCUMENTS).clone().as_ref(), data);

            let mut manifest: asml::Manifest = manifest.0.clone();
            manifest.add_service(resource_name.unwrap());
            manifest.write(manifest_dir.clone()).expect("could not write assemblylift.toml");
        }

        Some("function") => {
            let language = matches.value_of("language").unwrap_or("rust");
            let resource_name = resource_name.unwrap().to_string();
            let function_name: Vec<&str> = resource_name.split(".").collect();
            if function_name.len() != 2 {
                panic!("syntax is `make function <service>.<function>`")
            }

            let service_dir = &*project.service_dir(function_name[0].into()).dir().clone();
            let mut manifest_file = service_dir.clone();
            manifest_file.push("service.toml");
            let mut service_manifest = service::Manifest::read(&manifest_file).unwrap();
            service_manifest.add_function(function_name[1], language);
            service_manifest.write(service_dir.clone()).unwrap();

            match language {
                "rust" => {
                    let data = &mut Map::<String, Json>::new();
                    data.insert("function_name".to_string(), to_json(function_name[1]));
                    let path = project
                        .service_dir(String::from(function_name[0]))
                        .function_dir(String::from(function_name[1]));
                    write_documents(&path, (*RUST_FUNCTION_DOCUMENTS).clone().as_ref(), data);
                }
                "ruby" => {
                    let path = project
                        .service_dir(String::from(function_name[0]))
                        .function_dir(String::from(function_name[1]));
                    write_documents(
                        &path,
                        (*RUBY_FUNCTION_DOCUMENTS).clone().as_ref(),
                        &mut Map::<String, Json>::new(),
                    );
                }
                lang => panic!("function language `{}` is not supported", lang),
            }
        }

        Some(_) => panic!("must specify either 'service' or 'function' as an argument to make"),
        None => panic!("must specify either 'service' or 'function' as an argument to make"),
    }
}
