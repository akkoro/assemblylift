use std::path::PathBuf;
use std::rc::Rc;

use anyhow::anyhow;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

use super::projectfs::Project as ProjectFs;
use crate::providers::{Platform, Provider, ProviderFactory};
use crate::{
    concat, concat_cast, snake_case, toml, CastError, CastResult, ContentType, Fragment, StringMap,
};

// NOTE The Context structure should provide everything at each level needed to cast to HCL, meaning
// there may be duplication that is not present in the TOML spec. The Context is IMMUTABLE so this should not be an issue,
// i.e. we are not modifying it directly and risking parts of the context going out-of-sync with itself

/// `Context` is a state object, containing the configuration of a project as deserialized from the
/// project and service manifests (TOML). `Context` is `Castable` and is the entrypoint of the `cast`
/// operation.
///
/// See docs/cli-transpiler.md
#[derive(Serialize, Deserialize)]
pub struct Context {
    pub project: Project,
    pub terraform: Option<Terraform>,
    pub platforms: Vec<Platform>,
    pub domains: Vec<Domain>,
    pub services: Vec<Service>,
    pub authorizers: Vec<Authorizer>,
    pub iomods: Vec<Iomod>,
    pub registries: Vec<Registry>,
}

impl Context {
    pub fn from_project(
        project: Rc<ProjectFs>,
        manifest: toml::asml::Manifest,
    ) -> Result<Self, String> {
        let mut ctx_services: Vec<Service> = Vec::new();
        let mut ctx_iomods: Vec<Iomod> = Vec::new();

        let ctx_platforms: Vec<Platform> = manifest.platforms.iter().map(Platform::from).collect();

        fn find_platform(platforms: Vec<Platform>, platform_id: &str, resource_id: &str) -> Result<Platform, String> {
            platforms.into_iter().find(|p| p.id.eq(platform_id)).ok_or(format!(
                "platform with id `{}` not found in assemblylift.toml manifest when casting provider for `{}`",
                platform_id,
                resource_id,
            ))
        }

        fn validate_provider_platform(
            provider: &Box<dyn Provider>,
            platform: Option<Platform>,
        ) -> Result<(), String> {
            match platform {
                Some(platform) => match provider.compatible_platforms().contains(&platform.name)
                    || provider.compatible_platforms().contains(&"*".into())
                {
                    true => Ok(()),
                    false => Err(format!(
                        "Provider `{}` is incompatible with Platform `{}`. Requires one of {:?}",
                        &provider.name(),
                        &platform.name,
                        provider.compatible_platforms(),
                    )),
                },
                None => Ok(()),
            }
        }

        let ctx_registries: Vec<Registry> = manifest
            .registries()
            .iter()
            .map(|reg| {
                let platform = match reg.provider.platform_id.as_deref() {
                    Some(pid) => Some(find_platform(ctx_platforms.clone(), pid, &reg.id)?),
                    None => None,
                };

                let provider = ProviderFactory::new_provider(
                    &reg.provider.name,
                    reg.provider.options.clone(),
                    platform.clone(),
                )
                .unwrap();

                validate_provider_platform(&provider, platform)?;

                Ok(Registry {
                    id: reg.id.clone(),
                    provider,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;

        let ctx_domains: Vec<Domain> = manifest
            .domains()
            .iter()
            .map(|domain| {
                let platform = match domain.provider.platform_id.as_deref() {
                    Some(pid) => Some(find_platform(ctx_platforms.clone(), pid, &domain.dns_name)?),
                    None => None,
                };

                let provider = ProviderFactory::new_provider(
                    &domain.provider.name,
                    domain.provider.options.clone(),
                    platform.clone(),
                )
                .unwrap();

                validate_provider_platform(&provider, platform)?;

                Ok(Domain {
                    dns_name: domain.dns_name.clone(),
                    map_to_root: domain.map_to_root,
                    provider,
                })
            })
            .collect::<Result<Vec<Domain>, String>>()?;

        // Authorizers are _defined_ in the root manifest so that custom authorizers (e.g. Lambda auth) can be defined once
        // They are generally _deployed_ per-service
        let ctx_authorizers: Vec<Authorizer> = manifest
            .authorizers()
            .iter()
            .map(|authorizer| Authorizer {
                id: authorizer.id.clone(),
                r#type: authorizer.auth_type.clone(),
                scopes: authorizer.scopes.clone().unwrap_or(Vec::<String>::new()),
                jwt_config: match authorizer.auth_type.clone().to_lowercase().as_str() {
                    "jwt" => Some(AuthorizerJwt {
                        audience: authorizer
                            .audience
                            .clone()
                            .expect("JWT authorizer requires audience field"),
                        issuer: authorizer
                            .issuer
                            .clone()
                            .expect("JWT authorizer requires issuer field"),
                    }),
                    _ => None,
                },
            })
            .collect();

        for service_ref in manifest.services {
            let mut service_path = project.service_dir(service_ref.name.clone()).dir();
            service_path.push("service.toml");
            let service_manifest = toml::service::Manifest::read(&service_path).unwrap();

            // let service = service_manifest.service();
            let functions = service_manifest.functions.clone();
            let iomods = service_manifest.iomods();

            let service_provider = service_ref.clone().provider;
            let gateway_provider = service_manifest.gateway.provider;

            let mut ctx_functions: Vec<Function> = Vec::new();
            for function in functions {
                // FIXME precompile should not be optional at context level
                let precompile = function.precompile.unwrap_or(true);
                let ext = match precompile {
                    true => "wasm.bin",
                    false => "wasm",
                };
                // FIXME language should not be optional at context level
                let language = function.language.clone().unwrap_or("rust".to_string());
                let environment_variables = function
                    .environment
                    .clone()
                    .unwrap_or(StringMap::<String>::new())
                    .iter()
                    .map(|e| (format!("__ASML_{}", e.0.clone()), e.1.clone()))
                    .collect::<StringMap<String>>();

                ctx_functions.push(Function {
                    name: function.name.clone(),
                    service_name: service_ref.name.clone(),
                    project_name: project.name.clone(),
                    coordinates: format!(
                        "{}.{}.{}",
                        &project.name, &service_ref.name, &function.name
                    ),
                    language: language.clone(),
                    // TODO this should not be inferred at context level
                    // handler_name: match language.clone().as_str() {
                    //     "rust" => format!("{}.component.{}", function.name.clone(), ext),
                    //     "ruby" => format!("ruby.component.{}", ext),
                    //     _ => "handler.wasm.bin".into(),
                    // },
                    handler_name: format!("{}.component.{}", function.name.clone(), ext),
                    // TODO this should not be inferred at context level
                    runtime_environment: match language.as_str() {
                        "rust" => "native",
                        "ruby" => "ruby",
                        _ => return Err("default".into()),
                    }
                    .into(),
                    // FIXME don't hardcode
                    runtime_version: "0.4.0-beta.0".into(),
                    size: function.size_mb.unwrap_or(1024u16),
                    timeout: function.timeout_seconds.unwrap_or(5u16),
                    cpu_compat_mode: function
                        .cpu_compat_mode
                        .clone()
                        .unwrap_or("default".to_string()),
                    precompiled: precompile,
                    http: match &function.clone().http.as_ref() {
                        Some(http) => Some(Http {
                            verb: http.verb.clone(),
                            path: http.path.clone(),
                        }),
                        None => None,
                    },
                    authorizer: match function.authorizer_id {
                        Some(auth_id) => match ctx_authorizers.iter().find(|&a| a.id == auth_id) {
                            Some(a) => Some(a.clone()),
                            None => {
                                return Err(format!(
                                "authorizer with id `{}` not found in assemblylift.toml manifest",
                                &auth_id
                            ))
                            }
                        },
                        None => None,
                    },
                    environment_variables,
                });
            }

            if service_ref.provider.platform_id.is_none() {
                return Err(format!(
                    "service `{}` has no defined platform_id",
                    &service_ref.name
                ));
            }
            
            let service_platform = Some(find_platform(
                ctx_platforms.clone(),
                &service_ref.provider.platform_id.unwrap(),
                &service_ref.name
            )?);

            let service_provider = ProviderFactory::new_provider(
                &service_provider.name,
                service_provider.options,
                service_platform.clone(),
            )
            .unwrap();

            validate_provider_platform(&service_provider, service_platform)?;

            let gateway_platform = service_provider.platform();
            if gateway_provider.platform_id.is_some() {
                return Err("gateway providers cannot currently define platform independent of service provider".into());
            }
            // let gateway_platform = match gateway_provider.platform_id {
            //     Some(pid) => Some(find_platform(ctx_platforms.clone(), &pid)?),
            //     None => service_provider.platform(),
            // };

            let gateway_provider = ProviderFactory::new_provider(
                &gateway_provider.name,
                gateway_provider.options,
                gateway_platform.clone(),
            )
            .unwrap();

            validate_provider_platform(&gateway_provider, gateway_platform)?;

            let gateway_compatible_service_providers = gateway_provider
                .as_gateway_provider()
                .unwrap()
                .compatible_service_providers();
            if !gateway_compatible_service_providers.contains(&service_provider.name()) {
                return Err(format!(
                    "Gateway Provider `{}` is incompatible with Service Provider `{}`. Requires one of {:?}",
                    &gateway_provider.name(),
                    &service_provider.name(),
                    gateway_compatible_service_providers,
                ));
            }

            ctx_services.push(Service {
                id: format!("{}_{}", service_ref.name.clone(), service_provider.platform().unwrap().id),
                name: service_ref.name.clone(),
                project_name: project.name.clone(),
                provider: service_provider,
                gateway: Gateway {
                    provider: gateway_provider,
                },
                functions: ctx_functions.clone(),
                container_registry: match service_ref.registry_id {
                    Some(rid) => match ctx_registries.iter().find(|&r| r.id.eq(&rid)) {
                        Some(registry) => Some(registry.into()),
                        None => {
                            return Err(format!(
                                "registry with id `{}` not found in assemblylift.toml manifest",
                                &rid
                            ))
                        }
                    },
                    None => None,
                },
                // TODO lift and validate against gateway provider
                domain: match service_ref.domain_name {
                    Some(service_domain) => ctx_domains
                        .iter()
                        .find(|&d| d.dns_name.eq(&service_domain))
                        .map(|d| d.into()),
                    None => None,
                },
                is_root: service_ref.is_root,
                has_ruby: ctx_functions.clone().iter().find(|&f| f.language.eq("ruby")).is_some(),
            });

            for iomod in iomods {
                let coords: Vec<&str> = iomod.coordinates.split('.').collect();
                let name = coords.get(2).unwrap();
                ctx_iomods.push(Iomod {
                    name: name.to_string(),
                    service_name: service_ref.name.clone(),
                    coordinates: iomod.coordinates.clone(),
                    version: iomod.version.clone(),
                });
            }
        }

        // TODO validate that IDs are unique

        Ok(Context {
            project: Project {
                name: manifest.project.name.clone(),
                path: (*project.dir()).to_str().unwrap().into(),
            },
            terraform: match manifest.terraform {
                Some(tf) => Some(Terraform {
                    state_bucket_name: tf.state_bucket_name,
                    lock_table_name: tf.lock_table_name,
                }),
                None => None,
            },
            platforms: ctx_platforms,
            domains: ctx_domains,
            services: ctx_services,
            authorizers: ctx_authorizers,
            iomods: ctx_iomods,
            registries: ctx_registries,
        })
    }

    pub fn service(&self, name: &str) -> Option<&Service> {
        self.services.iter().find(|&s| &s.name == name)
    }

    pub fn cast(&self) -> CastResult<Vec<Fragment>> {
        let mut fragments: Vec<Fragment> = Vec::new();

        let mut svc_out = self
            .services
            .iter()
            .map(|svc| {
                let gwy_provider = svc.gateway.provider.as_gateway_provider().unwrap();
                if !gwy_provider.is_booted() {
                    gwy_provider
                        .boot()
                        .map_err(|e| CastError(e.to_string()))?
                }
                let gwy_fragments = gwy_provider.cast_service(&svc);

                let cnr_provider = match &svc.container_registry {
                    Some(registry) => {
                        Some(registry.provider.as_container_registry_provider().unwrap())
                    }
                    None => None,
                };
                let cnr_fragments = match cnr_provider {
                    Some(cnr_provider) => {
                        if !cnr_provider.is_booted() {
                            cnr_provider.boot().map_err(|e| CastError(e.to_string()))?
                        }
                        cnr_provider.cast_service(&svc)
                    }
                    None => Ok(Vec::new()),
                };

                let dns_fragments = match &svc.domain {
                    Some(domain) => {
                        let dns_provider = domain.provider.as_dns_provider().unwrap();
                        match dns_provider
                            .compatible_gateway_providers()
                            .iter()
                            .find(|&p| p.eq(&gwy_provider.name()))
                        {
                            Some(_) => {
                                if !dns_provider.is_booted() {
                                    dns_provider.boot().map_err(|e| CastError(e.to_string()))?
                                }
                                dns_provider.cast_service(&svc)
                            }
                            None => Err(CastError(format!(
                                "DNS provider `{}` is not compatible with API provider `{}`",
                                dns_provider.name(),
                                gwy_provider.name(),
                            ))),
                        }
                    }
                    None => Ok(Vec::new()),
                };

                let svc_provider = svc.provider.as_service_provider().unwrap();
                if !svc_provider.is_booted() {
                    svc_provider.boot().map_err(|e| CastError(e.to_string()))?
                }
                let svc_fragments = svc_provider.cast_service(&svc);

                let mut out = Vec::new();
                out.append(&mut gwy_fragments?);
                out.append(&mut cnr_fragments?);
                out.append(&mut dns_fragments?);
                out.append(&mut svc_fragments?);

                Ok(out)
            })
            .reduce(concat_cast)
            .unwrap_or(Ok(Vec::new()))?;

        let mut dns_out = self
            .domains
            .iter()
            .map(|domain| {
                domain
                    .provider
                    .as_dns_provider()
                    .unwrap()
                    .cast_domain(&domain)
            })
            .reduce(concat_cast)
            .unwrap_or(Ok(Vec::new()))?;

        let mut hbs = Handlebars::new();
        hbs.register_helper("concat", Box::new(concat));
        hbs.register_helper("snake_case", Box::new(snake_case));
        hbs.register_template_string("context", include_str!("templates/context.tf.handlebars"))
            .unwrap();
        hbs.register_template_string(
            &crate::providers::aws_lambda::provider_name(),
            include_str!("providers/aws_lambda/templates/service_inst.tf.handlebars"),
        )
        .unwrap();
        hbs.register_template_string(
            &crate::providers::api_gateway::provider_name(),
            include_str!("providers/api_gateway/templates/api_inst.tf.handlebars"),
        )
        .unwrap();
        hbs.register_template_string(
            &format!("{}-root", crate::providers::route53::provider_name()),
            include_str!("providers/route53/templates/dns_inst_root.tf.handlebars"),
        )
        .unwrap();
        hbs.register_template_string(
            &crate::providers::route53::provider_name(),
            include_str!("providers/route53/templates/dns_inst.tf.handlebars"),
        )
        .unwrap();
        hbs.register_template_string(
            &format!("{}-root", crate::providers::ecr::provider_name()),
            include_str!("providers/ecr/templates/ecr_inst_root.tf.handlebars"),
        )
        .unwrap();
        hbs.register_template_string(
            &crate::providers::ecr::provider_name(),
            include_str!("providers/ecr/templates/ecr_inst.tf.handlebars"),
        )
        .unwrap();
        hbs.register_template_string(
            &crate::providers::kubernetes::provider_name(),
            include_str!("providers/kubernetes/templates/service_inst.tf.handlebars"),
        )
        .unwrap();
        hbs.register_template_string(
            &crate::providers::gloo::provider_name(),
            include_str!("providers/gloo/templates/api_inst.tf.handlebars"),
        )
        .unwrap();

        let mut ctx_out = vec![Fragment {
            content_type: ContentType::HCL,
            content: hbs.render("context", &self.as_json().unwrap()).unwrap(),
            write_path: PathBuf::from(format!("net/{}.tf", self.project.name)),
        }];

        fragments.append(&mut svc_out);
        fragments.append(&mut dns_out);
        fragments.append(&mut ctx_out);

        Ok(fragments)
    }

    pub fn as_json(&self) -> anyhow::Result<serde_json::Value> {
        serde_json::to_value(self).map_err(|e| anyhow!(e))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub path: String,
    //    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct Terraform {
    pub state_bucket_name: String,
    pub lock_table_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Registry {
    pub id: String,
    pub provider: Box<dyn Provider>,
}

impl From<&Registry> for Registry {
    fn from(value: &Registry) -> Self {
        Self {
            id: value.id.clone(),
            provider: ProviderFactory::new_provider(
                &value.provider.name(),
                value.provider.options().clone(),
                value.provider.platform().clone(),
            )
            .unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Domain {
    pub dns_name: String,
    pub map_to_root: bool,
    pub provider: Box<dyn Provider>,
}

impl Domain {
    pub fn as_json(&self) -> anyhow::Result<serde_json::Value> {
        serde_json::to_value(self).map_err(|e| anyhow!(e))
    }
}

impl From<&Domain> for Domain {
    fn from(value: &Domain) -> Self {
        Self {
            dns_name: value.dns_name.clone(),
            map_to_root: value.map_to_root.clone(),
            provider: ProviderFactory::new_provider(
                &value.provider.name(),
                value.provider.options().clone(),
                value.provider.platform().clone(),
            )
            .unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Gateway {
    pub provider: Box<dyn Provider>,
}

#[derive(Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    pub name: String,
    pub project_name: String,
    pub provider: Box<dyn Provider>,
    pub gateway: Gateway,
    pub functions: Vec<Function>,
    pub container_registry: Option<Registry>,
    pub domain: Option<Domain>,
    pub is_root: Option<bool>,
    pub has_ruby: bool,
}

impl Service {
    pub fn as_json(&self) -> anyhow::Result<serde_json::Value> {
        serde_json::to_value(self).map_err(|e| anyhow!(e))
    }
}

impl From<&Service> for Service {
    fn from(value: &Service) -> Self {
        Self {
            id: value.id.clone(),
            name: value.name.clone(),
            project_name: value.project_name.clone(),
            provider: ProviderFactory::new_provider(
                &value.provider.name(),
                value.provider.options().clone(),
                value.provider.platform().clone(),
            )
            .unwrap(),
            gateway: Gateway {
                provider: ProviderFactory::new_provider(
                    &value.gateway.provider.name(),
                    value.gateway.provider.options().clone(),
                    value.gateway.provider.platform().clone(),
                )
                .unwrap(),
            },
            functions: value.functions.clone(),
            container_registry: match value.container_registry.as_ref() {
                Some(reg) => Some(reg.into()),
                None => None,
            },
            domain: match value.domain.as_ref() {
                Some(domain) => Some(domain.into()),
                None => None,
            },
            is_root: value.is_root,
            has_ruby: value.has_ruby,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Function {
    pub name: String,
    pub service_name: String,
    pub project_name: String,
    pub coordinates: String,
    pub language: String,
    pub handler_name: String,
    pub runtime_environment: String,
    pub runtime_version: String,
    pub environment_variables: StringMap<String>,
    pub http: Option<Http>,
    pub authorizer: Option<Authorizer>,
    pub size: u16,
    pub timeout: u16,
    pub cpu_compat_mode: String,
    pub precompiled: bool,
}

impl Function {
    pub fn as_json(&self) -> anyhow::Result<serde_json::Value> {
        serde_json::to_value(self).map_err(|e| anyhow!(e))
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Http {
    pub verb: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Authorizer {
    pub id: String,
    pub r#type: String,
    pub scopes: Vec<String>,
    pub jwt_config: Option<AuthorizerJwt>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthorizerJwt {
    pub issuer: String,
    pub audience: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Iomod {
    pub name: String,
    pub service_name: String,
    pub coordinates: String,
    pub version: String,
}
