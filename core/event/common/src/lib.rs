use serde::{Deserialize, Serialize};

pub mod constants;

// TODO move this and try adding a field for `writer`

#[derive(Clone, Deserialize, Serialize)]
pub struct EventMemoryDocument {
    pub start: usize,
    pub length: usize,
}
