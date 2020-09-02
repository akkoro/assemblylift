use std::cell::RefCell;
use std::rc::Rc;

use capnp::capability::Promise;

use crate::iomod_capnp::{agent, iomod};

pub mod iomod_capnp;
pub mod macros;
pub mod registry;

pub struct Agent {
    iomod_client: Rc<RefCell<iomod::Client>>
}

impl Agent {
    pub fn new(iomod_client: Rc<RefCell<iomod::Client>>) -> Self {
        Self {
            iomod_client
        }
    }
}

impl agent::Server for Agent {
    fn invoke(&mut self,
              params: agent::InvokeParams,
              mut results: agent::InvokeResults
    ) -> Promise<(), capnp::Error> {
        let client = self.iomod_client.clone();

        Promise::from_future(async move {
            let mut invoke = client.borrow_mut().invoke_request();
            invoke.get().set_coordinates(params.get().unwrap().get_coordinates().unwrap());
            invoke.get().set_input(params.get().unwrap().get_input().unwrap());

            let invoke_response = invoke.send().promise.await.unwrap();
            results.get().set_result(invoke_response.get().unwrap().get_result().unwrap());

            Ok(())
        })
    }
}
