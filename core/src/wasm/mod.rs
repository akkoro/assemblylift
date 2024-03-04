mod cache;

use std::borrow::Cow;
use std::fs::File;
use std::path::Path;
use std::string::ToString;
use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Context};
use wasmtime_wasi::sync::Dir;
use wasmtime_wasi::preview2;
use wasmtime_wasi::preview2::{DirPerms, FilePerms, WasiView};
pub use crossbeam_channel::bounded as status_channel;
use once_cell::sync::Lazy;
use tracing::debug;
use uuid::Uuid;
use wasmtime::{component, AsContextMut, AsContext};
use wasmtime::component::{Component, ResourceTable};
use wasmtime::{Config, Engine, Store};
use wit_component::{ComponentEncoder, StringEncoding};
use wasm_encoder::{Encode, Section};
use wit_parser::{PackageId, Resolve, UnresolvedPackage, WorldId};

use assemblylift_core_iomod::registry::RegistryTx;

pub use asml_wit::akkoro::assemblylift::asml_io;
pub use asml_wit::akkoro::assemblylift::asml_rt;
use jwt_wit::akkoro::jwt;
use opa_wit::akkoro::opa;
use secrets_wit::akkoro::secrets::secret_storage;

use crate::jwt::keyset::KeyStore as JwtKeyStore;
use crate::policy_manager::PolicyManager;
use crate::threader::Threader;
use crate::wasm::cache::Cache;
use crate::RuntimeAbi;

// pub type State<R, S> = AsmlFunctionState<R, S>;
pub type StatusTx<S> = crossbeam_channel::Sender<S>;
pub type StatusRx<S> = crossbeam_channel::Receiver<S>;

pub static CPU_COMPAT_MODE: Lazy<String> =
    Lazy::new(|| std::env::var("ASML_CPU_COMPAT_MODE").unwrap_or("default".to_string()));

mod asml_wit { wasmtime::component::bindgen!("assemblylift" in "wit/assemblylift"); }
mod jwt_wit { wasmtime::component::bindgen!("jwt" in "wit/jwt"); }
mod opa_wit { wasmtime::component::bindgen!("opa" in "wit/opa"); }
mod secrets_wit { wasmtime::component::bindgen!("secrets" in "wit/secrets"); }
// bindgen!("wasi-secrets" in "components/wasi-secrets/wit");
pub struct Wasmtime<R, S>
where
    R: RuntimeAbi<S> + Send + 'static,
    S: Clone + Send + Sized + 'static,
{
    engine: Engine,
    component: Component,
    cache: Arc<Mutex<Cache>>,
    _phantom_r: std::marker::PhantomData<R>,
    _phantom_s: std::marker::PhantomData<S>,
}

impl<R, S> Wasmtime<R, S>
where
    R: RuntimeAbi<S> + Send + 'static,
    S: Clone + Send + Sized + 'static,
{
    fn new_component(path: &Path) -> anyhow::Result<(Engine, Component)> {
        match path.extension().unwrap().to_str().unwrap() {
            "bin" => {
                let target = Self::get_target();
                let engine = new_engine(target, None)?;
                let component = unsafe { Component::deserialize_file(&engine, path) }
                    .expect("could not deserialize component");
                Ok((engine, component))
            }
            "wasm" => {
                let engine = new_engine(None, None)?;
                let component = Component::from_file(&engine, path)
                    .expect("could not deserialize component");
                Ok((engine, component))
            }
            _ => {
                Err(anyhow!(
                    "invalid module extension; must be .wasm or .wasm.bin"
                ))
            }
        }
    }

    fn get_target() -> Option<&'static str> {
        match std::env::consts::OS {
            "macos" => Some("x86_64-apple-darwin"),
            "linux" => Some("x86_64-linux-gnu"),
            _ => None,
        }
    }
}

impl<R, S> Wasmtime<R, S>
where
    R: RuntimeAbi<S> + Send + 'static,
    S: Clone + Send + Sized + 'static,
{
    pub fn new_from_path(path: &Path) -> anyhow::Result<Self> {
        let ec = Self::new_component(path);
        match ec {
            Ok(ec) => Ok(Self {
                engine: ec.0,
                component: ec.1,
                cache: Arc::new(Mutex::new(Cache::new())),
                _phantom_r: Default::default(),
                _phantom_s: Default::default(),
            }),
            Err(err) => Err(anyhow!(err)),
        }
    }

    pub async fn link_wasi_component(
        &mut self,
        registry_tx: RegistryTx,
        status_tx: StatusTx<S>,
        environment_vars: Vec<(String, String)>,
        runtime_environment: String,
        bind_paths: Vec<(String, String)>,
        request_id: Option<String>,
        input: &[u8],
    ) -> anyhow::Result<(
        preview2::command::Command,
        Store<AsmlComponentFunctionState<R, S>>,
    )> {
        let threader = Arc::new(Mutex::new(Threader::new(registry_tx)));
        let mut linker: component::Linker<AsmlComponentFunctionState<R, S>> = component::Linker::new(&self.engine);

        wasmtime_wasi::preview2::command::add_to_linker(&mut linker)
            .expect("could not link wasi runtime component");
        asml_wit::Assemblylift::add_to_linker(&mut linker, |s| s)
            .expect("could not link assemblylift runtime component");
        jwt_wit::Jwt::add_to_linker(&mut linker, |s| s)
            .expect("could not link jwt runtime component");
        opa_wit::Opa::add_to_linker(&mut linker, |s| s)
            .expect("could not link opa runtime component");
        secrets_wit::Secrets::add_to_linker(&mut linker, |s| s)
            .expect("could not link secrets runtime component");

        let mut builder = &mut preview2::WasiCtxBuilder::new();
        for e in environment_vars {
            builder = builder.env(&*e.0, &*e.1);
        }

        match runtime_environment.as_str() {
            "ruby" => builder = builder.inherit_stdio().env("RUBY_PLATFORM", "wasm32-wasi").args(&["ruby", "/src/handler.rb"]),
            _ => (),
        }

        for path in bind_paths {
            debug!("binding {} to {}", path.0, path.1);
            builder = builder.preopened_dir(
                Dir::from_std_file(
                    File::open(path.0).unwrap(),
                ),
                DirPerms::all(), 
                FilePerms::all(),
                path.1,
            );
        }
        builder = builder.preopened_dir(
            Dir::from_std_file(
                File::open("/tmp/asmltmp").unwrap(),
            ),
            DirPerms::all(), 
            FilePerms::all(),
            "/tmp",
        );
        builder = builder.inherit_stdout();
        builder = builder.inherit_stderr();

        let table = ResourceTable::new();
        let wasi = builder.build();

        let state = AsmlComponentFunctionState {
            function_input: input.to_vec(),
            status_sender: status_tx,
            policy_manager: Arc::new(Mutex::new(PolicyManager::new())),
            threader,
            request_id,
            cache: self.cache.clone(),
            wasi,
            table,
            _phantom: Default::default(),
        };
        let mut store = Store::new(&self.engine, state);

        match wasmtime_wasi::preview2::command::Command::instantiate_async(
            &mut store,
            &self.component,
            &linker,
        )
        .await
        {
            Ok((wasi, _instance)) => Ok((wasi, store)),
            Err(err) => Err(anyhow!(err)),
        }
    }

    pub async fn run_component<CTX>(
        &mut self,
        wasi: wasmtime_wasi::preview2::command::Command,
        mut store: CTX,
    ) -> anyhow::Result<()>
    where
        CTX: AsContextMut,
        <CTX as AsContext>::Data: std::marker::Send,
    {
        wasi
            .wasi_cli_run()
            .call_run(&mut store)
            .await?
            .map_err(|()| anyhow::anyhow!("command returned with failing exit status"))
    }
}

pub struct AsmlComponentFunctionState<R, S>
where
    R: RuntimeAbi<S> + Send + 'static,
    S: Clone + Send + Sized + 'static,
{
    status_sender: StatusTx<S>,
    threader: Arc<Mutex<Threader<S>>>,
    policy_manager: Arc<Mutex<PolicyManager>>,
    function_input: Vec<u8>,
    request_id: Option<String>,
    cache: Arc<Mutex<Cache>>,
    wasi: preview2::WasiCtx,
    table: ResourceTable,
    _phantom: std::marker::PhantomData<R>,
}

impl<R, S> WasiView for AsmlComponentFunctionState<R, S>
where
    R: RuntimeAbi<S> + Send + 'static,
    S: Clone + Send + Sized + 'static,
{
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    // fn table_mut(&mut self) -> &mut wasmtime_wasi::preview2::Table {
    //     &mut self.table
    // }

    fn ctx(&mut self) -> &mut preview2::WasiCtx {
        &mut self.wasi
    }

    // fn ctx_mut(&mut self) -> &mut preview2::WasiCtx {
    //     &mut self.wasi
    // }
}

impl<R, S> asml_io::Host for AsmlComponentFunctionState<R, S>
where
    R: RuntimeAbi<S> + Send + 'static,
    S: Clone + Send + Sized + 'static,
{
    fn invoke(
        &mut self,
        path: String,
        input: String,
    ) -> anyhow::Result<Result<asml_io::Ioid, asml_io::IoError>> {
        let ioid = self
            .threader
            .clone()
            .lock()
            .unwrap()
            .next_ioid()
            .expect("unable to get a new IO ID");

        if let Err(err) = self.threader
            .clone()
            .lock()
            .unwrap()
            .invoke(&path, input.into_bytes(), ioid)
        {
            return Ok(Err(err));
        }

        Ok(Ok(ioid as asml_io::Ioid))
    }

    fn poll(&mut self, ioid: asml_io::Ioid) -> anyhow::Result<Result<Vec<u8>, asml_io::PollError>> {
        match self.threader.clone().lock().unwrap().poll(ioid) {
            Some(response) => Ok(Ok(response)),
            None => Ok(Err(asml_io::PollError::NotReady)),
        }
    }
}

impl<R, S> asml_rt::Host for AsmlComponentFunctionState<R, S>
where
    R: RuntimeAbi<S> + Send + 'static,
    S: Clone + Send + Sized + 'static,
{
    fn success(&mut self, response: Vec<u8>) -> anyhow::Result<()> {
        Ok(R::success(
            self.status_sender.clone(),
            response,
            self.request_id.clone(),
        ))
    }

    fn failure(&mut self, response: Vec<u8>) -> anyhow::Result<()> {
        Ok(R::failure(
            self.status_sender.clone(),
            response,
            self.request_id.clone(),
        ))
    }

    fn log(
        &mut self,
        log_level: asml_rt::LogLevel,
        context: String,
        message: String,
    ) -> anyhow::Result<()> {
        use asml_rt::LogLevel;
        match log_level {
            LogLevel::Debug => tracing::debug!("Function:{}: {}", context, message),
            LogLevel::Trace => tracing::trace!("Function:{}: {}", context, message),
            LogLevel::Info => tracing::info!("Function:{}: {}", context, message),
            LogLevel::Warn => tracing::warn!("Function:{}: {}", context, message),
            LogLevel::Error => tracing::error!("Function:{}: {}", context, message),
        }
        Ok(())
    }

    fn get_input(&mut self) -> anyhow::Result<Vec<u8>> {
        Ok(self.function_input.clone())
    }
}

impl<R, S> secret_storage::Host for AsmlComponentFunctionState<R, S>
where
    R: RuntimeAbi<S> + Send + 'static,
    S: Clone + Send + Sized + 'static,
{
    fn get_secret_value(&mut self, id: String) -> anyhow::Result<Result<secret_storage::Secret, secret_storage::Error>> {
        let value = R::get_secret(id.clone()).unwrap();
        Ok(Ok(secret_storage::Secret {
            id: id.clone(),
            value: Some(value),
        }))
    }

    fn set_secret_value(
        &mut self,
        id: String,
        value: Vec<u8>,
        key: secret_storage::Key,
    ) -> anyhow::Result<Result<secret_storage::Secret, secret_storage::Error>> {
        R::set_secret(id.clone(), value.clone(), Some(key)).unwrap();
        Ok(Ok(secret_storage::Secret {
            id: id.clone(),
            value: Some(value),
        }))
    }
}

impl<R, S> jwt::decoder::Host for AsmlComponentFunctionState<R, S>
where
    R: RuntimeAbi<S> + Send + 'static,
    S: Clone + Send + Sized + 'static,
{
    fn decode_verify(
        &mut self,
        token: String,
        jwks: String,
        _params: jwt::decoder::ValidationParams,
    ) -> anyhow::Result<Result<jwt::decoder::VerifyResult, jwt::decoder::JwtError>> {
        let mut cache = self.cache.lock().unwrap();
        let key_set = match cache.get("jwt.keyset")? {
            Some(key_set) => key_set,
            None => {
                let key_set = std::thread::spawn(move || JwtKeyStore::new_from_blocking(jwks.to_owned()).unwrap())
                    .join()
                    .unwrap();

                cache.put("jwt.keyset", &key_set).unwrap();
                key_set
            },
        };

        tracing::debug!("JWT token={}", &token);
        
        let jwt = match key_set.verify(&token) {
            Ok(jwt) => jwt,
            Err(err) => {
                tracing::error!("{}", err.to_string());
                return Ok(Err(jwt::decoder::JwtError::InvalidToken))
            },
        };
        
        Ok(Ok(jwt::decoder::VerifyResult { valid: jwt.valid().unwrap_or(false) }))
    }
}

impl<R, S> opa::module::Host for AsmlComponentFunctionState<R, S>
where
    R: RuntimeAbi<S> + Send + 'static,
    S: Clone + Send + Sized + 'static,
{
    fn new_policy(
        &mut self,
        policy_bytes: Vec<u8>,
    ) -> anyhow::Result<Result<opa::module::Policy, opa::module::PolicyError>> {
        let id = Uuid::new_v4().to_string();
        let entrypoints = self
            .policy_manager
            .lock()
            .unwrap()
            .load_policy_bundle(id.clone(), &*policy_bytes)
            .unwrap();
        Ok(Ok(opa::module::Policy {
            id: id.clone(),
            entrypoints,
        }))
    }

    fn eval(&mut self, id: String, data: String, input: String) -> anyhow::Result<String> {
        self.policy_manager.lock().unwrap().eval(id, data, input)
    }
}

pub fn precompile(module_path: &Path, target: &str, mode: &str) -> anyhow::Result<Vec<u8>> {
    let file_path = format!("{}.bin", module_path.display().to_string());
    let is_component = file_path.contains(".component.");
    println!("Precompiling WASM to {}...", file_path.clone());

    let wasm_bytes = match std::fs::read(module_path) {
        Ok(bytes) => bytes,
        Err(err) => return Err(err.into()),
    };
    let engine = new_engine(Some(target), Some(mode))?;
    match is_component {
        true => engine.precompile_component(&*wasm_bytes),
        false => engine.precompile_module(&*wasm_bytes),
    }
}

fn new_engine(target: Option<&str>, cpu_compat_mode: Option<&str>) -> anyhow::Result<Engine> {
    let mode = match cpu_compat_mode {
        Some(mode) => mode,
        None => CPU_COMPAT_MODE.as_str(),
    };
    let mut config = match mode {
        "default" => Config::new().clone(),
        "high" => unsafe {
            Config::new()
                .wasm_simd(false)
                .cranelift_flag_set("has_sse3", "false")
                .cranelift_flag_set("has_ssse3", "false")
                .cranelift_flag_set("has_sse41", "false")
                .cranelift_flag_set("has_sse42", "false")
                .clone()
        },
        "cpu:core2quad" => unsafe {
            Config::new()
                .wasm_simd(false)
                .cranelift_flag_set("has_sse3", "false")
                .cranelift_flag_set("has_ssse3", "true")
                .cranelift_flag_set("has_sse41", "true")
                .cranelift_flag_set("has_sse42", "false")
                .clone()
        },
        _ => Config::new().clone(),
    };
    let mut config = match target {
        Some(target) => config.target(target).unwrap().clone(),
        None => config,
    };
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    config.wasm_component_model(true);
    config.wasm_multi_memory(true);
    config.async_support(true);
    match Engine::new(&config) {
        Ok(engine) => Ok(engine),
        Err(err) => Err(anyhow!(err)),
    }
}

pub fn make_wasi_component(module: Vec<u8>, preview1: &[u8]) -> anyhow::Result<Vec<u8>> {
    println!("Encoding WASM Module as Component [{} bytes]...", module.len());
    let mut encoder = ComponentEncoder::default().validate(true).module(&module)?;

    encoder = encoder.adapter("wasi_snapshot_preview1", preview1)?;

    let bytes = encoder
        .encode()
        .context("failed to encode a component from module")?;

    Ok(bytes)
}

macro_rules! parse_wit {
    ($path:expr) => {
        {
            let mut resolve = Resolve::default();
            let id = {
                let contents = include_bytes!($path);
                let text = match std::str::from_utf8(contents) {
                    Ok(s) => s,
                    Err(_) => anyhow::bail!("input file is not valid utf-8"),
                };
                let pkg = UnresolvedPackage::parse(&Path::new($path), text)?;
                resolve.push(pkg)?
            };
            anyhow::Ok::<(Resolve, PackageId)>((resolve, id))
        }
    };
}

pub fn embed_asml_wit(module: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    println!("Embedding WIT in module...");
    let mut wasm = module.clone();
    
    fn push_world(mut wasm: &mut Vec<u8>, world: WorldId, resolve: Resolve) -> anyhow::Result<()>{
        let encoded = wit_component::metadata::encode(
            &resolve,
            world,
            StringEncoding::UTF8,
            None,
        )?;

        let section = wasm_encoder::CustomSection {
            name: "component-type".into(),
            data: Cow::Borrowed(&encoded),
        };
        wasm.push(section.id());
        section.encode(&mut wasm);

        Ok(())
    }

    let world = "assemblylift".to_string();
    let (resolve, id) = parse_wit!("../../wit/assemblylift/assemblylift.wit")?;
    let world = resolve.select_world(id, Some(&world))?;
    push_world(&mut wasm, world, resolve)?;

    let world: String = "jwt".to_string();
    let (resolve, id) = parse_wit!("../../wit/jwt/jwt.wit")?;
    let world = resolve.select_world(id, Some(&world))?;
    push_world(&mut wasm, world, resolve)?;

    let world = "opa".to_string();
    let (resolve, id) = parse_wit!("../../wit/opa/opa.wit")?;
    let world = resolve.select_world(id, Some(&world))?;
    push_world(&mut wasm, world, resolve)?;

    let world: String = "secrets".to_string();
    let (resolve, id) = parse_wit!("../../wit/secrets/secrets.wit")?;
    let world = resolve.select_world(id, Some(&world))?;
    push_world(&mut wasm, world, resolve)?;

    Ok(wasm)
}
