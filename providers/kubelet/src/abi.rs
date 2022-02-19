use assemblylift_core::abi::RuntimeAbi;
use assemblylift_core::threader::ThreaderEnv;

pub struct KubeletAbi;

impl<S> RuntimeAbi<S> for KubeletAbi
where
    S: Clone + Send + Sized + 'static
{
    fn log(env: &ThreaderEnv<S>, ptr: u32, len: u32) {
        todo!()
    }

    fn success(env: &ThreaderEnv<S>, ptr: u32, len: u32) {
        todo!()
        // FIXME not sure yet how to handle this with k8s
        //      can a status sender be attached to the threaderenv?
    }
}
