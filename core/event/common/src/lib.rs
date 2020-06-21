use serde::{Deserialize, Serialize};

pub mod constants;

#[derive(Clone, Deserialize, Serialize)]
pub struct EventMemoryDocument {
    pub start: usize,
    pub length: usize
}
