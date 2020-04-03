use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

pub mod constants;
pub mod serde;
pub mod builder;
pub mod executor;
pub mod manager;

#[cfg(feature = "client")]
pub mod widget;

use crate::event::constants::*;

fn __get_index() -> usize {
    23
} // STUB to event manager

pub fn event_engine_sanity_check() {
    print!("Sanity check... ");

    let event_size = std::mem::size_of::<Event>();
    std::assert!(EVENT_SIZE_BYTES == event_size,
    "Size of EVENT_BUFFER has changed! Expected: {} Got: {}", EVENT_SIZE_BYTES, event_size);

    println!("OK!");
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub enum State {
    Unknown,
    Pending,
    Resolved,
    Failed
}

#[repr(C, packed)]
pub struct EventState {
    /// where the event is stored in EVENT_BUFFER
    pub id: usize,
    state: State
}

pub struct Event {
    pub state: EventState,
    waker: Option<Waker>
}

impl Event {
    pub fn new() -> Self {
        let id = __get_index();

        // the 'thread' corresponding to this event lives in the host
        Event {
            state: EventState {
                id,
                state: State::Pending
            },
            waker: None
        }
    }
}

impl Future for Event {
    type Output = ();

    /// poll is called by the Executor
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // The Event state is updated by the host. All we need to do here is query it.
        match self.state.state {
            State::Resolved => Poll::Ready(()),
            State::Failed => Poll::Ready(()),
            _ => {
                self.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}
