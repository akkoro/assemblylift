use std::io::Write;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use assemblylift_core::wasm;
use assemblylift_generator::context::Function;
use assemblylift_generator::projectfs::Project;

use crate::commands::cast::CastableFunction;

use super::CompileStatus;

pub struct RustFunction {
    project: Rc<Project>,
    service_name: String,
    function_name: String,
    net_path: String,
    enable_precompile: bool,
    target: String,
    mode: String,
    cpu_compat_mode: String,
}

impl RustFunction {
    pub fn new(function: &Function, project: Rc<Project>) -> Self {
        let service_name = function.service_name.clone();
        let net_path = project
            .net_dir()
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
            net_path,
            enable_precompile: function.precompiled,
            mode: "release".to_string(),
            target: "wasm32-wasi".to_string(),
            cpu_compat_mode: function.cpu_compat_mode.clone(),
        }
    }

    pub fn source_wasm_path(&self) -> String {
        let copy_from = format!(
            "{}/target/{}/{}/{}.wasm",
            self.project
                .clone()
                .service_dir(self.service_name.clone())
                .function_dir(self.function_name.clone())
                .to_str()
                .unwrap(),
            self.target,
            self.mode,
            self.function_name,
        );
        let copy_from = match std::fs::metadata(&copy_from) {
            Ok(_) => copy_from,
            Err(_) => format!(
                "{}/target/{}/{}/{}.wasm",
                self.project.clone().dir().to_str().unwrap(),
                self.target,
                self.mode,
                self.function_name,
            ),
        };
        copy_from
    }
}

impl CastableFunction for RustFunction {
    fn compile(&self, wasi_snapshot_preview1: Vec<u8>) -> Result<CompileStatus, String> {
        let manifest_path = PathBuf::from(format!(
            "{}/Cargo.toml",
            self.project
                .clone()
                .service_dir(self.service_name.clone())
                .function_dir(self.function_name.clone())
                .to_str()
                .unwrap()
        ));

        println!("🛠️ > Compiling function `{}`...", self.function_name.clone());
        let cargo_build = std::process::Command::new("cargo")
            .arg("build")
            .arg(format!("--{}", self.mode.clone()))
            .arg("--manifest-path")
            .arg(manifest_path)
            .arg("--target")
            .arg(self.target.clone())
            .output()
            .unwrap();

        let build_log = std::str::from_utf8(&cargo_build.stderr).unwrap();
        if cargo_build.status.code().unwrap() != 0 {
            return Err(format!(
                "Unable to compile function {}:\n{}",
                &self.function_name, build_log
            ));
        }

        let mut component_path = PathBuf::from(self.source_wasm_path());
        component_path.set_extension("component.wasm");
        {
            let module = std::fs::read(self.source_wasm_path()).unwrap();
            let component = wasm::make_wasi_component(module, wasi_snapshot_preview1.as_slice())
                .expect("unable to make component of the provided module");
            std::fs::write(component_path.clone(), component).unwrap();
        }

        Ok(CompileStatus { wasm_path: component_path.clone(), changed: true })
    }

    fn compose(&self) {
        todo!()
    }

    fn precompile(&self, target: Option<&str>) {
        println!("⚡️ > Precompiling function `{}`...", &self.function_name);
        let mut path = PathBuf::from(self.source_wasm_path());
        path.set_extension("component.wasm");
        let bytes = wasm::precompile(
            Path::new(&path),
            &target.unwrap_or("x86_64-linux-gnu"),
            &self.cpu_compat_mode.clone(),
        )
        .unwrap();
        path.set_extension("wasm.bin");
        std::fs::write(&path, bytes).unwrap();
        println!("📄 > Wrote {}", path.to_str().unwrap());
    }

    fn artifact_path(&self) -> PathBuf {
        match self.enable_precompile {
            true => PathBuf::from(format!(
                "{}/{}.component.wasm.bin",
                self.net_path.clone(),
                &self.function_name
            )),
            false => PathBuf::from(format!(
                "{}/{}.component.wasm",
                self.net_path.clone(),
                &self.function_name
            )),
        }
    }
}
