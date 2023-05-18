use assemblylift_core::wasm::StatusTx;
use assemblylift_core::{KeysAbi, RuntimeAbi, SecretsAbi};
use assemblylift_wasi_secrets_in_memory::InMemorySecrets;

#[derive(Clone)]
pub enum Status {
    Success((Option<String>, serde_json::Value)),
    Failure((Option<String>, serde_json::Value)),
}

pub struct Abi;

impl KeysAbi for Abi {
    fn encrypt(id: String, plaintext: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        InMemorySecrets::encrypt(id, plaintext)
    }

    fn decrypt(id: String, ciphertext: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        InMemorySecrets::decrypt(id, ciphertext)
    }
}

impl SecretsAbi for Abi {
    fn get_secret(id: String) -> anyhow::Result<Vec<u8>> {
        // TODO detect secret manager from id, e.g. AWS should be an ARN
        //      may need to enforce prefixes for other managers e.g. vault/adfuuid-deadb33f-adfd
        InMemorySecrets::get_secret(id)
    }

    fn set_secret(id: String, value: Vec<u8>, key_id: Option<String>) -> anyhow::Result<()> {
        InMemorySecrets::set_secret(id, value, key_id)
    }
}

impl RuntimeAbi<Status> for Abi {
    fn success(status_tx: StatusTx<Status>, response: Vec<u8>, request_id: Option<String>) {
        let response = serde_json::from_slice(response.as_slice()).unwrap();
        status_tx
            .send(Status::Success((request_id, response)))
            .unwrap();
    }

    fn failure(status_tx: StatusTx<Status>, response: Vec<u8>, request_id: Option<String>) {
        let response = serde_json::from_slice(response.as_slice()).unwrap();
        status_tx
            .send(Status::Failure((request_id, response)))
            .unwrap();
    }
}
