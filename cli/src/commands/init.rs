use std::process;

use assemblylift_generator::projectfs::Project;
use clap::ArgMatches;
use handlebars::to_json;
use serde_json::value::{Map, Value as Json};

use crate::templates::project::{ROOT_DOCUMENTS, SERVICE_DOCUMENTS};
use crate::templates::write_documents;

pub fn command(matches: Option<&ArgMatches>) {
    let matches = match matches {
        Some(matches) => matches,
        _ => panic!("could not get matches for init command"),
    };

    let default_service_name = "my-service";
    let default_function_name = "my-function";
    let project_name = matches.value_of("project_name").unwrap();
    // let function_language = matches.value_of("language").unwrap();

    let project = Project::new(project_name.parse().unwrap(), None);

    {
        let data = &mut Map::<String, Json>::new();
        data.insert(
            "project_name".to_string(),
            to_json(project_name.to_string()),
        );
        data.insert(
            "default_service_name".to_string(),
            to_json(default_service_name.to_string()),
        );
        write_documents(&project.dir(), (*ROOT_DOCUMENTS).clone().as_ref(), data);
    }

    {
        let data = &mut Map::<String, Json>::new();
        data.insert(
            "service_name".to_string(),
            to_json(default_service_name.to_string()),
        );
        // data.insert(
        //     "function_language".to_string(),
        //     to_json(function_language.to_string()),
        // );
        write_documents(
            &project
                .service_dir(String::from(default_service_name))
                .dir(),
            (*SERVICE_DOCUMENTS).clone().as_ref(),
            data,
        );
    }

    // match function_language {
    //     "rust" => {
    //         assert_prereqs();
    //
    //         std::fs::create_dir_all(format!(
    //             "{}/src",
    //             project
    //                 .service_dir(String::from(default_service_name))
    //                 .function_dir(String::from(default_function_name))
    //                 .to_str()
    //                 .unwrap()
    //         ))
    //         .unwrap();
    //
    //         let data = &mut Map::<String, Json>::new();
    //         data.insert(
    //             "function_name".to_string(),
    //             to_json(default_function_name.to_string()),
    //         );
    //         write_documents(
    //             &project
    //                 .service_dir(String::from(default_service_name))
    //                 .function_dir(String::from(default_function_name)),
    //             (*RUST_FUNCTION_DOCUMENTS).clone().as_ref(),
    //             data,
    //         );
    //     }
    //     "ruby" => {
    //         write_documents(
    //             &project
    //                 .service_dir(String::from(default_service_name))
    //                 .function_dir(String::from(default_function_name)),
    //             (*RUBY_FUNCTION_DOCUMENTS).clone().as_ref(),
    //             &mut Map::<String, Json>::new(),
    //         );
    //     }
    //     unknown => panic!("unsupported language: {}", unknown),
    // }

    println!("\r\n✅  Done! Your project root is: {:?}", project.dir());
}

fn check_rust_prereqs() -> bool {
    let cargo_version = process::Command::new("cargo").arg("--version").output();

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
