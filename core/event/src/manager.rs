use std::collections::HashMap;
use std::sync::Mutex;
use crate::Event;

use indexmap::IndexMap;

pub struct EventManager {
    pub event_id_to_future: IndexMap<i32, Box<dyn EventFuture>>
}

impl EventManager {
    pub fn new() -> Self {
        EventManager {
            event_id_to_future: IndexMap::new()
        }
    }

    // pub fn bind_event_to_future(&mut self, event_id: i32, future: Box<dyn Future<Output=()>>) {
    //     self.event_to_future.entry(event_id).or_insert(future);
    // }

    pub fn bind_future_to_event(&mut self, future: Box<dyn EventFuture>) -> Event {
        let event = Event::new();
        // future.bind(self, event)
        self.event_to_future.entry(event_index as i32).or_insert(future);
    }
}

pub trait EventFuture {
    fn bind(&self, event_manager: &mut EventManager, event: Event) -> Event;
}