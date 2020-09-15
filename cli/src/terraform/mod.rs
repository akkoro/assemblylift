use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path;
use std::path::PathBuf;
use std::process;
use std::process::Stdio;

use clap::crate_version;
use handlebars::{to_json, Handlebars};
use serde_derive::{Deserialize, Serialize};
use serde_json::value::{Map, Value as Json};

use crate::artifact;
use crate::projectfs;
use crate::templates;

fn get_relative_path() -> &'static str {
    ".asml/bin/terraform"
}

pub fn extract(canonical_project_path: &PathBuf) {
    use std::io::Read;

    // TODO check first if file is present

    let terraform_path = format!(
        "{}/{}",
        canonical_project_path.display(),
        get_relative_path()
    );
    println!("Extracting terraform to {}", terraform_path);

    let mut terraform_zip = Vec::new();

    #[cfg(target_os = "linux")]
    let mut response = reqwest::blocking::get(
        "https://releases.hashicorp.com/terraform/0.12.26/terraform_0.12.26_linux_amd64.zip",
    )
    .unwrap();
    #[cfg(target_os = "macos")]
    let mut response = reqwest::blocking::get(
        "https://releases.hashicorp.com/terraform/0.12.26/terraform_0.12.26_darwin_amd64.zip",
    )
    .unwrap();

    response.read_to_end(&mut terraform_zip);

    if let Err(_) = fs::create_dir_all(terraform_path.replace("/terraform", "")) {
        panic!("could not create directory ./.asml/bin")
    }

    artifact::unzip_to(terraform_zip, &terraform_path);

    let mut perms = fs::metadata(&terraform_path).unwrap().permissions();
    perms.set_mode(0o755);
    if let Err(_) = fs::set_permissions(&terraform_path, perms) {
        panic!("could not make terraform binary executable")
    }
}

pub fn run_terraform_init() {
    let mut terraform_result = process::Command::new(get_relative_path())
        .arg("init")
        .arg("./net")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    match terraform_result.wait() {
        Ok(_) => {}
        Err(_) => {}
    }
}

pub fn run_terraform_plan() {
    let mut terraform_result = process::Command::new(get_relative_path())
        .arg("plan")
        .arg("-out=./net/plan")
        .arg("./net")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    match terraform_result.wait() {
        Ok(_) => {}
        Err(_) => {}
    }
}

pub fn run_terraform_apply() {
    let mut terraform_result = process::Command::new(get_relative_path())
        .arg("apply")
        .arg("./net/plan")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    match terraform_result.wait() {
        Ok(_) => {}
        Err(_) => {}
    }
}

pub fn run_terraform_destroy() {
    let mut terraform_result = process::Command::new(get_relative_path())
        .arg("destroy")
        .arg("./net")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    match terraform_result.wait() {
        Ok(_) => {}
        Err(_) => {}
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TerraformFunction {
    pub name: String,
    pub handler_name: String,
    pub service: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TerraformService {
    pub name: String,
}

pub fn write_root_terraform(
    canonical_project_path: &PathBuf,
    functions: Vec<TerraformFunction>,
    services: Vec<TerraformService>,
) -> Result<(), io::Error> {
    let file_name = "main.tf";

    let mut reg = Handlebars::new();
    reg.register_template_string(file_name, templates::TERRAFORM_ROOT)
        .unwrap();

    let mut data = Map::<String, Json>::new();
    data.insert("asml_version".to_string(), to_json(crate_version!()));
    data.insert("aws_region".to_string(), to_json("us-east-1"));
    data.insert("functions".to_string(), to_json(functions));
    data.insert("services".to_string(), to_json(services));

    let render = reg.render(file_name, &data).unwrap();

    let path_str = &format!("{}/net/{}", canonical_project_path.display(), file_name);
    let path = path::Path::new(path_str);

    projectfs::write_to_file(&path, render)
}

pub fn write_service_terraform(
    canonical_project_path: &PathBuf,
    service: TerraformService,
) -> Result<(), io::Error> {
    let file_name = "service.tf";

    let mut reg = Handlebars::new();
    reg.register_template_string(file_name, templates::TERRAFORM_SERVICE)
        .unwrap(); // templates are known at compile-time

    let mut data = Map::<String, Json>::new();
    data.insert("asml_version".to_string(), to_json(crate_version!()));
    data.insert("name".to_string(), to_json(&service.name));

    let render = reg.render(file_name, &data).unwrap();

    let path = &format!(
        "{}/net/services/{}",
        canonical_project_path.display(),
        &service.name
    );

    fs::create_dir_all(path);

    let file_path = &format!("{}/{}", path, file_name);
    let file_path = path::Path::new(file_path);

    projectfs::write_to_file(&file_path, render)
}

pub fn write_function_terraform(
    canonical_project_path: &PathBuf,
    function: &TerraformFunction,
) -> Result<(), io::Error> {
    let file_name = "function.tf";

    let mut reg = Handlebars::new();
    reg.register_template_string(file_name, templates::TERRAFORM_FUNCTION)
        .unwrap(); // templates are known at compile-time

    let mut data = Map::<String, Json>::new();
    data.insert("asml_version".to_string(), to_json(crate_version!()));
    data.insert("name".to_string(), to_json(&function.name));
    data.insert("handler_name".to_string(), to_json(&function.handler_name));
    data.insert("service".to_string(), to_json(&function.service));

    let render = reg.render(file_name, &data).unwrap();

    let path_str = &format!(
        "{}/net/services/{}/{}/{}",
        canonical_project_path.display(),
        function.service,
        function.name,
        file_name
    );
    let path = path::Path::new(path_str);

    projectfs::write_to_file(&path, render)
}
