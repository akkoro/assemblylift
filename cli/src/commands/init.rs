use std::process;
use std::fs;
use std::path;
use std::io;

use clap::{crate_version, ArgMatches};
use handlebars::{to_json, Handlebars};
use serde_json::value::{Map, Value as Json};
use std::error::Error;
use std::io::Write;
use std::path::PathBuf;

pub fn init(matches: Option<&ArgMatches>) {
    let matches = match matches {
        Some(matches) => matches,
        _ => panic!("could not get matches for init")
    };

    match matches.value_of("language") {
        Some("rust") => {
            assert_prereqs();

            let project_name = matches.value_of("project_name").unwrap();
            fs::create_dir(path::Path::new(&format!("./{}", project_name)));
            fs::create_dir(path::Path::new(&format!("./{}/services", project_name)));

            let canonical_project_path =
                &fs::canonicalize(path::Path::new(&format!("./{}", project_name)))
                    .unwrap();

            write_manifest(canonical_project_path, project_name).unwrap();
        }
        Some(unknown) => panic!("unsupported language: {}", unknown),
        _ => {}
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

fn write_to_file(path: &path::Path, contents: String) -> Result<(), io::Error> {
    let mut file = match fs::File::create(path) {
        Err(why) => panic!("couldn't create file {}: {}", path.display(), why.to_string()),
        Ok(file) => file
    };

    file.write_all(contents.as_bytes())
}

fn write_manifest(canonical_project_path: &PathBuf, project_name: &str) -> Result<(), io::Error> {
    let mut reg = Handlebars::new();
    reg.register_template_string("assemblylift.toml",
                                 templates::ASSEMBLYLIFT_TOML).unwrap();

    let mut assemblylift_toml_data = Map::<String, Json>::new();
    assemblylift_toml_data.insert("project_name".to_string(),
                                  to_json(project_name.to_string()));
    assemblylift_toml_data.insert("asml_version".to_string(), to_json(crate_version!()));

    let assemblylift_toml_render = reg.render("assemblylift.toml",
                                              &assemblylift_toml_data).unwrap();

    let path_str = &format!("{}/assemblylift.toml", canonical_project_path.display());
    let path = path::Path::new(path_str);
    write_to_file(&path, assemblylift_toml_render);
    println!("Wrote {}", path_str);

    Ok(())
}

mod templates {
    pub(crate) static ASSEMBLYLIFT_TOML: &str =
r#"# Generated with assemblylift-cli {{asml_version}}

[project]
name = "{{project_name}}"
version = "0.1.0"
"#;
}
