use std::fs::File;
use std::iter::FromIterator;
use std::path::Path;
use std::string::ToString;
use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Context};
use assemblylift_wasi_cap_std_sync::{dir, Dir, WasiCtxBuilder};
use assemblylift_wasi_common::WasiCtx;
pub use crossbeam_channel::bounded as status_channel;
use once_cell::sync::Lazy;
use wasmtime::{Config, Engine, Store};
use wasmtime::component::{bindgen, Component, Linker};
use wit_component::ComponentEncoder;

use assemblylift_core_iomod::registry::RegistryTx;

use crate::RuntimeAbi;
use crate::threader::Threader;
use crate::wasm::secrets::{Error, Key, Secret};

pub type State<R, S> = AsmlFunctionState<R, S>;
pub type StatusTx<S> = crossbeam_channel::Sender<S>;
pub type StatusRx<S> = crossbeam_channel::Receiver<S>;

pub static CPU_COMPAT_MODE: Lazy<String> =
    Lazy::new(|| std::env::var("ASML_CPU_COMPAT_MODE").unwrap_or("default".to_string()));

bindgen!("assemblylift");
bindgen!("wasi-secrets" in "components/wasi-secrets/wit");

pub struct Wasmtime<R, S>
where
    R: RuntimeAbi<S> + Send + 'static,
    S: Clone + Send + Sized + 'static,
{
    engine: Engine,
    component: Component,
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
        request_id: Option<String>,
    ) -> anyhow::Result<(assemblylift_wasi_host::WasiCommand, Store<State<R, S>>)> {
        let threader = Arc::new(Mutex::new(Threader::new(registry_tx)));
        let mut linker: Linker<State<R, S>> = Linker::new(&self.engine);

        assemblylift_wasi_host::add_to_linker(&mut linker, |s| &mut s.wasi)
            .expect("could not link wasi runtime component");
        Assemblylift::add_to_linker(&mut linker, |s| s)
            .expect("could not link assemblylift runtime component");
        WasiSecrets::add_to_linker(&mut linker, |s| s)
            .expect("could not link wasi-secrets runtime component");

        // env vars prefixed with __ASML_ are defined in the function definition;
        // the prefix indicates that they are to be mapped to the module environment
        // FIXME this only really works when running inside a container/single-function environment
        let envs: Vec<(String, String)> = Vec::from_iter(
            std::env::vars()
                .into_iter()
                .filter(|e| e.0.starts_with("__ASML_"))
                .map(|e| (e.0.replace("__ASML_", ""), e.1))
                .into_iter(),
        );
        let mut wasi = WasiCtxBuilder::new().build();
        for e in envs {
            wasi.push_env(&*e.0, &*e.1)
        }
        // FIXME this might be confusingly named (confused with the function env vars)
        let function_env = std::env::var("ASML_FUNCTION_ENV").unwrap_or("default".into());
        match function_env.as_str() {
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
            threader,
            request_id,
            wasi,
            _phantom: Default::default(),
        };
        let mut store = Store::new(&self.engine, state);

        match assemblylift_wasi_host::WasiCommand::instantiate_async(
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
        wasi: assemblylift_wasi_host::WasiCommand,
        mut store: &mut Store<State<R, S>>,
    ) -> anyhow::Result<()> {
        wasi.call_command(
            &mut store,
            0 as assemblylift_wasi_host::wasi_io::InputStream,
            1 as assemblylift_wasi_host::wasi_io::OutputStream,
            &[], // TODO args
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
    function_input: Vec<u8>,
    request_id: Option<String>,
    wasi: WasiCtx,
    _phantom: std::marker::PhantomData<R>,
}

impl<R, S> asml_io::AsmlIo for AsmlFunctionState<R, S>
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

        self.threader
            .clone()
            .lock()
            .unwrap()
            .invoke(&path, input.into_bytes(), ioid);

        Ok(Ok(ioid as asml_io::Ioid))
    }

    fn poll(&mut self, ioid: asml_io::Ioid) -> anyhow::Result<Result<Vec<u8>, asml_io::PollError>> {
        match self.threader.clone().lock().unwrap().poll(ioid) {
            Some(response) => Ok(Ok(response)),
            None => Ok(Err(asml_io::PollError::NotReady)),
        }
    }
}

impl<R, S> asml_rt::AsmlRt for AsmlFunctionState<R, S>
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

    fn get_input(&mut self) -> anyhow::Result<Vec<u8>> {
        Ok(self.function_input.clone())
    }
}

impl<R, S> secrets::Secrets for AsmlFunctionState<R, S>
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

    fn set_secret_value(&mut self, id: String, value: Vec<u8>, key: Key) -> anyhow::Result<Result<Secret, Error>> {
        R::set_secret(id.clone(), value.clone()).unwrap();
        Ok(Ok(Secret {
            id: id.clone(),
            value: Some(value),
        }))
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
