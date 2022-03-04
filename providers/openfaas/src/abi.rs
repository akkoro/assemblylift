use assemblylift_core::abi::RuntimeAbi;
use assemblylift_core::threader::ThreaderEnv;

pub struct OpenFaasAbi;

impl<S> RuntimeAbi<S> for OpenFaasAbi
where
    S: Clone + Send + Sized + 'static,
{
    fn log(env: &ThreaderEnv<S>, ptr: u32, len: u32) {
        todo!()
    }

    fn success(env: &ThreaderEnv<S>, ptr: u32, len: u32) {
        todo!()
    }
}
