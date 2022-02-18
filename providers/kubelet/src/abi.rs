use assemblylift_core::abi::RuntimeAbi;
use assemblylift_core::threader::ThreaderEnv;

pub struct KubeletAbi;

impl RuntimeAbi for KubeletAbi {
    fn log(env: &ThreaderEnv, ptr: u32, len: u32) {
        todo!()
    }

    fn success(env: &ThreaderEnv, ptr: u32, len: u32) {
        // TODO interrupt or state transition (or both!)
        todo!()
    }
}
