use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use clap::crate_version;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

use assemblylift_core::buffers::LinearBuffer;
use assemblylift_core::wasm;

use crate::abi::OpenFaasAbi;

mod abi;

async fn launcher(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // todo!("deserialize body");
    //
    // let (module, resolver, threader_env)
    //     = match wasm::build_module_from_path::<OpenFaasAbi, ()>()
    // {
    //     Ok(module) => (Arc::new(module.0), module.1, module.2),
    //     Err(_) => {}
    // };
    //
    // let instance = match wasm::new_instance(module.clone(), resolver.clone()) {
    //     Ok(instance) => Arc::new(instance),
    //     Err(why) => panic!("PANIC {}", why.to_string()),
    // };
    // threader_env.host_input_buffer.clone().lock().unwrap()
    //     .initialize(event.event_body.into_bytes());

    let bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let s = String::from_utf8(bytes.to_vec()).unwrap();
    println!("body={}", s.clone());
    Ok(Response::new(Body::from(s)))
}

#[tokio::main]
async fn main() {
    println!(
        "Starting AssemblyLift OpenFaas runtime {}",
        crate_version!()
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(launcher))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
