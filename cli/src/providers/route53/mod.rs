use std::rc::Rc;
use std::sync::Arc;

use handlebars::Handlebars;
use serde::Serialize;

use crate::providers::{Options, Provider, ProviderError};
use crate::transpiler::{Artifact, Bindable, Bootable, Castable, CastError, ContentType, Template};
use crate::transpiler::context::Context;

pub struct DnsProvider {
    /// access_key_id, secret_key, aws_region, hosted_zone_id
    options: Arc<Options>,
}

impl DnsProvider {
    pub fn new() -> Self {
        Self {
            options: Arc::new(Options::new()),
        }
    }
}

impl Castable for DnsProvider {
    fn cast(&self, ctx: Rc<Context>, _selector: Option<&str>) -> Result<Vec<Artifact>, CastError> {
        let project_name = ctx.project.name.clone();
        // let domains = ctx.domains;

        let rendered_hcl = Route53Template {
            project_name,
            hosted_zone_id: "".to_string(),
        }.render();
        let out = Artifact {
            content_type: ContentType::HCL("HCL"),
            content: rendered_hcl,
            write_path: "net/plan.tf".into(),
        };
        Ok(vec![out])
    }
}

impl Bindable for DnsProvider {
    fn bind(&self, ctx: Rc<Context>) -> Result<(), CastError> {
        Ok(())
    }
}

impl Bootable for DnsProvider {
    fn boot(&self, ctx: Rc<Context>) -> Result<(), CastError> {
        Ok(())
    }

    fn is_booted(&self, ctx: Rc<Context>) -> bool {
        true
    }
}

impl Provider for DnsProvider {
    fn name(&self) -> String {
        String::from("route53")
    }

    fn options(&self) -> Arc<Options> {
        self.options.clone()
    }

    fn set_options(&mut self, opts: Arc<Options>) -> Result<(), ProviderError> {
        self.options = opts;
        Ok(())
    }
}

#[derive(Serialize)]
struct Route53Template {
    project_name: String,
    hosted_zone_id: String,
}

impl Template for Route53Template {
    fn render(&self) -> String {
        let mut reg = Box::new(Handlebars::new());
        reg.register_template_string("tmpl", Self::tmpl()).unwrap();
        reg.render("tmpl", &self).unwrap()
    }

    fn tmpl() -> &'static str {
        r#"#Begin Route53
        "#
    }
}
