const RESPONSE_BUFFER_SIZE: usize = 2048;
static mut RESPONSE_BUFFER: [u8; RESPONSE_BUFFER_SIZE] = [0; RESPONSE_BUFFER_SIZE];

#[no_mangle]
pub fn __al_get_response_buffer_pointer() -> *const u8 {
    unsafe { RESPONSE_BUFFER.as_ptr() }
}

pub struct Guest {}

impl Guest {
    // TODO: this will eventually be a method to wait on some kind of future by id
    pub fn get_response(&self) -> &'static str {
        unsafe { std::str::from_utf8(&RESPONSE_BUFFER[..RESPONSE_BUFFER_SIZE]).unwrap() }
    }
}

pub trait GuestCore {
    // static methods
    fn console_log(message: String);
    fn success(response: String);

    fn test(&self);
}
