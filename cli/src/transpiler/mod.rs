use std::rc::Rc;

pub mod asml;
pub mod hcl;
pub mod kube;
pub mod toml;

pub type Map<K, V> = std::collections::HashMap<K, V>;
pub type StringMap<V> = Map<String, V>;

pub enum ContentType {
    HCL(&'static str),
    Dockerfile(&'static str),
    KubeYaml(&'static str),
}

/// A net-castable artifact
pub trait Castable {
    /// Cast the implementor into Strings; binary artifacts must be encoded with e.g. base64
    fn cast(&mut self, ctx: Rc<asml::Context>, name: &str) -> Result<Vec<String>, CastError>;
    /// The types of document this Castable will `cast` to
    fn content_type(&self) -> Vec<ContentType>;
}

#[derive(Debug)]
pub struct CastError(pub String);
