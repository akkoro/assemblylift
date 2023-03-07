use std::collections::BTreeMap;
use std::sync::Mutex;

use anyhow::anyhow;
use chacha20poly1305::aead::Aead;
use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit, Nonce};
use once_cell::sync::Lazy;
use rand::RngCore;
use tracing::{debug, error, info};

use assemblylift_core::wasm::StatusTx;
use assemblylift_core::{KeysAbi, RuntimeAbi, SecretsAbi};

use crate::Status;

pub static INMEM_KEYS: Lazy<Mutex<BTreeMap<String, &[u8; 32]>>> = Lazy::new(|| {
    let mut map = BTreeMap::new();
    map.insert(
        "default".to_string(),
        b"This key is not secure\0\0\0\0\0\0\0\0\0\0",
    );
    Mutex::new(map)
});
pub static INMEM_SECRETS: Lazy<Mutex<BTreeMap<String, (String, Vec<u8>)>>> =
    Lazy::new(|| Mutex::new(BTreeMap::new()));

pub struct Abi;

impl KeysAbi for Abi {
    fn encrypt(id: String, plaintext: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let mut rng = rand::thread_rng();
        let mut nonce_bytes: [u8; 12] = [0; 12];
        rng.fill_bytes(&mut nonce_bytes);

        let keys = INMEM_KEYS.lock().unwrap();
        let key_bytes = keys.get(&*id).unwrap();
        let key = Key::from_slice(*key_bytes);
        let cipher = ChaCha20Poly1305::new(&key);
        let nonce = Nonce::from(nonce_bytes);

        let encrypted = cipher
            .encrypt(&nonce, &*plaintext)
            .map_err(|e| anyhow!(e.to_string()))?;

        let mut ret: Vec<u8> = Vec::new();
        ret.extend(nonce_bytes);
        ret.extend(encrypted);

        Ok(ret)
    }

    fn decrypt(id: String, ciphertext: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let raw_data: &[u8] = ciphertext.as_ref();
        if raw_data.len() <= 44 {
            return Err(anyhow!("Encrypted data too short"));
        }

        let mut nonce_bytes: [u8; 12] = [0; 12];
        nonce_bytes.clone_from_slice(&raw_data[..12]);

        let mut sealed: Vec<u8> = Vec::new();
        sealed.extend(&raw_data[12..]);

        let keys = INMEM_KEYS.lock().unwrap();
        let key_bytes = keys.get(&*id).unwrap();
        let key = Key::from_slice(key_bytes.as_ref());
        let cipher = ChaCha20Poly1305::new(&key);
        let nonce = Nonce::from(nonce_bytes);

        let decrypted = cipher
            .decrypt(&nonce, sealed.as_ref())
            .map_err(|e| anyhow!(e.to_string()))?;

        Ok(decrypted)
    }
}

impl SecretsAbi for Abi {
    fn get_secret(id: String) -> anyhow::Result<Vec<u8>> {
        // TODO detect secret manager from id, e.g. AWS should be an ARN
        //      may need to enforce prefixes for other managers e.g. vault/adfuuid-deadb33f-adfd
        let secrets = INMEM_SECRETS.lock().unwrap();
        let secret_pair = secrets.get(&id.clone()).unwrap();
        Ok(Abi::decrypt(secret_pair.0.clone(), secret_pair.1.clone())?)
    }

    fn set_secret(id: String, value: Vec<u8>, key_id: Option<String>) -> anyhow::Result<()> {
        info!("storing secret id={}", id.clone());
        let key_id = key_id.unwrap_or("default".to_string());
        let ciphertext = Abi::encrypt(key_id.clone(), value)?;
        INMEM_SECRETS
            .lock()
            .unwrap()
            .insert(id.clone(), (key_id.clone(), ciphertext));
        Ok(())
    }
}

impl RuntimeAbi<Status> for Abi {
    fn success(status_tx: StatusTx<Status>, response: Vec<u8>, _request_id: Option<String>) {
        std::thread::spawn(move || {
            if let Err(e) = status_tx.send(Status::Success(response)) {
                error!("could not send status: {:?}", e.to_string())
            }
        });
    }

    fn failure(status_tx: StatusTx<Status>, response: Vec<u8>, _request_id: Option<String>) {
        std::thread::spawn(move || {
            if let Err(e) = status_tx.send(Status::Failure(response)) {
                error!("could not send status: {:?}", e.to_string())
            }
        });
    }
}
