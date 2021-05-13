use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use clap::crate_version;
use handlebars::to_json;
use serde::Deserialize;
use serde_json::{Map, Value as Json};

use crate::bom::{write_documents, Document, DocumentSet};

static ROOT_GITIGNORE: &str = r#".asml/
net/
"#;

static ASSEMBLYLIFT_TOML: &str = r#"# Generated with assemblylift-cli {{asml_version}}

[project]
name = "{{project_name}}"
version = "0.1.0"

[services]
default = { name = "{{default_service_name}}" }
"#;

#[derive(Deserialize)]
pub struct Manifest {
    pub project: Project,
    pub services: HashMap<String, Service>, // map service_id -> service
}

#[derive(Deserialize)]
pub struct Project {
    pub name: String,
}

#[derive(Deserialize)]
pub struct Service {
    pub name: String,
}

impl DocumentSet<'_, Manifest> for Manifest {
    fn file_names() -> Vec<Document> {
        Vec::from([
            Document {
                file_name: "assemblylift.toml",
                document: String::from(ASSEMBLYLIFT_TOML),
            },
            Document {
                file_name: ".gitignore",
                document: String::from(ROOT_GITIGNORE),
            },
        ])
    }

    fn read(path: &PathBuf) -> Manifest {
        let mut path = PathBuf::from(path);
        path.push(Manifest::file_names()[0].file_name);

        let manifest_contents = match fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(why) => panic!("could not read assemblylift.toml: {}", why.to_string()),
        };

        let manifest: Manifest = match toml::from_str(&manifest_contents) {
            Ok(config) => config,
            Err(why) => panic!("could not parse assemblylift.toml: {}", why.to_string()),
        };

        manifest
    }

    fn write(path: &PathBuf, data: &mut Map<String, Json>) {
        data.insert("asml_version".to_string(), to_json(crate_version!()));
        write_documents(path, Manifest::file_names(), data)
    }
}
