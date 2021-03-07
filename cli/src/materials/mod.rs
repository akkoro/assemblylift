pub mod hcl;
pub mod models;
pub mod toml;

pub type Map<K, V> = std::collections::HashMap<K, V>;
pub type StringMap<V> = Map<String, V>;

pub enum ContentType {
    HCL(&'static str),
    TOML(&'static str),
}

/* Represents a compiled `cast` artifact */
pub trait Artifact {
    fn content_type(&self) -> ContentType;
    fn content(&self) -> Option<String>;
    fn cast(&mut self) -> Result<String, ArtifactError>;
}

#[derive(Debug)]
pub struct ArtifactError;
