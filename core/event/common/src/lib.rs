use serde::{Deserialize, Serialize};

pub mod constants;

#[derive(Clone, Deserialize, Serialize)]
pub struct EventMemoryDocument {
    pub start: usize,
    pub length: usize,
}

pub fn sanitize_buffer(buffer: &[u8]) -> &[u8] {
    let mut len: usize = 0;
    unsafe {
        for (i, &b) in buffer.iter().enumerate() {
            if b == '\0' as u8 {
                len = i;
                break;
            }
        }
    }
    let slice = unsafe { &buffer[0..len] };
    slice
}
