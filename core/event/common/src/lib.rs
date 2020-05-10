use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{MapAccess, Visitor};

pub mod constants;

#[derive(Clone, Deserialize, Serialize)]
pub struct EventMemoryDocument {
    pub start: usize,
    pub length: usize
}
