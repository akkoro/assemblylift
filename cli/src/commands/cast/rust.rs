use std::io::Write;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use assemblylift_core::wasm;

use crate::commands::cast::CastableFunction;
use crate::projectfs::Project;
use crate::transpiler::toml::service::Function;

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
    pub fn new(function: &crate::transpiler::context::Function) -> Self {
        let service_name = function.service_name.clone();
        let net_path = function
            .project
            .net_dir()
            .service_dir(&service_name.clone())
            .function_dir(function.name.clone())
            .into_os_string()
            .into_string()
            .unwrap();
        std::fs::create_dir_all(PathBuf::from(&net_path))
            .expect(&*format!("unable to create path {}", &net_path));
        Self {
            project: function.project.clone(),
            service_name,
            function_name: function.name.clone(),
            net_path,
            enable_precompile: function.precompile,
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
                .into_os_string()
                .into_string()
                .unwrap(),
            self.target,
            self.mode,
            self.function_name,
        );
        let copy_from = match std::fs::metadata(&copy_from) {
            Ok(_) => copy_from,
            Err(_) => format!(
                "{}/target/{}/{}/{}.wasm",
                self.project
                    .clone()
                    .dir()
                    .into_os_string()
                    .into_string()
                    .unwrap(),
                self.target,
                self.mode,
                self.function_name,
            ),
        };
        copy_from
    }
}

impl CastableFunction for RustFunction {
    fn compile(&self, wasi_snapshot_preview1: Vec<u8>) {
        let manifest_path = PathBuf::from(format!(
            "{}/Cargo.toml",
            self.project
                .clone()
                .service_dir(self.service_name.clone())
                .function_dir(self.function_name.clone())
                .into_os_string()
                .into_string()
                .unwrap()
        ));

        println!(
            "🛠️  > Compiling function `{}`...",
            self.function_name.clone()
        );
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
        std::io::stderr().write_all(&cargo_build.stderr).unwrap();
        if build_log.contains("error") {
            // TODO https://github.com/akkoro/assemblylift/issues/new?title=Detect%20build%20output
            std::process::exit(-1);
        }

        let move_from = self.source_wasm_path();
        let move_to = format!("{}/{}.wasm", self.net_path.clone(), &self.function_name);
        let move_result = std::fs::rename(move_from.clone(), move_to.clone());
        if move_result.is_err() {
            println!(
                "ERROR move from={} to={}",
                move_from.clone(),
                move_to.clone()
            );
            panic!("{:?}", move_result.err());
        }

        {
            let module = std::fs::read(move_to.clone()).unwrap();
            let component = wasm::make_wasi_component(module, wasi_snapshot_preview1.as_slice())
                .expect("TODO: panic message");
            std::fs::write(move_to.clone(), component).unwrap();
        }
    }

    fn compose(&self) {
        todo!()
    }

    // TODO don't love the use of format! everywhere to make the artifact path
    fn precompile(&self) {
        println!("⚡️ > Precompiling function `{}`...", &self.function_name);
        let path = format!("{}/{}.wasm", &self.net_path, &self.function_name);
        let bytes = wasm::precompile(
            Path::new(&path),
            "x86_64-linux-gnu",
            &self.cpu_compat_mode.clone(),
        )
        .unwrap();
        let out_path = format!("{}.bin", path);
        std::fs::write(&out_path, bytes).unwrap();
        println!("📄 > Wrote {}", &out_path);
    }

    fn artifact_path(&self) -> PathBuf {
        match self.enable_precompile {
            true => PathBuf::from(format!(
                "{}/{}.wasm.bin",
                self.net_path.clone(),
                &self.function_name
            )),
            false => PathBuf::from(format!(
                "{}/{}.wasm",
                self.net_path.clone(),
                &self.function_name
            )),
        }
    }
}

// pub fn compile(project: Rc<Project>, service_name: &str, function: &Function) -> PathBuf {
//     let function_name = function.name.clone();
//     let function_artifact_path = project
//         .net_dir()
//         .service_dir(service_name)
//         .function_dir(function_name.clone())
//         .into_os_string()
//         .into_string()
//         .unwrap();
//
//     let function_path = PathBuf::from(format!(
//         "{}/Cargo.toml",
//         project
//             .clone()
//             .service_dir(service_name.into())
//             .function_dir(function_name.clone())
//             .into_os_string()
//             .into_string()
//             .unwrap()
//     ));
//
//     let mode = "release";
//     let target = "wasm32-wasi";
//
//     println!("Compiling function `{}`...", function_name.clone());
//     let cargo_build = std::process::Command::new("cargo")
//         .arg("build")
//         .arg(format!("--{}", mode))
//         .arg("--manifest-path")
//         .arg(function_path)
//         .arg("--target")
//         .arg(target)
//         .output()
//         .unwrap();
//
//     let build_log = std::str::from_utf8(&cargo_build.stderr).unwrap();
//     std::io::stderr().write_all(&cargo_build.stderr).unwrap();
//     if build_log.contains("error") {
//         // TODO https://github.com/akkoro/assemblylift/issues/new?title=Detect%20build%20output
//         std::process::exit(-1);
//     }
//
//     let copy_from = format!(
//         "{}/target/{}/{}/{}.wasm",
//         project
//             .clone()
//             .service_dir(service_name.into())
//             .function_dir(function_name.clone())
//             .into_os_string()
//             .into_string()
//             .unwrap(),
//         target,
//         mode,
//         function_name,
//     );
//     let copy_from = match std::fs::metadata(&copy_from) {
//         Ok(_) => copy_from,
//         Err(_) => format!(
//             "{}/target/{}/{}/{}.wasm",
//             project
//                 .clone()
//                 .dir()
//                 .into_os_string()
//                 .into_string()
//                 .unwrap(),
//             target,
//             mode,
//             function_name,
//         ),
//     };
//     let copy_to = format!("{}/{}.wasm", function_artifact_path.clone(), &function_name);
//     let copy_result = std::fs::copy(copy_from.clone(), copy_to.clone());
//     if copy_result.is_err() {
//         println!(
//             "ERROR COPY from={} to={}",
//             copy_from.clone(),
//             copy_to.clone()
//         );
//         panic!("{:?}", copy_result.err());
//     }
//
//     if function.precompile.unwrap_or(true) {
//         wasm::precompile(
//             Path::new(&copy_to),
//             "x86_64-linux-gnu",
//             &function.cpu_compat_mode.clone().unwrap_or("default".to_string()),
//         )
//             .unwrap()
//     } else {
//         PathBuf::from(&copy_to)
//     }
// }
