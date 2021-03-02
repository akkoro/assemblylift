pub mod hcl;
pub mod models;
pub mod toml;

pub type Map<K, V> = std::collections::HashMap<K, V>;
pub type StringMap<V> = Map<String, V>;

pub enum ContentType {
    HCL(&'static str),
    TOML(&'static str),
}

/* Represents source data for an AssemblyLift service */
pub trait Source {
    fn content_type(&self) -> ContentType;
    
    fn service(&self) -> models::Service;
    fn functions(&self) -> models::Functions;
    fn iomods(&self) -> models::Iomods;
}

/* Represents a compiled `cast` artifact */
pub trait Artifact {
    fn content_type(&self) -> ContentType;
    fn content(&self) -> String;
}
