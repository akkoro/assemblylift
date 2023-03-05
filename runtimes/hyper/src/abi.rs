use std::collections::BTreeMap;
use std::sync::Mutex;

use clap::__macro_refs::once_cell::sync::Lazy;
use tracing::{debug, error, info};

use assemblylift_core::{RuntimeAbi, SecretsAbi};
use assemblylift_core::wasm::StatusTx;

use crate::Status;

pub static INMEM_SECRETS: Lazy<Mutex<BTreeMap<String, Vec<u8>>>> = Lazy::new(|| Mutex::new(BTreeMap::new()));

pub struct Abi;

impl SecretsAbi for Abi {
    fn get_secret(id: String) -> anyhow::Result<Vec<u8>> {
        // TODO detect secret manager from id, e.g. AWS should be an ARN
        //      may need to enforce prefixes for other managers e.g. vault/adfuuid-deadb33f-adfd
        Ok(INMEM_SECRETS.lock().unwrap().get(&id.clone()).unwrap().to_vec())
    }

    fn set_secret(id: String, value: Vec<u8>) -> anyhow::Result<()> {
        info!("storing secret id={}", id.clone());
        INMEM_SECRETS.lock().unwrap().insert(id.clone(), value.clone());
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
            if let Err(e) = status_tx.send(Status::Success(response)) {
                error!("could not send status: {:?}", e.to_string())
            }
        });
    }
}
