use capnp::capability::Promise;
use capnp::Error;

use assemblylift_core_iomod::iomod_capnp::*;

struct Iomod;

impl iomod::Server for Iomod {
    fn get_declaration(&mut self,
                       params: iomod::GetDeclarationParams,
                       results: iomod::GetDeclarationResults)
        -> Promise<(), Error> {

        Promise::ok(())
    }

    fn invoke(&mut self,
              params: iomod::InvokeParams,
              results: iomod::InvokeResults)
        -> Promise<(), Error> {
        // TODO invoke the "real" method here
        Promise::ok(())
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
