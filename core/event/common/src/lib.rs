use serde::{Deserialize, Serialize};

pub const NUM_EVENT_HANDLES: usize = 20;
pub type EventHandles = [(u32, bool); NUM_EVENT_HANDLES];

#[derive(Clone, Deserialize, Serialize)]
pub struct EventStatus(pub EventHandles);
