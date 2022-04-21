use std::cell::Cell;
use std::error::Error;
use std::io;
use std::io::ErrorKind;
use wasmer::MemoryView;
use assemblylift_core::abi::RuntimeAbi;
use assemblylift_core::threader::ThreaderEnv;

pub struct LambdaAbi;

impl<S> RuntimeAbi<S> for LambdaAbi
where
    S: Clone + Send + Sized + 'static,
{
    fn log(env: &ThreaderEnv<S>, ptr: u32, len: u32) {
        let string = runtime_ptr_to_string(env, ptr, len).unwrap();
        println!("LOG: {}", string);
    }

    fn success(env: &ThreaderEnv<S>, ptr: u32, len: u32) {
        let lambda_runtime = &crate::LAMBDA_RUNTIME;
        let response = runtime_ptr_to_string(env, ptr, len).unwrap();
        let threader = env.threader.clone();

        let respond = lambda_runtime.respond(response.to_string());
        threader.lock().unwrap().spawn(respond);
    }
}

fn runtime_ptr_to_string<S>(env: &ThreaderEnv<S>, ptr: u32, len: u32) -> Result<String, io::Error>
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
