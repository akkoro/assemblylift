use std::io::Read;
use crate::event::Event;
use crate::event::constants::EventBuffer;

#[cfg(feature = "host")]
use wasmer_runtime::Instance;

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>(),
    )
}

#[cfg(feature = "client")]
pub unsafe fn serialize_event(id: usize, event: &Event) {
    use crate::event::widget::EVENT_BUFFER;

    let event_size = std::mem::size_of::<Event>();
    let mut idx = id * event_size;
    let buffer = any_as_u8_slice(event);
    for b in buffer {
        EVENT_BUFFER[idx] = *b;
        idx += 1;
    }
}

#[cfg(feature = "host")]
pub unsafe fn serialize_event_from_host(id: usize, event: &Event, instance: &Instance) {
    let event_size = std::mem::size_of::<Event>();
    let mut idx = id * event_size;
    let buffer = any_as_u8_slice(event);
    for b in buffer {
        // TODO get pointer to event buffer
        idx += 1;
    }
}

pub unsafe fn deserialize_event(id: usize) -> Box<Event> {
    use crate::event::widget::EVENT_BUFFER;

    let event_size = std::mem::size_of::<Event>();
    let buffer_size = std::mem::size_of::<EventBuffer>();

    let mut buffer: &[u8] = EVENT_BUFFER[id * event_size..buffer_size].as_ref();
    let mut event: Event = std::mem::zeroed();

    let event_slice = std::slice::from_raw_parts_mut(&mut event as *mut _ as *mut u8, event_size);
    buffer.read_exact(event_slice).unwrap();

    Box::from(event)
}
