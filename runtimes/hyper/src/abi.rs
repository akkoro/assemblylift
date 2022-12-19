use tracing::{debug, error, info};

use assemblylift_core::abi::RuntimeAbi;
use assemblylift_core::Caller;
use assemblylift_core::wasm::{State, Wasmtime};

use crate::Status;

pub struct GenericDockerAbi;

impl RuntimeAbi<Status> for GenericDockerAbi {
    fn log(mut caller: Caller<'_, State<Status>>, ptr: u32, len: u32) {
        let s = Wasmtime::<Self, Status>::ptr_to_string(&mut caller, ptr, len).unwrap();
        info!("Guest: {}", s);
    }

    fn success(mut caller: Caller<'_, State<Status>>, ptr: u32, len: u32) {
        debug!("called success");
        let tx = caller.data().status_sender.clone();
        let s = Wasmtime::<Self, Status>::ptr_to_string(&mut caller, ptr, len).unwrap();
        std::thread::spawn(move || {
            if let Err(e) = tx.send(Status::Success(s.into())) {
                error!("could not send status: {:?}", e.to_string())
            }
        });
    }
}
