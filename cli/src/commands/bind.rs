use std::rc::Rc;

use clap::ArgMatches;

use crate::projectfs::Project;
use crate::terraform;
use crate::transpiler::context::Context;
use crate::transpiler::toml;
use crate::transpiler::Bindable;

pub fn command(matches: Option<&ArgMatches>) {
    let _matches = match matches {
        Some(matches) => matches,
        _ => panic!("could not get matches for bind command"),
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
    ctx.bind(ctx.clone()).unwrap();

    // TODO terraform should be refactored around Tool trait
    terraform::commands::init();
    terraform::commands::apply();
}
