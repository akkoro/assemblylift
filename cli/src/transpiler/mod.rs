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

/// A net-castable artifact
pub trait Castable {
    /// Cast the implementor into Artifacts; binary artifacts must be encoded with e.g. base64
    fn cast(&self, ctx: Rc<Context>, selector: Option<&str>) -> Result<Vec<Artifact>, CastError>;
}

pub trait Bindable {
    fn bind(&self, ctx: Rc<Context>) -> Result<(), CastError>;
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
