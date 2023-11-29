use std::path::PathBuf;

use handlebars::handlebars_helper;

pub mod context;
mod drivers;
pub mod projectfs;
pub mod providers;
pub mod toml;

handlebars_helper!(concat: |s1: String, s2: String| format!("{}{}", s1, s2));
handlebars_helper!(snake_case: |s: String| s.replace("-", "_").replace(".", "_"));

// fn main() {
//     let cwd = std::env::current_dir().unwrap();
//     let mut manifest_path = cwd.clone();
//     manifest_path.push("assemblylift.toml");

//     // TODO manifest should be returned from Project
//     let asml_manifest =
//         toml::asml::Manifest::read(&manifest_path).expect("could not read assemblylift.toml");
//     let project = Project::new(asml_manifest.project.name.clone(), Some(cwd));

//     let ctx = Context::from_project(project, asml_manifest)
//         .expect("could not make context from manifest");

//     if let Err(err) = ctx.cast() {
//         println!("Cast Error: {}", err.0);
//     } else {
//         println!("Cast Complete!");
//     }
// }

pub type CastResult<T> = std::result::Result<T, CastError>;
pub type Map<K, V> = std::collections::HashMap<K, V>;
pub type StringMap<V> = Map<String, V>;
pub type Options = StringMap<String>;

/// A `Fragment` is the output of a `cast` operation. Its contents may be part of, or the entirety of,
/// some document which will be output at `write_path`.
#[derive(Debug, Clone)]
pub struct Fragment {
    pub content_type: ContentType,
    pub content: String,
    pub write_path: PathBuf,
}

#[derive(Debug)]
pub struct CastError(pub String);

#[derive(Clone, Debug, PartialEq)]
pub enum ContentType {
    HCL,
    Dockerfile,
}

pub fn concat_cast<T>(accum: CastResult<Vec<T>>, v: CastResult<Vec<T>>) -> CastResult<Vec<T>> {
    let mut out = Vec::new();
    out.append(&mut accum?);
    out.append(&mut v?);
    Ok(out)
}
