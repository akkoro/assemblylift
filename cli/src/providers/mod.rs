use std::rc::Rc;
use std::sync::{Arc, LockResult, Mutex, MutexGuard};

use once_cell::sync::Lazy;

use crate::transpiler::{Artifact, Bindable, Bootable, Castable, StringMap};

pub mod aws_lambda;
pub mod aws_lambda_alpine;
pub mod gloo;
pub mod k8s;
pub mod route53;

pub type ProviderBox = Box<dyn Provider + Send + Sync>;
pub type ProviderMap = StringMap<LockBox>;

pub struct LockBox(Mutex<ProviderBox>);

impl LockBox {
    pub fn new(p: impl Provider + Send + Sync + 'static) -> Self {
        Self(Mutex::new(Box::new(p)))
    }

    pub fn lock(&self) -> LockResult<MutexGuard<'_, ProviderBox>> {
        self.0.lock()
    }
}

pub static AWS_LAMBDA_PROVIDER_NAME: &str = "aws-lambda";
pub static KUBERNETES_PROVIDER_NAME: &str = "k8s";
pub static ROUTE53_PROVIDER_NAME: &str = "route53";

pub static PROVIDERS: Lazy<ProviderMap> = Lazy::new(|| {
    let mut map = ProviderMap::new();
    map.insert(
        String::from(AWS_LAMBDA_PROVIDER_NAME),
        LockBox::new(aws_lambda::AwsLambdaProvider::new()),
    );
    map.insert(
        String::from(KUBERNETES_PROVIDER_NAME),
        LockBox::new(k8s::KubernetesProvider::new()),
    );
    map
});

pub static DNS_PROVIDERS: Lazy<ProviderMap> = Lazy::new(|| {
    let mut dns_providers = ProviderMap::new();
    dns_providers.insert(
        String::from(ROUTE53_PROVIDER_NAME),
        LockBox::new(route53::DnsProvider::new()),
    );
    dns_providers
});

pub type Options = StringMap<String>;

pub trait Provider: Castable + Bindable + Bootable {
    fn name(&self) -> String;
    fn options(&self) -> Arc<Options>;
    fn set_options(&mut self, opts: Arc<Options>) -> Result<(), ProviderError>;
}

#[derive(Debug)]
pub enum ProviderError {
    TransformationError(String),
}

fn render_string_list(list: Rc<Vec<String>>) -> String {
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

fn render_string_map(map: StringMap<String>) -> String {
    let mut out = String::from("{");
    for (i, p) in map.iter().enumerate() {
        out.push_str(&format!("\"{}\"=\"{}\"", p.0, p.1));
        if i < map.len() - 1 {
            out.push_str(",");
        }
    }
    out.push_str("}");
    out
}

fn flatten(mut accum: Vec<Artifact>, mut v: Vec<Artifact>) -> Vec<Artifact> {
    let mut out = Vec::new();
    out.append(&mut accum);
    out.append(&mut v);
    out
}
