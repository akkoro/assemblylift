use std::fs;
use std::io;
use std::path;
use std::path::PathBuf;
use std::process;
use std::process::Stdio;
use std::os::unix::fs::PermissionsExt;

use clap::crate_version;
use handlebars::{Handlebars, to_json};
use serde_derive::{Serialize, Deserialize};
use serde_json::value::{Map, Value as Json};

use crate::templates;
use crate::projectfs;

fn get_relative_path() -> &'static str {
    "./.asml/bin/terraform"
}

pub fn extract() {
    // TODO check first if file is present

    let terraform_path = get_relative_path();
    let terraform: &'static [u8] = include_bytes!("../../resources/bin/linux64/terraform");
    if let Err(_) = fs::create_dir_all(terraform_path.replace("/terraform", ""))
        .and_then(|_| fs::write(terraform_path, terraform)) 
    {
        panic!("could not copy terraform binary to ./.asml/bin")
    }

    let mut perms = fs::metadata(terraform_path).unwrap().permissions();
    perms.set_mode(0o755);
    if let Err(_) = fs::set_permissions(terraform_path, perms) {
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
        Ok(_) => {},
        Err(_) => {}
    }
}

// TODO return Result
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
        Ok(_) => {},
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
        Ok(_) => {},
        Err(_) => {}
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TerraformFunction {
    pub name: String,
    pub handler_name: String,
    pub service: String
}

pub fn write_root_terraform(canonical_project_path: &PathBuf, functions: Vec<TerraformFunction>) 
    -> Result<(), io::Error> 
{
    let file_name = "main.tf";

    let mut reg = Handlebars::new();
    reg.register_template_string(file_name, templates::TERRAFORM_ROOT).unwrap();

    let mut data = Map::<String, Json>::new();
    data.insert("asml_version".to_string(), to_json(crate_version!()));
    data.insert("aws_region".to_string(), to_json("us-east-1"));
    data.insert("functions".to_string(), to_json(functions));

    let render = reg.render(file_name, &data).unwrap();

    let path_str = &format!("{}/net/{}", canonical_project_path.display(), file_name);
    let path = path::Path::new(path_str);

    projectfs::write_to_file(&path, render)
}

pub fn write_function_terraform(canonical_project_path: &PathBuf, function: &TerraformFunction) 
    -> Result<(), io::Error> 
{
    let file_name = "function.tf";

    let mut reg = Handlebars::new();
    reg.register_template_string(file_name, templates::TERRAFORM_FUNCTION).unwrap();

    let mut data = Map::<String, Json>::new();
    data.insert("asml_version".to_string(), to_json(crate_version!()));
    data.insert("name".to_string(), to_json(&function.name));
    data.insert("handler_name".to_string(), to_json(&function.handler_name));
    data.insert("service".to_string(), to_json(&function.service));

    let render = reg.render(file_name, &data).unwrap();

    let path_str = &format!("{}/net/services/{}/{}/{}", canonical_project_path.display(), function.service, function.name, file_name);
    let path = path::Path::new(path_str);

    projectfs::write_to_file(&path, render)
}
