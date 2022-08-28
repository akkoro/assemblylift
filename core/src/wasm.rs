use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use wasmer::{
    imports, ChainableNamedResolver, CpuFeature, Cranelift, Function, ImportObject, Instance,
    InstantiationError, Module, NamedResolverChain, Store, Target, Triple, Universal,
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

pub fn deserialize_module_from_bytes<R, S>(module_bytes: &[u8]) -> anyhow::Result<(Module, Store)>
where
    R: RuntimeAbi<S> + 'static,
    S: Clone + Send + Sized + 'static,
{
    let compiler = Cranelift::default();
    let store = Store::new(&Universal::new(compiler).engine());
    Ok((
        unsafe { Module::deserialize(&store, module_bytes) }
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
        "ruby-docker" => WasiState::new(module_name.clone())
            .arg("/src/handler.rb")
            .env("RUBY_PLATFORM", "wasm32-wasi")
            .map_dir("/src", "/usr/bin/ruby-wasm32-wasi/src")
            .expect("could not preopen `src` directory")
            .map_dir("/usr", "/usr/bin/ruby-wasm32-wasi/usr")
            .expect("could not map ruby fs")
            .finalize()
            .expect("could not init WASI env"),
        "ruby-lambda" => WasiState::new(module_name.clone())
            .arg("/src/handler.rb")
            .env("RUBY_PLATFORM", "wasm32-wasi")
            .map_dir("/src", "/tmp/rubysrc")
            .expect("could not preopen `src` directory")
            .map_dir("/usr", "/tmp/rubyusr")
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

pub fn precompile(module_path: PathBuf) -> Result<PathBuf, &'static str> {
    // TODO compiler configuration
    let is_wasmu = module_path
        .extension()
        .unwrap_or("wasm".as_ref())
        .eq("wasmu");
    match is_wasmu {
        false => {
            let file_path = format!("{}u", module_path.as_path().display().to_string());
            println!("Precompiling WASM to {}...", file_path.clone());

            let compiler = Cranelift::default();
            let triple = Triple::from_str("x86_64-unknown-unknown").unwrap();
            let mut cpuid = CpuFeature::set();
            cpuid.insert(CpuFeature::SSE2); // required for x86
            let store = Store::new(
                &/*Native*/Universal::new(compiler)
                .target(Target::new(triple, cpuid))
                .engine(),
            );

            let wasm_bytes = match std::fs::read(module_path.clone()) {
                Ok(bytes) => bytes,
                Err(err) => panic!("{}", err.to_string()),
            };
            let module = Module::new(&store, wasm_bytes).unwrap();
            let module_bytes = module.serialize().unwrap();
            let mut module_file = match std::fs::File::create(file_path.clone()) {
                Ok(file) => file,
                Err(err) => panic!("{}", err.to_string()),
            };
            println!("📄 > Wrote {}", &file_path);
            module_file.write_all(&module_bytes).unwrap();

            Ok(PathBuf::from(file_path))
        }

        true => Ok(module_path),
    }
}
