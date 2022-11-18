use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use assemblylift_core::wasm;
use path_abs::PathInfo;

use crate::archive::unzip;
use crate::projectfs::Project;
use crate::transpiler::toml::service::Function;

pub fn compile(project: Rc<Project>, service_name: &str, function: &Function) -> PathBuf {
    let function_name = function.name.clone();
    let service_artifact_path = format!("./net/services/{}", service_name);
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

    if !Path::new(&format!("{}/ruby-wasm32-wasi", service_artifact_path)).exists() {
        let mut zip = Vec::new();
        println!("Fetching additional Ruby runtime archive...");
        let mut response = reqwest::blocking::get(
            "http://public.assemblylift.akkoro.io/runtime/ruby/ruby-wasm32-wasi.zip",
        )
        .expect("could not fetch ruby runtime zip");
        response.read_to_end(&mut zip).unwrap();
        unzip(&zip, &service_artifact_path).unwrap();
    }

    let ruby_bin = PathBuf::from(format!(
        "{}/ruby-wasm32-wasi/usr/local/bin/ruby",
        service_artifact_path
    ));
    let mut ruby_wasm = ruby_bin.clone();
    ruby_wasm.set_extension("wasm");
    if Path::new(&ruby_bin).exists() {
        std::fs::rename(ruby_bin.clone(), ruby_wasm.clone()).unwrap();
    }
    let mut ruby_wasmu = ruby_bin.clone();
    ruby_wasmu.set_extension("wasm.bin");
    if !Path::new(&ruby_wasmu).exists() {
        wasm::precompile(Path::new(&ruby_wasm), "x86_64-linux-gnu").unwrap();
    }
    let copy_to = format!("{}/ruby.wasm.bin", function_artifact_path.clone());
    let copy_result = std::fs::copy(ruby_wasmu.clone(), copy_to.clone());
    if copy_result.is_err() {
        println!(
            "ERROR COPY from={} to={}",
            ruby_wasmu.as_path().display(),
            copy_to.clone()
        );
        panic!("{:?}", copy_result.err());
    }

    PathBuf::from(copy_to)
}
