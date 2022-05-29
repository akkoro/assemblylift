use std::rc::Rc;
use std::sync::{Arc, Mutex};

use handlebars::{Handlebars, to_json};
use once_cell::sync::Lazy;
use serde::Serialize;

use crate::transpiler::{Castable, CastError, ContentType, StringMap};
use crate::transpiler::context;
use crate::transpiler::context::Context;

pub mod aws_lambda;
pub mod aws_lambda_alpine;
pub mod gloo;
pub mod k8s_hyper_alpine;

pub type ProviderMap = StringMap<Mutex<Box<dyn Provider + Send + Sync>>>;

pub static SERVICE_PROVIDERS: Lazy<ProviderMap> = Lazy::new(|| {
    let mut map = ProviderMap::new();
    map.insert(
        String::from("aws-lambda"),
        Mutex::new(Box::new(aws_lambda::ServiceProvider::new())),
    );
    // map.insert(
    //     String::from("aws-lambda-alpine"),
    //     Mutex::new(Box::new(aws_lambda_alpine::ServiceProvider::new())),
    // );
    map.insert(
        String::from("k8s-hyper-alpine"),
        Mutex::new(Box::new(k8s_hyper_alpine::KubernetesProvider::new())),
    );
    map
});

pub type Options = StringMap<String>;

pub trait Provider: Castable {
    fn name(&self) -> String;
    fn options(&self) -> Arc<Options>;
    fn set_options(&mut self, opts: Arc<Options>) -> Result<(), ProviderError>;
}

#[derive(Debug)]
pub enum ProviderError {
    TransformationError(String),
}

pub fn render_string_list(list: Rc<Vec<String>>) -> String {
    let mut out = String::from("[");
    for (i, l) in list.iter().enumerate() {
        out.push_str(&format!("\"{}\"", l));
        if i < list.len() - 1 {
            out.push_str(",");
        }
    }
    out.push_str("]");
    out
}
