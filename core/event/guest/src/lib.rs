use std::ffi::c_void;
use std::future::Future;
use std::io::Read;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

const MAX_EVENTS: usize              = 1024;
const EVENT_SIZE_BYTES: usize        = 32;
const EVENT_BUFFER_SIZE_BYTES: usize = MAX_EVENTS * EVENT_SIZE_BYTES;

// Raw buffer holding serialized Event-Future's
pub static mut EVENT_BUFFER: [u8; EVENT_BUFFER_SIZE_BYTES] = [0; EVENT_BUFFER_SIZE_BYTES];

#[no_mangle]
pub fn __asml_get_event_buffer_pointer() -> *const u8 {
    unsafe { EVENT_BUFFER.as_ptr() }
}

pub struct Event {
    pub id: u32,
    waker: Option<Waker>
}

impl Event {
    pub fn new(id: u32) -> Self {
        Event { id, waker: None }
    }
}

impl Future for Event {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // TODO read state from event memory

        // match self.state {
        //     State::Resolved => Poll::Ready(()),
        //     State::Failed => Poll::Ready(()),
        //     _ => {
        //         self.waker = Some(cx.waker().clone());
        //         Poll::Pending
        //     }
        // }
        self.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}
