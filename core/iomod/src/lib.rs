use capnp::capability::Promise;

use crate::iomod_capnp::agent;

pub mod iomod_capnp;
pub mod macros;
pub mod registry;

struct Agent {}

impl agent::Server for Agent {
    fn invoke(&mut self,
              params: agent::InvokeParams,
              results: agent::InvokeResults
    ) -> Promise<(), capnp::Error> {
        Promise::from_future(async move {

            Ok(())
        })
    }
}
