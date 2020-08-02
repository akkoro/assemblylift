pub mod macros {
    #[macro_export]
    macro_rules! export_iomod_guest {
        () => {
            extern "C" {
                fn __asml_abi_invoke(
                    mem: *const u8,
                    name_ptr: *const u8,
                    name_len: usize,
                    input_ptr: *const u8,
                    input_len: usize,
                ) -> i32;
            }
        }
    }

    #[macro_export]
    macro_rules! call {
        ($org:ident => $ns:ident => $name:ident, $input:ty => $output:ty) => {
            paste::item_with_macros! {
                pub fn [<$org _ $ns _ $name>]<'a>(input: $input) -> Event<'a, $output> {
                    use serde_json;
                    use assemblylift_core_event_guest::{Event, EVENT_BUFFER};

                    let event_id: i32;
                    unsafe {
                        let serialized: Box<Vec<u8>> = Box::from(serde_json::to_vec(&input).unwrap());

                        let name = format!("{}.{}.{}",
                            std::stringify!($org),
                            std::stringify!($ns),
                            std::stringify!($name));

                        event_id = crate::__asml_abi_invoke(EVENT_BUFFER.as_ptr(),
                                                            name.as_ptr(), name.len(),
                                                            serialized.as_ptr(), serialized.len());
                    }

                    match event_id {
                        -1 => panic!("unable to invoke fn $org.$ns.$name"),
                        _ => Event::<$output>::new(event_id as u32)
                    }
                };
            }
        };
    }
}