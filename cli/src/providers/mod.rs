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
        Mutex::new(Box::new(aws_lambda::ServiceProvider)),
    );
    // map.insert(
    //     String::from("aws-lambda-alpine"),
    //     Mutex::new(Box::new(aws_lambda_alpine::ServiceProvider::new())),
    // );
    map.insert(
        String::from("k8s-hyper-alpine"),
        Mutex::new(Box::new(k8s_hyper_alpine::ServiceProvider::new())),
    );
    map
});
pub static FUNCTION_PROVIDERS: Lazy<ProviderMap> = Lazy::new(|| {
    let mut map = ProviderMap::new();
    map.insert(
        String::from("aws-lambda"),
        Mutex::new(Box::new(aws_lambda::FunctionProvider)),
    );
    // map.insert(
    //     String::from("aws-lambda-alpine"),
    //     Mutex::new(Box::new(aws_lambda_alpine::FunctionProvider::new())),
    // );
    map.insert(
        String::from("k8s-hyper-alpine"),
        Mutex::new(Box::new(k8s_hyper_alpine::FunctionProvider::new())),
    );
    map
});
pub static ROOT_PROVIDERS: Lazy<ProviderMap> = Lazy::new(|| {
    let mut map = ProviderMap::new();
    map.insert(
        String::from("root"),
        Mutex::new(Box::new(RootProvider::new())),
    );
    map
});

pub type BoxedCastable = Box<dyn Castable>;
pub type Options = StringMap<String>;

pub trait Provider: Castable {
    fn name(&self) -> String;
    fn init(&self, ctx: Rc<Context>, name: &str) -> Result<(), ProviderError>;
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

#[derive(Serialize)]
pub struct RootData {
    pub project_name: String,
    pub project_path: String,
    pub user_inject: bool,
    pub remote_state: bool,
    pub state_bucket_name: Option<String>,
    pub lock_table_name: Option<String>,
}

pub struct RootProvider<'a> {
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl RootProvider<'_> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a> Castable for RootProvider<'a> {
    fn cast(&mut self, ctx: Rc<Context>, name: &str) -> Result<Vec<String>, CastError> {
        use std::fs;
        use std::path::PathBuf;

        let mut usermod_path = PathBuf::from(
            crate::projectfs::locate_asml_manifest()
                .expect("could not locate assemblylift.toml")
                .1,
        );
        usermod_path.pop();
        usermod_path.push("user_tf/");
        let user_inject: bool = fs::metadata(usermod_path.clone()).is_ok();
        let (remote_state, state_bucket_name, lock_table_name) = match &ctx.terraform {
            Some(tf) => (
                true,
                Some(tf.state_bucket_name.clone()),
                Some(tf.lock_table_name.clone()),
            ),
            None => (false, None, None),
        };

        let data = RootData {
            project_name: ctx.project.name.clone(),
            project_path: ctx.project.path.clone(),
            user_inject,
            remote_state,
            state_bucket_name,
            lock_table_name,
        };
        let data = to_json(data);

        let mut reg = Box::new(Handlebars::new());
        reg.register_template_string("hcl_template", ROOT_HCL_TEMPLATE)
            .unwrap();
        let rendered_hcl = reg.render("hcl_template", &data).unwrap();

        Ok(vec![rendered_hcl])
    }

    fn content_type(&self) -> Vec<ContentType> {
        vec![ContentType::HCL("HCL")]
    }
}

impl<'a> Provider for RootProvider<'a> {
    fn name(&self) -> String {
        String::from("root")
    }

    fn init(&self, ctx: Rc<Context>, name: &str) -> Result<(), ProviderError> {
        Ok(())
    }

    fn options(&self) -> Arc<Options> {
        Arc::new(Options::new())
    }

    fn set_options(&mut self, _opts: Arc<Options>) -> Result<(), ProviderError> {
        Ok(())
    }
}

static ROOT_HCL_TEMPLATE: &str = r#"terraform {
    required_providers {
        docker = {
            source  = "kreuzwerker/docker"
            version = ">= 2.16.0"
        }
        kubernetes = {
          source  = "hashicorp/kubernetes"
          version = ">= 2.0.0"
        }
    }
}

locals {
    project_name = "{{project_name}}"
    project_path = "{{project_path}}"
}
{{#if user_inject}}
module "usermod" {
  source = "../user_tf"
}
{{/if}}
{{#if remote_state}}
terraform {
  backend "s3" {
    encrypt = true
    bucket = "{{state_bucket_name}}"
    dynamodb_table = "{{lock_table_name}}"
    key    = "terraform.tfstate"
    region = "us-east-1"
  }
}
{{/if}}
"#;
