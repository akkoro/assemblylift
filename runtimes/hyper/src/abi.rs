use std::cell::Cell;
use std::error::Error;
use std::io;
use std::io::ErrorKind;

use tracing::{debug, error, info};
use wasmer::MemoryView;

use assemblylift_core::abi::RuntimeAbi;
use assemblylift_core::threader::ThreaderEnv;

use crate::Status;

pub struct GenericDockerAbi;

impl RuntimeAbi<Status> for GenericDockerAbi {
    fn log(env: &ThreaderEnv<Status>, ptr: u32, len: u32) {
        let s = ptr_to_string(env, ptr, len).unwrap();
        info!("Guest: {}", s);
    }

    fn success(env: &ThreaderEnv<Status>, ptr: u32, len: u32) {
        debug!("called success");
        let s = ptr_to_string(env, ptr, len).unwrap();
        let tx = env.status_sender.clone();
        std::thread::spawn(move || {
            if let Err(e) = tx.send(Status::Success(s)) {
                error!("could not send status: {:?}", e.to_string())
            }
        });
    }
}

fn ptr_to_string<S>(env: &ThreaderEnv<S>, ptr: u32, len: u32) -> Result<String, io::Error>
where
    S: Clone + Send + Sized + 'static,
{
    let memory = env.memory_ref().unwrap();
    let view: MemoryView<u8> = memory.view();

    let mut str_vec: Vec<u8> = Vec::new();
    for byte in view[ptr as usize..(ptr + len) as usize]
        .iter()
        .map(Cell::get)
    {
        str_vec.push(byte);
    }

    std::str::from_utf8(str_vec.as_slice())
        .map(String::from)
        .map_err(to_io_error)
}

fn to_io_error<E: Error>(err: E) -> io::Error {
    io::Error::new(ErrorKind::Other, err.to_string())
}
