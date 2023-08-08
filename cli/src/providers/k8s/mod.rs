use std::rc::Rc;
use std::sync::Arc;

use clap::crate_version;
use itertools::Itertools;

use crate::providers::{flatten, gloo, Options, Provider, ProviderError, KUBERNETES_PROVIDER_NAME};
use crate::tools::glooctl::GlooCtl;
use crate::transpiler::context::Context;
use crate::transpiler::{
    context, Artifact, Bindable, Bootable, CastError, Castable, ContentType, StringMap, Template,
};

use self::templates::*;

mod templates;

fn to_container_registry(r: &context::Registry) -> ContainerRegistry {
    ContainerRegistry {
        is_dockerhub: r.host.eq_ignore_ascii_case("dockerhub"),
        is_ecr: r.host.eq_ignore_ascii_case("ecr"),
        options: r.options.clone(),
    }
}

pub struct KubernetesProvider {
    api_provider: Arc<gloo::ApiProvider>,
    service_subprovider: KubernetesService,
    options: Arc<Options>,
}

impl KubernetesProvider {
    pub fn new() -> Self {
        let api_provider = Arc::new(gloo::ApiProvider::new());
        Self {
            api_provider: api_provider.clone(),
            service_subprovider: KubernetesService {
                api_provider: api_provider.clone(),
                options: Arc::new(Options::new()),
            },
            options: Arc::new(Options::new()),
        }
    }
}

impl Provider for KubernetesProvider {
    fn name(&self) -> String {
        String::from(KUBERNETES_PROVIDER_NAME)
    }

    fn options(&self) -> Arc<Options> {
        self.options.clone()
    }

    fn set_options(&mut self, opts: Arc<Options>) -> Result<(), ProviderError> {
        self.options = opts.clone();
        self.service_subprovider.options = opts.clone();
        Ok(())
    }
}

// TODO kube context name as provider option
impl Castable for KubernetesProvider {
    fn cast(&self, ctx: Rc<Context>, _selector: Option<&str>) -> Result<Vec<Artifact>, CastError> {
        GlooCtl::default().install_gateway();
        let registries = ctx.registries.iter().map(to_container_registry).collect();

        let mut service_artifacts = ctx
            .services
            .iter()
            .filter(|&s| s.provider.name == self.name())
            .map(|s| {
                self.service_subprovider
                    .cast(ctx.clone(), Some(&s.name))
                    .unwrap()
            })
            .reduce(flatten)
            .unwrap();

        let base_tmpl = KubernetesBaseTemplate {
            project_name: ctx.project.name.clone(),
            docker_config_path: self
                .options
                .get("docker_config_path")
                .unwrap_or(&"~/.docker/config.json".to_string())
                .clone(),
            registries,
        };
        let hcl = Artifact {
            content_type: ContentType::HCL("HCL"),
            content: base_tmpl.render(),
            write_path: "net/plan.tf".to_string(),
        };

        let mut out = vec![hcl];

        out.append(&mut service_artifacts);
        Ok(out)
    }
}

impl Bindable for KubernetesProvider {
    fn bind(&self, ctx: Rc<Context>) -> Result<(), CastError> {
        ctx.services
            .iter()
            .filter(|&s| s.provider.name == self.name())
            .map(|s| self.service_subprovider.bind(ctx.clone()))
            .collect_vec();
        Ok(())
    }
}

impl Bootable for KubernetesProvider {
    fn boot(&self, ctx: Rc<Context>) -> Result<(), CastError> {
        self.api_provider.boot(ctx)
    }

    fn is_booted(&self, ctx: Rc<Context>) -> bool {
        self.api_provider.is_booted(ctx)
    }
}

struct KubernetesService {
    api_provider: Arc<gloo::ApiProvider>,
    options: Arc<Options>,
}

impl Castable for KubernetesService {
    fn cast(&self, ctx: Rc<Context>, selector: Option<&str>) -> Result<Vec<Artifact>, CastError> {
        let name = selector
            .expect("selector must be a service name")
            .to_string();

        let registries = ctx.registries.iter().map(to_container_registry).collect();

        let hcl_content = ServiceTemplate {
            project_name: ctx.project.name.clone(),
            service_name: name.clone(),
            kube_config_path: self
                .options
                .get("kube_config_path")
                .unwrap_or(&"~/.kube/config".to_string())
                .clone(),
            registries,
        }
        .render();

        // TODO in future we want to support other API Gateway providers -- for now, just Gloo :)
        let mut api_artifacts = self.api_provider.cast(ctx.clone(), Some(&*name)).unwrap();

        let function_subprovider = KubernetesFunction {
            service_name: name.clone(),
            options: self.options.clone(),
        };
        let function_artifacts = ctx
            .functions
            .iter()
            .filter(|f| f.service_name == name)
            .map(|f| {
                function_subprovider
                    .cast(ctx.clone(), Some(&f.name))
                    .unwrap()
            })
            .reduce(flatten)
            .unwrap();
        let function_hcl = function_artifacts
            .iter()
            .filter(|a| a.content_type == ContentType::HCL("HCL"))
            .map(|artifact| artifact.content.clone())
            .reduce(|accum, s| format!("{}{}", &accum, &s))
            .unwrap();

        let hcl = Artifact {
            content_type: ContentType::HCL("HCL"),
            content: format!("{}{}", &hcl_content, &function_hcl),
            write_path: "net/plan.tf".into(),
        };
        let mut out = vec![hcl];
        out.append(&mut api_artifacts);
        out.append(
            &mut function_artifacts
                .iter()
                .filter(|a| a.content_type == ContentType::Dockerfile("Dockerfile"))
                .map(|a| a.clone())
                .collect::<Vec<Artifact>>(),
        );
        Ok(out)
    }
}

impl Bindable for KubernetesService {
    fn bind(&self, ctx: Rc<Context>) -> Result<(), CastError> {
        self.api_provider.bind(ctx)
    }
}

struct KubernetesFunction {
    service_name: String,
    options: Arc<Options>,
}

impl Castable for KubernetesFunction {
    fn cast(&self, ctx: Rc<Context>, selector: Option<&str>) -> Result<Vec<Artifact>, CastError> {
        let name = selector
            .expect("selector must be a function name")
            .to_string();
        match ctx
            .functions
            .iter()
            .filter(|&f| f.service_name == self.service_name)
            .find(|&f| f.name == name)
        {
            Some(function) => {
                let service = function.service_name.clone();

                let iomods: Vec<IomodContainer> = ctx
                    .iomods
                    .iter()
                    .filter(|i| i.service_name == service.clone())
                    .map(|i| {
                        let coords: Vec<&str> = i.coordinates.split(".").collect();
                        IomodContainer {
                            // TODO eventually we'll allow overriding which service/mirror the IOmods come from
                            image: format!(
                                "public.ecr.aws/{}/iomod/{}/{}:{}",
                                coords[0], coords[1], coords[2], i.version
                            ),
                            name: i.coordinates.clone().replacen('.', "-", 2),
                        }
                    })
                    .collect();

                let registries: Vec<ContainerRegistry> =
                    ctx.registries.iter().map(to_container_registry).collect();

                let environment: Vec<ContainerEnv> = function
                    .environment
                    .clone()
                    .unwrap_or(Rc::new(StringMap::<String>::new()))
                    .iter()
                    .map(|e| ContainerEnv {
                        name: format!("__ASML_{}", e.0.clone()),
                        value: e.1.clone(),
                    })
                    .collect();

                let ext = match function.precompile {
                    true => "component.wasm.bin",
                    false => "component.wasm",
                };

                let hcl_tmpl = FunctionTemplate {
                    base_image_version: crate_version!().to_string(),
                    project_name: ctx.project.name.clone(),
                    function_name: function.name.clone(),
                    service_name: service.clone(),
                    handler_name: match function.language.as_str() {
                        "rust" => format!("{}.{}", function.name.clone(), ext),
                        "ruby" => format!("ruby.{}", ext),
                        _ => "handler".into(),
                    },
                    iomods: iomods.clone(),
                    has_iomods: iomods.len() > 0,
                    registry: ContainerRegistry {
                        is_dockerhub: function.registry.eq_ignore_ascii_case("dockerhub"),
                        is_ecr: function.registry.eq_ignore_ascii_case("ecr"),
                        options: registries
                            .iter()
                            .find(|r| {
                                (r.is_ecr && function.registry.eq_ignore_ascii_case("ecr"))
                                    || (r.is_dockerhub
                                        && function.registry.eq_ignore_ascii_case("dockerhub"))
                            })
                            .expect(&*format!(
                                "no registry configured for `{}` for function `{}`",
                                &function.registry, &function.name
                            ))
                            .options
                            .clone(),
                    },
                    is_ruby: function.language == "ruby".to_string(),
                    cpu_compat_mode: function.cpu_compat_mode.clone(),
                    environment,
                };
                let hcl_content = hcl_tmpl.render();

                let dockerfile_content = DockerfileTemplate {
                    base_image_version: hcl_tmpl.base_image_version,
                    project_name: hcl_tmpl.project_name.clone(),
                    service_name: hcl_tmpl.service_name.clone(),
                    function_name: hcl_tmpl.function_name.clone(),
                    handler_name: hcl_tmpl.handler_name,
                    function_coordinates: format!(
                        "{}.{}.{}",
                        hcl_tmpl.project_name.clone(),
                        hcl_tmpl.service_name.clone(),
                        hcl_tmpl.function_name.clone()
                    ),
                    function_precompiled: match function.precompile {
                        true => "true".into(),
                        false => "false".into(),
                    },
                    is_ruby: hcl_tmpl.is_ruby,
                }
                .render();

                let hcl = Artifact {
                    content_type: ContentType::HCL("HCL"),
                    content: hcl_content,
                    write_path: "net/plan.tf".into(),
                };
                let dockerfile = Artifact {
                    content_type: ContentType::Dockerfile("Dockerfile"),
                    content: dockerfile_content,
                    write_path: format!(
                        "net/services/{}/{}/Dockerfile",
                        service.clone(),
                        function.name.clone()
                    )
                    .into(),
                };
                Ok(vec![hcl, dockerfile])
            }
            None => Err(CastError(format!(
                "unable to find function {} in context",
                name.clone()
            ))),
        }
    }
}
