use std::env::current_dir;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use assemblylift_core::wasm;

use crate::projectfs::Project;
use crate::transpiler::toml::service::Function;

pub fn compile(
    project: Rc<Project>,
    service_name: &str,
    function: &Function,
) -> Result<PathBuf, String> {
    let function_name = function.name.clone();
    let curr_dir = std::env::current_dir().unwrap();
    let target_dir = format!("{}/target", curr_dir.to_str().unwrap());
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

    let cargo_build = std::process::Command::new("cargo")
        .arg("build")
        .arg(format!("--{}", mode))
        .arg("--manifest-path")
        .arg(function_path)
        .arg("--target")
        .arg(target)
        .arg("--target-dir")
        .arg(target_dir)
        .output()
        .unwrap();

    let build_log = std::str::from_utf8(&cargo_build.stderr).unwrap();
    std::io::stderr().write_all(&cargo_build.stderr).unwrap();
    if build_log.contains("error") {
        if cargo_build.status.code().unwrap() != 0 {
            return Err(format!(
                "Unable to compile function {}:\n{}",
                function_name, build_log
            ));
        }
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
    let copy_from = match std::fs::metadata(&copy_from) {
        Ok(_) => copy_from,
        Err(_) => format!(
            "{}/target/{}/{}/{}.wasm",
            project
                .clone()
                .dir()
                .into_os_string()
                .into_string()
                .unwrap(),
            target,
            mode,
            function_name,
        ),
    };
    let copy_to = format!("{}/{}.wasm", function_artifact_path.clone(), &function_name);
    let copy_result = std::fs::copy(copy_from.clone(), copy_to.clone());

    match copy_result {
        Ok(_) => println!("Copied {} to {}", copy_from.clone(), copy_to.clone()),
        Err(e) => {
            println!(
                "ERROR COPY from={} to={}",
                copy_from.clone(),
                copy_to.clone()
            );
            if e.kind() == std::io::ErrorKind::NotFound {
                return Err(format!(
                    "Unable to find compiled function at {}",
                    copy_from.clone()
                ));
            }
            return Err(format!(
                "Unable to copy compiled function from {} to {}",
                copy_from.clone(),
                copy_to.clone()
            ));
        }
    }

    let precompile = function.precompile.unwrap_or(true);
    if precompile {
        let precompile_result = wasm::precompile(
            Path::new(&copy_to),
            "x86_64-linux-gnu",
            &function
                .cpu_compat_mode
                .clone()
                .unwrap_or("default".to_string()),
        );

        match precompile_result {
            Ok(path) => Ok(path),
            Err(e) => {
                return Err(format!("Unable to precompile function: {}", e));
            }
        }
    } else {
        Ok(PathBuf::from(&copy_to))
    }
}
