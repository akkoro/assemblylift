use std::collections::HashMap;
use std::sync::Mutex;
use std::any::*;
use crate::Event;

use indexmap::IndexMap;
use std::pin::Pin;

// TODO: do we need this map? could just serialize the ANY-future to the event buffer (becoming just the rpc buffer).
//          impls still return an event id which is an offset into the buffer.
//          structure buffer like [ len, 0..len, len1, 0..len1]
//          EventManager is then concerned with determining where the dynamically sized Futures can fit in the static buffer
pub struct EventManager {}

impl EventManager {
    pub fn new() -> Self {
        EventManager {}
    }
}