use std::io::Read;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use path_abs::PathInfo;

use crate::archive::unzip;
use crate::projectfs::Project;
use crate::transpiler::toml::service::Function;

pub fn compile(project: Rc<Project>, service_name: &str, function: &Function) -> PathBuf {
    let function_name = function.name.clone();
    let function_artifact_path = format!("./net/services/{}/{}", service_name, function_name);

    let rubysrc_path = format!("{}/rubysrc", function_artifact_path);
    if !Path::new(&rubysrc_path).exists() {
        std::fs::create_dir(&rubysrc_path).unwrap();
    }

    let function_dir = project
        .service_dir(service_name.into())
        .function_dir(function_name);

    fn copy_entries(dir: &PathBuf, to: &PathBuf) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                let copy_to = format!(
                    "{}/{}",
                    to.to_str().unwrap(),
                    entry.file_name().to_str().unwrap()
                );
                std::fs::copy(entry.path(), copy_to).unwrap();
            } else if entry.file_type().unwrap().is_dir() {
                let mut copy_to = PathBuf::from(to);
                copy_to.push(entry.path().iter().last().unwrap());
                std::fs::create_dir_all(&copy_to).unwrap();
                copy_entries(&entry.path(), &copy_to);
            }
        }
    }
    copy_entries(&function_dir, &PathBuf::from(rubysrc_path));

    if !Path::new(&format!("{}/ruby-wasm32-wasi", function_artifact_path)).exists() {
        let mut zip = Vec::new();
        let mut response = reqwest::blocking::get(
            "http://public.assemblylift.akkoro.io/runtime/ruby/ruby-wasm32-wasi.zip",
        )
        .expect("could not fetch ruby runtime zip");
        response.read_to_end(&mut zip).unwrap();
        unzip(&zip, &function_artifact_path).unwrap();
    }

    let copy_from = format!(
        "{}/ruby-wasm32-wasi/usr/local/bin/ruby.wasmu",
        function_artifact_path
    );
    let copy_to = format!("{}/ruby.wasmu", function_artifact_path.clone());
    let copy_result = std::fs::copy(copy_from.clone(), copy_to.clone());
    if copy_result.is_err() {
        println!(
            "ERROR COPY from={} to={}",
            copy_from.clone(),
            copy_to.clone()
        );
        panic!("{:?}", copy_result.err());
    }

    PathBuf::from(copy_to)
}
