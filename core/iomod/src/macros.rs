pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

#[macro_export]
macro_rules! call {
    ($call_name:ident => $call:item) => {
        $call

        pub fn $call_name (ctx: &mut vm::Ctx, mem: WasmBufferPtr, input: WasmBufferPtr, input_len: u32) -> i32 {
            let input_vec = __wasm_buffer_as_vec!(ctx, input, input_len);
            let call = paste::expr! { [<$call_name _impl>] };

            call(input_vec)
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
