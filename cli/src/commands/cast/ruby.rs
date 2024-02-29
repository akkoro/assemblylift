use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use path_abs::PathInfo;

use assemblylift_core::wasm;
use assemblylift_generator::context::Function;
use assemblylift_generator::projectfs::{NetDir, Project};
use sha2::{Digest, Sha256};
use sha2::digest::FixedOutput;

use crate::archive::unzip;

use super::{CastableFunction, CompileStatus};

pub struct RubyFunction {
    project: Rc<Project>,
    service_name: String,
    function_name: String,
    net_dir: NetDir,
    enable_precompile: bool,
    cpu_compat_mode: String,
    ruby_version: String,
}

impl RubyFunction {
    pub fn new(function: &Function, project: Rc<Project>) -> Self {
        let service_name = function.service_name.clone();
        let net_dir = project.net_dir();
        let net_path = net_dir
            .service_dir(&service_name.clone())
            .function_dir(function.name.clone())
            .to_str()
            .unwrap()
            .to_string();
        std::fs::create_dir_all(PathBuf::from(&net_path))
            .expect(&*format!("unable to create path {}", &net_path));
        Self {
            project: project.clone(),
            service_name,
            function_name: function.name.clone(),
            net_dir,
            enable_precompile: function.precompiled,
            cpu_compat_mode: function.cpu_compat_mode.clone(),
            ruby_version: "3.3.0-dev".into(),
        }
    }
}

impl CastableFunction for RubyFunction {
    fn compile(&self, wasi_snapshot_preview1: Vec<u8>) -> Result<CompileStatus, String> {
        let function_name = &self.function_name;
        let service_name = &self.service_name;
        let service_net_dir = self.net_dir.service_dir(&service_name.clone());
        let service_artifact_path = service_net_dir.dir().to_str().unwrap().to_string();
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

        let mut ruby_changed = false;

        let ruby_runtime_path = format!("{}/ruby/{}", self.project.net_dir().runtime_dir().to_str().unwrap(), self.ruby_version);
        std::fs::create_dir_all(&ruby_runtime_path).unwrap();
        if !Path::new(&format!("{}/ruby-wasm32-wasi", ruby_runtime_path)).exists() {
            let mut zip = Vec::new();
            let url =
                format!("http://public.assemblylift.akkoro.io/runtime/ruby/{}/ruby-wasm32-wasi.zip", self.ruby_version);
            println!("Fetching Ruby runtime archive from {}...", url);
            let mut response =
                reqwest::blocking::get(url).expect("could not fetch ruby runtime zip");
            response.read_to_end(&mut zip).unwrap();
            std::fs::write(format!("{}/ruby-wasm32-wasi.zip", &ruby_runtime_path), &zip).unwrap();
            unzip(&zip, &ruby_runtime_path).unwrap();

            ruby_changed = true;
        }

        let ruby_bin = PathBuf::from(format!(
            "{}/ruby-wasm32-wasi/usr/local/bin/ruby",
            ruby_runtime_path,
        ));
        let mut ruby_wasm = ruby_bin.clone();
        ruby_wasm.set_extension("wasm");
        if Path::new(&ruby_bin).exists() {
            std::fs::rename(ruby_bin.clone(), ruby_wasm.clone()).unwrap();
        }
        let component_wasm = format!(
            "{}.component.wasm",
            &ruby_bin.to_string_lossy(),
        );

        {
            let module = std::fs::read(ruby_wasm.clone()).unwrap();
            let embedded = wasm::embed_asml_wit(module).expect("unable to embed assemblylift WIT");
            let component = wasm::make_wasi_component(embedded, wasi_snapshot_preview1.as_slice())
                .expect("unable to make component of the provided module");

            if !ruby_changed {
                if !Path::new(&component_wasm).exists() {  
                    ruby_changed = true;
                } else {
                    let artifact_content = std::fs::read(component_wasm.clone()).unwrap();
                    let mut artifact_hasher = Sha256::new();
                    artifact_hasher.update(artifact_content);
                    let artifact_hash = artifact_hasher.finalize_fixed();

                    let mut component_hasher = Sha256::new();
                    component_hasher.update(component.clone());
                    let component_hash = component_hasher.finalize_fixed();
                    if artifact_hash != component_hash {
                        ruby_changed = true;
                    }
                }
            }

            std::fs::write(component_wasm.clone(), component.clone()).unwrap();
        }

        Ok(CompileStatus { wasm_path: PathBuf::from(&component_wasm), changed: ruby_changed })
    }

    fn compose(&self) {
        todo!()
    }

    // TODO projectfs should handle mapping the precompiled bin path
    fn precompile(&self, target: Option<&str>) {
        println!("⚡️ > Precompiling function `{}`...", &self.function_name);
        let net_path = self
            .net_dir
            .runtime_dir()
            .to_str()
            .unwrap()
            .to_string();
        let ruby_runtime_path = format!("{}/ruby/{}", net_path, self.ruby_version);
        let path = format!("{}/ruby-wasm32-wasi/usr/local/bin/ruby.component.wasm", &ruby_runtime_path);
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
                "{}/{}.component.wasm.bin",
                net_path.clone(),
                &self.function_name
            )),
            false => PathBuf::from(format!(
                "{}/{}.component.wasm",
                net_path.clone(),
                &self.function_name
            )),
        }
    }
}
