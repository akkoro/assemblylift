use std::path::PathBuf;

use anyhow::{Context, Result};
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};

use host::{add_to_linker, WasiCommand};
use wasi_cap_std_sync::WasiCtxBuilder;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let input = PathBuf::from(
        std::env::args()
            .collect::<Vec<String>>()
            .get(1)
            .context("must provide an input file")?,
    );

    let mut config = Config::new();
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    config.wasm_component_model(true);
    config.async_support(true);

    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, &input)?;
    let mut linker = Linker::new(&engine);
    add_to_linker(&mut linker, |x| x)?;

    let mut store = Store::new(
        &engine,
        WasiCtxBuilder::new()
            .inherit_stdin()
            .inherit_stdout()
            .build(),
    );

    let (wasi, _instance) = WasiCommand::instantiate_async(&mut store, &component, &linker).await?;

    let result: Result<(), ()> = wasi.command(&mut store, 0, 1, &[], &[], &[]).await?;

    if result.is_err() {
        anyhow::bail!("command returned with failing exit status");
    }

    Ok(())
}
