use std::sync::Arc;

use once_cell::sync::Lazy;
use tokio::runtime::{Builder, Runtime};

pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

#[macro_export]
macro_rules! export_iomod {
    ($org:ident.$ns:ident.$name:ident => $module:ident) => {
        extern "C" fn register(registry: &mut ModuleRegistry) {
            println!("DEBUG: registering IOmod");
            $module::register(registry);
            println!("DEBUG: registered IOmod");
        }

        #[doc(hidden)]
        #[no_mangle]
        pub static __ASML_IOMOD_PLUGIN_DECL: assemblylift_core::IoModulePlugin =
            assemblylift_core::IoModulePlugin {
                organization: stringify!($org),
                namespace: stringify!($ns),
                name: stringify!($name),
                rustc_version: assemblylift_core_iomod::macros::RUSTC_VERSION,
                asml_core_version: assemblylift_core_iomod::macros::CORE_VERSION,
                runtime: Lazy::new(|| {
                    // Box::new(
                    Builder::new()
                        .threaded_scheduler()
                        .enable_all()
                        .build()
                        .unwrap()
                    // )
                }),
                register,
            };
    };
}

#[macro_export]
macro_rules! register_calls {
    ($reg:expr, $org_name:ident => { $ns_name:ident => $ns:tt }) => {{
        let org_name = stringify!($org_name);
        let ns_name = stringify!($ns_name);

        let mut namespace_map = HashMap::new();

        let mut name_map = __register_calls!($ns);
        namespace_map.entry(ns_name.to_string()).or_insert(name_map);

        println!("DEBUG: registering IOmod calls");

        $reg.modules
            .entry(org_name.to_string())
            .or_insert(namespace_map);

        println!("DEBUG: registered IOmod calls");
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __register_calls {
    ({ $( $call_name:ident => $call:expr ),* $(,)? }) => {{
        let mut name_map = HashMap::new();
        $(
            let call_name = String::from(stringify!($call_name));
            // SEGFAULT goes away when we don't __register_calls, so I'm assuming it's this access
            //      of the runtime that's fucking things
            let call_runtime = (*__ASML_IOMOD_PLUGIN_DECL.runtime);
            name_map.entry(call_name).or_insert(($call as AsmlAbiFn, call_runtime));
        )*
        name_map
    }};
}

#[macro_export]
macro_rules! call {
    ($call_name:ident => $call:item) => {
        $call

        pub fn $call_name (ctx: &mut vm::Ctx, mem: WasmBufferPtr, input: WasmBufferPtr, input_len: u32) -> i32 {
            use assemblylift_core_event::spawn_event;

            println!("TRACE: {}", stringify!($call_name));
            let input_vec = __wasm_buffer_as_vec!(ctx, input, input_len);
            let call = paste::expr! { [<$call_name _impl>] }(input_vec);
            spawn_event(&__ASML_IOMOD_PLUGIN_DECL, ctx, mem, call)
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __wasm_buffer_as_vec {
    ($ctx:ident, $input:ident, $input_len:ident) => {{
        let wasm_instance_memory = $ctx.memory(0);
        let input_deref: &[AtomicCell<u8>] = match $input.deref(wasm_instance_memory, 0, $input_len)
        {
            Some(memory) => memory,
            None => panic!("could not dereference WASM guest memory in __wasm_buffer_as_vec"),
        };

        let mut as_vec: Vec<u8> = Vec::new();
        for (idx, b) in input_deref.iter().enumerate() {
            as_vec.insert(idx, b.load());
        }

        as_vec
    }};
}
