use std::rc::Rc;

use serde::Deserialize;

use crate::materials::StringMap;
use crate::providers::Transformable;

pub type Functions = StringMap<Function>;
pub type Iomods = StringMap<Dependency>;

#[derive(Deserialize)]
pub struct Service {
    pub name: String,
    pub provider: Option<String>,
}

impl Transformable for Service {
    const TYPE: &'static str = "service";
}

#[derive(Deserialize)]
pub struct Api {
    pub functions: Rc<StringMap<Function>>, // map function_id -> function
    pub authorizers: Rc<Option<StringMap<HttpAuth>>> // map auth_id -> authorizer
}

#[derive(Deserialize)]
pub struct HttpAuth {
    pub auth_type: String,
    pub audience: Rc<Option<Vec<String>>>, // TODO do these actually need to be Rc?
    pub issuer: Rc<Option<String>>,
}

#[derive(Deserialize)]
pub struct HttpFunction {
    pub verb: String,
    pub path: String,
}

#[derive(Deserialize)]
pub struct Function {
    pub name: String,
    pub provider: Option<String>,
    pub handler_name: Option<String>,

    pub http: Rc<Option<HttpFunction>>,
    pub authorizer_id: Option<String>,

    pub timeout_seconds: Option<u16>,
    pub size_mb: Option<u16>,
}

impl Transformable for Function {
    const TYPE: &'static str = "function";
}

#[derive(Deserialize)]
pub struct Iomod {
    pub dependencies: Rc<StringMap<Dependency>>, // map dependency_id -> dependency
}

#[derive(Clone, Deserialize)]
pub struct Dependency {
    pub from: String,
    pub version: String,
    #[serde(alias = "type")]
    pub dependency_type: String,
}
