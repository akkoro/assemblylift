mod cache;

use std::borrow::Cow;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::string::ToString;
use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Context};
use assemblylift_wasi_cap_std_sync::{dir, Dir, WasiCtxBuilder};
use assemblylift_wasi_common::WasiCtx;
pub use crossbeam_channel::bounded as status_channel;
use once_cell::sync::Lazy;
use uuid::Uuid;
use wasmtime::component::{bindgen, Component, Linker};
use wasmtime::{Config, Engine, Store};
use wit_component::{ComponentEncoder, DecodedWasm, StringEncoding, WitPrinter};
use wasm_encoder::{Encode, Section};
use wit_parser::{PackageId, Resolve, UnresolvedPackage};

use assemblylift_core_iomod::registry::RegistryTx;

use crate::jwt::keyset::KeyStore as JwtKeyStore;
use crate::policy_manager::PolicyManager;
use crate::threader::Threader;
use crate::wasm::cache::Cache;
use crate::wasm::secrets::{Error, Key, Secret};
use crate::RuntimeAbi;

pub type State<R, S> = AsmlFunctionState<R, S>;
pub type StatusTx<S> = crossbeam_channel::Sender<S>;
pub type StatusRx<S> = crossbeam_channel::Receiver<S>;

pub static CPU_COMPAT_MODE: Lazy<String> =
    Lazy::new(|| std::env::var("ASML_CPU_COMPAT_MODE").unwrap_or("default".to_string()));

bindgen!("assemblylift");
bindgen!("jwt");
bindgen!("opa");
bindgen!("wasi-secrets" in "components/wasi-secrets/wit");

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
    pub fn new_from_path(module_path: &Path) -> anyhow::Result<Self> {
        let m = match module_path.extension().unwrap().to_str().unwrap() {
            "bin" => {
                let target = match std::env::consts::OS {
                    "macos" => Some("x86_64-apple-darwin"),
                    "linux" => Some("x86_64-linux-gnu"),
                    _ => None,
                };
                let engine = new_engine(target, None)?;
                let module = unsafe { Component::deserialize_file(&engine, module_path) };
                (engine, module)
            }
            "wasm" => {
                let engine = new_engine(None, None)?;
                let module = Component::from_file(&engine, module_path);
                (engine, module)
            }
            _ => {
                return Err(anyhow!(
                    "invalid module extension; must be .wasm or .wasm.bin"
                ))
            }
        };
        match m.1 {
            Ok(component) => Ok(Self {
                engine: m.0,
                component,
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
        request_id: Option<String>,
    ) -> anyhow::Result<(
        assemblylift_wasi_host::command::wasi::Command,
        Store<State<R, S>>,
    )> {
        let threader = Arc::new(Mutex::new(Threader::new(registry_tx)));
        let mut linker: Linker<State<R, S>> = Linker::new(&self.engine);

        assemblylift_wasi_host::command::add_to_linker(&mut linker, |s| &mut s.wasi)
            .expect("could not link wasi runtime component");
        Assemblylift::add_to_linker(&mut linker, |s| s)
            .expect("could not link assemblylift runtime component");
        Jwt::add_to_linker(&mut linker, |s| s).expect("could not link jwt runtime component");
        Opa::add_to_linker(&mut linker, |s| s).expect("could not link opa runtime component");
        WasiSecrets::add_to_linker(&mut linker, |s| s)
            .expect("could not link wasi-secrets runtime component");

        let mut wasi = WasiCtxBuilder::new().build();
        for e in environment_vars {
            wasi.push_env(&*e.0, &*e.1)
        }

        match runtime_environment.as_str() {
            "ruby-docker" => {
                wasi.push_env("RUBY_PLATFORM", "wasm32-wasi");
                wasi.push_preopened_dir(
                    Box::new(dir::Dir::from_cap_std(Dir::from_std_file(
                        File::open("/usr/bin/ruby-wasm32-wasi/src").unwrap(),
                    ))),
                    "/src",
                )
                .expect("could not push preopened wasi dir");
                wasi.push_preopened_dir(
                    Box::new(dir::Dir::from_cap_std(Dir::from_std_file(
                        File::open("/usr/bin/ruby-wasm32-wasi/usr").unwrap(),
                    ))),
                    "/usr",
                )
                .expect("could not push preopened wasi dir");
                wasi.push_preopened_dir(
                    Box::new(dir::Dir::from_cap_std(Dir::from_std_file(
                        File::open("/tmp/asmltmp").unwrap(),
                    ))),
                    "/tmp",
                )
                .expect("could not push preopened wasi dir");
            }
            "ruby-lambda" => {
                wasi.push_env("RUBY_PLATFORM", "wasm32-wasi");
                wasi.push_preopened_dir(
                    Box::new(dir::Dir::from_cap_std(Dir::from_std_file(
                        File::open("/tmp/rubysrc").unwrap(),
                    ))),
                    "/src",
                )
                .expect("could not push preopened wasi dir");
                wasi.push_preopened_dir(
                    Box::new(dir::Dir::from_cap_std(Dir::from_std_file(
                        File::open("/tmp/rubyusr").unwrap(),
                    ))),
                    "/usr",
                )
                .expect("could not push preopened wasi dir");
                wasi.push_preopened_dir(
                    Box::new(dir::Dir::from_cap_std(Dir::from_std_file(
                        File::open("/tmp/asmltmp").unwrap(),
                    ))),
                    "/tmp",
                )
                .expect("could not push preopened wasi dir");
            }
            _ => {
                wasi.push_preopened_dir(
                    Box::new(dir::Dir::from_cap_std(Dir::from_std_file(
                        File::open("/tmp/asmltmp").unwrap(),
                    ))),
                    "/tmp",
                )
                .expect("could not push preopened wasi dir");
            }
        }

        let state = State {
            function_input: Vec::with_capacity(512usize),
            status_sender: status_tx,
            policy_manager: Arc::new(Mutex::new(PolicyManager::new())),
            threader,
            request_id,
            cache: self.cache.clone(),
            wasi,
            _phantom: Default::default(),
        };
        let mut store = Store::new(&self.engine, state);

        match assemblylift_wasi_host::command::wasi::Command::instantiate_async(
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

    pub fn initialize_function_input_buffer(
        &mut self,
        store: &mut Store<State<R, S>>,
        input: &[u8],
    ) -> anyhow::Result<()> {
        store.data_mut().function_input.clear();
        store
            .data_mut()
            .function_input
            .append(&mut Vec::from(input));
        Ok(())
    }

    pub async fn run(
        &mut self,
        wasi: assemblylift_wasi_host::command::wasi::Command,
        mut store: &mut Store<State<R, S>>,
        args: Vec<&str>,
    ) -> anyhow::Result<()> {
        wasi.call_main(
            &mut store,
            &args,
        )
        .await?
        .map_err(|()| anyhow::anyhow!("command returned with failing exit status"))
    }
}

pub struct AsmlFunctionState<R, S>
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
    wasi: WasiCtx,
    _phantom: std::marker::PhantomData<R>,
}

impl<R, S> asml_io::Host for AsmlFunctionState<R, S>
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

impl<R, S> asml_rt::Host for AsmlFunctionState<R, S>
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

impl<R, S> secrets::Host for AsmlFunctionState<R, S>
where
    R: RuntimeAbi<S> + Send + 'static,
    S: Clone + Send + Sized + 'static,
{
    fn get_secret_value(&mut self, id: String) -> anyhow::Result<Result<Secret, Error>> {
        let value = R::get_secret(id.clone()).unwrap();
        Ok(Ok(Secret {
            id: id.clone(),
            value: Some(value),
        }))
    }

    fn set_secret_value(
        &mut self,
        id: String,
        value: Vec<u8>,
        key: Key,
    ) -> anyhow::Result<Result<Secret, Error>> {
        R::set_secret(id.clone(), value.clone(), Some(key)).unwrap();
        Ok(Ok(Secret {
            id: id.clone(),
            value: Some(value),
        }))
    }
}

impl<R, S> jwt::Host for AsmlFunctionState<R, S>
where
    R: RuntimeAbi<S> + Send + 'static,
    S: Clone + Send + Sized + 'static,
{
    fn decode_verify(
        &mut self,
        token: String,
        jwks: String,
        _params: jwt::ValidationParams,
    ) -> anyhow::Result<Result<jwt::VerifyResult, jwt::JwtError>> {
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
        
        let jwt = match key_set.verify(&token) {
            Ok(jwt) => jwt,
            Err(_err) => return Ok(Err(jwt::JwtError::InvalidToken)),
        };
        
        Ok(Ok(jwt::VerifyResult { valid: jwt.valid().unwrap_or(false) }))
    }
}

impl<R, S> opa::Host for AsmlFunctionState<R, S>
where
    R: RuntimeAbi<S> + Send + 'static,
    S: Clone + Send + Sized + 'static,
{
    fn new_policy(
        &mut self,
        policy_bytes: Vec<u8>,
    ) -> anyhow::Result<Result<opa::Policy, opa::PolicyError>> {
        let id = Uuid::new_v4().to_string();
        let entrypoints = self
            .policy_manager
            .lock()
            .unwrap()
            .load_policy_bundle(id.clone(), &*policy_bytes)
            .unwrap();
        Ok(Ok(opa::Policy {
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
    println!("      --> precompiling WASM to {}...", file_path.clone());

    let wasm_bytes = match std::fs::read(module_path.clone()) {
        Ok(bytes) => bytes,
        Err(err) => return Err(err.into()),
    };
    let engine = new_engine(Some(target), Some(mode))?;
    engine.precompile_component(&*wasm_bytes)
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
    println!("      --> encoding WASM Module as Component...");
    let mut encoder = ComponentEncoder::default().validate(true).module(&module)?;

    encoder = encoder.adapter("wasi_snapshot_preview1", preview1)?;

    let bytes = encoder
        .encode()
        .context("failed to encode a component from module")?;

    Ok(bytes)
}

pub fn embed_wit(module: Vec<u8>, wit: PathBuf, world: String) -> anyhow::Result<Vec<u8>> {
    let mut wasm = module.clone();
    let (resolve, id) = parse_wit(&wit)?;
    let world = resolve.select_world(id, Some(&world))?;

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

    Ok(wasm)
}

fn parse_wit(path: &Path) -> anyhow::Result<(Resolve, PackageId)> {
    let mut resolve = Resolve::default();
    let id = if path.is_dir() {
        resolve.push_dir(&path)?.0
    } else {
        let contents =
            std::fs::read(&path).with_context(|| format!("failed to read file {path:?}"))?;
        if is_wasm(&contents) {
            let bytes = wat::parse_bytes(&contents).map_err(|mut e| {
                e.set_path(path);
                e
            })?;
            match wit_component::decode(&bytes)? {
                DecodedWasm::Component(..) => {
                    anyhow::bail!("specified path is a component, not a wit package")
                }
                DecodedWasm::WitPackage(resolve, pkg) => return Ok((resolve, pkg)),
            }
        } else {
            let text = match std::str::from_utf8(&contents) {
                Ok(s) => s,
                Err(_) => anyhow::bail!("input file is not valid utf-8"),
            };
            let pkg = UnresolvedPackage::parse(&path, text)?;
            resolve.push(pkg)?
        }
    };
    Ok((resolve, id))
}

fn is_wasm(bytes: &[u8]) -> bool {
    use wast::lexer::{Lexer, Token};

    if bytes.starts_with(b"\0asm") {
        return true;
    }
    let text = match std::str::from_utf8(bytes) {
        Ok(s) => s,
        Err(_) => return true,
    };

    let mut lexer = Lexer::new(text);

    while let Some(next) = lexer.next() {
        match next {
            Ok(Token::Whitespace(_)) | Ok(Token::BlockComment(_)) | Ok(Token::LineComment(_)) => {}
            Ok(Token::LParen(_)) => return true,
            _ => break,
        }
    }

    false
}
