pub use wasmtime::{AsContextMut, Caller};

pub mod buffers;
pub mod threader;
pub mod wasm;

pub trait RuntimeAbi<S>: SecretsAbi
where
    S: Clone + Send + Sized + 'static,
{
    fn success(status_tx: crate::wasm::StatusTx<S>, response: Vec<u8>, request_id: Option<String>);
    fn failure(status_tx: crate::wasm::StatusTx<S>, response: Vec<u8>, request_id: Option<String>);
}

pub trait SecretsAbi {
    fn get_secret(id: String) -> anyhow::Result<Vec<u8>>;
    fn set_secret(id: String, value: Vec<u8>) -> anyhow::Result<()>;
}
