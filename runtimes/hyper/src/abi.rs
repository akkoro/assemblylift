use tracing::error;

use assemblylift_core::RuntimeAbi;
use assemblylift_core::wasm::StatusTx;

use crate::Status;

pub struct Abi;

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
