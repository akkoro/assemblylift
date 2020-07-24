pub struct Guest {}

pub trait GuestCore {
    // static methods
    fn console_log(message: String);
    fn success(response: String);
}
