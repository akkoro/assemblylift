pub mod asml;
pub mod hcl;
pub mod toml;

pub type Map<K, V> = std::collections::HashMap<K, V>;
pub type StringMap<V> = Map<String, V>;

pub enum ContentType {
    HCL(&'static str),
}

pub trait Artifact {
    fn content_type(&self) -> ContentType;
    fn content(&self) -> std::rc::Rc<Option<String>>;
    fn cast(&mut self) -> Result<String, ArtifactError>;
}

#[derive(Debug)]
pub struct ArtifactError(String);
