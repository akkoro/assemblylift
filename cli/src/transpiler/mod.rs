use std::rc::Rc;

use crate::transpiler::context::Context;

pub mod context;
pub mod toml;

pub type Map<K, V> = std::collections::HashMap<K, V>;
pub type StringMap<V> = Map<String, V>;

#[derive(Clone, Debug, PartialEq)]
pub enum ContentType {
    HCL(&'static str),
    Dockerfile(&'static str),
    KubeYaml(&'static str),
}

pub trait Castable {
    /// Generate Artifacts from the project Context for this implementor
    fn cast(&self, ctx: Rc<Context>, selector: Option<&str>) -> Result<Vec<Artifact>, CastError>;
}

pub trait Bindable {
    /// Bind the implementor to the backend provider
    fn bind(&self, ctx: Rc<Context>) -> Result<(), CastError>;
}

pub trait Bootable {
    /// Provides an opportunity deploy prerequisite infra if not already present
    fn boot(&self, ctx: Rc<Context>) -> Result<(), CastError>;
    /// Query to see if boot step has already been run
    fn is_booted(&self, ctx: Rc<Context>) -> bool;
}

/// A renderable Handlebars template
pub trait Template {
    fn render(&self) -> String;
    fn tmpl() -> &'static str;
}

#[derive(Debug)]
pub struct CastError(pub String);

#[derive(Debug, Clone)]
pub struct Artifact {
    pub content_type: ContentType,
    pub content: String,
    pub write_path: String,
}
