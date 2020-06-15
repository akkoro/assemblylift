use std::{fs, io, path};
use std::io::Write;
use std::path::PathBuf;

use clap::crate_version;
use handlebars::{Handlebars, to_json};
use serde_json::value::{Map, Value as Json};

use crate::templates;

pub fn initialize_project_directories(project_name: &str, default_service_name: &str, default_function_name: &str) -> Result<(), io::Error> {
    fs::create_dir(path::Path::new(&format!("./{}", project_name)));
    fs::create_dir(path::Path::new(&format!("./{}/services", project_name)));
    fs::create_dir(path::Path::new(&format!("./{}/services/{}",
                                            project_name, default_service_name)));
    fs::create_dir(path::Path::new(&format!("./{}/services/{}/{}",
                                            project_name, default_service_name, default_function_name)));
    fs::create_dir(path::Path::new(&format!("./{}/services/{}/{}/src",
                                           project_name, default_service_name, default_function_name)));
    fs::create_dir(path::Path::new(&format!("./{}/services/{}/{}/.cargo",
                                            project_name, default_service_name, default_function_name)));

    Ok(())
}

pub fn write_project_manifest(canonical_project_path: &PathBuf, project_name: &str, default_service_name: &str) -> Result<(), io::Error> {
    let file_name = "assemblylift.toml";

    let mut reg = Handlebars::new();
    reg.register_template_string(file_name, templates::ASSEMBLYLIFT_TOML).unwrap();

    let mut data = Map::<String, Json>::new();
    data.insert("project_name".to_string(), to_json(project_name.to_string()));
    data.insert("default_service_name".to_string(), to_json(default_service_name.to_string()));
    data.insert("asml_version".to_string(), to_json(crate_version!()));

    let render = reg.render(file_name, &data).unwrap();

    let path_str = &format!("{}/assemblylift.toml", canonical_project_path.display());
    let path = path::Path::new(path_str);
    write_to_file(&path, render);

    Ok(())
}

pub fn write_service_manifest(canonical_project_path: &PathBuf, service_name: &str) -> Result<(), io::Error> {
    let file_name = "service.toml";

    let mut reg = Handlebars::new();
    reg.register_template_string(file_name, templates::SERVICE_TOML).unwrap();

    let mut data = Map::<String, Json>::new();
    data.insert("service_name".to_string(), to_json(service_name.to_string()));
    data.insert("asml_version".to_string(), to_json(crate_version!()));

    let render = reg.render(file_name, &data).unwrap();

    let path_str = &format!("{}/services/{}/service.toml",
                            canonical_project_path.display(),
                            service_name);
    let path = path::Path::new(path_str);
    write_to_file(&path, render);

    Ok(())
}

pub fn write_function_manifest(canonical_project_path: &PathBuf, service_name: &str, function_name: &str) -> Result<(), io::Error> {
    let file_name = "Cargo.toml";

    let mut reg = Handlebars::new();
    reg.register_template_string(file_name, templates::FUNCTION_CARGO_TOML).unwrap();

    let mut data = Map::<String, Json>::new();
    data.insert("function_name".to_string(), to_json(function_name.to_string()));
    data.insert("asml_version".to_string(), to_json(crate_version!()));

    let render = reg.render(file_name, &data).unwrap();

    let path_str = &format!("{}/services/{}/{}/Cargo.toml",
                            canonical_project_path.display(),
                            service_name, function_name);
    let path = path::Path::new(path_str);
    write_to_file(&path, render);

    Ok(())
}

pub fn write_function_cargo_config(canonical_project_path: &PathBuf, service_name: &str, function_name: &str) -> Result<(), io::Error> {
    let file_name = "config";

    let mut reg = Handlebars::new();
    reg.register_template_string(file_name, templates::FUNCTION_CARGO_CONFIG).unwrap();

    let mut data = Map::<String, Json>::new();
    data.insert("asml_version".to_string(), to_json(crate_version!()));

    let render = reg.render(file_name, &data).unwrap();

    let path_str = &format!("{}/services/{}/{}/.cargo/config",
                            canonical_project_path.display(),
                            service_name, function_name);
    let path = path::Path::new(path_str);
    write_to_file(&path, render);

    Ok(())
}

pub fn write_function_lib(canonical_project_path: &PathBuf, service_name: &str, function_name: &str) -> Result<(), io::Error> {
    let file_name = "lib.rs";

    let mut reg = Handlebars::new();
    reg.register_template_string(file_name, templates::FUNCTION_LIB_RS).unwrap();

    let mut data = Map::<String, Json>::new();
    data.insert("asml_version".to_string(), to_json(crate_version!()));

    let render = reg.render(file_name, &data).unwrap();

    let path_str = &format!("{}/services/{}/{}/src/lib.rs",
                            canonical_project_path.display(),
                            service_name, function_name);
    let path = path::Path::new(path_str);
    write_to_file(&path, render);

    Ok(())
}

pub fn write_function_gitignore(canonical_project_path: &PathBuf, service_name: &str, function_name: &str) -> Result<(), io::Error> {
    let file_name = ".gitignore";

    let mut reg = Handlebars::new();
    reg.register_template_string(file_name, templates::FUNCTION_GITIGNORE).unwrap();

    let mut data = Map::<String, Json>::new();
    data.insert("asml_version".to_string(), to_json(crate_version!()));

    let render = reg.render(file_name, &data).unwrap();

    let path_str = &format!("{}/services/{}/{}/.gitignore",
                            canonical_project_path.display(),
                            service_name, function_name);
    let path = path::Path::new(path_str);
    write_to_file(&path, render);

    Ok(())
}

fn write_to_file(path: &path::Path, contents: String) -> Result<(), io::Error> {
    let mut file = match fs::File::create(path) {
        Err(why) => panic!("couldn't create file {}: {}", path.display(), why.to_string()),
        Ok(file) => file
    };

    println!("📄 Wrote {}", path.display());
    file.write_all(contents.as_bytes())
}
