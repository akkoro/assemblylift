use std::fs::File;
use std::io::Write;
use std::mem::ManuallyDrop;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use anyhow::anyhow;
use tokio::sync::mpsc;
use wasmtime::{Caller, Config, Engine, Func, Instance, Linker, Module, Store};
use wasmtime_wasi::{Dir, WasiCtx, WasiCtxBuilder};

use assemblylift_core_iomod::registry::RegistryTx;

use crate::abi::*;
use crate::buffers::FunctionInputBuffer;
use crate::threader::Threader;

pub type State<S> = AsmlFunctionState<S>;

pub type BufferElement = (usize, u8);
pub type MemoryTx = mpsc::Sender<BufferElement>;
pub type MemoryRx = mpsc::Receiver<BufferElement>;
pub type MemoryChannel = (MemoryTx, MemoryRx);

pub struct Wasmtime<R, S>
where
    R: RuntimeAbi<S> + 'static,
    S: Clone + Send + Sized + 'static,
{
    engine: Engine,
    module: Module,
    _phantom_r: std::marker::PhantomData<R>,
    _phantom_s: std::marker::PhantomData<S>,
}

impl<R, S> Wasmtime<R, S>
where
    R: RuntimeAbi<S> + 'static,
    S: Clone + Send + Sized + 'static,
{
    pub fn new_from_path(module_path: &Path) -> anyhow::Result<Self> {
        let engine = Engine::default();
        match unsafe { Module::deserialize_file(&engine, module_path) } {
            Ok(module) => Ok(Self {
                engine,
                module,
                _phantom_r: Default::default(),
                _phantom_s: Default::default(),
            }),
            Err(err) => Err(anyhow!(err)),
        }
    }

    pub fn new_from_bytes(module_bytes: &[u8]) -> anyhow::Result<Self> {
        let engine = Engine::default();
        match unsafe { Module::deserialize(&engine, module_bytes) } {
            Ok(module) => Ok(Self {
                engine,
                module,
                _phantom_r: Default::default(),
                _phantom_s: Default::default(),
            }),
            Err(err) => Err(anyhow!(err)),
        }
    }

    pub fn link_module(
        &mut self,
        registry_tx: RegistryTx,
        status_sender: crossbeam_channel::Sender<S>,
    ) -> anyhow::Result<(Instance, Store<State<S>>)> {
        let threader = ManuallyDrop::new(Arc::new(Mutex::new(Threader::new(registry_tx))));
        let mut linker: Linker<State<S>> = Linker::new(&self.engine);

        let function_env = std::env::var("ASML_FUNCTION_ENV").unwrap_or("default".into());

        if let Err(err) = wasmtime_wasi::add_to_linker(&mut linker, |s| &mut s.wasi) {
            return Err(anyhow!(err));
        }
        let wasi = match function_env.as_str() {
            "ruby-docker" => WasiCtxBuilder::new()
                .arg("/src/handler.rb")
                .unwrap()
                .env("RUBY_PLATFORM", "wasm32-wasi")
                .unwrap()
                .preopened_dir(
                    Dir::from_std_file(File::open("/usr/bin/ruby-wasm32-wasi/src").unwrap()),
                    "/src",
                )
                .expect("could not map guest dir -- is the image built correctly?")
                .preopened_dir(
                    Dir::from_std_file(File::open("/usr/bin/ruby-wasm32-wasi/usr").unwrap()),
                    "/usr",
                )
                .expect("could not map guest dir -- is the image built correctly?")
                .preopened_dir(
                    Dir::from_std_file(File::open("/tmp/asmltmp").unwrap()),
                    "/tmp",
                )
                .expect("could not map guest dir -- is the image built correctly?")
                .build(),
            "ruby-lambda" => WasiCtxBuilder::new()
                .arg("/src/handler.rb")
                .unwrap()
                .env("RUBY_PLATFORM", "wasm32-wasi")
                .unwrap()
                .preopened_dir(
                    Dir::from_std_file(File::open("/tmp/rubysrc").unwrap()),
                    "/src",
                )
                .expect("could not map guest dir -- is the image built correctly?")
                .preopened_dir(
                    Dir::from_std_file(File::open("/tmp/rubyusr").unwrap()),
                    "/usr",
                )
                .expect("could not map guest dir -- is the image built correctly?")
                .preopened_dir(
                    Dir::from_std_file(File::open("/tmp/asmltmp").unwrap()),
                    "/tmp",
                )
                .expect("could not map guest tmpfs -- is /tmp accessible?")
                .build(),
            _ => WasiCtxBuilder::new()
                .preopened_dir(
                    Dir::from_std_file(File::open("/tmp/asmltmp").unwrap()),
                    "/tmp",
                )
                .expect("could not map guest tmpfs -- is /tmp accessible?")
                .build(),
        };

        let state = State {
            function_input_buffer: FunctionInputBuffer::new(),
            status_sender,
            threader,
            wasi,
            io_buffer_ptr: None,
            function_input_buffer_ptr: None,
        };
        let mut store = Store::new(&self.engine, state);

        linker
            .func_wrap("env", "__asml_abi_runtime_log", R::log)
            .unwrap();
        linker
            .func_wrap("env", "__asml_abi_runtime_success", R::success)
            .unwrap();
        linker
            .func_wrap("env", "__asml_abi_invoke", asml_abi_io_invoke::<R, S>)
            .unwrap();
        linker
            .func_wrap("env", "__asml_abi_io_invoke", asml_abi_io_invoke::<R, S>)
            .unwrap();
        linker
            .func_wrap("env", "__asml_abi_io_poll", asml_abi_io_poll::<S>)
            .unwrap();
        linker
            .func_wrap("env", "__asml_abi_io_len", asml_abi_io_len::<S>)
            .unwrap();
        linker
            .func_wrap("env", "__asml_abi_io_load", asml_abi_io_load::<S>)
            .unwrap();
        linker
            .func_wrap("env", "__asml_abi_io_next", asml_abi_io_next::<S>)
            .unwrap();
        linker
            .func_wrap("env", "__asml_abi_clock_time_get", asml_abi_clock_time_get)
            .unwrap();
        linker
            .func_wrap("env", "__asml_abi_input_start", asml_abi_input_start)
            .unwrap();
        linker
            .func_wrap("env", "__asml_abi_input_next", asml_abi_input_next)
            .unwrap();
        linker
            .func_wrap(
                "env",
                "__asml_abi_input_length_get",
                asml_abi_input_length_get,
            )
            .unwrap();

        match linker.instantiate(&mut store, &self.module) {
            Ok(instance) => {
                let get_ptr = instance
                    .get_export(&mut store, "__asml_guest_get_io_buffer_pointer")
                    .unwrap()
                    .into_func()
                    .unwrap();
                store.data_mut().io_buffer_ptr = Some(get_ptr);

                let get_ptr = instance
                    .get_export(&mut store, "__asml_guest_get_function_input_buffer_pointer")
                    .unwrap()
                    .into_func()
                    .unwrap();
                store.data_mut().function_input_buffer_ptr = Some(get_ptr);

                Ok((instance, store))
            }
            Err(err) => Err(anyhow!(err)),
        }
    }

    pub fn initialize_function_input_buffer(
        &mut self,
        store: &mut Store<State<S>>,
        input: &[u8],
    ) -> anyhow::Result<()> {
        store.data_mut().function_input_buffer.set(input.to_vec());
        Ok(())
    }

    pub fn start(
        &mut self,
        mut store: &mut Store<State<S>>,
        instance: Instance,
    ) -> anyhow::Result<()> {
        match instance
            .get_func(&mut store, "_start")
            .expect("could not find default function")
            .typed::<(), (), _>(&mut store)
            .expect("invalid default function signature")
            .call(&mut store, ())
        {
            Ok(_) => Ok(()),
            Err(trap) => Err(trap.into()),
        }
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

pub fn precompile(module_path: &Path, target: &str) -> anyhow::Result<PathBuf> {
    // TODO compiler configuration
    let file_path = format!("{}.bin", module_path.display().to_string());
    println!("Precompiling WASM to {}...", file_path.clone());

    let wasm_bytes = match std::fs::read(module_path.clone()) {
        Ok(bytes) => bytes,
        Err(err) => return Err(err.into()),
    };
    let engine = Engine::new(Config::new().target(target).unwrap()).unwrap();
    let compiled_bytes = engine
        .precompile_module(&*wasm_bytes)
        .expect("TODO: panic message");
    let mut module_file = match File::create(file_path.clone()) {
        Ok(file) => file,
        Err(err) => return Err(err.into()),
    };
    module_file.write_all(&compiled_bytes).unwrap();
    println!("📄 > Wrote {}", &file_path);

    Ok(PathBuf::from(file_path))
}
