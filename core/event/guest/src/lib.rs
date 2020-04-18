use assemblylift_core_event::constants::*;
use assemblylift_core_event::Event;

// Raw buffer holding serialized Event-Future's
pub static mut EVENT_BUFFER: [u8; EVENT_BUFFER_SIZE_BYTES] = [0; EVENT_BUFFER_SIZE_BYTES];

#[no_mangle]
pub fn __al_get_event_buffer_pointer() -> *const u8 {
    unsafe { EVENT_BUFFER.as_ptr() }
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>(),
    )
}

pub unsafe fn serialize_event_from_guest(id: usize, event: &Event) {
    let event_size = std::mem::size_of::<Event>();
    let mut idx = id * event_size;
    let buffer = any_as_u8_slice(event);
    for b in buffer {
        EVENT_BUFFER[idx] = *b;
        idx += 1;
    }
}
