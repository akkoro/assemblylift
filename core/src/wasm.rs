use std::fs::File;
use std::io::Write;
use std::iter::FromIterator;
use std::mem::ManuallyDrop;
use std::path::{Path, PathBuf};
use std::string::ToString;
use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Context};
use once_cell::sync::Lazy;
use wasmtime::{Caller, Config, Engine, Func, Store};
use wasmtime::component::{bindgen, Component, Instance, Linker};
use wit_component::ComponentEncoder;

use assemblylift_core_iomod::registry::RegistryTx;
use assemblylift_wasi_cap_std_sync::WasiCtxBuilder;
use assemblylift_wasi_common::WasiCtx;

use crate::abi::*;
use crate::buffers::FunctionInputBuffer;
use crate::threader::Threader;

pub type BufferElement = (usize, u8);

pub type State<S> = AsmlFunctionState<S>;

pub static CPU_COMPAT_MODE: Lazy<String> =
    Lazy::new(|| std::env::var("ASML_CPU_COMPAT_MODE").unwrap_or("default".to_string()));

bindgen!("assemblylift");

pub struct Wasmtime<R, S>
where
    R: RuntimeAbi<S> + 'static,
    S: Clone + Send + Sized + 'static,
{
    engine: Engine,
    // module: Module,
    component: Component,
    _phantom_r: std::marker::PhantomData<R>,
    _phantom_s: std::marker::PhantomData<S>,
}

impl<R, S> Wasmtime<R, S>
where
    R: RuntimeAbi<S> + 'static,
    S: Clone + Send + Sized + 'static,
{
    pub fn new_from_path(module_path: &Path) -> anyhow::Result<Self> {
        let m = match module_path.extension().unwrap().to_str().unwrap() {
            "bin" => {
                let engine = new_engine(Some("x86_64-apple-darwin"), None)?;
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

    // pub fn new_from_bytes(module_bytes: &[u8]) -> anyhow::Result<Self> {
    //     let engine = new_engine(Some("x86_64-linux-gnu"), None)?;
    //     match unsafe { Module::deserialize(&engine, module_bytes) } {
    //         Ok(module) => Ok(Self {
    //             engine,
    //             module,
    //             _phantom_r: Default::default(),
    //             _phantom_s: Default::default(),
    //         }),
    //         Err(err) => Err(anyhow!(err)),
    //     }
    // }

    pub async fn link_wasi_component(
        &mut self,
        registry_tx: RegistryTx,
        status_sender: crossbeam_channel::Sender<S>,
    ) -> anyhow::Result<(assemblylift_wasi_host::WasiCommand, Store<State<S>>)> {
        let threader = ManuallyDrop::new(Arc::new(Mutex::new(Threader::new(registry_tx))));
        let mut linker: Linker<State<S>> = Linker::new(&self.engine);

        // FIXME this might be confusingly named (confused with the function env vars)
        let function_env = std::env::var("ASML_FUNCTION_ENV").unwrap_or("default".into());

        // if let Err(err) = wasmtime_wasi::add_to_linker(&mut linker, |s| &mut s.wasi) {
        //     return Err(anyhow!(err));
        // }
        assemblylift_wasi_host::add_to_linker(&mut linker, |s| &mut s.wasi)
            .expect("TODO: panic message");
        // env vars prefixed with __ASML_ are defined in the function definition;
        // the prefix indicates that they are to be mapped to the module environment
        let envs: Vec<(String, String)> = Vec::from_iter(
            std::env::vars()
                .into_iter()
                .filter(|e| e.0.starts_with("__ASML_"))
                .map(|e| (e.0.replace("__ASML_", ""), e.1))
                .into_iter(),
        );
        let wasi = WasiCtxBuilder::new().build();
        // let wasi = match function_env.as_str() {
        //     "ruby-docker" => WasiCtxBuilder::new()
        //         .arg("/src/handler.rb")
        //         .unwrap()
        //         .env("RUBY_PLATFORM", "wasm32-wasi")
        //         .unwrap()
        //         .envs(&*envs)
        //         .unwrap()
        //         .preopened_dir(
        //             Dir::from_std_file(File::open("/usr/bin/ruby-wasm32-wasi/src").unwrap()),
        //             "/src",
        //         )
        //         .expect("could not map guest dir -- is the image built correctly?")
        //         .preopened_dir(
        //             Dir::from_std_file(File::open("/usr/bin/ruby-wasm32-wasi/usr").unwrap()),
        //             "/usr",
        //         )
        //         .expect("could not map guest dir -- is the image built correctly?")
        //         .preopened_dir(
        //             Dir::from_std_file(File::open("/tmp/asmltmp").unwrap()),
        //             "/tmp",
        //         )
        //         .expect("could not map guest dir -- is the image built correctly?")
        //         .build(),
        //     "ruby-lambda" => WasiCtxBuilder::new()
        //         .arg("/src/handler.rb")
        //         .unwrap()
        //         .env("RUBY_PLATFORM", "wasm32-wasi")
        //         .unwrap()
        //         .envs(&*envs)
        //         .unwrap()
        //         .preopened_dir(
        //             Dir::from_std_file(File::open("/tmp/rubysrc").unwrap()),
        //             "/src",
        //         )
        //         .expect("could not map guest dir -- is the image built correctly?")
        //         .preopened_dir(
        //             Dir::from_std_file(File::open("/tmp/rubyusr").unwrap()),
        //             "/usr",
        //         )
        //         .expect("could not map guest dir -- is the image built correctly?")
        //         .preopened_dir(
        //             Dir::from_std_file(File::open("/tmp/asmltmp").unwrap()),
        //             "/tmp",
        //         )
        //         .expect("could not map guest tmpfs -- is /tmp accessible?")
        //         .build(),
        //     _ => WasiCtxBuilder::new()
        //         .envs(&*envs)
        //         .unwrap()
        //         .preopened_dir(
        //             Dir::from_std_file(File::open("/tmp/asmltmp").unwrap()),
        //             "/tmp",
        //         )
        //         .expect("could not map guest tmpfs -- is /tmp accessible?")
        //         .build(),
        // };

        let state = State {
            function_input_buffer: FunctionInputBuffer::new(),
            status_sender,
            threader,
            wasi,
            io_buffer_ptr: None,
            function_input_buffer_ptr: None,
        };
        let mut store = Store::new(&self.engine, state);

        // linker
        //     .func_wrap("env", "__asml_abi_runtime_log", R::log)
        //     .unwrap();
        // linker
        //     .func_wrap("env", "__asml_abi_runtime_success", R::success)
        //     .unwrap();
        // linker
        //     .func_wrap("env", "__asml_abi_invoke", asml_abi_io_invoke::<R, S>)
        //     .unwrap();
        // linker
        //     .func_wrap("env", "__asml_abi_io_invoke", asml_abi_io_invoke::<R, S>)
        //     .unwrap();
        // linker
        //     .func_wrap("env", "__asml_abi_io_poll", asml_abi_io_poll::<S>)
        //     .unwrap();
        // linker
        //     .func_wrap("env", "__asml_abi_io_len", asml_abi_io_len::<S>)
        //     .unwrap();
        // linker
        //     .func_wrap("env", "__asml_abi_io_load", asml_abi_io_load::<S>)
        //     .unwrap();
        // linker
        //     .func_wrap("env", "__asml_abi_io_next", asml_abi_io_next::<S>)
        //     .unwrap();
        // linker
        //     .func_wrap("env", "__asml_abi_clock_time_get", asml_abi_clock_time_get)
        //     .unwrap();
        // linker
        //     .func_wrap("env", "__asml_abi_input_start", asml_abi_input_start)
        //     .unwrap();
        // linker
        //     .func_wrap("env", "__asml_abi_input_next", asml_abi_input_next)
        //     .unwrap();
        // linker
        //     .func_wrap(
        //         "env",
        //         "__asml_abi_input_length_get",
        //         asml_abi_input_length_get,
        //     )
        //     .unwrap();
        //
        // match linker.instantiate(&mut store, &self.component) {
        //     Ok(instance) => {
        //         let get_ptr = instance
        //             .get_export(&mut store, "__asml_guest_get_io_buffer_pointer")
        //             .unwrap()
        //             .into_func()
        //             .unwrap();
        //         store.data_mut().io_buffer_ptr = Some(get_ptr);
        //
        //         let get_ptr = instance
        //             .get_export(&mut store, "__asml_guest_get_function_input_buffer_pointer")
        //             .unwrap()
        //             .into_func()
        //             .unwrap();
        //         store.data_mut().function_input_buffer_ptr = Some(get_ptr);
        //
        //         Ok((instance, store))
        //     }
        //     Err(err) => Err(anyhow!(err)),
        // }
        // match linker.instantiate_async(&mut store, &self.component).await {
        //     Ok(instance) => {
        //         instance.
        //         Ok((instance, store))
        //     }
        //     Err(err) => Err(anyhow!(err)),
        // }
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
        store: &mut Store<State<S>>,
        input: &[u8],
    ) -> anyhow::Result<()> {
        // store.data_mut().function_input_buffer.set(input.to_vec());
        Ok(())
    }

    pub async fn run(
        &mut self,
        wasi: assemblylift_wasi_host::WasiCommand,
        mut store: &mut Store<State<S>>,
    ) -> anyhow::Result<()> {
        // match instance
        //     .get_func(&mut store, "_start")
        //     .expect("could not find default function")
        //     .typed::<(), (), _>(&mut store)
        //     .expect("invalid default function signature")
        //     .call(&mut store, ())
        // {
        //     Ok(_) => Ok(()),
        //     Err(trap) => Err(trap.into()),
        // }
        wasi.call_command(
            &mut store,
            0 as assemblylift_wasi_host::wasi_io::InputStream,
            1 as assemblylift_wasi_host::wasi_io::OutputStream,
            &["test", "arg"], // args
            &[("PATH", ".")], // env
            &[], // file descriptors
        )
        .await?
        .map_err(|()| anyhow::anyhow!("command returned with failing exit status"))
    }

    pub fn ptr_to_string(
        caller: &mut Caller<'_, State<S>>,
        ptr: u32,
        len: u32,
    ) -> anyhow::Result<String> {
        let bytes = Self::ptr_to_bytes(caller, ptr, len).unwrap();
        let s = std::str::from_utf8(&bytes).unwrap();
        Ok(s.into())
    }

    pub fn ptr_to_bytes(
        caller: &mut Caller<'_, State<S>>,
        ptr: u32,
        len: u32,
    ) -> anyhow::Result<Vec<u8>> {
        let memory = caller
            .get_export("memory")
            .expect("could not find the default memory export named \"memory\"")
            .into_memory()
            .unwrap();
        let mut buffer: Vec<u8> = vec![0; len as usize];
        match memory.read(&caller, ptr as usize, &mut buffer) {
            Ok(_) => Ok(buffer),
            Err(err) => Err(err.into()),
        }
    }
}

pub struct AsmlFunctionState<S>
where
    S: Clone + Send + Sized + 'static,
{
    pub function_input_buffer: FunctionInputBuffer,
    pub status_sender: crossbeam_channel::Sender<S>,
    pub threader: ManuallyDrop<Arc<Mutex<Threader<S>>>>,
    pub io_buffer_ptr: Option<Func>,
    pub function_input_buffer_ptr: Option<Func>,
    wasi: WasiCtx,
}

impl<S> asml_io::AsmlIo for AsmlFunctionState<S>
where
    S: Clone + Send + Sized + 'static,
{
    fn invoke(
        &mut self,
        path: String,
        input: String,
    ) -> anyhow::Result<Result<asml_io::Id, asml_io::IoError>> {
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

        Ok(Ok(ioid as asml_io::Id))
    }

    fn poll(&mut self, ioid: asml_io::Id) -> anyhow::Result<Result<String, asml_io::PollError>> {
        match self.threader.clone().lock().unwrap().poll(ioid) {
            true => Ok(Ok("document".into())),
            false => Ok(Err(asml_io::PollError::NotReady)),
        }
    }
    // fn poll(&mut self, ioid: asml_io::Id) -> anyhow::Result<u32> {
    //     todo!()
    // }
    //
    // fn load(&mut self, ioid: asml_io::Id) -> anyhow::Result<u32> {
    //     todo!();
    // }
    //
    // fn next(&mut self) -> anyhow::Result<u32> {
    //     todo!();
    // }
    //
    // fn len(&mut self) -> anyhow::Result<u32> {
    //     todo!();
    // }
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
    // let mut module_file = match File::create(file_path.clone()) {
    //     Ok(file) => file,
    //     Err(err) => return Err(err.into()),
    // };
    // module_file.write_all(&compiled_bytes).unwrap();
    // println!("📄 > Wrote {}", &file_path);
    //
    // Ok(PathBuf::from(file_path))
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
