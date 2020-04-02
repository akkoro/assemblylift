pub const MAX_EVENTS: usize              = 1024;
pub const EVENT_SIZE_BYTES: usize        = 32;
pub const EVENT_BUFFER_SIZE_BYTES: usize = MAX_EVENTS * EVENT_SIZE_BYTES;

pub type EventBuffer = [u8; EVENT_BUFFER_SIZE_BYTES];
