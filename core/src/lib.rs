pub use wasmtime::AsContextMut;

pub mod buffers;
pub mod jwt;
pub mod policy_manager;
pub mod threader;
pub mod wasm;

pub trait RuntimeAbi<S>: SecretsAbi
where
    S: Clone + Send + Sized + 'static,
{
    fn success(status_tx: crate::wasm::StatusTx<S>, response: Vec<u8>, request_id: Option<String>);
    fn failure(status_tx: crate::wasm::StatusTx<S>, response: Vec<u8>, request_id: Option<String>);
}

pub trait SecretsAbi: KeysAbi {
    fn get_secret(id: String) -> anyhow::Result<Vec<u8>>;
    fn set_secret(id: String, value: Vec<u8>, key_id: Option<String>) -> anyhow::Result<()>;
}

pub trait KeysAbi {
    fn encrypt(id: String, plaintext: Vec<u8>) -> anyhow::Result<Vec<u8>>;
    fn decrypt(id: String, ciphertext: Vec<u8>) -> anyhow::Result<Vec<u8>>;
}
