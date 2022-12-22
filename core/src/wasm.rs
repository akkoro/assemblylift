use std::io::Write;
use std::mem::ManuallyDrop;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use tokio::sync::mpsc;
use wasmtime::{Caller, Config, Engine, Func, Instance, Linker, Module, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

use assemblylift_core_iomod::registry::RegistryTx;

use crate::abi::*;
use crate::buffers::{FunctionInputBuffer, LinearBuffer};
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
        let module = unsafe { Module::deserialize_file(&engine, module_path) }.expect("");
        Ok(Self {
            engine,
            module,
            _phantom_r: Default::default(),
            _phantom_s: Default::default(),
        })
    }

    pub fn new_from_bytes(module_bytes: &[u8]) -> anyhow::Result<Self> {
        let engine = Engine::default();
        let module = unsafe { Module::deserialize(&engine, module_bytes) }.expect("");
        Ok(Self {
            engine,
            module,
            _phantom_r: Default::default(),
            _phantom_s: Default::default(),
        })
    }

    pub fn link_module(
        &mut self,
        registry_tx: RegistryTx,
        status_sender: crossbeam_channel::Sender<S>,
    ) -> anyhow::Result<(Instance, Store<State<S>>)> {
        let threader = ManuallyDrop::new(Arc::new(Mutex::new(Threader::new(registry_tx))));
        let mut linker: Linker<State<S>> = Linker::new(&self.engine);

        wasmtime_wasi::add_to_linker(&mut linker, |s| &mut s.wasi).expect("");
        let wasi = WasiCtxBuilder::new().build();

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
            .expect("");
        linker
            .func_wrap("env", "__asml_abi_runtime_success", R::success)
            .expect("");
        linker
            .func_wrap("env", "__asml_abi_invoke", asml_abi_io_invoke::<R, S>)
            .expect("");
        linker
            .func_wrap("env", "__asml_abi_io_invoke", asml_abi_io_invoke::<R, S>)
            .expect("");
        linker
            .func_wrap("env", "__asml_abi_io_poll", asml_abi_io_poll::<S>)
            .expect("");
        linker
            .func_wrap("env", "__asml_abi_io_len", asml_abi_io_len::<S>)
            .expect("");
        linker
            .func_wrap("env", "__asml_abi_io_load", asml_abi_io_load::<S>)
            .expect("");
        linker
            .func_wrap("env", "__asml_abi_io_next", asml_abi_io_next::<S>)
            .expect("");
        linker
            .func_wrap("env", "__asml_abi_clock_time_get", asml_abi_clock_time_get)
            .expect("");
        linker
            .func_wrap("env", "__asml_abi_input_start", asml_abi_input_start)
            .expect("");
        linker
            .func_wrap("env", "__asml_abi_input_next", asml_abi_input_next)
            .expect("");
        linker
            .func_wrap(
                "env",
                "__asml_abi_input_length_get",
                asml_abi_input_length_get,
            )
            .expect("");

        let instance = linker.instantiate(&mut store, &self.module).expect("");

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

    pub fn initialize_function_input_buffer(
        &mut self,
        store: &mut Store<State<S>>,
        input: &[u8],
    ) -> anyhow::Result<()> {
        store
            .data_mut()
            .function_input_buffer
            .initialize(input.to_vec());
        Ok(())
    }

    pub fn start(
        &mut self,
        mut store: &mut Store<State<S>>,
        instance: Instance,
    ) -> anyhow::Result<()> {
        instance
            .get_func(&mut store, "_start")
            .expect("could not find default function")
            .typed::<(), (), _>(&mut store)
            .expect("invalid default function signature")
            .call(&mut store, ())
            .expect("Trap while executing module");
        Ok(())
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
        let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
        let mut buffer: Vec<u8> = vec![0; len as usize];
        memory.read(&caller, ptr as usize, &mut buffer).unwrap();
        Ok(buffer)
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

// pub fn build_module<R, S>(
//     registry_tx: RegistryTx,
//     status_sender: crossbeam_channel::Sender<S>,
//     module: Arc<Module>,
//     module_name: &str,
//     store: Arc<Store>,
// ) -> anyhow::Result<(Resolver, ThreaderEnv<S>)>
// where
//     R: RuntimeAbi<S> + 'static,
//     S: Clone + Send + Sized + 'static,
// {
//     let threader_env = ThreaderEnv::new(registry_tx, status_sender);
//     let function_env = std::env::var("ASML_FUNCTION_ENV").unwrap_or("default".into());
//     let mut wasi_env = match function_env.as_str() {
//         "ruby-docker" => WasiState::new(module_name.clone())
//             .arg("/src/handler.rb")
//             .env("RUBY_PLATFORM", "wasm32-wasi")
//             .map_dir("/src", "/usr/bin/ruby-wasm32-wasi/src")
//             .expect("could not preopen `src` directory")
//             .map_dir("/usr", "/usr/bin/ruby-wasm32-wasi/usr")
//             .expect("could not map ruby fs")
//             .finalize()
//             .expect("could not init WASI env"),
//         "ruby-lambda" => WasiState::new(module_name.clone())
//             .arg("/src/handler.rb")
//             .env("RUBY_PLATFORM", "wasm32-wasi")
//             .map_dir("/src", "/tmp/rubysrc")
//             .expect("could not preopen `src` directory")
//             .map_dir("/usr", "/tmp/rubyusr")
//             .expect("could not map ruby fs")
//             .finalize()
//             .expect("could not init WASI env"),
//         _ => WasiState::new(module_name.clone())
//             .finalize()
//             .expect("could not init WASI env"),
//     };
//
//     let wasi_imports = wasi_env
//         .import_object(&module)
//         .expect("could not get WASI import object");
//     let asml_imports = imports! {
//         "env" => {
//             "__asml_abi_runtime_log" => Function::new_native_with_env(&store, threader_env.clone(), R::log),
//             "__asml_abi_runtime_success" => Function::new_native_with_env(&store, threader_env.clone(), R::success),
//
//             "__asml_abi_invoke" => Function::new_native_with_env(&store, threader_env.clone(), asml_abi_io_invoke), // TODO deprecated, IOmod guests need to update
//             "__asml_abi_io_invoke" => Function::new_native_with_env(&store, threader_env.clone(), asml_abi_io_invoke),
//             "__asml_abi_io_poll" => Function::new_native_with_env(&store, threader_env.clone(), asml_abi_io_poll),
//             "__asml_abi_io_len" => Function::new_native_with_env(&store, threader_env.clone(), asml_abi_io_len),
//             "__asml_abi_io_load" => Function::new_native_with_env(&store, threader_env.clone(), asml_abi_io_load),
//             "__asml_abi_io_next" => Function::new_native_with_env(&store, threader_env.clone(), asml_abi_io_next),
//
//             "__asml_abi_clock_time_get" => Function::new_native_with_env(&store, threader_env.clone(), asml_abi_clock_time_get),
//
//             "__asml_abi_input_start" => Function::new_native_with_env(&store, threader_env.clone(), asml_abi_input_start),
//             "__asml_abi_input_next" => Function::new_native_with_env(&store, threader_env.clone(), asml_abi_input_next),
//             "__asml_abi_input_length_get" => Function::new_native_with_env(&store, threader_env.clone(), asml_abi_input_length_get),
//
//             "__asml_expabi_z85_encode" => Function::new_native_with_env(&store, threader_env.clone(), asml_abi_z85_encode),
//             "__asml_expabi_z85_decode" => Function::new_native_with_env(&store, threader_env.clone(), asml_abi_z85_decode),
//         },
//     };
//
//     let import_object: Resolver = asml_imports.chain_back(wasi_imports);
//
//     Ok((import_object, threader_env))
// }

pub fn precompile(module_path: &Path, target: &str) -> anyhow::Result<PathBuf> {
    // TODO compiler configuration
    let file_path = format!("{}.bin", module_path.display().to_string());
    println!("Precompiling WASM to {}...", file_path.clone());

    let wasm_bytes = match std::fs::read(module_path.clone()) {
        Ok(bytes) => bytes,
        Err(err) => panic!("{}", err.to_string()),
    };
    let engine = Engine::new(Config::new().target(target).unwrap()).unwrap();
    let compiled_bytes = engine
        .precompile_module(&*wasm_bytes)
        .expect("TODO: panic message");
    let mut module_file = match std::fs::File::create(file_path.clone()) {
        Ok(file) => file,
        Err(err) => panic!("{}", err.to_string()),
    };
    module_file.write_all(&compiled_bytes).unwrap();
    println!("📄 > Wrote {}", &file_path);

    Ok(PathBuf::from(file_path))
}
