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

/// A `Castable` implements the `cast` step, transforming some part of `Context` into an `Artifact`
pub trait Castable {
    /// Generate Artifacts from the project Context for this implementor
    fn cast(&self, ctx: Rc<Context>, selector: Option<&str>) -> Result<Vec<Artifact>, CastError>;
}

/// A `Bindable` implements the `bind` step, performing an action which updates the state of infrastructure
pub trait Bindable {
    /// Bind the implementor to the backend provider
    fn bind(&self, ctx: Rc<Context>) -> Result<(), CastError>;
}

/// A `Bootable` implements the `boot` step; `boot` is intended to be a run-once operation to prepare
/// any infrastructure that must exist prior to `bind`.
pub trait Bootable {
    /// Provides an opportunity to deploy prerequisite infra if not already present
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

/// An `Artifact` is the output of a `cast` operation. Its contents may be part of, or the entirety of,
/// some document which will be output at `write_path`.
#[derive(Debug, Clone)]
pub struct Artifact {
    pub content_type: ContentType,
    pub content: String,
    pub write_path: String,
}
