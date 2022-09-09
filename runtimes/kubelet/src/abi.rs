use std::cell::Cell;
use std::error::Error;
use std::io;
use std::io::ErrorKind;
use tracing::info;

use wasmer::MemoryView;

use assemblylift_core::abi::RuntimeAbi;
use assemblylift_core::threader::ThreaderEnv;

pub struct KubeletAbi;

impl<S> RuntimeAbi<S> for KubeletAbi
where
    S: Clone + Send + Sized + 'static,
{
    fn log(env: &ThreaderEnv<S>, ptr: u32, len: u32) {
        let string = ptr_to_string(env, ptr, len).unwrap();
        info!("{}", &string)
    }

    fn success(env: &ThreaderEnv<S>, ptr: u32, len: u32) {
        todo!()
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
