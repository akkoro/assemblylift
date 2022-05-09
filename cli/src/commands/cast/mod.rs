use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process;
use std::process::Stdio;
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
use crate::transpiler::{Artifact, asml, hcl, toml};

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
    let project_path = project.dir().into_os_string().into_string().unwrap();

    // Fetch the latest terraform binary to the project directory
    terraform::fetch(&*project.dir());

    let services = asml_manifest.services.clone();
    for (_id, service_ref) in services.as_ref() {
        let mut service_toml = project.service_dir(service_ref.name.clone()).dir();
        service_toml.push("service.toml");
        let service_manifest = toml::service::Manifest::read(&service_toml).unwrap();
        let service_name = service_manifest.service().name.clone();

        {
            // TODO copy ruby env if language==ruby for any function

            let iomod_path = format!("{}/net/services/{}/iomods", project_path, service_name);
            let _ = fs::remove_dir_all(iomod_path.clone()); // we don't care about this result
            fs::create_dir_all(iomod_path.clone()).expect("could not create iomod directory");

            // FIXME some dependency types are not supported by some providers
            // TODO maybe dependency types should be transparent? and orchestrated at the provider level instead of here
            let iomods = service_manifest.iomods().clone();
            let mut dependencies: Vec<String> = Vec::new();
            for (id, dependency) in iomods.as_ref() {
                let dependency_name = id.clone();
                match dependency.dependency_type.as_deref() {
                    Some("file") => {
                        let dependency_path = format!("{}/net/services/{}/iomods/{}", project_path, service_name, dependency_name);
                        let dependency_from = dependency.from.as_ref()
                            .expect("`from` must be defined when dependency type is `file`");
                        match fs::metadata(dependency_from.clone()) {
                            Ok(_) => {
                                fs::copy(dependency_from.clone(), &dependency_path).unwrap();
                                ()
                            }
                            Err(_) => panic!("ERROR: could not find file-type dependency named {} (check path)", dependency_name),
                        }

                        dependencies.push(dependency_path);
                    }
                    Some("registry") => {
                        let dependency_path = format!(
                            "{}/net/services/{}/iomods/{}@{}.iomod",
                            project_path,
                            service_name,
                            dependency.coordinates,
                            dependency.version,
                        );
                        let client = reqwest::blocking::ClientBuilder::new()
                            .build()
                            .expect("could not build blocking HTTP client");
                        let registry_url = format!(
                            "https://registry.assemblylift.akkoro.io/iomod/{}/{}",
                            dependency.coordinates,
                            dependency.version
                        );
                        let res: GetIomodAtResponse = client.get(registry_url)
                            .send()
                            .unwrap()
                            .json()
                            .unwrap();
                        let bytes = client.get(res.url)
                            .send()
                            .unwrap()
                            .bytes()
                            .unwrap();
                        fs::write(&dependency_path, &*bytes).expect("could not write iomod package");
                        dependencies.push(dependency_path);
                    }
                    Some("container") => {}
                    _ => unimplemented!("invalid dependency type (supported: [container, file, registry])"),
                }
            }

            archive::zip_files(
                dependencies,
                format!("./.asml/runtime/{}.zip", &service_name),
                Some("iomod/"),
                false,
            );
        }

        let functions = service_manifest.functions();
        for (_id, function) in functions.as_ref() {
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
                    let store = Store::new(&/*Native*/Universal::new(compiler)
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
        let ctx = asml::Context::from_project(project.clone(), asml_manifest)
            .expect("could not make context from manifest");
        let mut module = hcl::root::Module::new(Rc::new(ctx));
        let hcl_content = module.cast().expect("could not cast HCL modules");

        let path = String::from("./net/plan.tf");
        let mut file = match fs::File::create(path.clone()) {
            Err(why) => panic!(
                "couldn't create file {}: {}",
                path.clone(),
                why.to_string()
            ),
            Ok(file) => file,
        };

        println!("📄 > Wrote {}", path.clone());
        file.write_all(hcl_content.as_bytes()).expect("could not write plan.tf");
    }

    terraform::commands::init();
    terraform::commands::plan();
}
