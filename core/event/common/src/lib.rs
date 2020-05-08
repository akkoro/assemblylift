use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{MapAccess, Visitor};

use std::collections::HashMap;

pub const NUM_EVENT_HANDLES: usize = 20;
pub type EventHandles = [(u32, bool); NUM_EVENT_HANDLES];

#[derive(Clone, Deserialize, Serialize)]
pub struct EventStatus(pub EventHandles);

#[derive(Clone, Deserialize, Serialize)]
pub struct EventMemoryDocument {
    pub start: usize,
    pub length: usize
}
