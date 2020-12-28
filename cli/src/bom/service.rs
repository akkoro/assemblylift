use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use clap::crate_version;
use handlebars::to_json;
use serde::Deserialize;
use serde_json::value::{Map, Value as Json};

use crate::bom::{write_documents, Document, DocumentSet};
use std::rc::Rc;

static SERVICE_TOML: &str = r#"# Generated with assemblylift-cli {{asml_version}}

[service]
name = "{{service_name}}"
version = ""

[api]
name = "{{service_name}}-api"

[api.functions.my-function]
name = "my-function"
handler_name = "handler"
"#;

#[derive(Deserialize)]
pub struct Manifest {
    pub service: Service,
    pub api: Api,
    pub iomod: Option<Iomod>,
}

#[derive(Deserialize)]
pub struct Service {
    pub name: String,
}

#[derive(Deserialize)]
pub struct Api {
    pub name: String,
    pub functions: HashMap<String, Function>, // map function_id -> function
}

#[derive(Deserialize)]
pub struct JwtAuth;

#[derive(Deserialize)]
pub struct HttpAuth {
    pub auth_type: String,
    pub jwt: Option<JwtAuth>,
}

#[derive(Deserialize)]
pub struct HttpFunction {
    pub verb: String,
    pub path: String,
}

#[derive(Deserialize)]
pub struct Function {
    pub name: String,
    pub handler_name: String,

    pub http: Rc<Option<HttpFunction>>,
    pub http_auth: Option<HttpAuth>,
}

#[derive(Deserialize)]
pub struct Iomod {
    pub dependencies: HashMap<String, Dependency>, // map dependency_id -> dependency
}

#[derive(Clone, Deserialize)]
pub struct Dependency {
    pub from: String,
    pub version: String,
    #[serde(alias = "type")]
    pub dependency_type: String,
}

impl DocumentSet<'_, Manifest> for Manifest {
    fn file_names() -> Vec<Document> {
        Vec::from([Document {
            file_name: "service.toml",
            document: String::from(SERVICE_TOML),
        }])
    }

    fn read(path: &PathBuf) -> Manifest {
        let mut path = PathBuf::from(path);
        path.push(Manifest::file_names()[0].file_name);

        let service_config_contents = fs::read_to_string(path).unwrap();
        let service_config: Manifest = toml::from_str(&service_config_contents).unwrap();

        service_config
    }

    fn write(path: &PathBuf, data: &mut Map<String, Json>) {
        data.insert("asml_version".to_string(), to_json(crate_version!()));
        write_documents(path, Manifest::file_names(), data)
    }
}
