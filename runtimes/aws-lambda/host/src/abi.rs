use assemblylift_core::abi::RuntimeAbi;
use assemblylift_core::Caller;
use assemblylift_core::wasm::{State, Wasmtime};

pub type Status = ();

pub struct LambdaAbi;

impl RuntimeAbi<Status> for LambdaAbi {
    fn log(mut caller: Caller<'_, State<Status>>, ptr: u32, len: u32) {
        let s = Wasmtime::<Self, Status>::ptr_to_string(&mut caller, ptr, len).unwrap();
        println!("LOG: {}", s);
    }

    fn success(mut caller: Caller<'_, State<Status>>, ptr: u32, len: u32) {
        let lambda_runtime = &crate::LAMBDA_RUNTIME;
        let response = Wasmtime::<Self, Status>::ptr_to_string(&mut caller, ptr, len).unwrap();

        let respond = lambda_runtime.respond(response);
        let state = caller.data_mut();
        state.threader.clone().lock().unwrap().spawn(respond);
    }
}
