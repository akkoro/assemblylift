use std::collections::HashMap;
use std::future::Future;

pub struct EventManager {
    event_to_future: HashMap<i32, Box<dyn Future<Output=()>>>
}

impl EventManager {
    pub fn new() -> Self {
        EventManager {
            event_to_future: HashMap::new()
        }
    }

    pub fn bind_event_to_future(&mut self, event_id: i32, future: Box<dyn Future<Output=()>>) {
        self.event_to_future.entry(event_id).or_insert(future);
    }
}
