use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use handlebars::Handlebars;
use itertools::Itertools;
use jsonpath_lib::Selector;
use serde::Serialize;

use crate::providers::{Options, Provider, ProviderError};
use crate::tools::kubectl::KubeCtl;
use crate::transpiler::context::Context;
use crate::transpiler::{Artifact, Bindable, Bootable, CastError, Castable, ContentType, Template};

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

    fn gloo_proxy_ip(&self) -> String {
        let mut labels = HashMap::new();
        labels.insert("gloo".to_string(), "gateway-proxy".to_string());
        let kubectl = KubeCtl::default();
        let gateways = kubectl
            .get_in_namespace("services", "gloo-system", Some(labels))
            .unwrap();
        let mut selector = Selector::new();
        selector
            .str_path("$.items[0].status.loadBalancer.ingress[0].ip")
            .unwrap()
            .value(&gateways)
            .select_as_str()
            .unwrap()
    }
}

impl Castable for DnsProvider {
    fn cast(&self, ctx: Rc<Context>, _selector: Option<&str>) -> Result<Vec<Artifact>, CastError> {
        let project_name = ctx.project.name.clone();
        let zones = ctx
            .domains
            .iter()
            .filter(|&d| d.provider.name == self.name())
            .map(|d| Zone {
                name: d.dns_name.clone(),
                name_snaked: d.dns_name.replace(".", "_"),
            })
            .collect_vec();
        let records = ctx
            .services
            .iter()
            .filter(|&s| s.domain_name.is_some())
            .map(|s| Record {
                name: s.name.clone(),
                target: "".to_string(), // TODO gloo or apigw (will need to lookup aws, run kubectl for gloo)
                zone: Zone {
                    name: s.domain_name.as_ref().unwrap().clone(),
                    name_snaked: s.domain_name.as_ref().unwrap().replace(".", "_"),
                },
            })
            .collect_vec();

        let rendered_hcl = Route53Template {
            project_name,
            records,
            zones,
        }
        .render();
        let out = Artifact {
            content_type: ContentType::HCL("HCL"),
            content: rendered_hcl,
            write_path: "net/plan.tf".into(),
        };
        Ok(vec![out])
    }
}

impl Bindable for DnsProvider {
    fn bind(&self, _ctx: Rc<Context>) -> Result<(), CastError> {
        Ok(())
    }
}

impl Bootable for DnsProvider {
    fn boot(&self, _ctx: Rc<Context>) -> Result<(), CastError> {
        Ok(())
    }

    fn is_booted(&self, _ctx: Rc<Context>) -> bool {
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
    records: Vec<Record>,
    zones: Vec<Zone>,
}

#[derive(Serialize)]
struct Record {
    name: String,
    target: String,
    zone: Zone,
}

#[derive(Serialize)]
struct Zone {
    name: String,
    name_snaked: String,
}

impl Template for Route53Template {
    fn render(&self) -> String {
        let mut reg = Box::new(Handlebars::new());
        reg.register_template_string("tmpl", Self::tmpl()).unwrap();
        reg.render("tmpl", &self).unwrap()
    }

    fn tmpl() -> &'static str {
        r#"#Begin Route53
{{#each zones}}data aws_route53_zone {{this.name_snaked}} {
  name = {{this.name}}
}{{/each}}
{{#each records}}
resource aws_route53_record {{this.name}} {
  zone_id = data.aws_route53_zone.{{this.zone.name_snaked}}.zone_id
  name    = "{{this.name}}.{{this.zone.name}}"
  type    = "A"
  ttl     = "300"
  records = ["{{this.target}}"]
}
{{/each}}
"#
    }
}
