pub struct Guest {}

pub trait GuestCore {
    // static methods
    fn console_log(message: String);
    fn success(response: String);
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

                    let name = "$org.$ns.$name";
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
