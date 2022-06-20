use std::fs;
use std::io::Write;
use std::str::FromStr;

use clap::ArgMatches;
use path_abs::PathInfo;
use registry_common::models::GetIomodAtResponse;
use reqwest;
use wasmer::{Module, Store};
use wasmer_compiler::{CpuFeature, Target, Triple};
use wasmer_compiler_cranelift::Cranelift;
//use wasmer_compiler_llvm::LLVM;
//use wasmer_engine_native::Native;
use wasmer_engine_universal::Universal;

use crate::archive;
use crate::projectfs::Project;
use crate::terraform;
use crate::transpiler::{Castable, toml};
use crate::transpiler::context::Context;

mod ruby;
mod rust;

mod lang {
    use std::path::PathBuf;
    use std::rc::Rc;

    use crate::cast::{ruby, rust};
    use crate::projectfs::Project;
    use crate::transpiler::toml::service::Function;

    pub fn compile(project: Rc<Project>, service_name: &str, function: &Function) -> PathBuf {
        let function_name = function.name.clone();
        let function_artifact_path =
            format!("./net/services/{}/{}", service_name, function_name);
        std::fs::create_dir_all(PathBuf::from(function_artifact_path.clone())).expect(&*format!(
            "unable to create path {}",
            function_artifact_path
        ));

        match function.language.clone().unwrap_or("rust".into()).as_str() {
            "rust" => rust::compile(project, service_name, function),
            "ruby" => ruby::compile(project, service_name, function),
            _ => panic!("unsupported function language"),
        }
    }
}

pub fn command(matches: Option<&ArgMatches>) {
    use std::rc::Rc;

    let _matches = match matches {
        Some(matches) => matches,
        _ => panic!("could not get matches for cast command"),
    };

    // Init the project structure -- panic if the project isn't in the current working dir
    let cwd = std::env::current_dir().unwrap();
    let mut manifest_path = cwd.clone();
    manifest_path.push("assemblylift.toml");

    let asml_manifest = toml::asml::Manifest::read(&manifest_path)
        .expect("could not read assemblylift.toml");
    let project = Rc::new(Project::new(asml_manifest.project.name.clone(), Some(cwd)));

    // Fetch the latest terraform binary to the project directory
    terraform::fetch(&*project.dir());

    let services = asml_manifest.services.clone();
    for service_ref in services.as_ref() {
        let mut service_toml = project.service_dir(service_ref.name.clone()).dir();
        service_toml.push("service.toml");
        let service_manifest = toml::service::Manifest::read(&service_toml).unwrap();
        let service_name = service_manifest.service().name.clone();

        // TODO copy ruby env if language==ruby for any function

        let functions = service_manifest.functions();
        for function in functions.as_ref() {
            let function_name = function.name.clone();
            let function_artifact_path =
                format!("./net/services/{}/{}", service_name, function_name);

            let wasm_path = lang::compile(project.clone(), &service_name, function);
            let is_wasmu = wasm_path.extension().unwrap_or("wasm".as_ref()).eq("wasmu");
            let module_file_path = match is_wasmu {
                false => {
                    // TODO compiler configuration
                    let file_path = format!("{}.bin", wasm_path.to_str().unwrap());
                    println!("Precompiling WASM to {}...", file_path.clone());
                    let compiler = Cranelift::default();
                    let triple = Triple::from_str("x86_64-unknown-unknown").unwrap();
                    let mut cpuid = CpuFeature::set();
                    cpuid.insert(CpuFeature::SSE2); // required for x86
                    let store = Store::new(&/*Native*/Universal::new(compiler)
                        .target(Target::new(triple, cpuid))
                        .engine()
                    );

                    let wasm_bytes = match fs::read(wasm_path.clone()) {
                        Ok(bytes) => bytes,
                        Err(err) => panic!("{}", err.to_string()),
                    };
                    let module = Module::new(&store, wasm_bytes).unwrap();
                    let module_bytes = module.serialize().unwrap();
                    let mut module_file = match fs::File::create(file_path.clone()) {
                        Ok(file) => file,
                        Err(err) => panic!("{}", err.to_string()),
                    };
                    println!("📄 > Wrote {}", &file_path);
                    module_file.write_all(&module_bytes).unwrap();

                    file_path
                }

                true => wasm_path.to_str().unwrap().to_string()
            };

            // TODO not needed w/ container functions
            archive::zip_files(
                vec![module_file_path],
                format!("{}/{}.zip", function_artifact_path.clone(), &function_name),
                None,
                false,
            );
        }
    }

    {
        let ctx = Rc::new(Context::from_project(project.clone(), asml_manifest)
            .expect("could not make context from manifest"));
        let artifacts = ctx.cast(ctx.clone(), None)
            .expect("could not cast assemblylift context");
        for artifact in artifacts {
            let path = artifact.write_path;
            let mut file = match fs::File::create(path.clone()) {
                Err(why) => panic!(
                    "couldn't create file {}: {}",
                    path.clone(),
                    why.to_string()
                ),
                Ok(file) => file,
            };

            file.write_all(artifact.content.as_bytes()).expect("could not write artifact");
            println!("📄 > Wrote {}", path.clone());
        }
    }

    terraform::commands::init();
    terraform::commands::plan();
}
