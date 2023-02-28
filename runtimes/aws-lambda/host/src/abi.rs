use assemblylift_core::abi::RuntimeAbi;
use assemblylift_core::wasm::{State, StatusTx, Wasmtime};
use assemblylift_core::Caller;

#[derive(Clone)]
pub enum Status {
    Success((Option<String>, String)),
    Failure((Option<String>, String)),
}

pub struct LambdaAbi;

impl RuntimeAbi<Status> for LambdaAbi {
    fn success(status_tx: StatusTx<Status>, response: Vec<u8>, request_id: Option<String>) {
        let response = std::str::from_utf8(response.as_slice())
            .unwrap()
            .to_string();
        status_tx
            .send(Status::Success((request_id, response)))
            .unwrap();
    }

    fn failure(status_tx: StatusTx<Status>, response: Vec<u8>, request_id: Option<String>) {
        let response = std::str::from_utf8(response.as_slice())
            .unwrap()
            .to_string();
        status_tx
            .send(Status::Failure((request_id, response)))
            .unwrap();
    }
}
