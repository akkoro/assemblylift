use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;

use assemblylift_core::wasm;

use crate::projectfs::Project;
use crate::transpiler::toml::service::Function;

pub fn compile(project: Rc<Project>, service_name: &str, function: &Function) -> PathBuf {
    let function_name = function.name.clone();
    let function_artifact_path = format!("./net/services/{}/{}", service_name, function_name);

    let function_path = PathBuf::from(format!(
        "{}/Cargo.toml",
        project
            .clone()
            .service_dir(service_name.into())
            .function_dir(function_name.clone())
            .into_os_string()
            .into_string()
            .unwrap()
    ));

    let mode = "release";
    let target = "wasm32-wasi";

    println!("Compiling function {}...", function_name.clone());
    let cargo_build = std::process::Command::new("cargo")
        .arg("build")
        .arg(format!("--{}", mode))
        .arg("--manifest-path")
        .arg(function_path)
        .arg("--target")
        .arg(target)
        .output()
        .unwrap();

    let build_log = std::str::from_utf8(&cargo_build.stderr).unwrap();
    std::io::stderr().write_all(&cargo_build.stderr).unwrap();
    if build_log.contains("error") {
        std::process::exit(-1);
    }

    let copy_from = format!(
        "{}/target/{}/{}/{}.wasm",
        project
            .clone()
            .service_dir(service_name.into())
            .function_dir(function_name.clone())
            .into_os_string()
            .into_string()
            .unwrap(),
        target,
        mode,
        function_name,
    );
    let copy_to = format!("{}/{}.wasm", function_artifact_path.clone(), &function_name);
    let copy_result = std::fs::copy(copy_from.clone(), copy_to.clone());
    if copy_result.is_err() {
        println!(
            "ERROR COPY from={} to={}",
            copy_from.clone(),
            copy_to.clone()
        );
        panic!("{:?}", copy_result.err());
    }

    wasm::precompile(PathBuf::from(copy_to)).unwrap()
}
