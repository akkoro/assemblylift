use std::process;

use clap::ArgMatches;
use handlebars::{to_json, Handlebars};
use serde_json::value::{Map, Value as Json};

pub fn init(matches: Option<&ArgMatches>) {
    if let Some(matches) = matches {
        match matches.value_of("language") {
            Some("rust") => {
                assert_prereqs();

                let project_name = matches.value_of("project_name").unwrap();

                let mut reg = Handlebars::new();
                reg.register_template_string("assemblylift.toml",
                                             templates::ASSEMBLYLIFT_TOML).unwrap();

                let mut assemblylift_toml_data = Map::<String, Json>::new();
                assemblylift_toml_data.insert("project_name".to_string(),
                                              to_json(project_name.to_string()));
                let assemblylift_toml_render = reg.render("assemblylift.toml",
                                                          &assemblylift_toml_data).unwrap();
                println!("{}", assemblylift_toml_render)
            }
            Some(unknown) => panic!("unsupported language: {}", unknown),
            _ => {}
        }
    }
}

fn check_prereqs() -> bool {
    let cargo_version = process::Command::new("cargo")
        .arg("--version")
        .output();

    match cargo_version {
        Ok(version) => {
            println!("Found Cargo: {}", String::from_utf8_lossy(&version.stdout));
            true
        },
        Err(_) => {
            println!("ERROR: missing Cargo!");
            false
        }
    }
}

fn assert_prereqs() {
    if !check_prereqs() {
        panic!("missing system dependencies")
    }
}

mod templates {
    pub(crate) static ASSEMBLYLIFT_TOML: &str =
r"[project]
name = {{project_name}}
version = 0.1.0
";
}
