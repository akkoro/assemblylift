pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

#[macro_export]
macro_rules! iomod {
    ($ip:expr, $org:ident.$ns:ident.$name:ident => $calls:tt) => {
        use assemblylift_core_iomod::iomod_capnp::*;
        use assemblylift_core_iomod::{
            Call, CallChannel, CallMap, CallPtr, CallRequest, CallResponse, Iomod,
        };
        use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
        use futures::{AsyncReadExt, FutureExt};
        use tokio::net::TcpStream;
        use tokio::sync::mpsc;

        let org = stringify!($org);
        let ns = stringify!($ns);
        let name = stringify!($name);

        let iomod_coords = format!("{}.{}.{}", org, ns, name);
        println!("Starting AssemblyLift IO module {}", iomod_coords);

        let mut call_map: CallMap = $crate::__calls!($calls);
        let mut call_channel: CallChannel = mpsc::channel(100);

        let stream = TcpStream::connect(format!("{}:13555", $ip)).await.unwrap();
        stream.set_nodelay(true).unwrap();

        let (reader, writer) = tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();

        let rpc_network = Box::new(twoparty::VatNetwork::new(
            reader,
            writer,
            rpc_twoparty_capnp::Side::Client,
            Default::default(),
        ));

        let mut rpc_system = RpcSystem::new(rpc_network, None);
        let registry: registry::Client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

        let local = tokio::task::LocalSet::new();
        local
            .run_until(async move {
                let rpc_task = tokio::task::spawn_local(Box::pin(rpc_system.map(|_| ())));

                let mut register = registry.register_request();
                register
                    .get()
                    .set_iomod(capnp_rpc::new_client(Iomod::new(call_channel.0.clone())));
                register.get().set_coordinates(iomod_coords.as_str());
                register.send().promise.await.unwrap();

                let call_task = tokio::task::spawn_local(async move {
                    while let Some(mut call) = call_channel.1.recv().await {
                        let coords = call.coords.as_str();
                        let call_ptr = call_map.get(String::from(coords), call.input);

                        let response = call_ptr.await;

                        if let Err(why) = call
                            .responder
                            .send(CallResponse {
                                coords: String::from(coords),
                                payload: response,
                            })
                            .await
                        {
                            println!("ERROR {}", why)
                        }
                    }
                });

                let (_, _) = tokio::join!(rpc_task, call_task);
            })
            .await;
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __calls {
    ({ $( $call_name:ident => $call:expr ),* $(,)? }) => {{
        let mut call_map = CallMap::new();
        $(
            let call_name = stringify!($call_name);
            call_map.map.insert(call_name, CallPtr::new($call));
        )*
        call_map
    }};
}
