use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use clap::ArgMatches;
use path_abs::PathInfo;
use registry_common::models::GetIomodAtResponse;
use reqwest;

use crate::archive;
use crate::projectfs::Project;
use crate::terraform;
use crate::transpiler::context::Context;
use crate::transpiler::{toml, Castable};

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
        let function_artifact_path = format!("./net/services/{}/{}", service_name, function_name);
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

    let asml_manifest =
        toml::asml::Manifest::read(&manifest_path).expect("could not read assemblylift.toml");
    let project = Rc::new(Project::new(asml_manifest.project.name.clone(), Some(cwd)));

    // Fetch the latest terraform binary to the project directory
    terraform::fetch(&*project.dir());

    let services = asml_manifest.services.clone();
    for service_ref in services.as_ref() {
        let mut service_toml = project.service_dir(service_ref.name.clone()).dir();
        service_toml.push("service.toml");
        let service_manifest = toml::service::Manifest::read(&service_toml).unwrap();
        let service_name = service_manifest.service().name.clone();

        let functions = service_manifest.functions();
        for function in functions.as_ref() {
            let function_name = function.name.clone();
            let function_artifact_path =
                format!("./net/services/{}/{}", service_name, function_name);

            let wasm_path = lang::compile(project.clone(), &service_name, function);

            // TODO zip not needed w/ container functions
            let mut function_dirs = vec![wasm_path];
            if let Some("ruby") = function.language.clone().as_deref() {
                function_dirs.push(PathBuf::from(format!(
                    "{}/rubysrc",
                    &function_artifact_path
                )));
            }
            archive::zip_dirs(
                function_dirs,
                format!("{}/{}.zip", function_artifact_path.clone(), &function_name),
                Vec::new(),
            )
            .expect("unable to zip function artifacts");
        }
    }

    {
        let ctx = Rc::new(
            Context::from_project(project.clone(), asml_manifest)
                .expect("could not make context from manifest"),
        );
        let artifacts = ctx
            .cast(ctx.clone(), None)
            .expect("could not cast assemblylift context");
        for artifact in artifacts {
            let path = artifact.write_path;
            let mut file = match fs::File::create(path.clone()) {
                Err(why) => panic!("couldn't create file {}: {}", path.clone(), why.to_string()),
                Ok(file) => file,
            };

            file.write_all(artifact.content.as_bytes())
                .expect("could not write artifact");
            println!("📄 > Wrote {}", path.clone());
        }
    }

    terraform::commands::init();
    terraform::commands::plan();
}
