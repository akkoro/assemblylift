use assemblylift_core::abi::RuntimeAbi;
use assemblylift_core::Caller;
use assemblylift_core::wasm::{State, StatusTx, Wasmtime};

#[derive(Clone)]
pub enum Status {
    Success(String),
    Failure(String),
}

pub struct LambdaAbi;

impl RuntimeAbi<Status> for LambdaAbi {
    fn success(status_tx: StatusTx<Status>, response: Vec<u8>) {
        let response = std::str::from_utf8(response.as_slice()).unwrap().to_string();
        status_tx.send(Status::Success(response)).unwrap();
    }

    fn failure(status_tx: StatusTx<Status>, response: Vec<u8>) {
        let response = std::str::from_utf8(response.as_slice()).unwrap().to_string();
        status_tx.send(Status::Failure(response)).unwrap();
    }
}
