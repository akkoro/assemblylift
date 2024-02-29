use std::path::{Path, PathBuf};
use std::fs;
use std::fs::read_to_string;
use std::io::{Read, Write};
use std::rc::Rc;

use assemblylift_core::wasm;
use assemblylift_generator::context::Context;
use assemblylift_generator::projectfs::Project;
use assemblylift_generator::toml;
use assemblylift_tools::terraform::Terraform;
use clap::ArgMatches;
use sha2::{Digest, Sha256};
use sha2::digest::FixedOutput;

use crate::archive;

use self::ruby::RubyFunction;
use self::rust::RustFunction;

mod ruby;
mod rust;

pub struct CompileStatus {
    wasm_path: PathBuf,
    changed: bool,
}

pub trait CastableFunction {
    fn compile(&self, wasi_snapshot_preview1: Vec<u8>) -> Result<CompileStatus, String>;
    fn compose(&self);
    fn precompile(&self, target: Option<&str>);
    // FIXME should CastableFunction be responsible for constructing its net path like this?
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
    let project = Project::new(asml_manifest.project.name.clone(), Some(cwd));

    let ctx = Rc::new(
        Context::from_project(project.clone(), asml_manifest)
            .expect("could not make context from manifest"),
    );

    let wasi_snapshot_preview1 = include_bytes!("wasm/wasi_snapshot_preview1.command.wasm");

    // Compile WASM & package function
    let functions = ctx
        .services
        .iter()
        .map(|s| s.functions.clone())
        .flatten()
        .collect::<Vec<_>>();
    for function in functions {
        let castable_function: Box<dyn CastableFunction> = match function.language.clone().as_str()
        {
            "rust" => Box::new(RustFunction::new(&function, project.clone())),
            "ruby" => Box::new(RubyFunction::new(&function, project.clone())),
            lang => panic!("unsupported function language: {}", lang),
        };

        let mut function_artifact_path = castable_function.artifact_path();
        function_artifact_path.pop();
        let function_artifact_path = function_artifact_path.to_str().unwrap();

        match castable_function.compile(wasi_snapshot_preview1.clone().to_vec()) {
            Ok(status) => {
                let wasm_path_precompiled = PathBuf::from(&format!("{}.bin", status.wasm_path.to_str().unwrap()));
                if (function.precompiled && status.changed) || (function.precompiled && !wasm_path_precompiled.exists()) {
                    // TODO set target triple
                    castable_function.precompile(None);
                }

                let from_path = match function.precompiled {
                    true => wasm_path_precompiled,
                    false => status.wasm_path.clone(),
                };

                std::fs::copy(from_path, castable_function.artifact_path()).unwrap();
            },
            Err(e) => return println!("Error compiling function {}: {}", &function.name, e),
        }

        // Function archive is only needed for Lambda at this time
        if ctx
            .service(&function.service_name)
            .unwrap()
            .provider
            .name()
            .eq(&assemblylift_generator::providers::aws_lambda::provider_name())
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
        let fragments = ctx.cast().expect("could not cast assemblylift context");
        for fragment in fragments {
            let path = fragment.write_path.clone();
            if path.exists() {
                // if the artifact and the file are the same, we dont want to serialize file to string
                // so after we compare the hashes and if they are the same, we serialize to string.
                let artifact_content = fragment.content.as_bytes();
                let mut artifact_hasher = Sha256::new();
                artifact_hasher.update(artifact_content);
                let artifact_hash_result = artifact_hasher.finalize_fixed();

                let mut file_content_hash = Sha256::new();
                let file_content = read_to_string(path.clone()).expect("could not read file");
                file_content_hash.update(file_content.as_bytes());
                let file_content_hash_result = file_content_hash.finalize_fixed();

                if artifact_hash_result == file_content_hash_result {
                    println!("📄 > No change in {}, skipping...", path.display());
                    continue;
                }
            }

            fragment.write().expect("could not write fragment");
        }
    }

    tf.init();
    tf.plan();
}
