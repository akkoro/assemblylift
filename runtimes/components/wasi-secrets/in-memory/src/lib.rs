use std::collections::BTreeMap;
use std::sync::Mutex;

use anyhow::anyhow;
use chacha20poly1305::aead::Aead;
use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit, Nonce};
use once_cell::sync::Lazy;
use rand::RngCore;
use tracing::info;

use assemblylift_core::{KeysAbi, SecretsAbi};

static KEYS: Lazy<Mutex<BTreeMap<String, &[u8; 32]>>> = Lazy::new(|| {
    let mut map = BTreeMap::new();
    map.insert(
        "default".to_string(),
        b"This key is not secure\0\0\0\0\0\0\0\0\0\0",
    );
    Mutex::new(map)
});
static SECRETS: Lazy<Mutex<BTreeMap<String, (String, Vec<u8>)>>> =
    Lazy::new(|| Mutex::new(BTreeMap::new()));

pub struct InMemorySecrets;

impl KeysAbi for InMemorySecrets {
    fn encrypt(id: String, plaintext: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        info!("encrypting with key_id={}", &id);

        let mut rng = rand::thread_rng();
        let mut nonce_bytes: [u8; 12] = [0; 12];
        rng.fill_bytes(&mut nonce_bytes);

        let keys = KEYS.lock().unwrap();
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
        info!("decrypting with key_id={}", &id);

        let raw_data: &[u8] = ciphertext.as_ref();
        if raw_data.len() <= 44 {
            return Err(anyhow!("Encrypted data too short"));
        }

        let mut nonce_bytes: [u8; 12] = [0; 12];
        nonce_bytes.clone_from_slice(&raw_data[..12]);

        let mut sealed: Vec<u8> = Vec::new();
        sealed.extend(&raw_data[12..]);

        let keys = KEYS.lock().unwrap();
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

impl SecretsAbi for InMemorySecrets {
    fn get_secret(id: String) -> anyhow::Result<Vec<u8>> {
        info!("retrieving secret id={}", &id);
        // TODO detect secret manager from id, e.g. AWS should be an ARN
        //      may need to enforce prefixes for other managers e.g. vault/adfuuid-deadb33f-adfd
        let secrets = SECRETS.lock().unwrap();
        let secret_pair = secrets.get(&id.clone()).unwrap();
        Ok(Self::decrypt(secret_pair.0.clone(), secret_pair.1.clone())?)
    }

    fn set_secret(id: String, value: Vec<u8>, key_id: Option<String>) -> anyhow::Result<()> {
        info!("storing secret id={}", &id);
        let key_id = key_id.unwrap_or("default".to_string());
        let ciphertext = Self::encrypt(key_id.clone(), value)?;
        SECRETS
            .lock()
            .unwrap()
            .insert(id.clone(), (key_id.clone(), ciphertext));
        Ok(())
    }
}
