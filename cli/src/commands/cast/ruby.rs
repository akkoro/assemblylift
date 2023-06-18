use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use path_abs::PathInfo;

use assemblylift_core::wasm;

use crate::archive::unzip;
use crate::projectfs::{NetDir, Project};

use super::CastableFunction;

pub struct RubyFunction {
    project: Rc<Project>,
    service_name: String,
    function_name: String,
    // net_path: String,
    net_dir: NetDir,
    enable_precompile: bool,
    target: String,
    mode: String,
    cpu_compat_mode: String,
}

impl RubyFunction {
    pub fn new(function: &crate::transpiler::context::Function) -> Self {
        let service_name = function.service_name.clone();
        let net_path = function
            .project
            .net_dir()
            .service_dir(&service_name.clone())
            .function_dir(function.name.clone())
            .to_str()
            .unwrap()
            .to_string();
        let net_dir = function.project.net_dir();
        std::fs::create_dir_all(PathBuf::from(&net_path))
            .expect(&*format!("unable to create path {}", &net_path));
        Self {
            project: function.project.clone(),
            service_name,
            function_name: function.name.clone(),
            // net_path,
            net_dir,
            enable_precompile: function.precompile,
            mode: "release".to_string(),
            target: "wasm32-wasi".to_string(),
            cpu_compat_mode: function.cpu_compat_mode.clone(),
        }
    }
}

impl CastableFunction for RubyFunction {
    fn compile(&self, wasi_snapshot_preview1: Vec<u8>) {
        let function_name = &self.function_name;
        let service_name = &self.service_name;
        let service_net_dir = self
            .net_dir
            .service_dir(&service_name.clone());
        let service_artifact_path = service_net_dir
            .dir()
            .to_str()
            .unwrap()
            .to_string();
        let function_artifact_path = service_net_dir
            .function_dir(function_name.clone())
            .to_str()
            .unwrap()
            .to_string();

        let rubysrc_path = format!("{}/rubysrc", function_artifact_path);
        if !Path::new(&rubysrc_path).exists() {
            std::fs::create_dir(&rubysrc_path).unwrap();
        }

        let function_dir = self
            .project
            .service_dir(service_name.into())
            .function_dir(function_name.clone());

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
                "http://public.assemblylift.akkoro.io/runtime/ruby/3.3.0-dev/ruby-wasm32-wasi.zip",
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
        let copy_to = format!("{}/{}.wasm", &function_artifact_path, &function_name);
        let copy_result = std::fs::copy(ruby_wasm.clone(), copy_to.clone());
        if copy_result.is_err() {
            println!(
                "ERROR COPY from={} to={}",
                ruby_wasm.as_path().display(),
                copy_to.clone()
            );
            panic!("{:?}", copy_result.err());
        }

        {
            let module = std::fs::read(copy_to.clone()).unwrap();
            let component = wasm::make_wasi_component(module, wasi_snapshot_preview1.as_slice())
                .expect("unable to make component of the provided module");
            std::fs::write(copy_to.clone(), component).unwrap();
        }
    }

    fn compose(&self) {
        todo!()
    }

    // TODO projectfs should handle mapping the precompiled bin path
    fn precompile(&self, target: Option<&str>) {
        println!("⚡️ > Precompiling function `{}`...", &self.function_name);
        let net_path = self
            .net_dir
            .service_dir(&self.service_name.clone())
            .function_dir(self.function_name.clone())
            .to_str()
            .unwrap()
            .to_string();
        let path = format!("{}/{}.wasm", &net_path, &self.function_name);
        let bytes = wasm::precompile(
            Path::new(&path),
            &target.unwrap_or("x86_64-linux-gnu"),
            &self.cpu_compat_mode.clone(),
        )
        .unwrap();
        let out_path = format!("{}.bin", path);
        std::fs::write(&out_path, bytes).unwrap();
        println!("📄 > Wrote {}", &out_path);
    }

    fn artifact_path(&self) -> PathBuf {
        let net_path = self
            .net_dir
            .service_dir(&self.service_name.clone())
            .function_dir(self.function_name.clone())
            .to_str()
            .unwrap()
            .to_string();
        match self.enable_precompile {
            true => PathBuf::from(format!(
                "{}/{}.wasm.bin",
                net_path.clone(),
                &self.function_name
            )),
            false => PathBuf::from(format!(
                "{}/{}.wasm",
                net_path.clone(),
                &self.function_name
            )),
        }
    }
}
