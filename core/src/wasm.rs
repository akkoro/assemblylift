use std::sync::Arc;

use wasmer::{
    ChainableNamedResolver, Cranelift, Function, ImportObject, imports, Instance,
    InstantiationError, Module, NamedResolverChain, Store, Universal,
};
use wasmer_wasi::WasiState;

use assemblylift_core_iomod::registry::RegistryTx;

use crate::abi::*;
use crate::threader::ThreaderEnv;

pub type ModuleTreble<S> = (Module, Resolver, ThreaderEnv<S>);
pub type Resolver = NamedResolverChain<ImportObject, ImportObject>;

pub fn deserialize_module_from_path<R, S>(
    module_path: &str,
    module_name: &str,
) -> anyhow::Result<(Module, Store)>
where
    R: RuntimeAbi<S> + 'static,
    S: Clone + Send + Sized + 'static,
{
    let file_path = format!("{}/{}", module_path, module_name);

    let compiler = Cranelift::default();
    let store = Store::new(&Universal::new(compiler).engine());
    Ok((
        unsafe { Module::deserialize_from_file(&store, file_path.clone()) }
            .expect(&format!("could not load wasm from {}", file_path.clone())),
        store,
    ))
}

pub fn deserialize_module_from_bytes<R, S>(
    module_bytes: &[u8],
) -> anyhow::Result<(Module, Store)>
where
    R: RuntimeAbi<S> + 'static,
    S: Clone + Send + Sized + 'static,
{
    let compiler = Cranelift::default();
    let store = Store::new(&Universal::new(compiler).engine());
    Ok((
        unsafe { wasmer::Module::deserialize(&store, module_bytes) }
            .expect(&format!("could not load wasm from bytes")),
        store,
    ))
}

pub fn build_module<R, S>(
    registry_tx: RegistryTx,
    status_sender: crossbeam_channel::Sender<S>,
    module: Arc<Module>,
    module_name: &str,
    store: Arc<Store>,
) -> anyhow::Result<(Resolver, ThreaderEnv<S>)>
where
    R: RuntimeAbi<S> + 'static,
    S: Clone + Send + Sized + 'static,
{
    let threader_env = ThreaderEnv::new(registry_tx, status_sender);
    let function_env = std::env::var("ASML_FUNCTION_ENV").unwrap_or("default".into());
    let mut wasi_env = match function_env.as_str() {
        // TODO ruby-docker ruby-lambda
        "ruby" => WasiState::new(module_name.clone())
            .arg("/src/handler.rb")
            .env("RUBY_PLATFORM", "wasm32-wasi")
            .map_dir("/src", "/usr/bin/ruby-wasm32-wasi/src")
            .expect("could not preopen `src` directory")
            .map_dir("/usr", "/usr/bin/ruby-wasm32-wasi/usr")
            .expect("could not map ruby fs")
            .finalize()
            .expect("could not init WASI env"),
        _ => WasiState::new(module_name.clone())
            .finalize()
            .expect("could not init WASI env"),
    };

    let wasi_imports = wasi_env
        .import_object(&module)
        .expect("could not get WASI import object");
    let asml_imports = imports! {
        "env" => {
            "__asml_abi_runtime_log" => Function::new_native_with_env(&store, threader_env.clone(), R::log),
            "__asml_abi_runtime_success" => Function::new_native_with_env(&store, threader_env.clone(), R::success),

            "__asml_abi_invoke" => Function::new_native_with_env(&store, threader_env.clone(), asml_abi_io_invoke), // TODO deprecated, IOmod guests need to update
            "__asml_abi_io_invoke" => Function::new_native_with_env(&store, threader_env.clone(), asml_abi_io_invoke),
            "__asml_abi_io_poll" => Function::new_native_with_env(&store, threader_env.clone(), asml_abi_io_poll),
            "__asml_abi_io_len" => Function::new_native_with_env(&store, threader_env.clone(), asml_abi_io_len),
            "__asml_abi_io_load" => Function::new_native_with_env(&store, threader_env.clone(), asml_abi_io_load),
            "__asml_abi_io_next" => Function::new_native_with_env(&store, threader_env.clone(), asml_abi_io_next),

            "__asml_abi_clock_time_get" => Function::new_native_with_env(&store, threader_env.clone(), asml_abi_clock_time_get),

            "__asml_abi_input_start" => Function::new_native_with_env(&store, threader_env.clone(), asml_abi_input_start),
            "__asml_abi_input_next" => Function::new_native_with_env(&store, threader_env.clone(), asml_abi_input_next),
            "__asml_abi_input_length_get" => Function::new_native_with_env(&store, threader_env.clone(), asml_abi_input_length_get),

            "__asml_expabi_z85_encode" => Function::new_native_with_env(&store, threader_env.clone(), asml_abi_z85_encode),
            "__asml_expabi_z85_decode" => Function::new_native_with_env(&store, threader_env.clone(), asml_abi_z85_decode),
        },
    };

    let import_object: Resolver = asml_imports.chain_back(wasi_imports);

    Ok((import_object, threader_env))
}

pub fn new_instance(
    module: Arc<Module>,
    import_object: Resolver,
) -> Result<Instance, InstantiationError> {
    Instance::new(&module, &import_object)
}
