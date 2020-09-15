pub mod macros {
    #[macro_export]
    macro_rules! export_iomod_guest {
        ($org:ident, $namespace:ident, $name: ident) => {
            static IOMOD_ORG: &'static str = std::stringify!($org);
            static IOMOD_NAMESPACE: &'static str = std::stringify!($namespace);
            static IOMOD_NAME: &'static str = std::stringify!($name);

            extern "C" {
                fn __asml_abi_invoke(
                    mem: *const u8,
                    name_ptr: *const u8,
                    name_len: usize,
                    input_ptr: *const u8,
                    input_len: usize,
                ) -> i32;
            }
        };
    }

    #[macro_export]
    macro_rules! call {
        ($name:ident, $input:ty => $output:ty) => {
            pub fn $name<'a>(input: $input) -> Event<'a, $output> {
                use assemblylift_core_event_guest::{Event, EVENT_BUFFER};
                use serde_json;

                let name = std::stringify!($name);
                let method_path =
                    format!("{}.{}.{}.{}", IOMOD_ORG, IOMOD_NAMESPACE, IOMOD_NAME, name);

                let event_id: i32;
                unsafe {
                    let serialized: Box<Vec<u8>> = Box::from(serde_json::to_vec(&input).unwrap());
                    event_id = crate::__asml_abi_invoke(
                        EVENT_BUFFER.as_ptr(),
                        method_path.as_ptr(),
                        method_path.len(),
                        serialized.as_ptr(),
                        serialized.len(),
                    );
                }

                match event_id {
                    -1 => panic!("unable to invoke fn {}", name),
                    _ => Event::<$output>::new(event_id as u32),
                }
            }
        };
    }
}
