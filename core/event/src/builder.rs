use crate::{Event, State};

use std::collections::HashMap;
use std::borrow::{Borrow, BorrowMut};
use std::future::Future;
use futures::task::{Context, Poll};
use std::pin::Pin;

pub type StateMap = HashMap<State, Option<fn()>>;
pub type StateTable = HashMap<usize, StateMap>;

pub struct EventChain {
    pub state_table: StateTable,

    root_event_id: usize,
    last_state: State
}

impl EventChain {
    pub fn new(root_event_id: usize) -> EventChain {
        let mut event_chain = EventChain {
            root_event_id,

            state_table: HashMap::new(),
            last_state: State::Unknown
        };

        event_chain.state_table.entry(root_event_id).or_insert(HashMap::new());

        event_chain
    }

    pub fn is(&mut self, state: State) -> &mut EventChain {
        match self.state_table.get_mut(&self.root_event_id) {
            Some(event) => {
                event.entry(state).or_insert(None);
                self
            },
            _ => {
                self.last_state = state;
                self
            }
        }
    }

    pub fn then(&mut self, callback: fn()) -> &mut EventChain {
        match self.state_table.get_mut(&self.root_event_id) {
            Some(event) => {
                event.entry(self.last_state).or_insert(Some(callback));
                self
            },
            _ => self
        }
    }

    // MAYBE add a function `always`
    //  pub fn always(&mut self, callback: fn()) -> &mut EventChain {}

    // MAYBE refactor into returning a FutureObj
    pub fn build(self) -> EventChain {
        // self.state_table.g
        EventChain {
            state_table: Default::default(),
            root_event_id: 0,
            last_state: State::Unknown
        }
    }
}

impl Future for EventChain {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Q does this just poll contained events?
        unimplemented!()
    }
}

pub fn when(event: &Event) -> EventChain {
    EventChain::new(event.inner.id)
}
