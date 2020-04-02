use crate::event::constants::*;

// Raw buffer holding serialized Event-Future's
pub static mut EVENT_BUFFER: [u8; EVENT_BUFFER_SIZE_BYTES] = [0; EVENT_BUFFER_SIZE_BYTES];
