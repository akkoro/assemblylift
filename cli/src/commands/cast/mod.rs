use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;

use clap::ArgMatches;

use crate::archive;
use crate::commands::cast::rust::RustFunction;
use crate::projectfs::Project;
use crate::tools::terraform::Terraform;
use crate::transpiler::context::Context;
use crate::transpiler::{toml, Castable};

use self::ruby::RubyFunction;

mod ruby;
mod rust;

pub trait CastableFunction {
    fn compile(&self, wasi_snapshot_preview1: Vec<u8>);
    fn compose(&self);
    fn precompile(&self, target: Option<&str>);
    fn artifact_path(&self) -> PathBuf;
}

pub fn command(matches: Option<&ArgMatches>) {
    let tf = Terraform::default();

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

    let ctx = Rc::new(
        Context::from_project(project.clone(), asml_manifest)
            .expect("could not make context from manifest"),
    );

    let wasi_snapshot_preview1 = include_bytes!("wasm/wasi_snapshot_preview1.command.wasm");

    // Compile WASM & package function
    let functions = ctx.functions.as_slice();
    for function in functions {
        let function_artifact_path = project
            .net_dir()
            .service_dir(&function.service_name.clone())
            .function_dir(function.name.clone())
            .into_os_string()
            .into_string()
            .unwrap();

        let castable_function: Box<dyn CastableFunction> = match function.language.clone().as_str()
        {
            "rust" => Box::new(RustFunction::new(&function)),
            "ruby" => Box::new(RubyFunction::new(&function)),
            lang => panic!("unsupported function language: {}", lang),
        };
        castable_function.compile(wasi_snapshot_preview1.clone().to_vec());
        if function.precompile {
            // TODO set target triple
            castable_function.precompile(None);
        }

        // Function archive is only needed for Lambda at this time
        if ctx
            .service(&function.service_name)
            .unwrap()
            .provider
            .name
            .eq(crate::providers::AWS_LAMBDA_PROVIDER_NAME)
        {
            let mut function_dirs = vec![castable_function.artifact_path()];
            if "ruby" == function.language.clone().as_str() {
                function_dirs.push(PathBuf::from(format!(
                    "{}/rubysrc",
                    &function_artifact_path
                )));
            }
            archive::zip_dirs(
                function_dirs,
                format!("{}/{}.zip", function_artifact_path.clone(), &function.name),
                Vec::new(),
            )
            .expect("unable to zip function artifacts");
        }
    }

    // Cast Context to artifacts
    {
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

    tf.init();
    tf.plan();
}
