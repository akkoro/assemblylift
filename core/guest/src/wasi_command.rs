#[allow(clippy::all)]
pub mod wasi_clocks {
    /// A wall clock is a clock which measures the date and time according to some
    /// external reference.
    ///
    /// External references may be reset, so this clock is not necessarily
    /// monotonic, making it unsuitable for measuring elapsed time.
    ///
    /// It is intended for reporting the current date and time for humans.
    pub type WallClock = u32;
    /// A monotonic clock is a clock which has an unspecified initial value, and
    /// successive reads of the clock will produce non-decreasing values.
    ///
    /// It is intended for measuring elapsed time.
    pub type MonotonicClock = u32;
    /// A timestamp in nanoseconds.
    pub type Instant = u64;
    /// A time and date in seconds plus nanoseconds.
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Datetime {
        pub seconds: u64,
        pub nanoseconds: u32,
    }
    impl core::fmt::Debug for Datetime {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Datetime")
                .field("seconds", &self.seconds)
                .field("nanoseconds", &self.nanoseconds)
                .finish()
        }
    }
    #[allow(clippy::all)]
    /// Read the current value of the clock.
    ///
    /// As this the clock is monotonic, calling this function repeatedly will produce
    /// a sequence of non-decreasing values.
    pub fn monotonic_clock_now(clock: MonotonicClock) -> Instant {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[link(wasm_import_module = "wasi-clocks")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "monotonic-clock-now")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-clocks_monotonic-clock-now"
                )]
                fn wit_import(_: i32) -> i64;
            }
            let ret = wit_import(wit_bindgen_guest_rust::rt::as_i32(clock));
            ret as u64
        }
    }
    #[allow(clippy::all)]
    /// Query the resolution of the clock.
    pub fn monotonic_clock_resolution(clock: MonotonicClock) -> Instant {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[link(wasm_import_module = "wasi-clocks")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "monotonic-clock-resolution")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-clocks_monotonic-clock-resolution"
                )]
                fn wit_import(_: i32) -> i64;
            }
            let ret = wit_import(wit_bindgen_guest_rust::rt::as_i32(clock));
            ret as u64
        }
    }
    #[allow(clippy::all)]
    /// Read the current value of the clock.
    ///
    /// As this the clock is not monotonic, calling this function repeatedly will
    /// not necessarily produce a sequence of non-decreasing values.
    ///
    /// The returned timestamps represent the number of seconds since
    /// 1970-01-01T00:00:00Z, also known as [POSIX's Seconds Since the Epoch], also
    /// known as [Unix Time].
    ///
    /// The nanoseconds field of the output is always less than 1000000000.
    ///
    /// [POSIX's Seconds Since the Epoch]: https://pubs.opengroup.org/onlinepubs/9699919799/xrat/V4_xbd_chap04.html#tag_21_04_16
    /// [Unix Time]: https://en.wikipedia.org/wiki/Unix_time
    pub fn wall_clock_now(clock: WallClock) -> Datetime {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(8))]
            struct RetArea([u8; 16]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-clocks")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "wall-clock-now")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-clocks_wall-clock-now")]
                fn wit_import(_: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(clock), ptr0);
            Datetime {
                seconds: *((ptr0 + 0) as *const i64) as u64,
                nanoseconds: *((ptr0 + 8) as *const i32) as u32,
            }
        }
    }
    #[allow(clippy::all)]
    /// Query the resolution of the clock.
    ///
    /// The nanoseconds field of the output is always less than 1000000000.
    pub fn wall_clock_resolution(clock: WallClock) -> Datetime {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(8))]
            struct RetArea([u8; 16]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-clocks")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "wall-clock-resolution")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-clocks_wall-clock-resolution"
                )]
                fn wit_import(_: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(clock), ptr0);
            Datetime {
                seconds: *((ptr0 + 0) as *const i64) as u64,
                nanoseconds: *((ptr0 + 8) as *const i32) as u32,
            }
        }
    }
    #[allow(clippy::all)]
    /// Closes a monotonic clock handle.
    pub fn close_monotonic_clock(clock: MonotonicClock) -> () {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[link(wasm_import_module = "wasi-clocks")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "close-monotonic-clock")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-clocks_close-monotonic-clock"
                )]
                fn wit_import(_: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(clock));
        }
    }
    #[allow(clippy::all)]
    /// Closes a wall clock handle.
    pub fn close_wall_clock(clock: WallClock) -> () {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[link(wasm_import_module = "wasi-clocks")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "close-wall-clock")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-clocks_close-wall-clock"
                )]
                fn wit_import(_: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(clock));
        }
    }
}

#[allow(clippy::all)]
pub mod wasi_default_clocks {
    pub type MonotonicClock = super::wasi_clocks::MonotonicClock;
    pub type WallClock = super::wasi_clocks::WallClock;
    #[allow(clippy::all)]
    /// Return a default monotonic clock, suitable for general-purpose application
    /// needs.
    ///
    /// This allocates a new handle, so applications with frequent need of a clock
    /// handle should call this function once and reuse the handle instead of
    /// calling this function each time.
    pub fn default_monotonic_clock() -> MonotonicClock {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[link(wasm_import_module = "wasi-default-clocks")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "default-monotonic-clock")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-default-clocks_default-monotonic-clock"
                )]
                fn wit_import() -> i32;
            }
            let ret = wit_import();
            ret as u32
        }
    }
    #[allow(clippy::all)]
    /// Return a default wall clock, suitable for general-purpose application
    /// needs.
    ///
    /// This allocates a new handle, so applications with frequent need of a clock
    /// handle should call this function once and reuse the handle instead of
    pub fn default_wall_clock() -> WallClock {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[link(wasm_import_module = "wasi-default-clocks")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "default-wall-clock")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-default-clocks_default-wall-clock"
                )]
                fn wit_import() -> i32;
            }
            let ret = wit_import();
            ret as u32
        }
    }
}

#[allow(clippy::all)]
pub mod wasi_logging {
    /// A log level, describing a kind of message.
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Level {
        /// Describes messages about the values of variables and the flow of control
        /// within a program.
        Trace,
        /// Describes messages likely to be of interest to someone debugging a program.
        Debug,
        /// Describes messages likely to be of interest to someone monitoring a program.
        Info,
        /// Describes messages indicating hazardous situations.
        Warn,
        /// Describes messages indicating serious errors.
        Error,
    }
    impl core::fmt::Debug for Level {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Level::Trace => f.debug_tuple("Level::Trace").finish(),
                Level::Debug => f.debug_tuple("Level::Debug").finish(),
                Level::Info => f.debug_tuple("Level::Info").finish(),
                Level::Warn => f.debug_tuple("Level::Warn").finish(),
                Level::Error => f.debug_tuple("Level::Error").finish(),
            }
        }
    }
    #[allow(clippy::all)]
    /// Emit a log message.
    ///
    /// A log message has a `level` describing what kind of message is being sent,
    /// a context, which is an uninterpreted string meant to help consumers group
    /// similar messages, and a string containing the message text.
    pub fn log(level: Level, context: &str, message: &str) -> () {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            let vec0 = context;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            let vec1 = message;
            let ptr1 = vec1.as_ptr() as i32;
            let len1 = vec1.len() as i32;

            #[link(wasm_import_module = "wasi-logging")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "log")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-logging_log")]
                fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32);
            }
            wit_import(
                match level {
                    Level::Trace => 0,
                    Level::Debug => 1,
                    Level::Info => 2,
                    Level::Warn => 3,
                    Level::Error => 4,
                },
                ptr0,
                len0,
                ptr1,
                len1,
            );
        }
    }
}

#[allow(clippy::all)]
pub mod wasi_stderr {
    #[allow(clippy::all)]
    /// Print text to stderr.
    pub fn print(message: &str) -> () {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            let vec0 = message;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;

            #[link(wasm_import_module = "wasi-stderr")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "print")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-stderr_print")]
                fn wit_import(_: i32, _: i32);
            }
            wit_import(ptr0, len0);
        }
    }
    #[allow(clippy::all)]
    /// Test whether stderr is known to be a terminal.
    ///
    /// This is similar to `isatty` in POSIX.
    pub fn is_terminal() -> bool {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[link(wasm_import_module = "wasi-stderr")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "is-terminal")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-stderr_is-terminal")]
                fn wit_import() -> i32;
            }
            let ret = wit_import();
            match ret {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// If stderr is a terminal and the number of columns can be determined,
    /// return it.
    pub fn num_columns() -> Option<u16> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(2))]
            struct RetArea([u8; 4]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-stderr")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "num-columns")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-stderr_num-columns")]
                fn wit_import(_: i32);
            }
            wit_import(ptr0);
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => None,
                1 => Some(i32::from(*((ptr0 + 2) as *const u16)) as u16),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
}

#[allow(clippy::all)]
pub mod wasi_io {
    /// An error type returned from a stream operation. Currently this
    /// doesn't provide any additional information.
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct StreamError {}
    impl core::fmt::Debug for StreamError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("StreamError").finish()
        }
    }
    impl core::fmt::Display for StreamError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    impl std::error::Error for StreamError {}
    /// An output bytestream. In the future, this will be replaced by handle
    /// types.
    ///
    /// This conceptually represents a `stream<u8, _>`. It's temporary
    /// scaffolding until component-model's async features are ready.
    ///
    /// And at present, it is a `u32` instead of being an actual handle, until
    /// the wit-bindgen implementation of handles and resources is ready.
    pub type OutputStream = u32;
    /// An input bytestream. In the future, this will be replaced by handle
    /// types.
    ///
    /// This conceptually represents a `stream<u8, _>`. It's temporary
    /// scaffolding until component-model's async features are ready.
    ///
    /// And at present, it is a `u32` instead of being an actual handle, until
    /// the wit-bindgen implementation of handles and resources is ready.
    pub type InputStream = u32;
    #[allow(clippy::all)]
    /// Read bytes from a stream.
    ///
    /// This function returns a list of bytes containing the data that was
    /// read, along with a bool indicating whether the end of the stream
    /// was reached. The returned list will contain up to `len` bytes; it
    /// may return fewer than requested, but not more.
    ///
    /// Once a stream has reached the end, subsequent calls to read or
    /// `skip` will always report end-of-stream rather than producing more
    /// data.
    ///
    /// If `len` is 0, it represents a request to read 0 bytes, which should
    /// always succeed, assuming the stream hasn't reached its end yet, and
    /// return an empty list.
    ///
    /// The len here is a `u64`, but some callees may not be able to allocate
    /// a buffer as large as that would imply.
    /// FIXME: describe what happens if allocation fails.
    pub fn read(
        src: InputStream,
        len: u64,
    ) -> Result<(wit_bindgen_guest_rust::rt::vec::Vec<u8>, bool), StreamError> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(4))]
            struct RetArea([u8; 16]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-io")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "read")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-io_read")]
                fn wit_import(_: i32, _: i64, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(src),
                wit_bindgen_guest_rust::rt::as_i64(len),
                ptr0,
            );
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok({
                    let len1 = *((ptr0 + 8) as *const i32) as usize;

                    (
                        Vec::from_raw_parts(*((ptr0 + 4) as *const i32) as *mut _, len1, len1),
                        match i32::from(*((ptr0 + 12) as *const u8)) {
                            0 => false,
                            1 => true,
                            _ => panic!("invalid bool discriminant"),
                        },
                    )
                }),
                1 => Err(StreamError {}),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Skip bytes from a stream.
    ///
    /// This is similar to the `read` function, but avoids copying the
    /// bytes into the instance.
    ///
    /// Once a stream has reached the end, subsequent calls to read or
    /// `skip` will always report end-of-stream rather than producing more
    /// data.
    ///
    /// This function returns the number of bytes skipped, along with a bool
    /// indicating whether the end of the stream was reached. The returned
    /// value will be at most `len`; it may be less.
    pub fn skip(src: InputStream, len: u64) -> Result<(u64, bool), StreamError> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(8))]
            struct RetArea([u8; 24]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-io")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "skip")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-io_skip")]
                fn wit_import(_: i32, _: i64, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(src),
                wit_bindgen_guest_rust::rt::as_i64(len),
                ptr0,
            );
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok((
                    *((ptr0 + 8) as *const i64) as u64,
                    match i32::from(*((ptr0 + 16) as *const u8)) {
                        0 => false,
                        1 => true,
                        _ => panic!("invalid bool discriminant"),
                    },
                )),
                1 => Err(StreamError {}),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Write bytes to a stream.
    ///
    /// This function returns a `u64` indicating the number of bytes from
    /// `buf` that were written; it may be less than the full list.
    pub fn write(dst: OutputStream, buf: &[u8]) -> Result<u64, StreamError> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(8))]
            struct RetArea([u8; 16]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let vec0 = buf;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            let ptr1 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-io")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "write")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-io_write")]
                fn wit_import(_: i32, _: i32, _: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(dst), ptr0, len0, ptr1);
            match i32::from(*((ptr1 + 0) as *const u8)) {
                0 => Ok(*((ptr1 + 8) as *const i64) as u64),
                1 => Err(StreamError {}),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Write a single byte multiple times to a stream.
    ///
    /// This function returns a `u64` indicating the number of copies of
    /// `byte` that were written; it may be less than `len`.
    pub fn write_repeated(dst: OutputStream, byte: u8, len: u64) -> Result<u64, StreamError> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(8))]
            struct RetArea([u8; 16]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-io")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "write-repeated")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-io_write-repeated")]
                fn wit_import(_: i32, _: i32, _: i64, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(dst),
                wit_bindgen_guest_rust::rt::as_i32(byte),
                wit_bindgen_guest_rust::rt::as_i64(len),
                ptr0,
            );
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok(*((ptr0 + 8) as *const i64) as u64),
                1 => Err(StreamError {}),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Read from one stream and write to another.
    ///
    /// This function returns the number of bytes transferred; it may be less
    /// than `len`.
    pub fn splice(
        dst: OutputStream,
        src: InputStream,
        len: u64,
    ) -> Result<(u64, bool), StreamError> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(8))]
            struct RetArea([u8; 24]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-io")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "splice")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-io_splice")]
                fn wit_import(_: i32, _: i32, _: i64, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(dst),
                wit_bindgen_guest_rust::rt::as_i32(src),
                wit_bindgen_guest_rust::rt::as_i64(len),
                ptr0,
            );
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok((
                    *((ptr0 + 8) as *const i64) as u64,
                    match i32::from(*((ptr0 + 16) as *const u8)) {
                        0 => false,
                        1 => true,
                        _ => panic!("invalid bool discriminant"),
                    },
                )),
                1 => Err(StreamError {}),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Forward the entire contents of an input stream to an output stream.
    ///
    /// This function repeatedly reads from the input stream and writes
    /// the data to the output stream, until the end of the input stream
    /// is reached, or an error is encountered.
    ///
    /// This function returns the number of bytes transferred.
    pub fn forward(dst: OutputStream, src: InputStream) -> Result<u64, StreamError> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(8))]
            struct RetArea([u8; 16]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-io")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "forward")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-io_forward")]
                fn wit_import(_: i32, _: i32, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(dst),
                wit_bindgen_guest_rust::rt::as_i32(src),
                ptr0,
            );
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok(*((ptr0 + 8) as *const i64) as u64),
                1 => Err(StreamError {}),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Dispose of the specified input-stream, after which it may no longer
    /// be used.
    pub fn drop_input_stream(f: InputStream) -> () {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[link(wasm_import_module = "wasi-io")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "drop-input-stream")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-io_drop-input-stream")]
                fn wit_import(_: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(f));
        }
    }
    #[allow(clippy::all)]
    /// Dispose of the specified output-stream, after which it may no longer
    /// be used.
    pub fn drop_output_stream(f: OutputStream) -> () {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[link(wasm_import_module = "wasi-io")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "drop-output-stream")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-io_drop-output-stream")]
                fn wit_import(_: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(f));
        }
    }
}

#[allow(clippy::all)]
pub mod wasi_filesystem {
    pub type InputStream = super::wasi_io::InputStream;
    pub type OutputStream = super::wasi_io::OutputStream;
    pub type Datetime = super::wasi_clocks::Datetime;
    /// Size of a range of bytes in memory.
    pub type Size = u32;
    wit_bindgen_guest_rust::bitflags::bitflags! {
      /// Open flags used by `open-at`.
      pub struct OFlags: u8 {
        /// Create file if it does not exist.
        const CREATE = 1 << 0;
        /// Fail if not a directory.
        const DIRECTORY = 1 << 1;
        /// Fail if file already exists.
        const EXCL = 1 << 2;
        /// Truncate file to size 0.
        const TRUNC = 1 << 3;
      }
    }
    impl OFlags {
        /// Convert from a raw integer, preserving any unknown bits. See
        /// <https://github.com/bitflags/bitflags/issues/263#issuecomment-957088321>
        pub fn from_bits_preserve(bits: u8) -> Self {
            Self { bits }
        }
    }
    wit_bindgen_guest_rust::bitflags::bitflags! {
      /// Permissions mode used by `open-at`, `change-file-permissions-at`, and
      /// similar.
      pub struct Mode: u8 {
        /// True if the resource is considered readable by the containing
        /// filesystem.
        const READABLE = 1 << 0;
        /// True if the resource is considered writeable by the containing
        /// filesystem.
        const WRITEABLE = 1 << 1;
        /// True if the resource is considered executable by the containing
        /// filesystem. This does not apply to directories.
        const EXECUTABLE = 1 << 2;
      }
    }
    impl Mode {
        /// Convert from a raw integer, preserving any unknown bits. See
        /// <https://github.com/bitflags/bitflags/issues/263#issuecomment-957088321>
        pub fn from_bits_preserve(bits: u8) -> Self {
            Self { bits }
        }
    }
    /// Number of hard links to an inode.
    pub type Linkcount = u64;
    /// Filesystem object serial number that is unique within its file system.
    pub type Inode = u64;
    /// Non-negative file size or length of a region within a file.
    pub type Filesize = u64;
    /// Error codes returned by functions.
    /// Not all of these error codes are returned by the functions provided by this
    /// API; some are used in higher-level library layers, and others are provided
    /// merely for alignment with POSIX.
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Errno {
        /// Permission denied.
        Access,
        /// Resource unavailable, or operation would block.
        Again,
        /// Connection already in progress.
        Already,
        /// Bad descriptor.
        Badf,
        /// Device or resource busy.
        Busy,
        /// Resource deadlock would occur.
        Deadlk,
        /// Storage quota exceeded.
        Dquot,
        /// File exists.
        Exist,
        /// File too large.
        Fbig,
        /// Illegal byte sequence.
        Ilseq,
        /// Operation in progress.
        Inprogress,
        /// Interrupted function.
        Intr,
        /// Invalid argument.
        Inval,
        /// I/O error.
        Io,
        /// Is a directory.
        Isdir,
        /// Too many levels of symbolic links.
        Loop,
        /// Too many links.
        Mlink,
        /// Message too large.
        Msgsize,
        /// Filename too long.
        Nametoolong,
        /// No such device.
        Nodev,
        /// No such file or directory.
        Noent,
        /// No locks available.
        Nolck,
        /// Not enough space.
        Nomem,
        /// No space left on device.
        Nospc,
        /// Function not supported.
        Nosys,
        /// Not a directory or a symbolic link to a directory.
        Notdir,
        /// Directory not empty.
        Notempty,
        /// State not recoverable.
        Notrecoverable,
        /// Not supported, or operation not supported on socket.
        Notsup,
        /// Inappropriate I/O control operation.
        Notty,
        /// No such device or address.
        Nxio,
        /// Value too large to be stored in data type.
        Overflow,
        /// Operation not permitted.
        Perm,
        /// Broken pipe.
        Pipe,
        /// Read-only file system.
        Rofs,
        /// Invalid seek.
        Spipe,
        /// Text file busy.
        Txtbsy,
        /// Cross-device link.
        Xdev,
    }
    impl Errno {
        pub fn name(&self) -> &'static str {
            match self {
                Errno::Access => "access",
                Errno::Again => "again",
                Errno::Already => "already",
                Errno::Badf => "badf",
                Errno::Busy => "busy",
                Errno::Deadlk => "deadlk",
                Errno::Dquot => "dquot",
                Errno::Exist => "exist",
                Errno::Fbig => "fbig",
                Errno::Ilseq => "ilseq",
                Errno::Inprogress => "inprogress",
                Errno::Intr => "intr",
                Errno::Inval => "inval",
                Errno::Io => "io",
                Errno::Isdir => "isdir",
                Errno::Loop => "loop",
                Errno::Mlink => "mlink",
                Errno::Msgsize => "msgsize",
                Errno::Nametoolong => "nametoolong",
                Errno::Nodev => "nodev",
                Errno::Noent => "noent",
                Errno::Nolck => "nolck",
                Errno::Nomem => "nomem",
                Errno::Nospc => "nospc",
                Errno::Nosys => "nosys",
                Errno::Notdir => "notdir",
                Errno::Notempty => "notempty",
                Errno::Notrecoverable => "notrecoverable",
                Errno::Notsup => "notsup",
                Errno::Notty => "notty",
                Errno::Nxio => "nxio",
                Errno::Overflow => "overflow",
                Errno::Perm => "perm",
                Errno::Pipe => "pipe",
                Errno::Rofs => "rofs",
                Errno::Spipe => "spipe",
                Errno::Txtbsy => "txtbsy",
                Errno::Xdev => "xdev",
            }
        }
        pub fn message(&self) -> &'static str {
            match self {
                Errno::Access => "Permission denied.",
                Errno::Again => "Resource unavailable, or operation would block.",
                Errno::Already => "Connection already in progress.",
                Errno::Badf => "Bad descriptor.",
                Errno::Busy => "Device or resource busy.",
                Errno::Deadlk => "Resource deadlock would occur.",
                Errno::Dquot => "Storage quota exceeded.",
                Errno::Exist => "File exists.",
                Errno::Fbig => "File too large.",
                Errno::Ilseq => "Illegal byte sequence.",
                Errno::Inprogress => "Operation in progress.",
                Errno::Intr => "Interrupted function.",
                Errno::Inval => "Invalid argument.",
                Errno::Io => "I/O error.",
                Errno::Isdir => "Is a directory.",
                Errno::Loop => "Too many levels of symbolic links.",
                Errno::Mlink => "Too many links.",
                Errno::Msgsize => "Message too large.",
                Errno::Nametoolong => "Filename too long.",
                Errno::Nodev => "No such device.",
                Errno::Noent => "No such file or directory.",
                Errno::Nolck => "No locks available.",
                Errno::Nomem => "Not enough space.",
                Errno::Nospc => "No space left on device.",
                Errno::Nosys => "Function not supported.",
                Errno::Notdir => "Not a directory or a symbolic link to a directory.",
                Errno::Notempty => "Directory not empty.",
                Errno::Notrecoverable => "State not recoverable.",
                Errno::Notsup => "Not supported, or operation not supported on socket.",
                Errno::Notty => "Inappropriate I/O control operation.",
                Errno::Nxio => "No such device or address.",
                Errno::Overflow => "Value too large to be stored in data type.",
                Errno::Perm => "Operation not permitted.",
                Errno::Pipe => "Broken pipe.",
                Errno::Rofs => "Read-only file system.",
                Errno::Spipe => "Invalid seek.",
                Errno::Txtbsy => "Text file busy.",
                Errno::Xdev => "Cross-device link.",
            }
        }
    }
    impl core::fmt::Debug for Errno {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Errno")
                .field("code", &(*self as i32))
                .field("name", &self.name())
                .field("message", &self.message())
                .finish()
        }
    }
    impl core::fmt::Display for Errno {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} (error {})", self.name(), *self as i32)
        }
    }

    impl std::error::Error for Errno {}
    /// A directory entry stream. In the future, this will be replaced by an
    /// actual stream.
    pub type DirEntryStream = u32;
    /// Identifier for a device containing a file system. Can be used in combination
    /// with `inode` to uniquely identify a file or directory in the filesystem.
    pub type Device = u64;
    /// The type of a filesystem object referenced by a descriptor.
    ///
    /// Note: This was called `filetype` in earlier versions of WASI.
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum DescriptorType {
        /// The type of the descriptor or file is unknown or is different from
        /// any of the other types specified.
        Unknown,
        /// The descriptor refers to a block device inode.
        BlockDevice,
        /// The descriptor refers to a character device inode.
        CharacterDevice,
        /// The descriptor refers to a directory inode.
        Directory,
        /// The descriptor refers to a named pipe.
        Fifo,
        /// The file refers to a symbolic link inode.
        SymbolicLink,
        /// The descriptor refers to a regular file inode.
        RegularFile,
        /// The descriptor refers to a socket.
        Socket,
    }
    impl core::fmt::Debug for DescriptorType {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                DescriptorType::Unknown => f.debug_tuple("DescriptorType::Unknown").finish(),
                DescriptorType::BlockDevice => {
                    f.debug_tuple("DescriptorType::BlockDevice").finish()
                }
                DescriptorType::CharacterDevice => {
                    f.debug_tuple("DescriptorType::CharacterDevice").finish()
                }
                DescriptorType::Directory => f.debug_tuple("DescriptorType::Directory").finish(),
                DescriptorType::Fifo => f.debug_tuple("DescriptorType::Fifo").finish(),
                DescriptorType::SymbolicLink => {
                    f.debug_tuple("DescriptorType::SymbolicLink").finish()
                }
                DescriptorType::RegularFile => {
                    f.debug_tuple("DescriptorType::RegularFile").finish()
                }
                DescriptorType::Socket => f.debug_tuple("DescriptorType::Socket").finish(),
            }
        }
    }
    /// A directory entry.
    #[derive(Clone)]
    pub struct DirEntry {
        /// The serial number of the object referred to by this directory entry.
        /// May be none if the inode value is not known.
        ///
        /// When this is none, libc implementations might do an extra `stat-at`
        /// call to retrieve the inode number to fill their `d_ino` fields, so
        /// implementations which can set this to a non-none value should do so.
        pub ino: Option<Inode>,
        /// The type of the file referred to by this directory entry.
        pub type_: DescriptorType,
        /// The name of the object.
        pub name: wit_bindgen_guest_rust::rt::string::String,
    }
    impl core::fmt::Debug for DirEntry {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("DirEntry")
                .field("ino", &self.ino)
                .field("type", &self.type_)
                .field("name", &self.name)
                .finish()
        }
    }
    wit_bindgen_guest_rust::bitflags::bitflags! {
      /// Descriptor flags.
      ///
      /// Note: This was called `fdflags` in earlier versions of WASI.
      pub struct DescriptorFlags: u8 {
        /// Read mode: Data can be read.
        const READ = 1 << 0;
        /// Write mode: Data can be written to.
        const WRITE = 1 << 1;
        /// Write according to synchronized I/O data integrity completion. Only the
        /// data stored in the file is synchronized.
        const DSYNC = 1 << 2;
        /// Non-blocking mode.
        const NONBLOCK = 1 << 3;
        /// Synchronized read I/O operations.
        const RSYNC = 1 << 4;
        /// Write according to synchronized I/O file integrity completion. In
        /// addition to synchronizing the data stored in the file, the
        /// implementation may also synchronously update the file's metadata.
        const SYNC = 1 << 5;
      }
    }
    impl DescriptorFlags {
        /// Convert from a raw integer, preserving any unknown bits. See
        /// <https://github.com/bitflags/bitflags/issues/263#issuecomment-957088321>
        pub fn from_bits_preserve(bits: u8) -> Self {
            Self { bits }
        }
    }
    /// A "file" descriptor. In the future, this will be replaced by handle types.
    pub type Descriptor = u32;
    /// When setting a timestamp, this gives the value to set it to.
    #[derive(Clone, Copy)]
    pub enum NewTimestamp {
        /// Leave the timestamp set to its previous value.
        NoChange,
        /// Set the timestamp to the current time of the system clock associated
        /// with the filesystem.
        Now,
        /// Set the timestamp to the given value.
        Timestamp(Datetime),
    }
    impl core::fmt::Debug for NewTimestamp {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                NewTimestamp::NoChange => f.debug_tuple("NewTimestamp::NoChange").finish(),
                NewTimestamp::Now => f.debug_tuple("NewTimestamp::Now").finish(),
                NewTimestamp::Timestamp(e) => {
                    f.debug_tuple("NewTimestamp::Timestamp").field(e).finish()
                }
            }
        }
    }
    /// File attributes.
    ///
    /// Note: This was called `filestat` in earlier versions of WASI.
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DescriptorStat {
        /// Device ID of device containing the file.
        pub dev: Device,
        /// File serial number.
        pub ino: Inode,
        /// File type.
        pub type_: DescriptorType,
        /// Number of hard links to the file.
        pub nlink: Linkcount,
        /// For regular files, the file size in bytes. For symbolic links, the length
        /// in bytes of the pathname contained in the symbolic link.
        pub size: Filesize,
        /// Last data access timestamp.
        pub atim: Datetime,
        /// Last data modification timestamp.
        pub mtim: Datetime,
        /// Last file status change timestamp.
        pub ctim: Datetime,
    }
    impl core::fmt::Debug for DescriptorStat {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("DescriptorStat")
                .field("dev", &self.dev)
                .field("ino", &self.ino)
                .field("type", &self.type_)
                .field("nlink", &self.nlink)
                .field("size", &self.size)
                .field("atim", &self.atim)
                .field("mtim", &self.mtim)
                .field("ctim", &self.ctim)
                .finish()
        }
    }
    wit_bindgen_guest_rust::bitflags::bitflags! {
      /// Flags determining the method of how paths are resolved.
      pub struct AtFlags: u8 {
        /// As long as the resolved path corresponds to a symbolic link, it is expanded.
        const SYMLINK_FOLLOW = 1 << 0;
      }
    }
    impl AtFlags {
        /// Convert from a raw integer, preserving any unknown bits. See
        /// <https://github.com/bitflags/bitflags/issues/263#issuecomment-957088321>
        pub fn from_bits_preserve(bits: u8) -> Self {
            Self { bits }
        }
    }
    /// File or memory access pattern advisory information.
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Advice {
        /// The application has no advice to give on its behavior with respect to the specified data.
        Normal,
        /// The application expects to access the specified data sequentially from lower offsets to higher offsets.
        Sequential,
        /// The application expects to access the specified data in a random order.
        Random,
        /// The application expects to access the specified data in the near future.
        WillNeed,
        /// The application expects that it will not access the specified data in the near future.
        DontNeed,
        /// The application expects to access the specified data once and then not reuse it thereafter.
        NoReuse,
    }
    impl core::fmt::Debug for Advice {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Advice::Normal => f.debug_tuple("Advice::Normal").finish(),
                Advice::Sequential => f.debug_tuple("Advice::Sequential").finish(),
                Advice::Random => f.debug_tuple("Advice::Random").finish(),
                Advice::WillNeed => f.debug_tuple("Advice::WillNeed").finish(),
                Advice::DontNeed => f.debug_tuple("Advice::DontNeed").finish(),
                Advice::NoReuse => f.debug_tuple("Advice::NoReuse").finish(),
            }
        }
    }
    #[allow(clippy::all)]
    /// Provide file advisory information on a descriptor.
    ///
    /// This is similar to `posix_fadvise` in POSIX.
    pub fn fadvise(
        fd: Descriptor,
        offset: Filesize,
        len: Filesize,
        advice: Advice,
    ) -> Result<(), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "fadvise")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_fadvise")]
                fn wit_import(_: i32, _: i64, _: i64, _: i32, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(fd),
                wit_bindgen_guest_rust::rt::as_i64(offset),
                wit_bindgen_guest_rust::rt::as_i64(len),
                match advice {
                    Advice::Normal => 0,
                    Advice::Sequential => 1,
                    Advice::Random => 2,
                    Advice::WillNeed => 3,
                    Advice::DontNeed => 4,
                    Advice::NoReuse => 5,
                },
                ptr0,
            );
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok(()),
                1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Synchronize the data of a file to disk.
    ///
    /// Note: This is similar to `fdatasync` in POSIX.
    pub fn datasync(fd: Descriptor) -> Result<(), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "datasync")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_datasync")]
                fn wit_import(_: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(fd), ptr0);
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok(()),
                1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Get flags associated with a descriptor.
    ///
    /// Note: This returns similar flags to `fcntl(fd, F_GETFL)` in POSIX.
    ///
    /// Note: This returns the value that was the `fs_flags` value returned
    /// from `fdstat_get` in earlier versions of WASI.
    pub fn flags(fd: Descriptor) -> Result<DescriptorFlags, Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "flags")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_flags")]
                fn wit_import(_: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(fd), ptr0);
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok(DescriptorFlags::empty()
                    | DescriptorFlags::from_bits_preserve(
                        ((i32::from(*((ptr0 + 1) as *const u8)) as u8) << 0) as _,
                    )),
                1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Get the dynamic type of a descriptor.
    ///
    /// Note: This returns the same value as the `type` field of the `descriptor-stat`
    /// returned by `stat`, `stat-at` and similar.
    ///
    /// Note: This returns similar flags to the `st_mode & S_IFMT` value provided
    /// by `fstat` in POSIX.
    ///
    /// Note: This returns the value that was the `fs_filetype` value returned
    /// from `fdstat_get` in earlier versions of WASI.
    ///
    /// TODO: Remove the `todo-` when wit-bindgen is updated.
    pub fn todo_type(fd: Descriptor) -> Result<DescriptorType, Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "todo-type")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_todo-type")]
                fn wit_import(_: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(fd), ptr0);
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok(match i32::from(*((ptr0 + 1) as *const u8)) {
                    0 => DescriptorType::Unknown,
                    1 => DescriptorType::BlockDevice,
                    2 => DescriptorType::CharacterDevice,
                    3 => DescriptorType::Directory,
                    4 => DescriptorType::Fifo,
                    5 => DescriptorType::SymbolicLink,
                    6 => DescriptorType::RegularFile,
                    7 => DescriptorType::Socket,
                    _ => panic!("invalid enum discriminant"),
                }),
                1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Set flags associated with a descriptor.
    ///
    /// Note: This is similar to `fcntl(fd, F_SETFL, flags)` in POSIX.
    ///
    /// Note: This was called `fd_fdstat_set_flags` in earlier versions of WASI.
    pub fn set_flags(fd: Descriptor, flags: DescriptorFlags) -> Result<(), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let flags0 = flags;
            let ptr1 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "set-flags")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_set-flags")]
                fn wit_import(_: i32, _: i32, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(fd),
                (flags0.bits() >> 0) as i32,
                ptr1,
            );
            match i32::from(*((ptr1 + 0) as *const u8)) {
                0 => Ok(()),
                1 => Err(match i32::from(*((ptr1 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Adjust the size of an open file. If this increases the file's size, the
    /// extra bytes are filled with zeros.
    ///
    /// Note: This was called `fd_filestat_set_size` in earlier versions of WASI.
    pub fn set_size(fd: Descriptor, size: Filesize) -> Result<(), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "set-size")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_set-size")]
                fn wit_import(_: i32, _: i64, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(fd),
                wit_bindgen_guest_rust::rt::as_i64(size),
                ptr0,
            );
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok(()),
                1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Adjust the timestamps of an open file or directory.
    ///
    /// Note: This is similar to `futimens` in POSIX.
    ///
    /// Note: This was called `fd_filestat_set_times` in earlier versions of WASI.
    pub fn set_times(fd: Descriptor, atim: NewTimestamp, mtim: NewTimestamp) -> Result<(), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let (result1_0, result1_1, result1_2) = match atim {
                NewTimestamp::NoChange => (0i32, 0i64, 0i32),
                NewTimestamp::Now => (1i32, 0i64, 0i32),
                NewTimestamp::Timestamp(e) => {
                    let Datetime {
                        seconds: seconds0,
                        nanoseconds: nanoseconds0,
                    } = e;

                    (
                        2i32,
                        wit_bindgen_guest_rust::rt::as_i64(seconds0),
                        wit_bindgen_guest_rust::rt::as_i32(nanoseconds0),
                    )
                }
            };
            let (result3_0, result3_1, result3_2) = match mtim {
                NewTimestamp::NoChange => (0i32, 0i64, 0i32),
                NewTimestamp::Now => (1i32, 0i64, 0i32),
                NewTimestamp::Timestamp(e) => {
                    let Datetime {
                        seconds: seconds2,
                        nanoseconds: nanoseconds2,
                    } = e;

                    (
                        2i32,
                        wit_bindgen_guest_rust::rt::as_i64(seconds2),
                        wit_bindgen_guest_rust::rt::as_i32(nanoseconds2),
                    )
                }
            };
            let ptr4 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "set-times")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_set-times")]
                fn wit_import(_: i32, _: i32, _: i64, _: i32, _: i32, _: i64, _: i32, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(fd),
                result1_0,
                result1_1,
                result1_2,
                result3_0,
                result3_1,
                result3_2,
                ptr4,
            );
            match i32::from(*((ptr4 + 0) as *const u8)) {
                0 => Ok(()),
                1 => Err(match i32::from(*((ptr4 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Return a stream for reading from a file.
    ///
    /// Note: This allows using `read-stream`, which is similar to `read` in POSIX.
    pub fn read_via_stream(fd: Descriptor, offset: Filesize) -> Result<InputStream, Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(4))]
            struct RetArea([u8; 8]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "read-via-stream")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-filesystem_read-via-stream"
                )]
                fn wit_import(_: i32, _: i64, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(fd),
                wit_bindgen_guest_rust::rt::as_i64(offset),
                ptr0,
            );
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok(*((ptr0 + 4) as *const i32) as u32),
                1 => Err(match i32::from(*((ptr0 + 4) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Return a stream for writing to a file.
    ///
    /// Note: This allows using `write-stream`, which is similar to `write` in POSIX.
    pub fn write_via_stream(fd: Descriptor, offset: Filesize) -> Result<OutputStream, Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(4))]
            struct RetArea([u8; 8]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "write-via-stream")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-filesystem_write-via-stream"
                )]
                fn wit_import(_: i32, _: i64, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(fd),
                wit_bindgen_guest_rust::rt::as_i64(offset),
                ptr0,
            );
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok(*((ptr0 + 4) as *const i32) as u32),
                1 => Err(match i32::from(*((ptr0 + 4) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Return a stream for appending to a file.
    ///
    /// Note: This allows using `write-stream`, which is similar to `write` with
    /// `O_APPEND` in in POSIX.
    pub fn append_via_stream(fd: Descriptor) -> Result<OutputStream, Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(4))]
            struct RetArea([u8; 8]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "append-via-stream")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-filesystem_append-via-stream"
                )]
                fn wit_import(_: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(fd), ptr0);
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok(*((ptr0 + 4) as *const i32) as u32),
                1 => Err(match i32::from(*((ptr0 + 4) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Read from a file at a given offset.
    ///
    /// Note: This is similar to `pread` in POSIX.
    pub fn pread(
        fd: Descriptor,
        len: Size,
        offset: Filesize,
    ) -> Result<(wit_bindgen_guest_rust::rt::vec::Vec<u8>, bool), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(4))]
            struct RetArea([u8; 16]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "pread")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_pread")]
                fn wit_import(_: i32, _: i32, _: i64, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(fd),
                wit_bindgen_guest_rust::rt::as_i32(len),
                wit_bindgen_guest_rust::rt::as_i64(offset),
                ptr0,
            );
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok({
                    let len1 = *((ptr0 + 8) as *const i32) as usize;

                    (
                        Vec::from_raw_parts(*((ptr0 + 4) as *const i32) as *mut _, len1, len1),
                        match i32::from(*((ptr0 + 12) as *const u8)) {
                            0 => false,
                            1 => true,
                            _ => panic!("invalid bool discriminant"),
                        },
                    )
                }),
                1 => Err(match i32::from(*((ptr0 + 4) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Write to a file at a given offset.
    ///
    /// Note: This is similar to `pwrite` in POSIX.
    pub fn pwrite(fd: Descriptor, buf: &[u8], offset: Filesize) -> Result<Size, Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(4))]
            struct RetArea([u8; 8]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let vec0 = buf;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            let ptr1 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "pwrite")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_pwrite")]
                fn wit_import(_: i32, _: i32, _: i32, _: i64, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(fd),
                ptr0,
                len0,
                wit_bindgen_guest_rust::rt::as_i64(offset),
                ptr1,
            );
            match i32::from(*((ptr1 + 0) as *const u8)) {
                0 => Ok(*((ptr1 + 4) as *const i32) as u32),
                1 => Err(match i32::from(*((ptr1 + 4) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Read directory entries from a directory.
    ///
    /// This always returns a new stream which starts at the beginning of the
    /// directory.
    pub fn readdir(fd: Descriptor) -> Result<DirEntryStream, Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(4))]
            struct RetArea([u8; 8]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "readdir")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_readdir")]
                fn wit_import(_: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(fd), ptr0);
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok(*((ptr0 + 4) as *const i32) as u32),
                1 => Err(match i32::from(*((ptr0 + 4) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Closes a handle returned by `readdir`
    pub fn close_dir_entry_stream(s: DirEntryStream) -> () {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "close-dir-entry-stream")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-filesystem_close-dir-entry-stream"
                )]
                fn wit_import(_: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(s));
        }
    }
    #[allow(clippy::all)]
    /// Read a single directory entry from a `dir-entry-stream`.
    pub fn read_dir_entry(dir_stream: DirEntryStream) -> Result<Option<DirEntry>, Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(8))]
            struct RetArea([u8; 48]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "read-dir-entry")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-filesystem_read-dir-entry"
                )]
                fn wit_import(_: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(dir_stream), ptr0);
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok(match i32::from(*((ptr0 + 8) as *const u8)) {
                    0 => None,
                    1 => Some({
                        let len1 = *((ptr0 + 40) as *const i32) as usize;

                        DirEntry {
                            ino: match i32::from(*((ptr0 + 16) as *const u8)) {
                                0 => None,
                                1 => Some(*((ptr0 + 24) as *const i64) as u64),
                                _ => panic!("invalid enum discriminant"),
                            },
                            type_: match i32::from(*((ptr0 + 32) as *const u8)) {
                                0 => DescriptorType::Unknown,
                                1 => DescriptorType::BlockDevice,
                                2 => DescriptorType::CharacterDevice,
                                3 => DescriptorType::Directory,
                                4 => DescriptorType::Fifo,
                                5 => DescriptorType::SymbolicLink,
                                6 => DescriptorType::RegularFile,
                                7 => DescriptorType::Socket,
                                _ => panic!("invalid enum discriminant"),
                            },
                            name: String::from_utf8(Vec::from_raw_parts(
                                *((ptr0 + 36) as *const i32) as *mut _,
                                len1,
                                len1,
                            ))
                            .unwrap(),
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }),
                1 => Err(match i32::from(*((ptr0 + 8) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Synchronize the data and metadata of a file to disk.
    ///
    /// Note: This is similar to `fsync` in POSIX.
    pub fn sync(fd: Descriptor) -> Result<(), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "sync")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_sync")]
                fn wit_import(_: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(fd), ptr0);
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok(()),
                1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Create a directory.
    ///
    /// Note: This is similar to `mkdirat` in POSIX.
    pub fn create_directory_at(fd: Descriptor, path: &str) -> Result<(), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let vec0 = path;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            let ptr1 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "create-directory-at")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-filesystem_create-directory-at"
                )]
                fn wit_import(_: i32, _: i32, _: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(fd), ptr0, len0, ptr1);
            match i32::from(*((ptr1 + 0) as *const u8)) {
                0 => Ok(()),
                1 => Err(match i32::from(*((ptr1 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Return the attributes of an open file or directory.
    ///
    /// Note: This is similar to `fstat` in POSIX.
    ///
    /// Note: This was called `fd_filestat_get` in earlier versions of WASI.
    pub fn stat(fd: Descriptor) -> Result<DescriptorStat, Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(8))]
            struct RetArea([u8; 96]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "stat")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_stat")]
                fn wit_import(_: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(fd), ptr0);
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok(DescriptorStat {
                    dev: *((ptr0 + 8) as *const i64) as u64,
                    ino: *((ptr0 + 16) as *const i64) as u64,
                    type_: match i32::from(*((ptr0 + 24) as *const u8)) {
                        0 => DescriptorType::Unknown,
                        1 => DescriptorType::BlockDevice,
                        2 => DescriptorType::CharacterDevice,
                        3 => DescriptorType::Directory,
                        4 => DescriptorType::Fifo,
                        5 => DescriptorType::SymbolicLink,
                        6 => DescriptorType::RegularFile,
                        7 => DescriptorType::Socket,
                        _ => panic!("invalid enum discriminant"),
                    },
                    nlink: *((ptr0 + 32) as *const i64) as u64,
                    size: *((ptr0 + 40) as *const i64) as u64,
                    atim: Datetime {
                        seconds: *((ptr0 + 48) as *const i64) as u64,
                        nanoseconds: *((ptr0 + 56) as *const i32) as u32,
                    },
                    mtim: Datetime {
                        seconds: *((ptr0 + 64) as *const i64) as u64,
                        nanoseconds: *((ptr0 + 72) as *const i32) as u32,
                    },
                    ctim: Datetime {
                        seconds: *((ptr0 + 80) as *const i64) as u64,
                        nanoseconds: *((ptr0 + 88) as *const i32) as u32,
                    },
                }),
                1 => Err(match i32::from(*((ptr0 + 8) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Return the attributes of a file or directory.
    ///
    /// Note: This is similar to `fstatat` in POSIX.
    ///
    /// Note: This was called `fd_filestat_get` in earlier versions of WASI.
    pub fn stat_at(fd: Descriptor, at_flags: AtFlags, path: &str) -> Result<DescriptorStat, Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(8))]
            struct RetArea([u8; 96]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let flags0 = at_flags;
            let vec1 = path;
            let ptr1 = vec1.as_ptr() as i32;
            let len1 = vec1.len() as i32;
            let ptr2 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "stat-at")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_stat-at")]
                fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(fd),
                (flags0.bits() >> 0) as i32,
                ptr1,
                len1,
                ptr2,
            );
            match i32::from(*((ptr2 + 0) as *const u8)) {
                0 => Ok(DescriptorStat {
                    dev: *((ptr2 + 8) as *const i64) as u64,
                    ino: *((ptr2 + 16) as *const i64) as u64,
                    type_: match i32::from(*((ptr2 + 24) as *const u8)) {
                        0 => DescriptorType::Unknown,
                        1 => DescriptorType::BlockDevice,
                        2 => DescriptorType::CharacterDevice,
                        3 => DescriptorType::Directory,
                        4 => DescriptorType::Fifo,
                        5 => DescriptorType::SymbolicLink,
                        6 => DescriptorType::RegularFile,
                        7 => DescriptorType::Socket,
                        _ => panic!("invalid enum discriminant"),
                    },
                    nlink: *((ptr2 + 32) as *const i64) as u64,
                    size: *((ptr2 + 40) as *const i64) as u64,
                    atim: Datetime {
                        seconds: *((ptr2 + 48) as *const i64) as u64,
                        nanoseconds: *((ptr2 + 56) as *const i32) as u32,
                    },
                    mtim: Datetime {
                        seconds: *((ptr2 + 64) as *const i64) as u64,
                        nanoseconds: *((ptr2 + 72) as *const i32) as u32,
                    },
                    ctim: Datetime {
                        seconds: *((ptr2 + 80) as *const i64) as u64,
                        nanoseconds: *((ptr2 + 88) as *const i32) as u32,
                    },
                }),
                1 => Err(match i32::from(*((ptr2 + 8) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Adjust the timestamps of a file or directory.
    ///
    /// Note: This is similar to `utimensat` in POSIX.
    ///
    /// Note: This was called `path_filestat_set_times` in earlier versions of WASI.
    pub fn set_times_at(
        fd: Descriptor,
        at_flags: AtFlags,
        path: &str,
        atim: NewTimestamp,
        mtim: NewTimestamp,
    ) -> Result<(), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let flags0 = at_flags;
            let vec1 = path;
            let ptr1 = vec1.as_ptr() as i32;
            let len1 = vec1.len() as i32;
            let (result3_0, result3_1, result3_2) = match atim {
                NewTimestamp::NoChange => (0i32, 0i64, 0i32),
                NewTimestamp::Now => (1i32, 0i64, 0i32),
                NewTimestamp::Timestamp(e) => {
                    let Datetime {
                        seconds: seconds2,
                        nanoseconds: nanoseconds2,
                    } = e;

                    (
                        2i32,
                        wit_bindgen_guest_rust::rt::as_i64(seconds2),
                        wit_bindgen_guest_rust::rt::as_i32(nanoseconds2),
                    )
                }
            };
            let (result5_0, result5_1, result5_2) = match mtim {
                NewTimestamp::NoChange => (0i32, 0i64, 0i32),
                NewTimestamp::Now => (1i32, 0i64, 0i32),
                NewTimestamp::Timestamp(e) => {
                    let Datetime {
                        seconds: seconds4,
                        nanoseconds: nanoseconds4,
                    } = e;

                    (
                        2i32,
                        wit_bindgen_guest_rust::rt::as_i64(seconds4),
                        wit_bindgen_guest_rust::rt::as_i32(nanoseconds4),
                    )
                }
            };
            let ptr6 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "set-times-at")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-filesystem_set-times-at"
                )]
                fn wit_import(
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i64,
                    _: i32,
                    _: i32,
                    _: i64,
                    _: i32,
                    _: i32,
                );
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(fd),
                (flags0.bits() >> 0) as i32,
                ptr1,
                len1,
                result3_0,
                result3_1,
                result3_2,
                result5_0,
                result5_1,
                result5_2,
                ptr6,
            );
            match i32::from(*((ptr6 + 0) as *const u8)) {
                0 => Ok(()),
                1 => Err(match i32::from(*((ptr6 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Create a hard link.
    ///
    /// Note: This is similar to `linkat` in POSIX.
    pub fn link_at(
        fd: Descriptor,
        old_at_flags: AtFlags,
        old_path: &str,
        new_descriptor: Descriptor,
        new_path: &str,
    ) -> Result<(), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let flags0 = old_at_flags;
            let vec1 = old_path;
            let ptr1 = vec1.as_ptr() as i32;
            let len1 = vec1.len() as i32;
            let vec2 = new_path;
            let ptr2 = vec2.as_ptr() as i32;
            let len2 = vec2.len() as i32;
            let ptr3 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "link-at")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_link-at")]
                fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(fd),
                (flags0.bits() >> 0) as i32,
                ptr1,
                len1,
                wit_bindgen_guest_rust::rt::as_i32(new_descriptor),
                ptr2,
                len2,
                ptr3,
            );
            match i32::from(*((ptr3 + 0) as *const u8)) {
                0 => Ok(()),
                1 => Err(match i32::from(*((ptr3 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Open a file or directory.
    ///
    /// The returned descriptor is not guaranteed to be the lowest-numbered
    /// descriptor not currently open/ it is randomized to prevent applications
    /// from depending on making assumptions about indexes, since this is
    /// error-prone in multi-threaded contexts. The returned descriptor is
    /// guaranteed to be less than 2**31.
    ///
    /// Note: This is similar to `openat` in POSIX.
    pub fn open_at(
        fd: Descriptor,
        at_flags: AtFlags,
        path: &str,
        o_flags: OFlags,
        flags: DescriptorFlags,
        mode: Mode,
    ) -> Result<Descriptor, Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(4))]
            struct RetArea([u8; 8]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let flags0 = at_flags;
            let vec1 = path;
            let ptr1 = vec1.as_ptr() as i32;
            let len1 = vec1.len() as i32;
            let flags2 = o_flags;
            let flags3 = flags;
            let flags4 = mode;
            let ptr5 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "open-at")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_open-at")]
                fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(fd),
                (flags0.bits() >> 0) as i32,
                ptr1,
                len1,
                (flags2.bits() >> 0) as i32,
                (flags3.bits() >> 0) as i32,
                (flags4.bits() >> 0) as i32,
                ptr5,
            );
            match i32::from(*((ptr5 + 0) as *const u8)) {
                0 => Ok(*((ptr5 + 4) as *const i32) as u32),
                1 => Err(match i32::from(*((ptr5 + 4) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Close a file or directory handle.
    ///
    /// Until wit supports handles, use an explicit `close` function.
    ///
    /// Note: This is similar to `close` in POSIX.
    pub fn close(fd: Descriptor) -> () {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "close")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_close")]
                fn wit_import(_: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(fd));
        }
    }
    #[allow(clippy::all)]
    /// Read the contents of a symbolic link.
    ///
    /// Note: This is similar to `readlinkat` in POSIX.
    pub fn readlink_at(
        fd: Descriptor,
        path: &str,
    ) -> Result<wit_bindgen_guest_rust::rt::string::String, Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(4))]
            struct RetArea([u8; 12]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let vec0 = path;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            let ptr1 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "readlink-at")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_readlink-at")]
                fn wit_import(_: i32, _: i32, _: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(fd), ptr0, len0, ptr1);
            match i32::from(*((ptr1 + 0) as *const u8)) {
                0 => Ok({
                    let len2 = *((ptr1 + 8) as *const i32) as usize;

                    String::from_utf8(Vec::from_raw_parts(
                        *((ptr1 + 4) as *const i32) as *mut _,
                        len2,
                        len2,
                    ))
                    .unwrap()
                }),
                1 => Err(match i32::from(*((ptr1 + 4) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Remove a directory.
    ///
    /// Return `errno::notempty` if the directory is not empty.
    ///
    /// Note: This is similar to `unlinkat(fd, path, AT_REMOVEDIR)` in POSIX.
    pub fn remove_directory_at(fd: Descriptor, path: &str) -> Result<(), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let vec0 = path;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            let ptr1 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "remove-directory-at")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-filesystem_remove-directory-at"
                )]
                fn wit_import(_: i32, _: i32, _: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(fd), ptr0, len0, ptr1);
            match i32::from(*((ptr1 + 0) as *const u8)) {
                0 => Ok(()),
                1 => Err(match i32::from(*((ptr1 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Rename a filesystem object.
    ///
    /// Note: This is similar to `renameat` in POSIX.
    pub fn rename_at(
        fd: Descriptor,
        old_path: &str,
        new_descriptor: Descriptor,
        new_path: &str,
    ) -> Result<(), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let vec0 = old_path;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            let vec1 = new_path;
            let ptr1 = vec1.as_ptr() as i32;
            let len1 = vec1.len() as i32;
            let ptr2 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "rename-at")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_rename-at")]
                fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(fd),
                ptr0,
                len0,
                wit_bindgen_guest_rust::rt::as_i32(new_descriptor),
                ptr1,
                len1,
                ptr2,
            );
            match i32::from(*((ptr2 + 0) as *const u8)) {
                0 => Ok(()),
                1 => Err(match i32::from(*((ptr2 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Create a symbolic link.
    ///
    /// Note: This is similar to `symlinkat` in POSIX.
    pub fn symlink_at(fd: Descriptor, old_path: &str, new_path: &str) -> Result<(), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let vec0 = old_path;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            let vec1 = new_path;
            let ptr1 = vec1.as_ptr() as i32;
            let len1 = vec1.len() as i32;
            let ptr2 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "symlink-at")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_symlink-at")]
                fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(fd),
                ptr0,
                len0,
                ptr1,
                len1,
                ptr2,
            );
            match i32::from(*((ptr2 + 0) as *const u8)) {
                0 => Ok(()),
                1 => Err(match i32::from(*((ptr2 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Unlink a filesystem object that is not a directory.
    ///
    /// Return `errno::isdir` if the path refers to a directory.
    /// Note: This is similar to `unlinkat(fd, path, 0)` in POSIX.
    pub fn unlink_file_at(fd: Descriptor, path: &str) -> Result<(), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let vec0 = path;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            let ptr1 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "unlink-file-at")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-filesystem_unlink-file-at"
                )]
                fn wit_import(_: i32, _: i32, _: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(fd), ptr0, len0, ptr1);
            match i32::from(*((ptr1 + 0) as *const u8)) {
                0 => Ok(()),
                1 => Err(match i32::from(*((ptr1 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Change the permissions of a filesystem object that is not a directory.
    ///
    /// Note that the ultimate meanings of these permissions is
    /// filesystem-specific.
    ///
    /// Note: This is similar to `fchmodat` in POSIX.
    pub fn change_file_permissions_at(
        fd: Descriptor,
        at_flags: AtFlags,
        path: &str,
        mode: Mode,
    ) -> Result<(), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let flags0 = at_flags;
            let vec1 = path;
            let ptr1 = vec1.as_ptr() as i32;
            let len1 = vec1.len() as i32;
            let flags2 = mode;
            let ptr3 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "change-file-permissions-at")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-filesystem_change-file-permissions-at"
                )]
                fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(fd),
                (flags0.bits() >> 0) as i32,
                ptr1,
                len1,
                (flags2.bits() >> 0) as i32,
                ptr3,
            );
            match i32::from(*((ptr3 + 0) as *const u8)) {
                0 => Ok(()),
                1 => Err(match i32::from(*((ptr3 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Change the permissions of a directory.
    ///
    /// Note that the ultimate meanings of these permissions is
    /// filesystem-specific.
    ///
    /// Unlike in POSIX, the `executable` flag is not reinterpreted as a "search"
    /// flag. `read` on a directory implies readability and searchability, and
    /// `execute` is not valid for directories.
    ///
    /// Note: This is similar to `fchmodat` in POSIX.
    pub fn change_directory_permissions_at(
        fd: Descriptor,
        at_flags: AtFlags,
        path: &str,
        mode: Mode,
    ) -> Result<(), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let flags0 = at_flags;
            let vec1 = path;
            let ptr1 = vec1.as_ptr() as i32;
            let len1 = vec1.len() as i32;
            let flags2 = mode;
            let ptr3 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "change-directory-permissions-at")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-filesystem_change-directory-permissions-at"
                )]
                fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(fd),
                (flags0.bits() >> 0) as i32,
                ptr1,
                len1,
                (flags2.bits() >> 0) as i32,
                ptr3,
            );
            match i32::from(*((ptr3 + 0) as *const u8)) {
                0 => Ok(()),
                1 => Err(match i32::from(*((ptr3 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Request a shared advisory lock for an open file.
    ///
    /// This requests a *shared* lock; more than one shared lock can be held for
    /// a file at the same time.
    ///
    /// If the open file has an exclusive lock, this function downgrades the lock
    /// to a shared lock. If it has a shared lock, this function has no effect.
    ///
    /// This requests an *advisory* lock, meaning that the file could be accessed
    /// by other programs that don't hold the lock.
    ///
    /// It is unspecified how shared locks interact with locks acquired by
    /// non-WASI programs.
    ///
    /// This function blocks until the lock can be acquired.
    ///
    /// Not all filesystems support locking; on filesystems which don't support
    /// locking, this function returns `errno::notsup`.
    ///
    /// Note: This is similar to `flock(fd, LOCK_SH)` in Unix.
    pub fn lock_shared(fd: Descriptor) -> Result<(), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "lock-shared")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_lock-shared")]
                fn wit_import(_: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(fd), ptr0);
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok(()),
                1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Request an exclusive advisory lock for an open file.
    ///
    /// This requests an *exclusive* lock; no other locks may be held for the
    /// file while an exclusive lock is held.
    ///
    /// If the open file has a shared lock and there are no exclusive locks held
    /// for the fhile, this function upgrades the lock to an exclusive lock. If the
    /// open file already has an exclusive lock, this function has no effect.
    ///
    /// This requests an *advisory* lock, meaning that the file could be accessed
    /// by other programs that don't hold the lock.
    ///
    /// It is unspecified whether this function succeeds if the file descriptor
    /// is not opened for writing. It is unspecified how exclusive locks interact
    /// with locks acquired by non-WASI programs.
    ///
    /// This function blocks until the lock can be acquired.
    ///
    /// Not all filesystems support locking; on filesystems which don't support
    /// locking, this function returns `errno::notsup`.
    ///
    /// Note: This is similar to `flock(fd, LOCK_EX)` in Unix.
    pub fn lock_exclusive(fd: Descriptor) -> Result<(), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "lock-exclusive")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-filesystem_lock-exclusive"
                )]
                fn wit_import(_: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(fd), ptr0);
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok(()),
                1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Request a shared advisory lock for an open file.
    ///
    /// This requests a *shared* lock; more than one shared lock can be held for
    /// a file at the same time.
    ///
    /// If the open file has an exclusive lock, this function downgrades the lock
    /// to a shared lock. If it has a shared lock, this function has no effect.
    ///
    /// This requests an *advisory* lock, meaning that the file could be accessed
    /// by other programs that don't hold the lock.
    ///
    /// It is unspecified how shared locks interact with locks acquired by
    /// non-WASI programs.
    ///
    /// This function returns `errno::wouldblock` if the lock cannot be acquired.
    ///
    /// Not all filesystems support locking; on filesystems which don't support
    /// locking, this function returns `errno::notsup`.
    ///
    /// Note: This is similar to `flock(fd, LOCK_SH | LOCK_NB)` in Unix.
    pub fn try_lock_shared(fd: Descriptor) -> Result<(), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "try-lock-shared")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-filesystem_try-lock-shared"
                )]
                fn wit_import(_: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(fd), ptr0);
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok(()),
                1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Request an exclusive advisory lock for an open file.
    ///
    /// This requests an *exclusive* lock; no other locks may be held for the
    /// file while an exclusive lock is held.
    ///
    /// If the open file has a shared lock and there are no exclusive locks held
    /// for the fhile, this function upgrades the lock to an exclusive lock. If the
    /// open file already has an exclusive lock, this function has no effect.
    ///
    /// This requests an *advisory* lock, meaning that the file could be accessed
    /// by other programs that don't hold the lock.
    ///
    /// It is unspecified whether this function succeeds if the file descriptor
    /// is not opened for writing. It is unspecified how exclusive locks interact
    /// with locks acquired by non-WASI programs.
    ///
    /// This function returns `errno::wouldblock` if the lock cannot be acquired.
    ///
    /// Not all filesystems support locking; on filesystems which don't support
    /// locking, this function returns `errno::notsup`.
    ///
    /// Note: This is similar to `flock(fd, LOCK_EX | LOCK_NB)` in Unix.
    pub fn try_lock_exclusive(fd: Descriptor) -> Result<(), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "try-lock-exclusive")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-filesystem_try-lock-exclusive"
                )]
                fn wit_import(_: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(fd), ptr0);
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok(()),
                1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Release a shared or exclusive lock on an open file.
    ///
    /// Note: This is similar to `flock(fd, LOCK_UN)` in Unix.
    pub fn unlock(fd: Descriptor) -> Result<(), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-filesystem")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "unlock")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_unlock")]
                fn wit_import(_: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(fd), ptr0);
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok(()),
                1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Again,
                    2 => Errno::Already,
                    3 => Errno::Badf,
                    4 => Errno::Busy,
                    5 => Errno::Deadlk,
                    6 => Errno::Dquot,
                    7 => Errno::Exist,
                    8 => Errno::Fbig,
                    9 => Errno::Ilseq,
                    10 => Errno::Inprogress,
                    11 => Errno::Intr,
                    12 => Errno::Inval,
                    13 => Errno::Io,
                    14 => Errno::Isdir,
                    15 => Errno::Loop,
                    16 => Errno::Mlink,
                    17 => Errno::Msgsize,
                    18 => Errno::Nametoolong,
                    19 => Errno::Nodev,
                    20 => Errno::Noent,
                    21 => Errno::Nolck,
                    22 => Errno::Nomem,
                    23 => Errno::Nospc,
                    24 => Errno::Nosys,
                    25 => Errno::Notdir,
                    26 => Errno::Notempty,
                    27 => Errno::Notrecoverable,
                    28 => Errno::Notsup,
                    29 => Errno::Notty,
                    30 => Errno::Nxio,
                    31 => Errno::Overflow,
                    32 => Errno::Perm,
                    33 => Errno::Pipe,
                    34 => Errno::Rofs,
                    35 => Errno::Spipe,
                    36 => Errno::Txtbsy,
                    37 => Errno::Xdev,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
}

#[allow(clippy::all)]
pub mod wasi_random {
    #[allow(clippy::all)]
    /// Return `len` random bytes.
    ///
    /// This function must produce data from an adaquately seeded CSPRNG, so it
    /// must not block, and the returned data is always unpredictable.
    ///
    /// Deterministic environments must omit this function, rather than
    /// implementing it with deterministic data.
    pub fn get_random_bytes(len: u32) -> wit_bindgen_guest_rust::rt::vec::Vec<u8> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(4))]
            struct RetArea([u8; 8]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-random")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "get-random-bytes")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-random_get-random-bytes"
                )]
                fn wit_import(_: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(len), ptr0);
            let len1 = *((ptr0 + 4) as *const i32) as usize;
            Vec::from_raw_parts(*((ptr0 + 0) as *const i32) as *mut _, len1, len1)
        }
    }
    #[allow(clippy::all)]
    /// Return a random `u64` value.
    ///
    /// This function must produce data from an adaquately seeded CSPRNG, so it
    /// must not block, and the returned data is always unpredictable.
    ///
    /// Deterministic environments must omit this function, rather than
    /// implementing it with deterministic data.
    pub fn get_random_u64() -> u64 {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[link(wasm_import_module = "wasi-random")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "get-random-u64")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-random_get-random-u64")]
                fn wit_import() -> i64;
            }
            let ret = wit_import();
            ret as u64
        }
    }
}

#[allow(clippy::all)]
pub mod wasi_poll {
    pub type MonotonicClock = super::wasi_clocks::MonotonicClock;
    pub type Instant = super::wasi_clocks::Instant;
    pub type InputStream = super::wasi_io::InputStream;
    pub type OutputStream = super::wasi_io::OutputStream;
    /// A "pollable" handle.
    ///
    /// This is conceptually represents a `stream<_, _>`, or in other words,
    /// a stream that one can wait on, repeatedly, but which does not itself
    /// produce any data. It's temporary scaffolding until component-model's
    /// async features are ready.
    ///
    /// And at present, it is a `u32` instead of being an actual handle, until
    /// the wit-bindgen implementation of handles and resources is ready.
    ///
    /// Waitable lifetimes are not automatically managed. Users must ensure
    /// that they do not outlive the resource they reference.
    pub type Pollable = u32;
    #[allow(clippy::all)]
    /// Dispose of the specified `pollable`, after which it may no longer be used.
    pub fn drop_pollable(f: Pollable) -> () {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[link(wasm_import_module = "wasi-poll")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "drop-pollable")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-poll_drop-pollable")]
                fn wit_import(_: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(f));
        }
    }
    #[allow(clippy::all)]
    /// Create a `pollable` which will resolve once either the specified stream has bytes
    /// available to read or the other end of the stream has been closed.
    pub fn subscribe_read(s: InputStream) -> Pollable {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[link(wasm_import_module = "wasi-poll")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "subscribe-read")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-poll_subscribe-read")]
                fn wit_import(_: i32) -> i32;
            }
            let ret = wit_import(wit_bindgen_guest_rust::rt::as_i32(s));
            ret as u32
        }
    }
    #[allow(clippy::all)]
    /// Create a `pollable` which will resolve once either the specified stream is ready
    /// to accept bytes or the other end of the stream has been closed.
    pub fn subscribe_write(s: OutputStream) -> Pollable {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[link(wasm_import_module = "wasi-poll")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "subscribe-write")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-poll_subscribe-write")]
                fn wit_import(_: i32) -> i32;
            }
            let ret = wit_import(wit_bindgen_guest_rust::rt::as_i32(s));
            ret as u32
        }
    }
    #[allow(clippy::all)]
    /// Create a `pollable` which will resolve once the specified time has been reached.
    pub fn subscribe_monotonic_clock(
        clock: MonotonicClock,
        when: Instant,
        absolute: bool,
    ) -> Pollable {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[link(wasm_import_module = "wasi-poll")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "subscribe-monotonic-clock")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-poll_subscribe-monotonic-clock"
                )]
                fn wit_import(_: i32, _: i64, _: i32) -> i32;
            }
            let ret = wit_import(
                wit_bindgen_guest_rust::rt::as_i32(clock),
                wit_bindgen_guest_rust::rt::as_i64(when),
                match absolute {
                    true => 1,
                    false => 0,
                },
            );
            ret as u32
        }
    }
    #[allow(clippy::all)]
    /// Poll for completion on a set of pollables.
    ///
    /// The "oneoff" in the name refers to the fact that this function must do a
    /// linear scan through the entire list of subscriptions, which may be
    /// inefficient if the number is large and the same subscriptions are used
    /// many times. In the future, it may be accompanied by an API similar to
    /// Linux's `epoll` which allows sets of subscriptions to be registered and
    /// made efficiently reusable.
    ///
    /// Note that the return type would ideally be `list<bool>`, but that would
    /// be more difficult to polyfill given the current state of `wit-bindgen`.
    /// See https://github.com/bytecodealliance/preview2-prototyping/pull/11#issuecomment-1329873061
    /// for details.  For now, we use zero to mean "not ready" and non-zero to
    /// mean "ready".
    pub fn poll_oneoff(in_: &[Pollable]) -> wit_bindgen_guest_rust::rt::vec::Vec<u8> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(4))]
            struct RetArea([u8; 8]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let vec0 = in_;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            let ptr1 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-poll")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "poll-oneoff")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-poll_poll-oneoff")]
                fn wit_import(_: i32, _: i32, _: i32);
            }
            wit_import(ptr0, len0, ptr1);
            let len2 = *((ptr1 + 4) as *const i32) as usize;
            Vec::from_raw_parts(*((ptr1 + 0) as *const i32) as *mut _, len2, len2)
        }
    }
}

#[allow(clippy::all)]
pub mod wasi_net {
    /// A network, possibly virtual.
    pub type Network = u32;
}

#[allow(clippy::all)]
pub mod wasi_tcp {
    pub type InputStream = super::wasi_io::InputStream;
    pub type OutputStream = super::wasi_io::OutputStream;
    pub type Network = super::wasi_net::Network;
    /// A "socket" descriptor for a TCP listener. In the future, this will be
    /// replaced by handle types.
    pub type TcpListener = u32;
    wit_bindgen_guest_rust::bitflags::bitflags! {
      /// Listener flags.
      pub struct ListenerFlags: u8 {
        /// Equivalent to `O_NONBLOCK`.
        const NONBLOCK = 1 << 0;
      }
    }
    impl ListenerFlags {
        /// Convert from a raw integer, preserving any unknown bits. See
        /// <https://github.com/bitflags/bitflags/issues/263#issuecomment-957088321>
        pub fn from_bits_preserve(bits: u8) -> Self {
            Self { bits }
        }
    }
    /// A "socket" descriptor for a listener. In the future, this will be
    /// replaced by handle types.
    /// fixme: move this and related stuff into a separate wasi-socket?
    pub type Listener = u32;
    pub type Ipv6Address = (u16, u16, u16, u16, u16, u16, u16, u16);
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Ipv6SocketAddress {
        pub address: Ipv6Address,
        pub port: u16,
        pub flow_info: u32,
        pub scope_id: u32,
    }
    impl core::fmt::Debug for Ipv6SocketAddress {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Ipv6SocketAddress")
                .field("address", &self.address)
                .field("port", &self.port)
                .field("flow-info", &self.flow_info)
                .field("scope-id", &self.scope_id)
                .finish()
        }
    }
    pub type Ipv4Address = (u8, u8, u8, u8);
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Ipv4SocketAddress {
        pub address: Ipv4Address,
        pub port: u16,
    }
    impl core::fmt::Debug for Ipv4SocketAddress {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Ipv4SocketAddress")
                .field("address", &self.address)
                .field("port", &self.port)
                .finish()
        }
    }
    #[derive(Clone, Copy)]
    pub enum IpSocketAddress {
        Ipv4(Ipv4SocketAddress),
        Ipv6(Ipv6SocketAddress),
    }
    impl core::fmt::Debug for IpSocketAddress {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                IpSocketAddress::Ipv4(e) => {
                    f.debug_tuple("IpSocketAddress::Ipv4").field(e).finish()
                }
                IpSocketAddress::Ipv6(e) => {
                    f.debug_tuple("IpSocketAddress::Ipv6").field(e).finish()
                }
            }
        }
    }
    /// Size of a range of bytes that may be ready to be read.
    pub type IoSize = u64;
    /// Error codes returned by functions.
    /// Not all of these error codes are returned by the functions provided by this
    /// API; some are used in higher-level library layers, and others are provided
    /// merely for alignment with POSIX.
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Errno {
        /// Permission denied.
        Access,
        /// Address in use.
        Addrinuse,
        /// Address not available.
        Addrnotavail,
        /// Address family not supported.
        Afnosupport,
        /// Resource unavailable, or operation would block.
        Again,
        /// Connection already in progress.
        Already,
        /// Bad descriptor.
        Badf,
        /// Device or resource busy.
        Busy,
        /// Connection aborted.
        ConnectionAborted,
        /// Connection refused.
        ConnectionRefused,
        /// Connection reset.
        ConnectionReset,
        /// Resource deadlock would occur.
        Deadlock,
        /// Destination address required.
        Destaddrreq,
        /// Host is unreachable.
        HostUnreachable,
        /// Illegal byte sequence.
        Ilseq,
        /// Operation in progress.
        Inprogress,
        /// Interrupted function.
        Intr,
        /// Invalid argument.
        Inval,
        /// I/O error.
        Io,
        /// Socket is connected.
        Isconn,
        /// Message too large.
        Msgsize,
        /// Multihop attempted.
        Multihop,
        /// Filename too long.
        Nametoolong,
        /// Network is down.
        NetworkDown,
        /// Connection aborted by network.
        NetworkReset,
        /// Network unreachable.
        NetworkUnreachable,
        /// No buffer space available.
        Nobufs,
        /// No such file or directory.
        Noent,
        /// Not enough space.
        Nomem,
        /// Protocol not available.
        Noprotoopt,
        /// Function not supported.
        Nosys,
        /// State not recoverable.
        Notrecoverable,
        /// Not supported, or operation not supported on socket.
        Notsup,
        /// Value too large to be stored in data type.
        Overflow,
        /// Operation not permitted.
        Perm,
        /// Connection timed out.
        Timedout,
    }
    impl Errno {
        pub fn name(&self) -> &'static str {
            match self {
                Errno::Access => "access",
                Errno::Addrinuse => "addrinuse",
                Errno::Addrnotavail => "addrnotavail",
                Errno::Afnosupport => "afnosupport",
                Errno::Again => "again",
                Errno::Already => "already",
                Errno::Badf => "badf",
                Errno::Busy => "busy",
                Errno::ConnectionAborted => "connection-aborted",
                Errno::ConnectionRefused => "connection-refused",
                Errno::ConnectionReset => "connection-reset",
                Errno::Deadlock => "deadlock",
                Errno::Destaddrreq => "destaddrreq",
                Errno::HostUnreachable => "host-unreachable",
                Errno::Ilseq => "ilseq",
                Errno::Inprogress => "inprogress",
                Errno::Intr => "intr",
                Errno::Inval => "inval",
                Errno::Io => "io",
                Errno::Isconn => "isconn",
                Errno::Msgsize => "msgsize",
                Errno::Multihop => "multihop",
                Errno::Nametoolong => "nametoolong",
                Errno::NetworkDown => "network-down",
                Errno::NetworkReset => "network-reset",
                Errno::NetworkUnreachable => "network-unreachable",
                Errno::Nobufs => "nobufs",
                Errno::Noent => "noent",
                Errno::Nomem => "nomem",
                Errno::Noprotoopt => "noprotoopt",
                Errno::Nosys => "nosys",
                Errno::Notrecoverable => "notrecoverable",
                Errno::Notsup => "notsup",
                Errno::Overflow => "overflow",
                Errno::Perm => "perm",
                Errno::Timedout => "timedout",
            }
        }
        pub fn message(&self) -> &'static str {
            match self {
                Errno::Access => "Permission denied.",
                Errno::Addrinuse => "Address in use.",
                Errno::Addrnotavail => "Address not available.",
                Errno::Afnosupport => "Address family not supported.",
                Errno::Again => "Resource unavailable, or operation would block.",
                Errno::Already => "Connection already in progress.",
                Errno::Badf => "Bad descriptor.",
                Errno::Busy => "Device or resource busy.",
                Errno::ConnectionAborted => "Connection aborted.",
                Errno::ConnectionRefused => "Connection refused.",
                Errno::ConnectionReset => "Connection reset.",
                Errno::Deadlock => "Resource deadlock would occur.",
                Errno::Destaddrreq => "Destination address required.",
                Errno::HostUnreachable => "Host is unreachable.",
                Errno::Ilseq => "Illegal byte sequence.",
                Errno::Inprogress => "Operation in progress.",
                Errno::Intr => "Interrupted function.",
                Errno::Inval => "Invalid argument.",
                Errno::Io => "I/O error.",
                Errno::Isconn => "Socket is connected.",
                Errno::Msgsize => "Message too large.",
                Errno::Multihop => "Multihop attempted.",
                Errno::Nametoolong => "Filename too long.",
                Errno::NetworkDown => "Network is down.",
                Errno::NetworkReset => "Connection aborted by network.",
                Errno::NetworkUnreachable => "Network unreachable.",
                Errno::Nobufs => "No buffer space available.",
                Errno::Noent => "No such file or directory.",
                Errno::Nomem => "Not enough space.",
                Errno::Noprotoopt => "Protocol not available.",
                Errno::Nosys => "Function not supported.",
                Errno::Notrecoverable => "State not recoverable.",
                Errno::Notsup => "Not supported, or operation not supported on socket.",
                Errno::Overflow => "Value too large to be stored in data type.",
                Errno::Perm => "Operation not permitted.",
                Errno::Timedout => "Connection timed out.",
            }
        }
    }
    impl core::fmt::Debug for Errno {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Errno")
                .field("code", &(*self as i32))
                .field("name", &self.name())
                .field("message", &self.message())
                .finish()
        }
    }
    impl core::fmt::Display for Errno {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} (error {})", self.name(), *self as i32)
        }
    }

    impl std::error::Error for Errno {}
    wit_bindgen_guest_rust::bitflags::bitflags! {
      /// Connection flags.
      pub struct ConnectionFlags: u8 {
        /// Equivalent to `SO_KEEPALIVE`.
        const KEEPALIVE = 1 << 0;
        /// Equivalent to `O_NONBLOCK`.
        const NONBLOCK = 1 << 1;
        /// Equivalent to `TCP_NODELAY`.
        const NODELAY = 1 << 2;
      }
    }
    impl ConnectionFlags {
        /// Convert from a raw integer, preserving any unknown bits. See
        /// <https://github.com/bitflags/bitflags/issues/263#issuecomment-957088321>
        pub fn from_bits_preserve(bits: u8) -> Self {
            Self { bits }
        }
    }
    /// A "socket" descriptor for a TCP connection. In the future, this will be
    /// replaced by handle types.
    pub type Connection = u32;
    #[allow(clippy::all)]
    /// Creates a new listener.
    ///
    /// If the IP address is zero (`0.0.0.0` in IPv4, `::` in IPv6), the
    /// implementation will decide which network address to bind to.
    ///
    /// If the TCP/UDP port is zero, the socket will be bound to an
    /// unspecified free port.
    ///
    /// The listener should be destroyed with `close-tcp-listener` when no longer in use.
    pub fn listen(
        network: Network,
        address: IpSocketAddress,
        backlog: Option<u32>,
        flags: ListenerFlags,
    ) -> Result<TcpListener, Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(4))]
            struct RetArea([u8; 8]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let (
                result4_0,
                result4_1,
                result4_2,
                result4_3,
                result4_4,
                result4_5,
                result4_6,
                result4_7,
                result4_8,
                result4_9,
                result4_10,
                result4_11,
            ) = match address {
                IpSocketAddress::Ipv4(e) => {
                    let Ipv4SocketAddress {
                        address: address0,
                        port: port0,
                    } = e;
                    let (t1_0, t1_1, t1_2, t1_3) = address0;

                    (
                        0i32,
                        wit_bindgen_guest_rust::rt::as_i32(t1_0),
                        wit_bindgen_guest_rust::rt::as_i32(t1_1),
                        wit_bindgen_guest_rust::rt::as_i32(t1_2),
                        wit_bindgen_guest_rust::rt::as_i32(t1_3),
                        wit_bindgen_guest_rust::rt::as_i32(port0),
                        0i32,
                        0i32,
                        0i32,
                        0i32,
                        0i32,
                        0i32,
                    )
                }
                IpSocketAddress::Ipv6(e) => {
                    let Ipv6SocketAddress {
                        address: address2,
                        port: port2,
                        flow_info: flow_info2,
                        scope_id: scope_id2,
                    } = e;
                    let (t3_0, t3_1, t3_2, t3_3, t3_4, t3_5, t3_6, t3_7) = address2;

                    (
                        1i32,
                        wit_bindgen_guest_rust::rt::as_i32(t3_0),
                        wit_bindgen_guest_rust::rt::as_i32(t3_1),
                        wit_bindgen_guest_rust::rt::as_i32(t3_2),
                        wit_bindgen_guest_rust::rt::as_i32(t3_3),
                        wit_bindgen_guest_rust::rt::as_i32(t3_4),
                        wit_bindgen_guest_rust::rt::as_i32(t3_5),
                        wit_bindgen_guest_rust::rt::as_i32(t3_6),
                        wit_bindgen_guest_rust::rt::as_i32(t3_7),
                        wit_bindgen_guest_rust::rt::as_i32(port2),
                        wit_bindgen_guest_rust::rt::as_i32(flow_info2),
                        wit_bindgen_guest_rust::rt::as_i32(scope_id2),
                    )
                }
            };
            let (result5_0, result5_1) = match backlog {
                Some(e) => (1i32, wit_bindgen_guest_rust::rt::as_i32(e)),
                None => (0i32, 0i32),
            };
            let flags6 = flags;
            let ptr7 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-tcp")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "listen")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_listen")]
                fn wit_import(
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                );
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(network),
                result4_0,
                result4_1,
                result4_2,
                result4_3,
                result4_4,
                result4_5,
                result4_6,
                result4_7,
                result4_8,
                result4_9,
                result4_10,
                result4_11,
                result5_0,
                result5_1,
                (flags6.bits() >> 0) as i32,
                ptr7,
            );
            match i32::from(*((ptr7 + 0) as *const u8)) {
                0 => Ok(*((ptr7 + 4) as *const i32) as u32),
                1 => Err(match i32::from(*((ptr7 + 4) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Addrinuse,
                    2 => Errno::Addrnotavail,
                    3 => Errno::Afnosupport,
                    4 => Errno::Again,
                    5 => Errno::Already,
                    6 => Errno::Badf,
                    7 => Errno::Busy,
                    8 => Errno::ConnectionAborted,
                    9 => Errno::ConnectionRefused,
                    10 => Errno::ConnectionReset,
                    11 => Errno::Deadlock,
                    12 => Errno::Destaddrreq,
                    13 => Errno::HostUnreachable,
                    14 => Errno::Ilseq,
                    15 => Errno::Inprogress,
                    16 => Errno::Intr,
                    17 => Errno::Inval,
                    18 => Errno::Io,
                    19 => Errno::Isconn,
                    20 => Errno::Msgsize,
                    21 => Errno::Multihop,
                    22 => Errno::Nametoolong,
                    23 => Errno::NetworkDown,
                    24 => Errno::NetworkReset,
                    25 => Errno::NetworkUnreachable,
                    26 => Errno::Nobufs,
                    27 => Errno::Noent,
                    28 => Errno::Nomem,
                    29 => Errno::Noprotoopt,
                    30 => Errno::Nosys,
                    31 => Errno::Notrecoverable,
                    32 => Errno::Notsup,
                    33 => Errno::Overflow,
                    34 => Errno::Perm,
                    35 => Errno::Timedout,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Accepts a new incoming connection.
    ///
    /// When in non-blocking mode, this function will return `errno::again`
    /// when no new incoming connection is immediately available. This is an
    /// indication to poll for incoming data on the listener. Otherwise, this
    /// function will block until an incoming connection is available.
    ///
    /// Returns a tuple of a connection handle, and input stream, and an output
    /// stream for the socket.
    ///
    /// The connection should be destroyed with `close-connection` when no longer in use.
    pub fn accept(
        listener: Listener,
        flags: ConnectionFlags,
    ) -> Result<(Connection, InputStream, OutputStream), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(4))]
            struct RetArea([u8; 16]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let flags0 = flags;
            let ptr1 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-tcp")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "accept")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_accept")]
                fn wit_import(_: i32, _: i32, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(listener),
                (flags0.bits() >> 0) as i32,
                ptr1,
            );
            match i32::from(*((ptr1 + 0) as *const u8)) {
                0 => Ok((
                    *((ptr1 + 4) as *const i32) as u32,
                    *((ptr1 + 8) as *const i32) as u32,
                    *((ptr1 + 12) as *const i32) as u32,
                )),
                1 => Err(match i32::from(*((ptr1 + 4) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Addrinuse,
                    2 => Errno::Addrnotavail,
                    3 => Errno::Afnosupport,
                    4 => Errno::Again,
                    5 => Errno::Already,
                    6 => Errno::Badf,
                    7 => Errno::Busy,
                    8 => Errno::ConnectionAborted,
                    9 => Errno::ConnectionRefused,
                    10 => Errno::ConnectionReset,
                    11 => Errno::Deadlock,
                    12 => Errno::Destaddrreq,
                    13 => Errno::HostUnreachable,
                    14 => Errno::Ilseq,
                    15 => Errno::Inprogress,
                    16 => Errno::Intr,
                    17 => Errno::Inval,
                    18 => Errno::Io,
                    19 => Errno::Isconn,
                    20 => Errno::Msgsize,
                    21 => Errno::Multihop,
                    22 => Errno::Nametoolong,
                    23 => Errno::NetworkDown,
                    24 => Errno::NetworkReset,
                    25 => Errno::NetworkUnreachable,
                    26 => Errno::Nobufs,
                    27 => Errno::Noent,
                    28 => Errno::Nomem,
                    29 => Errno::Noprotoopt,
                    30 => Errno::Nosys,
                    31 => Errno::Notrecoverable,
                    32 => Errno::Notsup,
                    33 => Errno::Overflow,
                    34 => Errno::Perm,
                    35 => Errno::Timedout,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Accepts a new incoming connection on a TCP socket.
    ///
    /// This is the same as `accept`, but takes a `tcp-listener`
    /// and additionally returns an `ip-socket-address`.
    pub fn accept_tcp(
        listener: TcpListener,
        flags: ConnectionFlags,
    ) -> Result<(Connection, InputStream, OutputStream, IpSocketAddress), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(4))]
            struct RetArea([u8; 48]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let flags0 = flags;
            let ptr1 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-tcp")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "accept-tcp")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_accept-tcp")]
                fn wit_import(_: i32, _: i32, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(listener),
                (flags0.bits() >> 0) as i32,
                ptr1,
            );
            match i32::from(*((ptr1 + 0) as *const u8)) {
                0 => Ok((
                    *((ptr1 + 4) as *const i32) as u32,
                    *((ptr1 + 8) as *const i32) as u32,
                    *((ptr1 + 12) as *const i32) as u32,
                    match i32::from(*((ptr1 + 16) as *const u8)) {
                        0 => IpSocketAddress::Ipv4(Ipv4SocketAddress {
                            address: (
                                i32::from(*((ptr1 + 20) as *const u8)) as u8,
                                i32::from(*((ptr1 + 21) as *const u8)) as u8,
                                i32::from(*((ptr1 + 22) as *const u8)) as u8,
                                i32::from(*((ptr1 + 23) as *const u8)) as u8,
                            ),
                            port: i32::from(*((ptr1 + 24) as *const u16)) as u16,
                        }),
                        1 => IpSocketAddress::Ipv6(Ipv6SocketAddress {
                            address: (
                                i32::from(*((ptr1 + 20) as *const u16)) as u16,
                                i32::from(*((ptr1 + 22) as *const u16)) as u16,
                                i32::from(*((ptr1 + 24) as *const u16)) as u16,
                                i32::from(*((ptr1 + 26) as *const u16)) as u16,
                                i32::from(*((ptr1 + 28) as *const u16)) as u16,
                                i32::from(*((ptr1 + 30) as *const u16)) as u16,
                                i32::from(*((ptr1 + 32) as *const u16)) as u16,
                                i32::from(*((ptr1 + 34) as *const u16)) as u16,
                            ),
                            port: i32::from(*((ptr1 + 36) as *const u16)) as u16,
                            flow_info: *((ptr1 + 40) as *const i32) as u32,
                            scope_id: *((ptr1 + 44) as *const i32) as u32,
                        }),
                        _ => panic!("invalid enum discriminant"),
                    },
                )),
                1 => Err(match i32::from(*((ptr1 + 4) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Addrinuse,
                    2 => Errno::Addrnotavail,
                    3 => Errno::Afnosupport,
                    4 => Errno::Again,
                    5 => Errno::Already,
                    6 => Errno::Badf,
                    7 => Errno::Busy,
                    8 => Errno::ConnectionAborted,
                    9 => Errno::ConnectionRefused,
                    10 => Errno::ConnectionReset,
                    11 => Errno::Deadlock,
                    12 => Errno::Destaddrreq,
                    13 => Errno::HostUnreachable,
                    14 => Errno::Ilseq,
                    15 => Errno::Inprogress,
                    16 => Errno::Intr,
                    17 => Errno::Inval,
                    18 => Errno::Io,
                    19 => Errno::Isconn,
                    20 => Errno::Msgsize,
                    21 => Errno::Multihop,
                    22 => Errno::Nametoolong,
                    23 => Errno::NetworkDown,
                    24 => Errno::NetworkReset,
                    25 => Errno::NetworkUnreachable,
                    26 => Errno::Nobufs,
                    27 => Errno::Noent,
                    28 => Errno::Nomem,
                    29 => Errno::Noprotoopt,
                    30 => Errno::Nosys,
                    31 => Errno::Notrecoverable,
                    32 => Errno::Notsup,
                    33 => Errno::Overflow,
                    34 => Errno::Perm,
                    35 => Errno::Timedout,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Connect to a remote endpoint.
    ///
    /// If the local IP address is zero (`0.0.0.0` in IPv4, `::` in IPv6), the
    /// implementation will decide which network address to bind to.
    ///
    /// If the local TCP/UDP port is zero, the socket will be bound to an
    /// unspecified free port.
    ///
    /// The connection should be destroyed with `close-connection` when no longer in use.
    ///
    /// References
    /// - https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html
    /// - https://man7.org/linux/man-pages/man2/bind.2.html
    /// - https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html
    /// - https://man7.org/linux/man-pages/man2/connect.2.html
    pub fn connect(
        network: Network,
        local_address: IpSocketAddress,
        remote_address: IpSocketAddress,
        flags: ConnectionFlags,
    ) -> Result<(Connection, InputStream, OutputStream), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(4))]
            struct RetArea([u8; 72]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            *((ptr0 + 0) as *mut i32) = wit_bindgen_guest_rust::rt::as_i32(network);
            match local_address {
                IpSocketAddress::Ipv4(e) => {
                    *((ptr0 + 4) as *mut u8) = (0i32) as u8;
                    let Ipv4SocketAddress {
                        address: address1,
                        port: port1,
                    } = e;
                    let (t2_0, t2_1, t2_2, t2_3) = address1;
                    *((ptr0 + 8) as *mut u8) = (wit_bindgen_guest_rust::rt::as_i32(t2_0)) as u8;
                    *((ptr0 + 9) as *mut u8) = (wit_bindgen_guest_rust::rt::as_i32(t2_1)) as u8;
                    *((ptr0 + 10) as *mut u8) = (wit_bindgen_guest_rust::rt::as_i32(t2_2)) as u8;
                    *((ptr0 + 11) as *mut u8) = (wit_bindgen_guest_rust::rt::as_i32(t2_3)) as u8;
                    *((ptr0 + 12) as *mut u16) = (wit_bindgen_guest_rust::rt::as_i32(port1)) as u16;
                }
                IpSocketAddress::Ipv6(e) => {
                    *((ptr0 + 4) as *mut u8) = (1i32) as u8;
                    let Ipv6SocketAddress {
                        address: address3,
                        port: port3,
                        flow_info: flow_info3,
                        scope_id: scope_id3,
                    } = e;
                    let (t4_0, t4_1, t4_2, t4_3, t4_4, t4_5, t4_6, t4_7) = address3;
                    *((ptr0 + 8) as *mut u16) = (wit_bindgen_guest_rust::rt::as_i32(t4_0)) as u16;
                    *((ptr0 + 10) as *mut u16) = (wit_bindgen_guest_rust::rt::as_i32(t4_1)) as u16;
                    *((ptr0 + 12) as *mut u16) = (wit_bindgen_guest_rust::rt::as_i32(t4_2)) as u16;
                    *((ptr0 + 14) as *mut u16) = (wit_bindgen_guest_rust::rt::as_i32(t4_3)) as u16;
                    *((ptr0 + 16) as *mut u16) = (wit_bindgen_guest_rust::rt::as_i32(t4_4)) as u16;
                    *((ptr0 + 18) as *mut u16) = (wit_bindgen_guest_rust::rt::as_i32(t4_5)) as u16;
                    *((ptr0 + 20) as *mut u16) = (wit_bindgen_guest_rust::rt::as_i32(t4_6)) as u16;
                    *((ptr0 + 22) as *mut u16) = (wit_bindgen_guest_rust::rt::as_i32(t4_7)) as u16;
                    *((ptr0 + 24) as *mut u16) = (wit_bindgen_guest_rust::rt::as_i32(port3)) as u16;
                    *((ptr0 + 28) as *mut i32) = wit_bindgen_guest_rust::rt::as_i32(flow_info3);
                    *((ptr0 + 32) as *mut i32) = wit_bindgen_guest_rust::rt::as_i32(scope_id3);
                }
            };
            match remote_address {
                IpSocketAddress::Ipv4(e) => {
                    *((ptr0 + 36) as *mut u8) = (0i32) as u8;
                    let Ipv4SocketAddress {
                        address: address5,
                        port: port5,
                    } = e;
                    let (t6_0, t6_1, t6_2, t6_3) = address5;
                    *((ptr0 + 40) as *mut u8) = (wit_bindgen_guest_rust::rt::as_i32(t6_0)) as u8;
                    *((ptr0 + 41) as *mut u8) = (wit_bindgen_guest_rust::rt::as_i32(t6_1)) as u8;
                    *((ptr0 + 42) as *mut u8) = (wit_bindgen_guest_rust::rt::as_i32(t6_2)) as u8;
                    *((ptr0 + 43) as *mut u8) = (wit_bindgen_guest_rust::rt::as_i32(t6_3)) as u8;
                    *((ptr0 + 44) as *mut u16) = (wit_bindgen_guest_rust::rt::as_i32(port5)) as u16;
                }
                IpSocketAddress::Ipv6(e) => {
                    *((ptr0 + 36) as *mut u8) = (1i32) as u8;
                    let Ipv6SocketAddress {
                        address: address7,
                        port: port7,
                        flow_info: flow_info7,
                        scope_id: scope_id7,
                    } = e;
                    let (t8_0, t8_1, t8_2, t8_3, t8_4, t8_5, t8_6, t8_7) = address7;
                    *((ptr0 + 40) as *mut u16) = (wit_bindgen_guest_rust::rt::as_i32(t8_0)) as u16;
                    *((ptr0 + 42) as *mut u16) = (wit_bindgen_guest_rust::rt::as_i32(t8_1)) as u16;
                    *((ptr0 + 44) as *mut u16) = (wit_bindgen_guest_rust::rt::as_i32(t8_2)) as u16;
                    *((ptr0 + 46) as *mut u16) = (wit_bindgen_guest_rust::rt::as_i32(t8_3)) as u16;
                    *((ptr0 + 48) as *mut u16) = (wit_bindgen_guest_rust::rt::as_i32(t8_4)) as u16;
                    *((ptr0 + 50) as *mut u16) = (wit_bindgen_guest_rust::rt::as_i32(t8_5)) as u16;
                    *((ptr0 + 52) as *mut u16) = (wit_bindgen_guest_rust::rt::as_i32(t8_6)) as u16;
                    *((ptr0 + 54) as *mut u16) = (wit_bindgen_guest_rust::rt::as_i32(t8_7)) as u16;
                    *((ptr0 + 56) as *mut u16) = (wit_bindgen_guest_rust::rt::as_i32(port7)) as u16;
                    *((ptr0 + 60) as *mut i32) = wit_bindgen_guest_rust::rt::as_i32(flow_info7);
                    *((ptr0 + 64) as *mut i32) = wit_bindgen_guest_rust::rt::as_i32(scope_id7);
                }
            };
            let flags9 = flags;
            *((ptr0 + 68) as *mut u8) = ((flags9.bits() >> 0) as i32) as u8;
            let ptr10 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-tcp")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "connect")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_connect")]
                fn wit_import(_: i32, _: i32);
            }
            wit_import(ptr0, ptr10);
            match i32::from(*((ptr10 + 0) as *const u8)) {
                0 => Ok((
                    *((ptr10 + 4) as *const i32) as u32,
                    *((ptr10 + 8) as *const i32) as u32,
                    *((ptr10 + 12) as *const i32) as u32,
                )),
                1 => Err(match i32::from(*((ptr10 + 4) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Addrinuse,
                    2 => Errno::Addrnotavail,
                    3 => Errno::Afnosupport,
                    4 => Errno::Again,
                    5 => Errno::Already,
                    6 => Errno::Badf,
                    7 => Errno::Busy,
                    8 => Errno::ConnectionAborted,
                    9 => Errno::ConnectionRefused,
                    10 => Errno::ConnectionReset,
                    11 => Errno::Deadlock,
                    12 => Errno::Destaddrreq,
                    13 => Errno::HostUnreachable,
                    14 => Errno::Ilseq,
                    15 => Errno::Inprogress,
                    16 => Errno::Intr,
                    17 => Errno::Inval,
                    18 => Errno::Io,
                    19 => Errno::Isconn,
                    20 => Errno::Msgsize,
                    21 => Errno::Multihop,
                    22 => Errno::Nametoolong,
                    23 => Errno::NetworkDown,
                    24 => Errno::NetworkReset,
                    25 => Errno::NetworkUnreachable,
                    26 => Errno::Nobufs,
                    27 => Errno::Noent,
                    28 => Errno::Nomem,
                    29 => Errno::Noprotoopt,
                    30 => Errno::Nosys,
                    31 => Errno::Notrecoverable,
                    32 => Errno::Notsup,
                    33 => Errno::Overflow,
                    34 => Errno::Perm,
                    35 => Errno::Timedout,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Send bytes to the remote connection.
    ///
    /// This function may not successfully send all bytes. Check the number of
    /// bytes returned.
    ///
    /// Note: This is similar to `pwrite` in POSIX.
    pub fn send(connection: Connection, bytes: &[u8]) -> Result<IoSize, Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(8))]
            struct RetArea([u8; 16]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let vec0 = bytes;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            let ptr1 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-tcp")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "send")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_send")]
                fn wit_import(_: i32, _: i32, _: i32, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(connection),
                ptr0,
                len0,
                ptr1,
            );
            match i32::from(*((ptr1 + 0) as *const u8)) {
                0 => Ok(*((ptr1 + 8) as *const i64) as u64),
                1 => Err(match i32::from(*((ptr1 + 8) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Addrinuse,
                    2 => Errno::Addrnotavail,
                    3 => Errno::Afnosupport,
                    4 => Errno::Again,
                    5 => Errno::Already,
                    6 => Errno::Badf,
                    7 => Errno::Busy,
                    8 => Errno::ConnectionAborted,
                    9 => Errno::ConnectionRefused,
                    10 => Errno::ConnectionReset,
                    11 => Errno::Deadlock,
                    12 => Errno::Destaddrreq,
                    13 => Errno::HostUnreachable,
                    14 => Errno::Ilseq,
                    15 => Errno::Inprogress,
                    16 => Errno::Intr,
                    17 => Errno::Inval,
                    18 => Errno::Io,
                    19 => Errno::Isconn,
                    20 => Errno::Msgsize,
                    21 => Errno::Multihop,
                    22 => Errno::Nametoolong,
                    23 => Errno::NetworkDown,
                    24 => Errno::NetworkReset,
                    25 => Errno::NetworkUnreachable,
                    26 => Errno::Nobufs,
                    27 => Errno::Noent,
                    28 => Errno::Nomem,
                    29 => Errno::Noprotoopt,
                    30 => Errno::Nosys,
                    31 => Errno::Notrecoverable,
                    32 => Errno::Notsup,
                    33 => Errno::Overflow,
                    34 => Errno::Perm,
                    35 => Errno::Timedout,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Receive bytes from the remote connection.
    ///
    /// This function receives **at most** `length` bytes from the remote
    /// connection.
    ///
    /// Note: This is similar to `recv` in POSIX.
    pub fn receive(
        connection: Connection,
        length: IoSize,
    ) -> Result<(wit_bindgen_guest_rust::rt::vec::Vec<u8>, bool), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(4))]
            struct RetArea([u8; 16]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-tcp")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "receive")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_receive")]
                fn wit_import(_: i32, _: i64, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(connection),
                wit_bindgen_guest_rust::rt::as_i64(length),
                ptr0,
            );
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok({
                    let len1 = *((ptr0 + 8) as *const i32) as usize;

                    (
                        Vec::from_raw_parts(*((ptr0 + 4) as *const i32) as *mut _, len1, len1),
                        match i32::from(*((ptr0 + 12) as *const u8)) {
                            0 => false,
                            1 => true,
                            _ => panic!("invalid bool discriminant"),
                        },
                    )
                }),
                1 => Err(match i32::from(*((ptr0 + 4) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Addrinuse,
                    2 => Errno::Addrnotavail,
                    3 => Errno::Afnosupport,
                    4 => Errno::Again,
                    5 => Errno::Already,
                    6 => Errno::Badf,
                    7 => Errno::Busy,
                    8 => Errno::ConnectionAborted,
                    9 => Errno::ConnectionRefused,
                    10 => Errno::ConnectionReset,
                    11 => Errno::Deadlock,
                    12 => Errno::Destaddrreq,
                    13 => Errno::HostUnreachable,
                    14 => Errno::Ilseq,
                    15 => Errno::Inprogress,
                    16 => Errno::Intr,
                    17 => Errno::Inval,
                    18 => Errno::Io,
                    19 => Errno::Isconn,
                    20 => Errno::Msgsize,
                    21 => Errno::Multihop,
                    22 => Errno::Nametoolong,
                    23 => Errno::NetworkDown,
                    24 => Errno::NetworkReset,
                    25 => Errno::NetworkUnreachable,
                    26 => Errno::Nobufs,
                    27 => Errno::Noent,
                    28 => Errno::Nomem,
                    29 => Errno::Noprotoopt,
                    30 => Errno::Nosys,
                    31 => Errno::Notrecoverable,
                    32 => Errno::Notsup,
                    33 => Errno::Overflow,
                    34 => Errno::Perm,
                    35 => Errno::Timedout,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Get the flags set for the connection.
    pub fn get_flags(connection: Connection) -> Result<ConnectionFlags, Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-tcp")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "get-flags")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_get-flags")]
                fn wit_import(_: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(connection), ptr0);
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok(ConnectionFlags::empty()
                    | ConnectionFlags::from_bits_preserve(
                        ((i32::from(*((ptr0 + 1) as *const u8)) as u8) << 0) as _,
                    )),
                1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Addrinuse,
                    2 => Errno::Addrnotavail,
                    3 => Errno::Afnosupport,
                    4 => Errno::Again,
                    5 => Errno::Already,
                    6 => Errno::Badf,
                    7 => Errno::Busy,
                    8 => Errno::ConnectionAborted,
                    9 => Errno::ConnectionRefused,
                    10 => Errno::ConnectionReset,
                    11 => Errno::Deadlock,
                    12 => Errno::Destaddrreq,
                    13 => Errno::HostUnreachable,
                    14 => Errno::Ilseq,
                    15 => Errno::Inprogress,
                    16 => Errno::Intr,
                    17 => Errno::Inval,
                    18 => Errno::Io,
                    19 => Errno::Isconn,
                    20 => Errno::Msgsize,
                    21 => Errno::Multihop,
                    22 => Errno::Nametoolong,
                    23 => Errno::NetworkDown,
                    24 => Errno::NetworkReset,
                    25 => Errno::NetworkUnreachable,
                    26 => Errno::Nobufs,
                    27 => Errno::Noent,
                    28 => Errno::Nomem,
                    29 => Errno::Noprotoopt,
                    30 => Errno::Nosys,
                    31 => Errno::Notrecoverable,
                    32 => Errno::Notsup,
                    33 => Errno::Overflow,
                    34 => Errno::Perm,
                    35 => Errno::Timedout,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Sets the flags for the connection.
    pub fn set_flags(connection: Connection, flags: ConnectionFlags) -> Result<(), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let flags0 = flags;
            let ptr1 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-tcp")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "set-flags")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_set-flags")]
                fn wit_import(_: i32, _: i32, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(connection),
                (flags0.bits() >> 0) as i32,
                ptr1,
            );
            match i32::from(*((ptr1 + 0) as *const u8)) {
                0 => Ok(()),
                1 => Err(match i32::from(*((ptr1 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Addrinuse,
                    2 => Errno::Addrnotavail,
                    3 => Errno::Afnosupport,
                    4 => Errno::Again,
                    5 => Errno::Already,
                    6 => Errno::Badf,
                    7 => Errno::Busy,
                    8 => Errno::ConnectionAborted,
                    9 => Errno::ConnectionRefused,
                    10 => Errno::ConnectionReset,
                    11 => Errno::Deadlock,
                    12 => Errno::Destaddrreq,
                    13 => Errno::HostUnreachable,
                    14 => Errno::Ilseq,
                    15 => Errno::Inprogress,
                    16 => Errno::Intr,
                    17 => Errno::Inval,
                    18 => Errno::Io,
                    19 => Errno::Isconn,
                    20 => Errno::Msgsize,
                    21 => Errno::Multihop,
                    22 => Errno::Nametoolong,
                    23 => Errno::NetworkDown,
                    24 => Errno::NetworkReset,
                    25 => Errno::NetworkUnreachable,
                    26 => Errno::Nobufs,
                    27 => Errno::Noent,
                    28 => Errno::Nomem,
                    29 => Errno::Noprotoopt,
                    30 => Errno::Nosys,
                    31 => Errno::Notrecoverable,
                    32 => Errno::Notsup,
                    33 => Errno::Overflow,
                    34 => Errno::Perm,
                    35 => Errno::Timedout,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Gets the receive-buffer size.
    ///
    /// Note: this is only a hint. Implementations may internally handle this
    /// in any way, including ignoring it.
    ///
    /// Equivalent to `SO_RCVBUF`.
    pub fn get_receive_buffer_size(connection: Connection) -> Result<IoSize, Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(8))]
            struct RetArea([u8; 16]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-tcp")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "get-receive-buffer-size")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-tcp_get-receive-buffer-size"
                )]
                fn wit_import(_: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(connection), ptr0);
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok(*((ptr0 + 8) as *const i64) as u64),
                1 => Err(match i32::from(*((ptr0 + 8) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Addrinuse,
                    2 => Errno::Addrnotavail,
                    3 => Errno::Afnosupport,
                    4 => Errno::Again,
                    5 => Errno::Already,
                    6 => Errno::Badf,
                    7 => Errno::Busy,
                    8 => Errno::ConnectionAborted,
                    9 => Errno::ConnectionRefused,
                    10 => Errno::ConnectionReset,
                    11 => Errno::Deadlock,
                    12 => Errno::Destaddrreq,
                    13 => Errno::HostUnreachable,
                    14 => Errno::Ilseq,
                    15 => Errno::Inprogress,
                    16 => Errno::Intr,
                    17 => Errno::Inval,
                    18 => Errno::Io,
                    19 => Errno::Isconn,
                    20 => Errno::Msgsize,
                    21 => Errno::Multihop,
                    22 => Errno::Nametoolong,
                    23 => Errno::NetworkDown,
                    24 => Errno::NetworkReset,
                    25 => Errno::NetworkUnreachable,
                    26 => Errno::Nobufs,
                    27 => Errno::Noent,
                    28 => Errno::Nomem,
                    29 => Errno::Noprotoopt,
                    30 => Errno::Nosys,
                    31 => Errno::Notrecoverable,
                    32 => Errno::Notsup,
                    33 => Errno::Overflow,
                    34 => Errno::Perm,
                    35 => Errno::Timedout,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Gets the receive-buffer size.
    ///
    /// Note: this is only a hint. Implementations may internally handle this
    /// in any way, including ignoring it.
    ///
    /// Equivalent to `SO_RCVBUF`.
    pub fn set_receive_buffer_size(connection: Connection, value: IoSize) -> Result<(), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-tcp")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "set-receive-buffer-size")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-tcp_set-receive-buffer-size"
                )]
                fn wit_import(_: i32, _: i64, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(connection),
                wit_bindgen_guest_rust::rt::as_i64(value),
                ptr0,
            );
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok(()),
                1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Addrinuse,
                    2 => Errno::Addrnotavail,
                    3 => Errno::Afnosupport,
                    4 => Errno::Again,
                    5 => Errno::Already,
                    6 => Errno::Badf,
                    7 => Errno::Busy,
                    8 => Errno::ConnectionAborted,
                    9 => Errno::ConnectionRefused,
                    10 => Errno::ConnectionReset,
                    11 => Errno::Deadlock,
                    12 => Errno::Destaddrreq,
                    13 => Errno::HostUnreachable,
                    14 => Errno::Ilseq,
                    15 => Errno::Inprogress,
                    16 => Errno::Intr,
                    17 => Errno::Inval,
                    18 => Errno::Io,
                    19 => Errno::Isconn,
                    20 => Errno::Msgsize,
                    21 => Errno::Multihop,
                    22 => Errno::Nametoolong,
                    23 => Errno::NetworkDown,
                    24 => Errno::NetworkReset,
                    25 => Errno::NetworkUnreachable,
                    26 => Errno::Nobufs,
                    27 => Errno::Noent,
                    28 => Errno::Nomem,
                    29 => Errno::Noprotoopt,
                    30 => Errno::Nosys,
                    31 => Errno::Notrecoverable,
                    32 => Errno::Notsup,
                    33 => Errno::Overflow,
                    34 => Errno::Perm,
                    35 => Errno::Timedout,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Gets the send-buffer size.
    ///
    /// Note: this is only a hint. Implementations may internally handle this
    /// in any way, including ignoring it.
    ///
    /// Equivalent to `SO_SNDBUF`.
    pub fn get_send_buffer_size(connection: Connection) -> Result<IoSize, Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(8))]
            struct RetArea([u8; 16]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-tcp")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "get-send-buffer-size")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-tcp_get-send-buffer-size"
                )]
                fn wit_import(_: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(connection), ptr0);
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok(*((ptr0 + 8) as *const i64) as u64),
                1 => Err(match i32::from(*((ptr0 + 8) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Addrinuse,
                    2 => Errno::Addrnotavail,
                    3 => Errno::Afnosupport,
                    4 => Errno::Again,
                    5 => Errno::Already,
                    6 => Errno::Badf,
                    7 => Errno::Busy,
                    8 => Errno::ConnectionAborted,
                    9 => Errno::ConnectionRefused,
                    10 => Errno::ConnectionReset,
                    11 => Errno::Deadlock,
                    12 => Errno::Destaddrreq,
                    13 => Errno::HostUnreachable,
                    14 => Errno::Ilseq,
                    15 => Errno::Inprogress,
                    16 => Errno::Intr,
                    17 => Errno::Inval,
                    18 => Errno::Io,
                    19 => Errno::Isconn,
                    20 => Errno::Msgsize,
                    21 => Errno::Multihop,
                    22 => Errno::Nametoolong,
                    23 => Errno::NetworkDown,
                    24 => Errno::NetworkReset,
                    25 => Errno::NetworkUnreachable,
                    26 => Errno::Nobufs,
                    27 => Errno::Noent,
                    28 => Errno::Nomem,
                    29 => Errno::Noprotoopt,
                    30 => Errno::Nosys,
                    31 => Errno::Notrecoverable,
                    32 => Errno::Notsup,
                    33 => Errno::Overflow,
                    34 => Errno::Perm,
                    35 => Errno::Timedout,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Sets the send-buffer size.
    ///
    /// Note: this is only a hint. Implementations may internally handle this
    /// in any way, including ignoring it.
    ///
    /// Equivalent to `SO_SNDBUF`.
    pub fn set_send_buffer_size(connection: Connection, value: IoSize) -> Result<(), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-tcp")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "set-send-buffer-size")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "wasi-tcp_set-send-buffer-size"
                )]
                fn wit_import(_: i32, _: i64, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(connection),
                wit_bindgen_guest_rust::rt::as_i64(value),
                ptr0,
            );
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok(()),
                1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Addrinuse,
                    2 => Errno::Addrnotavail,
                    3 => Errno::Afnosupport,
                    4 => Errno::Again,
                    5 => Errno::Already,
                    6 => Errno::Badf,
                    7 => Errno::Busy,
                    8 => Errno::ConnectionAborted,
                    9 => Errno::ConnectionRefused,
                    10 => Errno::ConnectionReset,
                    11 => Errno::Deadlock,
                    12 => Errno::Destaddrreq,
                    13 => Errno::HostUnreachable,
                    14 => Errno::Ilseq,
                    15 => Errno::Inprogress,
                    16 => Errno::Intr,
                    17 => Errno::Inval,
                    18 => Errno::Io,
                    19 => Errno::Isconn,
                    20 => Errno::Msgsize,
                    21 => Errno::Multihop,
                    22 => Errno::Nametoolong,
                    23 => Errno::NetworkDown,
                    24 => Errno::NetworkReset,
                    25 => Errno::NetworkUnreachable,
                    26 => Errno::Nobufs,
                    27 => Errno::Noent,
                    28 => Errno::Nomem,
                    29 => Errno::Noprotoopt,
                    30 => Errno::Nosys,
                    31 => Errno::Notrecoverable,
                    32 => Errno::Notsup,
                    33 => Errno::Overflow,
                    34 => Errno::Perm,
                    35 => Errno::Timedout,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Query the specified `socket` for how many bytes are available to read.
    pub fn bytes_readable(s: Connection) -> Result<(IoSize, bool), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(8))]
            struct RetArea([u8; 24]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-tcp")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "bytes-readable")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_bytes-readable")]
                fn wit_import(_: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(s), ptr0);
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok((
                    *((ptr0 + 8) as *const i64) as u64,
                    match i32::from(*((ptr0 + 16) as *const u8)) {
                        0 => false,
                        1 => true,
                        _ => panic!("invalid bool discriminant"),
                    },
                )),
                1 => Err(match i32::from(*((ptr0 + 8) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Addrinuse,
                    2 => Errno::Addrnotavail,
                    3 => Errno::Afnosupport,
                    4 => Errno::Again,
                    5 => Errno::Already,
                    6 => Errno::Badf,
                    7 => Errno::Busy,
                    8 => Errno::ConnectionAborted,
                    9 => Errno::ConnectionRefused,
                    10 => Errno::ConnectionReset,
                    11 => Errno::Deadlock,
                    12 => Errno::Destaddrreq,
                    13 => Errno::HostUnreachable,
                    14 => Errno::Ilseq,
                    15 => Errno::Inprogress,
                    16 => Errno::Intr,
                    17 => Errno::Inval,
                    18 => Errno::Io,
                    19 => Errno::Isconn,
                    20 => Errno::Msgsize,
                    21 => Errno::Multihop,
                    22 => Errno::Nametoolong,
                    23 => Errno::NetworkDown,
                    24 => Errno::NetworkReset,
                    25 => Errno::NetworkUnreachable,
                    26 => Errno::Nobufs,
                    27 => Errno::Noent,
                    28 => Errno::Nomem,
                    29 => Errno::Noprotoopt,
                    30 => Errno::Nosys,
                    31 => Errno::Notrecoverable,
                    32 => Errno::Notsup,
                    33 => Errno::Overflow,
                    34 => Errno::Perm,
                    35 => Errno::Timedout,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Query the specified `socket` for the number of bytes ready to be accepted.
    pub fn bytes_writable(s: Connection) -> Result<(IoSize, bool), Errno> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(8))]
            struct RetArea([u8; 24]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-tcp")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "bytes-writable")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_bytes-writable")]
                fn wit_import(_: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(s), ptr0);
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok((
                    *((ptr0 + 8) as *const i64) as u64,
                    match i32::from(*((ptr0 + 16) as *const u8)) {
                        0 => false,
                        1 => true,
                        _ => panic!("invalid bool discriminant"),
                    },
                )),
                1 => Err(match i32::from(*((ptr0 + 8) as *const u8)) {
                    0 => Errno::Access,
                    1 => Errno::Addrinuse,
                    2 => Errno::Addrnotavail,
                    3 => Errno::Afnosupport,
                    4 => Errno::Again,
                    5 => Errno::Already,
                    6 => Errno::Badf,
                    7 => Errno::Busy,
                    8 => Errno::ConnectionAborted,
                    9 => Errno::ConnectionRefused,
                    10 => Errno::ConnectionReset,
                    11 => Errno::Deadlock,
                    12 => Errno::Destaddrreq,
                    13 => Errno::HostUnreachable,
                    14 => Errno::Ilseq,
                    15 => Errno::Inprogress,
                    16 => Errno::Intr,
                    17 => Errno::Inval,
                    18 => Errno::Io,
                    19 => Errno::Isconn,
                    20 => Errno::Msgsize,
                    21 => Errno::Multihop,
                    22 => Errno::Nametoolong,
                    23 => Errno::NetworkDown,
                    24 => Errno::NetworkReset,
                    25 => Errno::NetworkUnreachable,
                    26 => Errno::Nobufs,
                    27 => Errno::Noent,
                    28 => Errno::Nomem,
                    29 => Errno::Noprotoopt,
                    30 => Errno::Nosys,
                    31 => Errno::Notrecoverable,
                    32 => Errno::Notsup,
                    33 => Errno::Overflow,
                    34 => Errno::Perm,
                    35 => Errno::Timedout,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Test whether a connection is connected.
    ///
    /// In POSIX, this is typically done using `getpeername`.
    pub fn is_connected(connection: Connection) -> bool {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[link(wasm_import_module = "wasi-tcp")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "is-connected")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_is-connected")]
                fn wit_import(_: i32) -> i32;
            }
            let ret = wit_import(wit_bindgen_guest_rust::rt::as_i32(connection));
            match ret {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Closes a handle returned by `listen`.
    pub fn close_tcp_listener(listener: TcpListener) -> () {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[link(wasm_import_module = "wasi-tcp")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "close-tcp-listener")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_close-tcp-listener")]
                fn wit_import(_: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(listener));
        }
    }
    #[allow(clippy::all)]
    /// Closes a handle returned by `connect`, `accept`, or `accept-tcp`.
    pub fn close_connection(listener: TcpListener) -> () {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[link(wasm_import_module = "wasi-tcp")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "close-connection")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_close-connection")]
                fn wit_import(_: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(listener));
        }
    }
}

#[allow(clippy::all)]
pub mod wasi_ip {
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum IpAddressFamily {
        /// Similar to `AF_INET` in POSIX.
        Ipv4,
        /// Similar to `AF_INET6` in POSIX.
        Ipv6,
    }
    impl core::fmt::Debug for IpAddressFamily {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                IpAddressFamily::Ipv4 => f.debug_tuple("IpAddressFamily::Ipv4").finish(),
                IpAddressFamily::Ipv6 => f.debug_tuple("IpAddressFamily::Ipv6").finish(),
            }
        }
    }
}

#[allow(clippy::all)]
pub mod wasi_dns {
    wit_bindgen_guest_rust::bitflags::bitflags! {
      /// Resolution flags.
      pub struct ResolverFlags: u8 {
        /// Equivalent to `O_NONBLOCK`.
        const NONBLOCK = 1 << 0;
      }
    }
    impl ResolverFlags {
        /// Convert from a raw integer, preserving any unknown bits. See
        /// <https://github.com/bitflags/bitflags/issues/263#issuecomment-957088321>
        pub fn from_bits_preserve(bits: u8) -> Self {
            Self { bits }
        }
    }
    /// An iterator over resolution results.
    ///
    /// In the future, this will be replaced by handle types.
    pub type Resolver = u32;
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum ResolveNameError {
        InvalidName,
    }
    impl ResolveNameError {
        pub fn name(&self) -> &'static str {
            match self {
                ResolveNameError::InvalidName => "invalid-name",
            }
        }
        pub fn message(&self) -> &'static str {
            match self {
                ResolveNameError::InvalidName => "",
            }
        }
    }
    impl core::fmt::Debug for ResolveNameError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("ResolveNameError")
                .field("code", &(*self as i32))
                .field("name", &self.name())
                .field("message", &self.message())
                .finish()
        }
    }
    impl core::fmt::Display for ResolveNameError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} (error {})", self.name(), *self as i32)
        }
    }

    impl std::error::Error for ResolveNameError {}
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum ResolveError {
        /// The resolve is in non-blocking mode and the request would block.
        WouldBlock,
        /// The DNS resolver is unable to provide results.
        DnsUnavailable,
    }
    impl ResolveError {
        pub fn name(&self) -> &'static str {
            match self {
                ResolveError::WouldBlock => "would-block",
                ResolveError::DnsUnavailable => "dns-unavailable",
            }
        }
        pub fn message(&self) -> &'static str {
            match self {
                ResolveError::WouldBlock => {
                    "The resolve is in non-blocking mode and the request would block."
                }
                ResolveError::DnsUnavailable => "The DNS resolver is unable to provide results.",
            }
        }
    }
    impl core::fmt::Debug for ResolveError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("ResolveError")
                .field("code", &(*self as i32))
                .field("name", &self.name())
                .field("message", &self.message())
                .finish()
        }
    }
    impl core::fmt::Display for ResolveError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} (error {})", self.name(), *self as i32)
        }
    }

    impl std::error::Error for ResolveError {}
    pub type Network = u32;
    pub type Ipv6Address = (u16, u16, u16, u16, u16, u16, u16, u16);
    pub type Ipv4Address = (u8, u8, u8, u8);
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum IpAddressFamily {
        /// Similar to `AF_INET` in POSIX.
        Ipv4,
        /// Similar to `AF_INET6` in POSIX.
        Ipv6,
    }
    impl core::fmt::Debug for IpAddressFamily {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                IpAddressFamily::Ipv4 => f.debug_tuple("IpAddressFamily::Ipv4").finish(),
                IpAddressFamily::Ipv6 => f.debug_tuple("IpAddressFamily::Ipv6").finish(),
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum IpAddress {
        Ipv4(Ipv4Address),
        Ipv6(Ipv6Address),
    }
    impl core::fmt::Debug for IpAddress {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                IpAddress::Ipv4(e) => f.debug_tuple("IpAddress::Ipv4").field(e).finish(),
                IpAddress::Ipv6(e) => f.debug_tuple("IpAddress::Ipv6").field(e).finish(),
            }
        }
    }
    #[allow(clippy::all)]
    /// Starts resolving an internet host name to a list of IP addresses.
    ///
    /// This function returns a new resolver on success or an error if
    /// immediately available. For example, this function fails with
    /// `invalid-name` when `name` is:
    /// - empty
    /// - an IP address
    /// - a syntactically invalid domain name in another way
    pub fn resolve_name(
        network: Network,
        name: &str,
        address_family: Option<IpAddressFamily>,
        flags: ResolverFlags,
    ) -> Result<Resolver, ResolveNameError> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(4))]
            struct RetArea([u8; 8]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let vec0 = name;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            let (result1_0, result1_1) = match address_family {
                Some(e) => (
                    1i32,
                    match e {
                        IpAddressFamily::Ipv4 => 0,
                        IpAddressFamily::Ipv6 => 1,
                    },
                ),
                None => (0i32, 0i32),
            };
            let flags2 = flags;
            let ptr3 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-dns")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "resolve-name")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-dns_resolve-name")]
                fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32);
            }
            wit_import(
                wit_bindgen_guest_rust::rt::as_i32(network),
                ptr0,
                len0,
                result1_0,
                result1_1,
                (flags2.bits() >> 0) as i32,
                ptr3,
            );
            match i32::from(*((ptr3 + 0) as *const u8)) {
                0 => Ok(*((ptr3 + 4) as *const i32) as u32),
                1 => Err(match i32::from(*((ptr3 + 4) as *const u8)) {
                    0 => ResolveNameError::InvalidName,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Get the next address from the resolver.
    ///
    /// This function should be called multiple times. On each call, it will
    /// return the next address in connection order preference. If all
    /// addresses have been exhausted, this function returns `none`. If
    /// non-blocking mode is used, this function may return `errno::again`
    /// indicating that the caller should poll for incoming data.
    /// This function never returns IPv4-mapped IPv6 addresses.
    pub fn resolve_next(resolver: Resolver) -> Result<Option<IpAddress>, ResolveError> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(2))]
            struct RetArea([u8; 22]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "wasi-dns")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "resolve-next")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-dns_resolve-next")]
                fn wit_import(_: i32, _: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(resolver), ptr0);
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok(match i32::from(*((ptr0 + 2) as *const u8)) {
                    0 => None,
                    1 => Some(match i32::from(*((ptr0 + 4) as *const u8)) {
                        0 => IpAddress::Ipv4((
                            i32::from(*((ptr0 + 6) as *const u8)) as u8,
                            i32::from(*((ptr0 + 7) as *const u8)) as u8,
                            i32::from(*((ptr0 + 8) as *const u8)) as u8,
                            i32::from(*((ptr0 + 9) as *const u8)) as u8,
                        )),
                        1 => IpAddress::Ipv6((
                            i32::from(*((ptr0 + 6) as *const u16)) as u16,
                            i32::from(*((ptr0 + 8) as *const u16)) as u16,
                            i32::from(*((ptr0 + 10) as *const u16)) as u16,
                            i32::from(*((ptr0 + 12) as *const u16)) as u16,
                            i32::from(*((ptr0 + 14) as *const u16)) as u16,
                            i32::from(*((ptr0 + 16) as *const u16)) as u16,
                            i32::from(*((ptr0 + 18) as *const u16)) as u16,
                            i32::from(*((ptr0 + 20) as *const u16)) as u16,
                        )),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    _ => panic!("invalid enum discriminant"),
                }),
                1 => Err(match i32::from(*((ptr0 + 2) as *const u8)) {
                    0 => ResolveError::WouldBlock,
                    1 => ResolveError::DnsUnavailable,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Closes a handle returned by `resolve-name`.
    pub fn close_resolver(resolver: Resolver) -> () {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[link(wasm_import_module = "wasi-dns")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "close-resolver")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-dns_close-resolver")]
                fn wit_import(_: i32);
            }
            wit_import(wit_bindgen_guest_rust::rt::as_i32(resolver));
        }
    }
}

#[allow(clippy::all)]
pub mod wasi_exit {
    #[allow(clippy::all)]
    /// Exit the curerent instance and any linked instances.
    pub fn exit(status: Result<(), ()>) -> () {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            let result0 = match status {
                Ok(_) => 0i32,
                Err(_) => 1i32,
            };
            #[link(wasm_import_module = "wasi-exit")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "exit")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-exit_exit")]
                fn wit_import(_: i32);
            }
            wit_import(result0);
        }
    }
}

pub trait WasiCommand {
    fn command(
        stdin: u32,
        stdout: u32,
        args: wit_bindgen_guest_rust::rt::vec::Vec<wit_bindgen_guest_rust::rt::string::String>,
        env_vars: wit_bindgen_guest_rust::rt::vec::Vec<(
            wit_bindgen_guest_rust::rt::string::String,
            wit_bindgen_guest_rust::rt::string::String,
        )>,
        preopens: wit_bindgen_guest_rust::rt::vec::Vec<(
            u32,
            wit_bindgen_guest_rust::rt::string::String,
        )>,
    ) -> Result<(), ()>;
}

#[doc(hidden)]
pub unsafe fn call_command<T: WasiCommand>(
    arg0: i32,
    arg1: i32,
    arg2: i32,
    arg3: i32,
    arg4: i32,
    arg5: i32,
    arg6: i32,
    arg7: i32,
) -> i32 {
    #[allow(unused_imports)]
    use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
    let base1 = arg2;
    let len1 = arg3;
    let mut result1 = Vec::with_capacity(len1 as usize);
    for i in 0..len1 {
        let base = base1 + i * 8;
        result1.push({
            let len0 = *((base + 4) as *const i32) as usize;

            String::from_utf8(Vec::from_raw_parts(
                *((base + 0) as *const i32) as *mut _,
                len0,
                len0,
            ))
            .unwrap()
        });
    }
    wit_bindgen_guest_rust::rt::dealloc(base1, (len1 as usize) * 8, 4);
    let base4 = arg4;
    let len4 = arg5;
    let mut result4 = Vec::with_capacity(len4 as usize);
    for i in 0..len4 {
        let base = base4 + i * 16;
        result4.push({
            let len2 = *((base + 4) as *const i32) as usize;
            let len3 = *((base + 12) as *const i32) as usize;

            (
                String::from_utf8(Vec::from_raw_parts(
                    *((base + 0) as *const i32) as *mut _,
                    len2,
                    len2,
                ))
                .unwrap(),
                String::from_utf8(Vec::from_raw_parts(
                    *((base + 8) as *const i32) as *mut _,
                    len3,
                    len3,
                ))
                .unwrap(),
            )
        });
    }
    wit_bindgen_guest_rust::rt::dealloc(base4, (len4 as usize) * 16, 4);
    let base6 = arg6;
    let len6 = arg7;
    let mut result6 = Vec::with_capacity(len6 as usize);
    for i in 0..len6 {
        let base = base6 + i * 12;
        result6.push({
            let len5 = *((base + 8) as *const i32) as usize;

            (
                *((base + 0) as *const i32) as u32,
                String::from_utf8(Vec::from_raw_parts(
                    *((base + 4) as *const i32) as *mut _,
                    len5,
                    len5,
                ))
                .unwrap(),
            )
        });
    }
    wit_bindgen_guest_rust::rt::dealloc(base6, (len6 as usize) * 12, 4);
    let result7 = T::command(arg0 as u32, arg1 as u32, result1, result4, result6);
    let result8 = match result7 {
        Ok(_) => 0i32,
        Err(_) => 1i32,
    };
    result8
}

/// Declares the export of the component's world for the
/// given type.
#[macro_export]
macro_rules! export_wasi_command(($t:ident) => {
          const _: () = {

            #[doc(hidden)]
            #[export_name = "command"]
            #[allow(non_snake_case)]
            unsafe extern "C" fn __export_wasi_command_command(arg0: i32,arg1: i32,arg2: i32,arg3: i32,arg4: i32,arg5: i32,arg6: i32,arg7: i32,) -> i32 {
              call_command::<$t>(arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,)
            }

          };

          #[used]
          #[doc(hidden)]
          #[cfg(target_arch = "wasm32")]
          static __FORCE_SECTION_REF: fn() = __force_section_ref;
          #[doc(hidden)]
          #[cfg(target_arch = "wasm32")]
          fn __force_section_ref() {
            __link_section()
          }
        });

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wasi-command"]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 21145] = [
    2, 0, 3, 119, 105, 116, 12, 119, 97, 115, 105, 45, 99, 111, 109, 109, 97, 110, 100, 12, 119,
    97, 115, 105, 45, 99, 111, 109, 109, 97, 110, 100, 0, 97, 115, 109, 12, 0, 1, 0, 7, 119, 1, 65,
    2, 1, 66, 7, 1, 64, 1, 7, 109, 101, 115, 115, 97, 103, 101, 115, 1, 0, 4, 5, 112, 114, 105,
    110, 116, 0, 1, 0, 1, 64, 0, 0, 127, 4, 11, 105, 115, 45, 116, 101, 114, 109, 105, 110, 97,
    108, 0, 1, 1, 1, 107, 123, 1, 64, 0, 0, 2, 4, 11, 110, 117, 109, 45, 99, 111, 108, 117, 109,
    110, 115, 0, 1, 3, 4, 11, 119, 97, 115, 105, 45, 115, 116, 100, 101, 114, 114, 28, 112, 107,
    103, 58, 47, 119, 97, 115, 105, 45, 115, 116, 100, 101, 114, 114, 47, 119, 97, 115, 105, 45,
    115, 116, 100, 101, 114, 114, 5, 0, 11, 33, 1, 11, 119, 97, 115, 105, 45, 115, 116, 100, 101,
    114, 114, 16, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 115, 116, 100, 101, 114, 114, 3, 0,
    0, 7, 108, 1, 65, 2, 1, 66, 5, 1, 112, 125, 1, 64, 1, 3, 108, 101, 110, 121, 0, 0, 4, 16, 103,
    101, 116, 45, 114, 97, 110, 100, 111, 109, 45, 98, 121, 116, 101, 115, 0, 1, 1, 1, 64, 0, 0,
    119, 4, 14, 103, 101, 116, 45, 114, 97, 110, 100, 111, 109, 45, 117, 54, 52, 0, 1, 2, 4, 11,
    119, 97, 115, 105, 45, 114, 97, 110, 100, 111, 109, 28, 112, 107, 103, 58, 47, 119, 97, 115,
    105, 45, 114, 97, 110, 100, 111, 109, 47, 119, 97, 115, 105, 45, 114, 97, 110, 100, 111, 109,
    5, 0, 11, 33, 1, 11, 119, 97, 115, 105, 45, 114, 97, 110, 100, 111, 109, 16, 112, 107, 103, 58,
    47, 119, 97, 115, 105, 45, 114, 97, 110, 100, 111, 109, 3, 2, 0, 7, 56, 1, 65, 2, 1, 66, 2, 1,
    121, 4, 7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 0, 4, 8, 119, 97, 115, 105, 45, 110,
    101, 116, 22, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 110, 101, 116, 47, 119, 97, 115,
    105, 45, 110, 101, 116, 5, 0, 11, 27, 1, 8, 119, 97, 115, 105, 45, 110, 101, 116, 13, 112, 107,
    103, 58, 47, 119, 97, 115, 105, 45, 110, 101, 116, 3, 4, 0, 7, 133, 1, 1, 65, 2, 1, 66, 4, 1,
    109, 5, 5, 116, 114, 97, 99, 101, 5, 100, 101, 98, 117, 103, 4, 105, 110, 102, 111, 4, 119, 97,
    114, 110, 5, 101, 114, 114, 111, 114, 4, 5, 108, 101, 118, 101, 108, 0, 3, 0, 0, 1, 64, 3, 5,
    108, 101, 118, 101, 108, 1, 7, 99, 111, 110, 116, 101, 120, 116, 115, 7, 109, 101, 115, 115,
    97, 103, 101, 115, 1, 0, 4, 3, 108, 111, 103, 0, 1, 2, 4, 12, 119, 97, 115, 105, 45, 108, 111,
    103, 103, 105, 110, 103, 30, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 108, 111, 103, 103,
    105, 110, 103, 47, 119, 97, 115, 105, 45, 108, 111, 103, 103, 105, 110, 103, 5, 0, 11, 35, 1,
    12, 119, 97, 115, 105, 45, 108, 111, 103, 103, 105, 110, 103, 17, 112, 107, 103, 58, 47, 119,
    97, 115, 105, 45, 108, 111, 103, 103, 105, 110, 103, 3, 6, 0, 7, 184, 2, 1, 65, 2, 1, 66, 14,
    1, 111, 8, 123, 123, 123, 123, 123, 123, 123, 123, 4, 12, 105, 112, 118, 54, 45, 97, 100, 100,
    114, 101, 115, 115, 0, 3, 0, 0, 1, 114, 4, 7, 97, 100, 100, 114, 101, 115, 115, 1, 4, 112, 111,
    114, 116, 123, 9, 102, 108, 111, 119, 45, 105, 110, 102, 111, 121, 8, 115, 99, 111, 112, 101,
    45, 105, 100, 121, 4, 19, 105, 112, 118, 54, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100,
    114, 101, 115, 115, 0, 3, 0, 2, 1, 111, 4, 125, 125, 125, 125, 4, 12, 105, 112, 118, 52, 45,
    97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 4, 1, 114, 2, 7, 97, 100, 100, 114, 101, 115, 115,
    5, 4, 112, 111, 114, 116, 123, 4, 19, 105, 112, 118, 52, 45, 115, 111, 99, 107, 101, 116, 45,
    97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 6, 1, 113, 2, 4, 105, 112, 118, 52, 1, 7, 0, 4, 105,
    112, 118, 54, 1, 3, 0, 4, 17, 105, 112, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114,
    101, 115, 115, 0, 3, 0, 8, 1, 109, 2, 4, 105, 112, 118, 52, 4, 105, 112, 118, 54, 4, 17, 105,
    112, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 3, 0, 10, 1,
    113, 2, 4, 105, 112, 118, 52, 1, 5, 0, 4, 105, 112, 118, 54, 1, 1, 0, 4, 10, 105, 112, 45, 97,
    100, 100, 114, 101, 115, 115, 0, 3, 0, 12, 4, 7, 119, 97, 115, 105, 45, 105, 112, 20, 112, 107,
    103, 58, 47, 119, 97, 115, 105, 45, 105, 112, 47, 119, 97, 115, 105, 45, 105, 112, 5, 0, 11,
    25, 1, 7, 119, 97, 115, 105, 45, 105, 112, 12, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45,
    105, 112, 3, 8, 0, 7, 235, 2, 1, 65, 2, 1, 66, 28, 1, 114, 0, 4, 12, 115, 116, 114, 101, 97,
    109, 45, 101, 114, 114, 111, 114, 0, 3, 0, 0, 1, 121, 4, 13, 111, 117, 116, 112, 117, 116, 45,
    115, 116, 114, 101, 97, 109, 0, 3, 0, 2, 1, 121, 4, 12, 105, 110, 112, 117, 116, 45, 115, 116,
    114, 101, 97, 109, 0, 3, 0, 4, 1, 112, 125, 1, 111, 2, 6, 127, 1, 106, 1, 7, 1, 1, 1, 64, 2, 3,
    115, 114, 99, 5, 3, 108, 101, 110, 119, 0, 8, 4, 4, 114, 101, 97, 100, 0, 1, 9, 1, 111, 2, 119,
    127, 1, 106, 1, 10, 1, 1, 1, 64, 2, 3, 115, 114, 99, 5, 3, 108, 101, 110, 119, 0, 11, 4, 4,
    115, 107, 105, 112, 0, 1, 12, 1, 106, 1, 119, 1, 1, 1, 64, 2, 3, 100, 115, 116, 3, 3, 98, 117,
    102, 6, 0, 13, 4, 5, 119, 114, 105, 116, 101, 0, 1, 14, 1, 64, 3, 3, 100, 115, 116, 3, 4, 98,
    121, 116, 101, 125, 3, 108, 101, 110, 119, 0, 13, 4, 14, 119, 114, 105, 116, 101, 45, 114, 101,
    112, 101, 97, 116, 101, 100, 0, 1, 15, 1, 64, 3, 3, 100, 115, 116, 3, 3, 115, 114, 99, 5, 3,
    108, 101, 110, 119, 0, 11, 4, 6, 115, 112, 108, 105, 99, 101, 0, 1, 16, 1, 64, 2, 3, 100, 115,
    116, 3, 3, 115, 114, 99, 5, 0, 13, 4, 7, 102, 111, 114, 119, 97, 114, 100, 0, 1, 17, 1, 64, 1,
    1, 102, 5, 1, 0, 4, 17, 100, 114, 111, 112, 45, 105, 110, 112, 117, 116, 45, 115, 116, 114,
    101, 97, 109, 0, 1, 18, 1, 64, 1, 1, 102, 3, 1, 0, 4, 18, 100, 114, 111, 112, 45, 111, 117,
    116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 1, 19, 4, 7, 119, 97, 115, 105, 45,
    105, 111, 20, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 105, 111, 47, 119, 97, 115, 105,
    45, 105, 111, 5, 0, 11, 25, 1, 7, 119, 97, 115, 105, 45, 105, 111, 12, 112, 107, 103, 58, 47,
    119, 97, 115, 105, 45, 105, 111, 3, 10, 0, 7, 70, 1, 65, 2, 1, 66, 3, 1, 106, 0, 0, 1, 64, 1,
    6, 115, 116, 97, 116, 117, 115, 0, 1, 0, 4, 4, 101, 120, 105, 116, 0, 1, 1, 4, 9, 119, 97, 115,
    105, 45, 101, 120, 105, 116, 24, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 101, 120, 105,
    116, 47, 119, 97, 115, 105, 45, 101, 120, 105, 116, 5, 0, 11, 29, 1, 9, 119, 97, 115, 105, 45,
    101, 120, 105, 116, 14, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 101, 120, 105, 116, 3,
    12, 0, 7, 207, 3, 1, 65, 2, 1, 66, 28, 1, 110, 1, 8, 110, 111, 110, 98, 108, 111, 99, 107, 4,
    14, 114, 101, 115, 111, 108, 118, 101, 114, 45, 102, 108, 97, 103, 115, 0, 3, 0, 0, 1, 121, 4,
    8, 114, 101, 115, 111, 108, 118, 101, 114, 0, 3, 0, 2, 1, 109, 1, 12, 105, 110, 118, 97, 108,
    105, 100, 45, 110, 97, 109, 101, 4, 18, 114, 101, 115, 111, 108, 118, 101, 45, 110, 97, 109,
    101, 45, 101, 114, 114, 111, 114, 0, 3, 0, 4, 1, 109, 2, 11, 119, 111, 117, 108, 100, 45, 98,
    108, 111, 99, 107, 15, 100, 110, 115, 45, 117, 110, 97, 118, 97, 105, 108, 97, 98, 108, 101, 4,
    13, 114, 101, 115, 111, 108, 118, 101, 45, 101, 114, 114, 111, 114, 0, 3, 0, 6, 1, 121, 4, 7,
    110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 8, 1, 111, 8, 123, 123, 123, 123, 123, 123, 123,
    123, 4, 12, 105, 112, 118, 54, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 10, 1, 111, 4,
    125, 125, 125, 125, 4, 12, 105, 112, 118, 52, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0,
    12, 1, 109, 2, 4, 105, 112, 118, 52, 4, 105, 112, 118, 54, 4, 17, 105, 112, 45, 97, 100, 100,
    114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 3, 0, 14, 1, 113, 2, 4, 105, 112, 118,
    52, 1, 13, 0, 4, 105, 112, 118, 54, 1, 11, 0, 4, 10, 105, 112, 45, 97, 100, 100, 114, 101, 115,
    115, 0, 3, 0, 16, 1, 107, 15, 1, 106, 1, 3, 1, 5, 1, 64, 4, 7, 110, 101, 116, 119, 111, 114,
    107, 9, 4, 110, 97, 109, 101, 115, 14, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105,
    108, 121, 18, 5, 102, 108, 97, 103, 115, 1, 0, 19, 4, 12, 114, 101, 115, 111, 108, 118, 101,
    45, 110, 97, 109, 101, 0, 1, 20, 1, 107, 17, 1, 106, 1, 21, 1, 7, 1, 64, 1, 8, 114, 101, 115,
    111, 108, 118, 101, 114, 3, 0, 22, 4, 12, 114, 101, 115, 111, 108, 118, 101, 45, 110, 101, 120,
    116, 0, 1, 23, 1, 64, 1, 8, 114, 101, 115, 111, 108, 118, 101, 114, 3, 1, 0, 4, 14, 99, 108,
    111, 115, 101, 45, 114, 101, 115, 111, 108, 118, 101, 114, 0, 1, 24, 4, 8, 119, 97, 115, 105,
    45, 100, 110, 115, 22, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 100, 110, 115, 47, 119,
    97, 115, 105, 45, 100, 110, 115, 5, 0, 11, 27, 1, 8, 119, 97, 115, 105, 45, 100, 110, 115, 13,
    112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 100, 110, 115, 3, 14, 0, 7, 178, 4, 1, 65, 6, 1,
    66, 20, 1, 121, 4, 10, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 3, 0, 0, 1, 121, 4, 8,
    112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 2, 1, 121, 4, 15, 109, 111, 110, 111, 116, 111,
    110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 3, 0, 4, 1, 119, 4, 7, 105, 110, 115, 116, 97, 110,
    116, 0, 3, 0, 6, 1, 114, 2, 7, 115, 101, 99, 111, 110, 100, 115, 119, 11, 110, 97, 110, 111,
    115, 101, 99, 111, 110, 100, 115, 121, 4, 8, 100, 97, 116, 101, 116, 105, 109, 101, 0, 3, 0, 8,
    1, 64, 1, 5, 99, 108, 111, 99, 107, 5, 0, 7, 4, 19, 109, 111, 110, 111, 116, 111, 110, 105, 99,
    45, 99, 108, 111, 99, 107, 45, 110, 111, 119, 0, 1, 10, 4, 26, 109, 111, 110, 111, 116, 111,
    110, 105, 99, 45, 99, 108, 111, 99, 107, 45, 114, 101, 115, 111, 108, 117, 116, 105, 111, 110,
    0, 1, 10, 1, 64, 1, 5, 99, 108, 111, 99, 107, 1, 0, 9, 4, 14, 119, 97, 108, 108, 45, 99, 108,
    111, 99, 107, 45, 110, 111, 119, 0, 1, 11, 4, 21, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107,
    45, 114, 101, 115, 111, 108, 117, 116, 105, 111, 110, 0, 1, 11, 1, 64, 1, 5, 99, 108, 111, 99,
    107, 5, 1, 0, 4, 21, 99, 108, 111, 115, 101, 45, 109, 111, 110, 111, 116, 111, 110, 105, 99,
    45, 99, 108, 111, 99, 107, 0, 1, 12, 1, 64, 1, 5, 99, 108, 111, 99, 107, 1, 1, 0, 4, 16, 99,
    108, 111, 115, 101, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 1, 13, 4, 11, 119, 97,
    115, 105, 45, 99, 108, 111, 99, 107, 115, 28, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 99,
    108, 111, 99, 107, 115, 47, 119, 97, 115, 105, 45, 99, 108, 111, 99, 107, 115, 5, 0, 2, 3, 0,
    0, 15, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 2, 3, 0, 0, 10,
    119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 1, 66, 8, 2, 3, 2, 1, 1, 4, 15, 109, 111, 110,
    111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 3, 0, 0, 2, 3, 2, 1, 2, 4, 10, 119,
    97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 3, 0, 2, 1, 64, 0, 0, 1, 4, 23, 100, 101, 102, 97,
    117, 108, 116, 45, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 1,
    4, 1, 64, 0, 0, 3, 4, 18, 100, 101, 102, 97, 117, 108, 116, 45, 119, 97, 108, 108, 45, 99, 108,
    111, 99, 107, 0, 1, 5, 4, 19, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 99,
    108, 111, 99, 107, 115, 36, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 99, 108, 111, 99,
    107, 115, 47, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 99, 108, 111, 99,
    107, 115, 5, 3, 11, 33, 1, 11, 119, 97, 115, 105, 45, 99, 108, 111, 99, 107, 115, 16, 112, 107,
    103, 58, 47, 119, 97, 115, 105, 45, 99, 108, 111, 99, 107, 115, 3, 16, 0, 7, 199, 5, 1, 65, 12,
    1, 66, 10, 1, 121, 4, 10, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 3, 0, 0, 1, 121, 4,
    8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 2, 1, 121, 4, 15, 109, 111, 110, 111, 116,
    111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 3, 0, 4, 1, 119, 4, 7, 105, 110, 115, 116, 97,
    110, 116, 0, 3, 0, 6, 1, 114, 2, 7, 115, 101, 99, 111, 110, 100, 115, 119, 11, 110, 97, 110,
    111, 115, 101, 99, 111, 110, 100, 115, 121, 4, 8, 100, 97, 116, 101, 116, 105, 109, 101, 0, 3,
    0, 8, 3, 11, 119, 97, 115, 105, 45, 99, 108, 111, 99, 107, 115, 28, 112, 107, 103, 58, 47, 119,
    97, 115, 105, 45, 99, 108, 111, 99, 107, 115, 47, 119, 97, 115, 105, 45, 99, 108, 111, 99, 107,
    115, 5, 0, 1, 66, 6, 1, 114, 0, 4, 12, 115, 116, 114, 101, 97, 109, 45, 101, 114, 114, 111,
    114, 0, 3, 0, 0, 1, 121, 4, 13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109,
    0, 3, 0, 2, 1, 121, 4, 12, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0,
    4, 3, 7, 119, 97, 115, 105, 45, 105, 111, 20, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45,
    105, 111, 47, 119, 97, 115, 105, 45, 105, 111, 5, 1, 2, 3, 0, 0, 10, 119, 97, 108, 108, 45, 99,
    108, 111, 99, 107, 2, 3, 0, 0, 15, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108,
    111, 99, 107, 2, 3, 0, 0, 8, 100, 97, 116, 101, 116, 105, 109, 101, 2, 3, 0, 0, 7, 105, 110,
    115, 116, 97, 110, 116, 2, 3, 0, 1, 12, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97,
    109, 2, 3, 0, 1, 13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 1, 66, 26,
    2, 3, 2, 1, 2, 4, 10, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 3, 0, 0, 2, 3, 2, 1, 3,
    4, 15, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 3, 0, 2, 2, 3,
    2, 1, 4, 4, 8, 100, 97, 116, 101, 116, 105, 109, 101, 0, 3, 0, 4, 2, 3, 2, 1, 5, 4, 7, 105,
    110, 115, 116, 97, 110, 116, 0, 3, 0, 6, 2, 3, 2, 1, 6, 4, 12, 105, 110, 112, 117, 116, 45,
    115, 116, 114, 101, 97, 109, 0, 3, 0, 8, 2, 3, 2, 1, 7, 4, 13, 111, 117, 116, 112, 117, 116,
    45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 10, 1, 121, 4, 8, 112, 111, 108, 108, 97, 98, 108,
    101, 0, 3, 0, 12, 1, 64, 1, 1, 102, 13, 1, 0, 4, 13, 100, 114, 111, 112, 45, 112, 111, 108,
    108, 97, 98, 108, 101, 0, 1, 14, 1, 64, 1, 1, 115, 9, 0, 13, 4, 14, 115, 117, 98, 115, 99, 114,
    105, 98, 101, 45, 114, 101, 97, 100, 0, 1, 15, 1, 64, 1, 1, 115, 11, 0, 13, 4, 15, 115, 117,
    98, 115, 99, 114, 105, 98, 101, 45, 119, 114, 105, 116, 101, 0, 1, 16, 1, 64, 3, 5, 99, 108,
    111, 99, 107, 3, 4, 119, 104, 101, 110, 7, 8, 97, 98, 115, 111, 108, 117, 116, 101, 127, 0, 13,
    4, 25, 115, 117, 98, 115, 99, 114, 105, 98, 101, 45, 109, 111, 110, 111, 116, 111, 110, 105,
    99, 45, 99, 108, 111, 99, 107, 0, 1, 17, 1, 112, 13, 1, 112, 125, 1, 64, 1, 2, 105, 110, 18, 0,
    19, 4, 11, 112, 111, 108, 108, 45, 111, 110, 101, 111, 102, 102, 0, 1, 20, 4, 9, 119, 97, 115,
    105, 45, 112, 111, 108, 108, 24, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 112, 111, 108,
    108, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 5, 8, 11, 29, 1, 9, 119, 97, 115, 105, 45,
    112, 111, 108, 108, 14, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 3,
    18, 0, 7, 230, 16, 1, 65, 18, 1, 66, 10, 1, 121, 4, 10, 119, 97, 108, 108, 45, 99, 108, 111,
    99, 107, 0, 3, 0, 0, 1, 121, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 2, 1, 121, 4,
    15, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 3, 0, 4, 1, 119,
    4, 7, 105, 110, 115, 116, 97, 110, 116, 0, 3, 0, 6, 1, 114, 2, 7, 115, 101, 99, 111, 110, 100,
    115, 119, 11, 110, 97, 110, 111, 115, 101, 99, 111, 110, 100, 115, 121, 4, 8, 100, 97, 116,
    101, 116, 105, 109, 101, 0, 3, 0, 8, 3, 11, 119, 97, 115, 105, 45, 99, 108, 111, 99, 107, 115,
    28, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 99, 108, 111, 99, 107, 115, 47, 119, 97, 115,
    105, 45, 99, 108, 111, 99, 107, 115, 5, 0, 1, 66, 6, 1, 114, 0, 4, 12, 115, 116, 114, 101, 97,
    109, 45, 101, 114, 114, 111, 114, 0, 3, 0, 0, 1, 121, 4, 13, 111, 117, 116, 112, 117, 116, 45,
    115, 116, 114, 101, 97, 109, 0, 3, 0, 2, 1, 121, 4, 12, 105, 110, 112, 117, 116, 45, 115, 116,
    114, 101, 97, 109, 0, 3, 0, 4, 3, 7, 119, 97, 115, 105, 45, 105, 111, 20, 112, 107, 103, 58,
    47, 119, 97, 115, 105, 45, 105, 111, 47, 119, 97, 115, 105, 45, 105, 111, 5, 1, 2, 3, 0, 0, 10,
    119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 2, 3, 0, 0, 15, 109, 111, 110, 111, 116, 111,
    110, 105, 99, 45, 99, 108, 111, 99, 107, 2, 3, 0, 0, 8, 100, 97, 116, 101, 116, 105, 109, 101,
    2, 3, 0, 0, 7, 105, 110, 115, 116, 97, 110, 116, 2, 3, 0, 1, 12, 105, 110, 112, 117, 116, 45,
    115, 116, 114, 101, 97, 109, 2, 3, 0, 1, 13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114,
    101, 97, 109, 1, 66, 14, 2, 3, 2, 1, 2, 4, 10, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 0,
    3, 0, 0, 2, 3, 2, 1, 3, 4, 15, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111,
    99, 107, 0, 3, 0, 2, 2, 3, 2, 1, 4, 4, 8, 100, 97, 116, 101, 116, 105, 109, 101, 0, 3, 0, 4, 2,
    3, 2, 1, 5, 4, 7, 105, 110, 115, 116, 97, 110, 116, 0, 3, 0, 6, 2, 3, 2, 1, 6, 4, 12, 105, 110,
    112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 8, 2, 3, 2, 1, 7, 4, 13, 111, 117,
    116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 10, 1, 121, 4, 8, 112, 111, 108,
    108, 97, 98, 108, 101, 0, 3, 0, 12, 3, 9, 119, 97, 115, 105, 45, 112, 111, 108, 108, 24, 112,
    107, 103, 58, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 47, 119, 97, 115, 105, 45, 112,
    111, 108, 108, 5, 8, 1, 66, 2, 1, 121, 4, 7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 0, 3,
    8, 119, 97, 115, 105, 45, 110, 101, 116, 22, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 110,
    101, 116, 47, 119, 97, 115, 105, 45, 110, 101, 116, 5, 9, 2, 3, 0, 2, 8, 112, 111, 108, 108,
    97, 98, 108, 101, 2, 3, 0, 3, 7, 110, 101, 116, 119, 111, 114, 107, 1, 66, 76, 2, 3, 2, 1, 10,
    4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 2, 3, 2, 1, 6, 4, 12, 105, 110, 112,
    117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 2, 2, 3, 2, 1, 7, 4, 13, 111, 117, 116,
    112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 4, 2, 3, 2, 1, 11, 4, 7, 110, 101,
    116, 119, 111, 114, 107, 0, 3, 0, 6, 1, 121, 4, 12, 116, 99, 112, 45, 108, 105, 115, 116, 101,
    110, 101, 114, 0, 3, 0, 8, 1, 110, 1, 8, 110, 111, 110, 98, 108, 111, 99, 107, 4, 14, 108, 105,
    115, 116, 101, 110, 101, 114, 45, 102, 108, 97, 103, 115, 0, 3, 0, 10, 1, 121, 4, 8, 108, 105,
    115, 116, 101, 110, 101, 114, 0, 3, 0, 12, 1, 111, 8, 123, 123, 123, 123, 123, 123, 123, 123,
    4, 12, 105, 112, 118, 54, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 14, 1, 114, 4, 7, 97,
    100, 100, 114, 101, 115, 115, 15, 4, 112, 111, 114, 116, 123, 9, 102, 108, 111, 119, 45, 105,
    110, 102, 111, 121, 8, 115, 99, 111, 112, 101, 45, 105, 100, 121, 4, 19, 105, 112, 118, 54, 45,
    115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 16, 1, 111, 4, 125,
    125, 125, 125, 4, 12, 105, 112, 118, 52, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 18, 1,
    114, 2, 7, 97, 100, 100, 114, 101, 115, 115, 19, 4, 112, 111, 114, 116, 123, 4, 19, 105, 112,
    118, 52, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 20, 1,
    113, 2, 4, 105, 112, 118, 52, 1, 21, 0, 4, 105, 112, 118, 54, 1, 17, 0, 4, 17, 105, 112, 45,
    115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 22, 1, 119, 4, 7,
    105, 111, 45, 115, 105, 122, 101, 0, 3, 0, 24, 1, 109, 36, 6, 97, 99, 99, 101, 115, 115, 9, 97,
    100, 100, 114, 105, 110, 117, 115, 101, 12, 97, 100, 100, 114, 110, 111, 116, 97, 118, 97, 105,
    108, 11, 97, 102, 110, 111, 115, 117, 112, 112, 111, 114, 116, 5, 97, 103, 97, 105, 110, 7, 97,
    108, 114, 101, 97, 100, 121, 4, 98, 97, 100, 102, 4, 98, 117, 115, 121, 18, 99, 111, 110, 110,
    101, 99, 116, 105, 111, 110, 45, 97, 98, 111, 114, 116, 101, 100, 18, 99, 111, 110, 110, 101,
    99, 116, 105, 111, 110, 45, 114, 101, 102, 117, 115, 101, 100, 16, 99, 111, 110, 110, 101, 99,
    116, 105, 111, 110, 45, 114, 101, 115, 101, 116, 8, 100, 101, 97, 100, 108, 111, 99, 107, 11,
    100, 101, 115, 116, 97, 100, 100, 114, 114, 101, 113, 16, 104, 111, 115, 116, 45, 117, 110,
    114, 101, 97, 99, 104, 97, 98, 108, 101, 5, 105, 108, 115, 101, 113, 10, 105, 110, 112, 114,
    111, 103, 114, 101, 115, 115, 4, 105, 110, 116, 114, 5, 105, 110, 118, 97, 108, 2, 105, 111, 6,
    105, 115, 99, 111, 110, 110, 7, 109, 115, 103, 115, 105, 122, 101, 8, 109, 117, 108, 116, 105,
    104, 111, 112, 11, 110, 97, 109, 101, 116, 111, 111, 108, 111, 110, 103, 12, 110, 101, 116,
    119, 111, 114, 107, 45, 100, 111, 119, 110, 13, 110, 101, 116, 119, 111, 114, 107, 45, 114,
    101, 115, 101, 116, 19, 110, 101, 116, 119, 111, 114, 107, 45, 117, 110, 114, 101, 97, 99, 104,
    97, 98, 108, 101, 6, 110, 111, 98, 117, 102, 115, 5, 110, 111, 101, 110, 116, 5, 110, 111, 109,
    101, 109, 10, 110, 111, 112, 114, 111, 116, 111, 111, 112, 116, 5, 110, 111, 115, 121, 115, 14,
    110, 111, 116, 114, 101, 99, 111, 118, 101, 114, 97, 98, 108, 101, 6, 110, 111, 116, 115, 117,
    112, 8, 111, 118, 101, 114, 102, 108, 111, 119, 4, 112, 101, 114, 109, 8, 116, 105, 109, 101,
    100, 111, 117, 116, 4, 5, 101, 114, 114, 110, 111, 0, 3, 0, 26, 1, 110, 3, 9, 107, 101, 101,
    112, 97, 108, 105, 118, 101, 8, 110, 111, 110, 98, 108, 111, 99, 107, 7, 110, 111, 100, 101,
    108, 97, 121, 4, 16, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 45, 102, 108, 97, 103,
    115, 0, 3, 0, 28, 1, 121, 4, 10, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 0, 3, 0, 30,
    1, 107, 121, 1, 106, 1, 9, 1, 27, 1, 64, 4, 7, 110, 101, 116, 119, 111, 114, 107, 7, 7, 97,
    100, 100, 114, 101, 115, 115, 23, 7, 98, 97, 99, 107, 108, 111, 103, 32, 5, 102, 108, 97, 103,
    115, 11, 0, 33, 4, 6, 108, 105, 115, 116, 101, 110, 0, 1, 34, 1, 111, 3, 31, 3, 5, 1, 106, 1,
    35, 1, 27, 1, 64, 2, 8, 108, 105, 115, 116, 101, 110, 101, 114, 13, 5, 102, 108, 97, 103, 115,
    29, 0, 36, 4, 6, 97, 99, 99, 101, 112, 116, 0, 1, 37, 1, 111, 4, 31, 3, 5, 23, 1, 106, 1, 38,
    1, 27, 1, 64, 2, 8, 108, 105, 115, 116, 101, 110, 101, 114, 9, 5, 102, 108, 97, 103, 115, 29,
    0, 39, 4, 10, 97, 99, 99, 101, 112, 116, 45, 116, 99, 112, 0, 1, 40, 1, 64, 4, 7, 110, 101,
    116, 119, 111, 114, 107, 7, 13, 108, 111, 99, 97, 108, 45, 97, 100, 100, 114, 101, 115, 115,
    23, 14, 114, 101, 109, 111, 116, 101, 45, 97, 100, 100, 114, 101, 115, 115, 23, 5, 102, 108,
    97, 103, 115, 29, 0, 36, 4, 7, 99, 111, 110, 110, 101, 99, 116, 0, 1, 41, 1, 112, 125, 1, 106,
    1, 25, 1, 27, 1, 64, 2, 10, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 31, 5, 98, 121,
    116, 101, 115, 42, 0, 43, 4, 4, 115, 101, 110, 100, 0, 1, 44, 1, 111, 2, 42, 127, 1, 106, 1,
    45, 1, 27, 1, 64, 2, 10, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 31, 6, 108, 101, 110,
    103, 116, 104, 25, 0, 46, 4, 7, 114, 101, 99, 101, 105, 118, 101, 0, 1, 47, 1, 106, 1, 29, 1,
    27, 1, 64, 1, 10, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 31, 0, 48, 4, 9, 103, 101,
    116, 45, 102, 108, 97, 103, 115, 0, 1, 49, 1, 106, 0, 1, 27, 1, 64, 2, 10, 99, 111, 110, 110,
    101, 99, 116, 105, 111, 110, 31, 5, 102, 108, 97, 103, 115, 29, 0, 50, 4, 9, 115, 101, 116, 45,
    102, 108, 97, 103, 115, 0, 1, 51, 1, 64, 1, 10, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110,
    31, 0, 43, 4, 23, 103, 101, 116, 45, 114, 101, 99, 101, 105, 118, 101, 45, 98, 117, 102, 102,
    101, 114, 45, 115, 105, 122, 101, 0, 1, 52, 1, 64, 2, 10, 99, 111, 110, 110, 101, 99, 116, 105,
    111, 110, 31, 5, 118, 97, 108, 117, 101, 25, 0, 50, 4, 23, 115, 101, 116, 45, 114, 101, 99,
    101, 105, 118, 101, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 53, 4, 20,
    103, 101, 116, 45, 115, 101, 110, 100, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101,
    0, 1, 52, 4, 20, 115, 101, 116, 45, 115, 101, 110, 100, 45, 98, 117, 102, 102, 101, 114, 45,
    115, 105, 122, 101, 0, 1, 53, 1, 111, 2, 25, 127, 1, 106, 1, 54, 1, 27, 1, 64, 1, 1, 115, 31,
    0, 55, 4, 14, 98, 121, 116, 101, 115, 45, 114, 101, 97, 100, 97, 98, 108, 101, 0, 1, 56, 4, 14,
    98, 121, 116, 101, 115, 45, 119, 114, 105, 116, 97, 98, 108, 101, 0, 1, 56, 1, 64, 1, 10, 99,
    111, 110, 110, 101, 99, 116, 105, 111, 110, 31, 0, 127, 4, 12, 105, 115, 45, 99, 111, 110, 110,
    101, 99, 116, 101, 100, 0, 1, 57, 1, 64, 1, 8, 108, 105, 115, 116, 101, 110, 101, 114, 9, 1, 0,
    4, 18, 99, 108, 111, 115, 101, 45, 116, 99, 112, 45, 108, 105, 115, 116, 101, 110, 101, 114, 0,
    1, 58, 4, 16, 99, 108, 111, 115, 101, 45, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 0, 1,
    58, 4, 8, 119, 97, 115, 105, 45, 116, 99, 112, 22, 112, 107, 103, 58, 47, 119, 97, 115, 105,
    45, 116, 99, 112, 47, 119, 97, 115, 105, 45, 116, 99, 112, 5, 12, 11, 27, 1, 8, 119, 97, 115,
    105, 45, 116, 99, 112, 13, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 116, 99, 112, 3, 20,
    0, 7, 138, 20, 1, 65, 9, 1, 66, 6, 1, 114, 0, 4, 12, 115, 116, 114, 101, 97, 109, 45, 101, 114,
    114, 111, 114, 0, 3, 0, 0, 1, 121, 4, 13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101,
    97, 109, 0, 3, 0, 2, 1, 121, 4, 12, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109,
    0, 3, 0, 4, 3, 7, 119, 97, 115, 105, 45, 105, 111, 20, 112, 107, 103, 58, 47, 119, 97, 115,
    105, 45, 105, 111, 47, 119, 97, 115, 105, 45, 105, 111, 5, 0, 1, 66, 10, 1, 121, 4, 10, 119,
    97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 3, 0, 0, 1, 121, 4, 8, 112, 111, 108, 108, 97, 98,
    108, 101, 0, 3, 0, 2, 1, 121, 4, 15, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108,
    111, 99, 107, 0, 3, 0, 4, 1, 119, 4, 7, 105, 110, 115, 116, 97, 110, 116, 0, 3, 0, 6, 1, 114,
    2, 7, 115, 101, 99, 111, 110, 100, 115, 119, 11, 110, 97, 110, 111, 115, 101, 99, 111, 110,
    100, 115, 121, 4, 8, 100, 97, 116, 101, 116, 105, 109, 101, 0, 3, 0, 8, 3, 11, 119, 97, 115,
    105, 45, 99, 108, 111, 99, 107, 115, 28, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 99, 108,
    111, 99, 107, 115, 47, 119, 97, 115, 105, 45, 99, 108, 111, 99, 107, 115, 5, 1, 2, 3, 0, 0, 12,
    105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 2, 3, 0, 0, 13, 111, 117, 116, 112,
    117, 116, 45, 115, 116, 114, 101, 97, 109, 2, 3, 0, 1, 8, 100, 97, 116, 101, 116, 105, 109,
    101, 1, 66, 119, 2, 3, 2, 1, 2, 4, 12, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97,
    109, 0, 3, 0, 0, 2, 3, 2, 1, 3, 4, 13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101,
    97, 109, 0, 3, 0, 2, 2, 3, 2, 1, 4, 4, 8, 100, 97, 116, 101, 116, 105, 109, 101, 0, 3, 0, 4, 1,
    121, 4, 4, 115, 105, 122, 101, 0, 3, 0, 6, 1, 110, 4, 6, 99, 114, 101, 97, 116, 101, 9, 100,
    105, 114, 101, 99, 116, 111, 114, 121, 4, 101, 120, 99, 108, 5, 116, 114, 117, 110, 99, 4, 7,
    111, 45, 102, 108, 97, 103, 115, 0, 3, 0, 8, 1, 110, 3, 8, 114, 101, 97, 100, 97, 98, 108, 101,
    9, 119, 114, 105, 116, 101, 97, 98, 108, 101, 10, 101, 120, 101, 99, 117, 116, 97, 98, 108,
    101, 4, 4, 109, 111, 100, 101, 0, 3, 0, 10, 1, 119, 4, 9, 108, 105, 110, 107, 99, 111, 117,
    110, 116, 0, 3, 0, 12, 1, 119, 4, 5, 105, 110, 111, 100, 101, 0, 3, 0, 14, 1, 119, 4, 8, 102,
    105, 108, 101, 115, 105, 122, 101, 0, 3, 0, 16, 1, 120, 4, 9, 102, 105, 108, 101, 100, 101,
    108, 116, 97, 0, 3, 0, 18, 1, 109, 38, 6, 97, 99, 99, 101, 115, 115, 5, 97, 103, 97, 105, 110,
    7, 97, 108, 114, 101, 97, 100, 121, 4, 98, 97, 100, 102, 4, 98, 117, 115, 121, 6, 100, 101, 97,
    100, 108, 107, 5, 100, 113, 117, 111, 116, 5, 101, 120, 105, 115, 116, 4, 102, 98, 105, 103, 5,
    105, 108, 115, 101, 113, 10, 105, 110, 112, 114, 111, 103, 114, 101, 115, 115, 4, 105, 110,
    116, 114, 5, 105, 110, 118, 97, 108, 2, 105, 111, 5, 105, 115, 100, 105, 114, 4, 108, 111, 111,
    112, 5, 109, 108, 105, 110, 107, 7, 109, 115, 103, 115, 105, 122, 101, 11, 110, 97, 109, 101,
    116, 111, 111, 108, 111, 110, 103, 5, 110, 111, 100, 101, 118, 5, 110, 111, 101, 110, 116, 5,
    110, 111, 108, 99, 107, 5, 110, 111, 109, 101, 109, 5, 110, 111, 115, 112, 99, 5, 110, 111,
    115, 121, 115, 6, 110, 111, 116, 100, 105, 114, 8, 110, 111, 116, 101, 109, 112, 116, 121, 14,
    110, 111, 116, 114, 101, 99, 111, 118, 101, 114, 97, 98, 108, 101, 6, 110, 111, 116, 115, 117,
    112, 5, 110, 111, 116, 116, 121, 4, 110, 120, 105, 111, 8, 111, 118, 101, 114, 102, 108, 111,
    119, 4, 112, 101, 114, 109, 4, 112, 105, 112, 101, 4, 114, 111, 102, 115, 5, 115, 112, 105,
    112, 101, 6, 116, 120, 116, 98, 115, 121, 4, 120, 100, 101, 118, 4, 5, 101, 114, 114, 110, 111,
    0, 3, 0, 20, 1, 121, 4, 16, 100, 105, 114, 45, 101, 110, 116, 114, 121, 45, 115, 116, 114, 101,
    97, 109, 0, 3, 0, 22, 1, 119, 4, 6, 100, 101, 118, 105, 99, 101, 0, 3, 0, 24, 1, 109, 8, 7,
    117, 110, 107, 110, 111, 119, 110, 12, 98, 108, 111, 99, 107, 45, 100, 101, 118, 105, 99, 101,
    16, 99, 104, 97, 114, 97, 99, 116, 101, 114, 45, 100, 101, 118, 105, 99, 101, 9, 100, 105, 114,
    101, 99, 116, 111, 114, 121, 4, 102, 105, 102, 111, 13, 115, 121, 109, 98, 111, 108, 105, 99,
    45, 108, 105, 110, 107, 12, 114, 101, 103, 117, 108, 97, 114, 45, 102, 105, 108, 101, 6, 115,
    111, 99, 107, 101, 116, 4, 15, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 45, 116, 121,
    112, 101, 0, 3, 0, 26, 1, 107, 15, 1, 114, 3, 3, 105, 110, 111, 28, 4, 116, 121, 112, 101, 27,
    4, 110, 97, 109, 101, 115, 4, 9, 100, 105, 114, 45, 101, 110, 116, 114, 121, 0, 3, 0, 29, 1,
    110, 6, 4, 114, 101, 97, 100, 5, 119, 114, 105, 116, 101, 5, 100, 115, 121, 110, 99, 8, 110,
    111, 110, 98, 108, 111, 99, 107, 5, 114, 115, 121, 110, 99, 4, 115, 121, 110, 99, 4, 16, 100,
    101, 115, 99, 114, 105, 112, 116, 111, 114, 45, 102, 108, 97, 103, 115, 0, 3, 0, 31, 1, 121, 4,
    10, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 0, 3, 0, 33, 1, 113, 3, 9, 110, 111, 45,
    99, 104, 97, 110, 103, 101, 0, 0, 3, 110, 111, 119, 0, 0, 9, 116, 105, 109, 101, 115, 116, 97,
    109, 112, 1, 5, 0, 4, 13, 110, 101, 119, 45, 116, 105, 109, 101, 115, 116, 97, 109, 112, 0, 3,
    0, 35, 1, 114, 8, 3, 100, 101, 118, 25, 3, 105, 110, 111, 15, 4, 116, 121, 112, 101, 27, 5,
    110, 108, 105, 110, 107, 13, 4, 115, 105, 122, 101, 17, 4, 97, 116, 105, 109, 5, 4, 109, 116,
    105, 109, 5, 4, 99, 116, 105, 109, 5, 4, 15, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114,
    45, 115, 116, 97, 116, 0, 3, 0, 37, 1, 110, 1, 14, 115, 121, 109, 108, 105, 110, 107, 45, 102,
    111, 108, 108, 111, 119, 4, 8, 97, 116, 45, 102, 108, 97, 103, 115, 0, 3, 0, 39, 1, 109, 6, 6,
    110, 111, 114, 109, 97, 108, 10, 115, 101, 113, 117, 101, 110, 116, 105, 97, 108, 6, 114, 97,
    110, 100, 111, 109, 9, 119, 105, 108, 108, 45, 110, 101, 101, 100, 9, 100, 111, 110, 116, 45,
    110, 101, 101, 100, 8, 110, 111, 45, 114, 101, 117, 115, 101, 4, 6, 97, 100, 118, 105, 99, 101,
    0, 3, 0, 41, 1, 106, 0, 1, 21, 1, 64, 4, 2, 102, 100, 34, 6, 111, 102, 102, 115, 101, 116, 17,
    3, 108, 101, 110, 17, 6, 97, 100, 118, 105, 99, 101, 42, 0, 43, 4, 7, 102, 97, 100, 118, 105,
    115, 101, 0, 1, 44, 1, 64, 1, 2, 102, 100, 34, 0, 43, 4, 8, 100, 97, 116, 97, 115, 121, 110,
    99, 0, 1, 45, 1, 106, 1, 32, 1, 21, 1, 64, 1, 2, 102, 100, 34, 0, 46, 4, 5, 102, 108, 97, 103,
    115, 0, 1, 47, 1, 106, 1, 27, 1, 21, 1, 64, 1, 2, 102, 100, 34, 0, 48, 4, 9, 116, 111, 100,
    111, 45, 116, 121, 112, 101, 0, 1, 49, 1, 64, 2, 2, 102, 100, 34, 5, 102, 108, 97, 103, 115,
    32, 0, 43, 4, 9, 115, 101, 116, 45, 102, 108, 97, 103, 115, 0, 1, 50, 1, 64, 2, 2, 102, 100,
    34, 4, 115, 105, 122, 101, 17, 0, 43, 4, 8, 115, 101, 116, 45, 115, 105, 122, 101, 0, 1, 51, 1,
    64, 3, 2, 102, 100, 34, 4, 97, 116, 105, 109, 36, 4, 109, 116, 105, 109, 36, 0, 43, 4, 9, 115,
    101, 116, 45, 116, 105, 109, 101, 115, 0, 1, 52, 1, 106, 1, 1, 1, 21, 1, 64, 2, 2, 102, 100,
    34, 6, 111, 102, 102, 115, 101, 116, 17, 0, 53, 4, 15, 114, 101, 97, 100, 45, 118, 105, 97, 45,
    115, 116, 114, 101, 97, 109, 0, 1, 54, 1, 106, 1, 3, 1, 21, 1, 64, 2, 2, 102, 100, 34, 6, 111,
    102, 102, 115, 101, 116, 17, 0, 55, 4, 16, 119, 114, 105, 116, 101, 45, 118, 105, 97, 45, 115,
    116, 114, 101, 97, 109, 0, 1, 56, 1, 64, 1, 2, 102, 100, 34, 0, 55, 4, 17, 97, 112, 112, 101,
    110, 100, 45, 118, 105, 97, 45, 115, 116, 114, 101, 97, 109, 0, 1, 57, 1, 112, 125, 1, 111, 2,
    58, 127, 1, 106, 1, 59, 1, 21, 1, 64, 3, 2, 102, 100, 34, 3, 108, 101, 110, 7, 6, 111, 102,
    102, 115, 101, 116, 17, 0, 60, 4, 5, 112, 114, 101, 97, 100, 0, 1, 61, 1, 106, 1, 7, 1, 21, 1,
    64, 3, 2, 102, 100, 34, 3, 98, 117, 102, 58, 6, 111, 102, 102, 115, 101, 116, 17, 0, 62, 4, 6,
    112, 119, 114, 105, 116, 101, 0, 1, 63, 1, 106, 1, 23, 1, 21, 1, 64, 1, 2, 102, 100, 34, 0,
    192, 0, 4, 7, 114, 101, 97, 100, 100, 105, 114, 0, 1, 65, 1, 64, 1, 1, 115, 23, 1, 0, 4, 22,
    99, 108, 111, 115, 101, 45, 100, 105, 114, 45, 101, 110, 116, 114, 121, 45, 115, 116, 114, 101,
    97, 109, 0, 1, 66, 1, 107, 30, 1, 106, 1, 195, 0, 1, 21, 1, 64, 1, 10, 100, 105, 114, 45, 115,
    116, 114, 101, 97, 109, 23, 0, 196, 0, 4, 14, 114, 101, 97, 100, 45, 100, 105, 114, 45, 101,
    110, 116, 114, 121, 0, 1, 69, 4, 4, 115, 121, 110, 99, 0, 1, 45, 1, 64, 2, 2, 102, 100, 34, 4,
    112, 97, 116, 104, 115, 0, 43, 4, 19, 99, 114, 101, 97, 116, 101, 45, 100, 105, 114, 101, 99,
    116, 111, 114, 121, 45, 97, 116, 0, 1, 70, 1, 106, 1, 38, 1, 21, 1, 64, 1, 2, 102, 100, 34, 0,
    199, 0, 4, 4, 115, 116, 97, 116, 0, 1, 72, 1, 64, 3, 2, 102, 100, 34, 8, 97, 116, 45, 102, 108,
    97, 103, 115, 40, 4, 112, 97, 116, 104, 115, 0, 199, 0, 4, 7, 115, 116, 97, 116, 45, 97, 116,
    0, 1, 73, 1, 64, 5, 2, 102, 100, 34, 8, 97, 116, 45, 102, 108, 97, 103, 115, 40, 4, 112, 97,
    116, 104, 115, 4, 97, 116, 105, 109, 36, 4, 109, 116, 105, 109, 36, 0, 43, 4, 12, 115, 101,
    116, 45, 116, 105, 109, 101, 115, 45, 97, 116, 0, 1, 74, 1, 64, 5, 2, 102, 100, 34, 12, 111,
    108, 100, 45, 97, 116, 45, 102, 108, 97, 103, 115, 40, 8, 111, 108, 100, 45, 112, 97, 116, 104,
    115, 14, 110, 101, 119, 45, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 34, 8, 110, 101,
    119, 45, 112, 97, 116, 104, 115, 0, 43, 4, 7, 108, 105, 110, 107, 45, 97, 116, 0, 1, 75, 1,
    106, 1, 34, 1, 21, 1, 64, 6, 2, 102, 100, 34, 8, 97, 116, 45, 102, 108, 97, 103, 115, 40, 4,
    112, 97, 116, 104, 115, 7, 111, 45, 102, 108, 97, 103, 115, 9, 5, 102, 108, 97, 103, 115, 32,
    4, 109, 111, 100, 101, 11, 0, 204, 0, 4, 7, 111, 112, 101, 110, 45, 97, 116, 0, 1, 77, 1, 64,
    1, 2, 102, 100, 34, 1, 0, 4, 5, 99, 108, 111, 115, 101, 0, 1, 78, 1, 106, 1, 115, 1, 21, 1, 64,
    2, 2, 102, 100, 34, 4, 112, 97, 116, 104, 115, 0, 207, 0, 4, 11, 114, 101, 97, 100, 108, 105,
    110, 107, 45, 97, 116, 0, 1, 80, 4, 19, 114, 101, 109, 111, 118, 101, 45, 100, 105, 114, 101,
    99, 116, 111, 114, 121, 45, 97, 116, 0, 1, 70, 1, 64, 4, 2, 102, 100, 34, 8, 111, 108, 100, 45,
    112, 97, 116, 104, 115, 14, 110, 101, 119, 45, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114,
    34, 8, 110, 101, 119, 45, 112, 97, 116, 104, 115, 0, 43, 4, 9, 114, 101, 110, 97, 109, 101, 45,
    97, 116, 0, 1, 81, 1, 64, 3, 2, 102, 100, 34, 8, 111, 108, 100, 45, 112, 97, 116, 104, 115, 8,
    110, 101, 119, 45, 112, 97, 116, 104, 115, 0, 43, 4, 10, 115, 121, 109, 108, 105, 110, 107, 45,
    97, 116, 0, 1, 82, 4, 14, 117, 110, 108, 105, 110, 107, 45, 102, 105, 108, 101, 45, 97, 116, 0,
    1, 70, 1, 64, 4, 2, 102, 100, 34, 8, 97, 116, 45, 102, 108, 97, 103, 115, 40, 4, 112, 97, 116,
    104, 115, 4, 109, 111, 100, 101, 11, 0, 43, 4, 26, 99, 104, 97, 110, 103, 101, 45, 102, 105,
    108, 101, 45, 112, 101, 114, 109, 105, 115, 115, 105, 111, 110, 115, 45, 97, 116, 0, 1, 83, 4,
    31, 99, 104, 97, 110, 103, 101, 45, 100, 105, 114, 101, 99, 116, 111, 114, 121, 45, 112, 101,
    114, 109, 105, 115, 115, 105, 111, 110, 115, 45, 97, 116, 0, 1, 83, 4, 11, 108, 111, 99, 107,
    45, 115, 104, 97, 114, 101, 100, 0, 1, 45, 4, 14, 108, 111, 99, 107, 45, 101, 120, 99, 108,
    117, 115, 105, 118, 101, 0, 1, 45, 4, 15, 116, 114, 121, 45, 108, 111, 99, 107, 45, 115, 104,
    97, 114, 101, 100, 0, 1, 45, 4, 18, 116, 114, 121, 45, 108, 111, 99, 107, 45, 101, 120, 99,
    108, 117, 115, 105, 118, 101, 0, 1, 45, 4, 6, 117, 110, 108, 111, 99, 107, 0, 1, 45, 4, 15,
    119, 97, 115, 105, 45, 102, 105, 108, 101, 115, 121, 115, 116, 101, 109, 36, 112, 107, 103, 58,
    47, 119, 97, 115, 105, 45, 102, 105, 108, 101, 115, 121, 115, 116, 101, 109, 47, 119, 97, 115,
    105, 45, 102, 105, 108, 101, 115, 121, 115, 116, 101, 109, 5, 5, 11, 41, 1, 15, 119, 97, 115,
    105, 45, 102, 105, 108, 101, 115, 121, 115, 116, 101, 109, 20, 112, 107, 103, 58, 47, 119, 97,
    115, 105, 45, 102, 105, 108, 101, 115, 121, 115, 116, 101, 109, 3, 22, 0, 7, 131, 51, 1, 65, 2,
    1, 65, 42, 1, 66, 20, 1, 121, 4, 10, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 3, 0, 0,
    1, 121, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 2, 1, 121, 4, 15, 109, 111, 110,
    111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 3, 0, 4, 1, 119, 4, 7, 105, 110,
    115, 116, 97, 110, 116, 0, 3, 0, 6, 1, 114, 2, 7, 115, 101, 99, 111, 110, 100, 115, 119, 11,
    110, 97, 110, 111, 115, 101, 99, 111, 110, 100, 115, 121, 4, 8, 100, 97, 116, 101, 116, 105,
    109, 101, 0, 3, 0, 8, 1, 64, 1, 5, 99, 108, 111, 99, 107, 5, 0, 7, 4, 19, 109, 111, 110, 111,
    116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 45, 110, 111, 119, 0, 1, 10, 4, 26, 109,
    111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 45, 114, 101, 115, 111, 108,
    117, 116, 105, 111, 110, 0, 1, 10, 1, 64, 1, 5, 99, 108, 111, 99, 107, 1, 0, 9, 4, 14, 119, 97,
    108, 108, 45, 99, 108, 111, 99, 107, 45, 110, 111, 119, 0, 1, 11, 4, 21, 119, 97, 108, 108, 45,
    99, 108, 111, 99, 107, 45, 114, 101, 115, 111, 108, 117, 116, 105, 111, 110, 0, 1, 11, 1, 64,
    1, 5, 99, 108, 111, 99, 107, 5, 1, 0, 4, 21, 99, 108, 111, 115, 101, 45, 109, 111, 110, 111,
    116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 1, 12, 1, 64, 1, 5, 99, 108, 111, 99,
    107, 1, 1, 0, 4, 16, 99, 108, 111, 115, 101, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107,
    0, 1, 13, 3, 11, 119, 97, 115, 105, 45, 99, 108, 111, 99, 107, 115, 28, 112, 107, 103, 58, 47,
    119, 97, 115, 105, 45, 99, 108, 111, 99, 107, 115, 47, 119, 97, 115, 105, 45, 99, 108, 111, 99,
    107, 115, 5, 0, 2, 3, 0, 0, 15, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111,
    99, 107, 2, 3, 0, 0, 10, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 1, 66, 8, 2, 3, 2, 1, 1,
    4, 15, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 3, 0, 0, 2, 3,
    2, 1, 2, 4, 10, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 3, 0, 2, 1, 64, 0, 0, 1, 4,
    23, 100, 101, 102, 97, 117, 108, 116, 45, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99,
    108, 111, 99, 107, 0, 1, 4, 1, 64, 0, 0, 3, 4, 18, 100, 101, 102, 97, 117, 108, 116, 45, 119,
    97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 1, 5, 3, 19, 119, 97, 115, 105, 45, 100, 101, 102,
    97, 117, 108, 116, 45, 99, 108, 111, 99, 107, 115, 36, 112, 107, 103, 58, 47, 119, 97, 115,
    105, 45, 99, 108, 111, 99, 107, 115, 47, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108,
    116, 45, 99, 108, 111, 99, 107, 115, 5, 3, 1, 66, 4, 1, 109, 5, 5, 116, 114, 97, 99, 101, 5,
    100, 101, 98, 117, 103, 4, 105, 110, 102, 111, 4, 119, 97, 114, 110, 5, 101, 114, 114, 111,
    114, 4, 5, 108, 101, 118, 101, 108, 0, 3, 0, 0, 1, 64, 3, 5, 108, 101, 118, 101, 108, 1, 7, 99,
    111, 110, 116, 101, 120, 116, 115, 7, 109, 101, 115, 115, 97, 103, 101, 115, 1, 0, 4, 3, 108,
    111, 103, 0, 1, 2, 3, 12, 119, 97, 115, 105, 45, 108, 111, 103, 103, 105, 110, 103, 30, 112,
    107, 103, 58, 47, 119, 97, 115, 105, 45, 108, 111, 103, 103, 105, 110, 103, 47, 119, 97, 115,
    105, 45, 108, 111, 103, 103, 105, 110, 103, 5, 4, 1, 66, 7, 1, 64, 1, 7, 109, 101, 115, 115,
    97, 103, 101, 115, 1, 0, 4, 5, 112, 114, 105, 110, 116, 0, 1, 0, 1, 64, 0, 0, 127, 4, 11, 105,
    115, 45, 116, 101, 114, 109, 105, 110, 97, 108, 0, 1, 1, 1, 107, 123, 1, 64, 0, 0, 2, 4, 11,
    110, 117, 109, 45, 99, 111, 108, 117, 109, 110, 115, 0, 1, 3, 3, 11, 119, 97, 115, 105, 45,
    115, 116, 100, 101, 114, 114, 28, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 115, 116, 100,
    101, 114, 114, 47, 119, 97, 115, 105, 45, 115, 116, 100, 101, 114, 114, 5, 5, 1, 66, 28, 1,
    114, 0, 4, 12, 115, 116, 114, 101, 97, 109, 45, 101, 114, 114, 111, 114, 0, 3, 0, 0, 1, 121, 4,
    13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 2, 1, 121, 4, 12,
    105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 4, 1, 112, 125, 1, 111, 2,
    6, 127, 1, 106, 1, 7, 1, 1, 1, 64, 2, 3, 115, 114, 99, 5, 3, 108, 101, 110, 119, 0, 8, 4, 4,
    114, 101, 97, 100, 0, 1, 9, 1, 111, 2, 119, 127, 1, 106, 1, 10, 1, 1, 1, 64, 2, 3, 115, 114,
    99, 5, 3, 108, 101, 110, 119, 0, 11, 4, 4, 115, 107, 105, 112, 0, 1, 12, 1, 106, 1, 119, 1, 1,
    1, 64, 2, 3, 100, 115, 116, 3, 3, 98, 117, 102, 6, 0, 13, 4, 5, 119, 114, 105, 116, 101, 0, 1,
    14, 1, 64, 3, 3, 100, 115, 116, 3, 4, 98, 121, 116, 101, 125, 3, 108, 101, 110, 119, 0, 13, 4,
    14, 119, 114, 105, 116, 101, 45, 114, 101, 112, 101, 97, 116, 101, 100, 0, 1, 15, 1, 64, 3, 3,
    100, 115, 116, 3, 3, 115, 114, 99, 5, 3, 108, 101, 110, 119, 0, 11, 4, 6, 115, 112, 108, 105,
    99, 101, 0, 1, 16, 1, 64, 2, 3, 100, 115, 116, 3, 3, 115, 114, 99, 5, 0, 13, 4, 7, 102, 111,
    114, 119, 97, 114, 100, 0, 1, 17, 1, 64, 1, 1, 102, 5, 1, 0, 4, 17, 100, 114, 111, 112, 45,
    105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 1, 18, 1, 64, 1, 1, 102, 3, 1, 0,
    4, 18, 100, 114, 111, 112, 45, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109,
    0, 1, 19, 3, 7, 119, 97, 115, 105, 45, 105, 111, 20, 112, 107, 103, 58, 47, 119, 97, 115, 105,
    45, 105, 111, 47, 119, 97, 115, 105, 45, 105, 111, 5, 6, 2, 3, 0, 4, 12, 105, 110, 112, 117,
    116, 45, 115, 116, 114, 101, 97, 109, 2, 3, 0, 4, 13, 111, 117, 116, 112, 117, 116, 45, 115,
    116, 114, 101, 97, 109, 2, 3, 0, 0, 8, 100, 97, 116, 101, 116, 105, 109, 101, 1, 66, 119, 2, 3,
    2, 1, 7, 4, 12, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 0, 2, 3, 2,
    1, 8, 4, 13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 2, 2, 3,
    2, 1, 9, 4, 8, 100, 97, 116, 101, 116, 105, 109, 101, 0, 3, 0, 4, 1, 121, 4, 4, 115, 105, 122,
    101, 0, 3, 0, 6, 1, 110, 4, 6, 99, 114, 101, 97, 116, 101, 9, 100, 105, 114, 101, 99, 116, 111,
    114, 121, 4, 101, 120, 99, 108, 5, 116, 114, 117, 110, 99, 4, 7, 111, 45, 102, 108, 97, 103,
    115, 0, 3, 0, 8, 1, 110, 3, 8, 114, 101, 97, 100, 97, 98, 108, 101, 9, 119, 114, 105, 116, 101,
    97, 98, 108, 101, 10, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 4, 4, 109, 111, 100, 101,
    0, 3, 0, 10, 1, 119, 4, 9, 108, 105, 110, 107, 99, 111, 117, 110, 116, 0, 3, 0, 12, 1, 119, 4,
    5, 105, 110, 111, 100, 101, 0, 3, 0, 14, 1, 119, 4, 8, 102, 105, 108, 101, 115, 105, 122, 101,
    0, 3, 0, 16, 1, 120, 4, 9, 102, 105, 108, 101, 100, 101, 108, 116, 97, 0, 3, 0, 18, 1, 109, 38,
    6, 97, 99, 99, 101, 115, 115, 5, 97, 103, 97, 105, 110, 7, 97, 108, 114, 101, 97, 100, 121, 4,
    98, 97, 100, 102, 4, 98, 117, 115, 121, 6, 100, 101, 97, 100, 108, 107, 5, 100, 113, 117, 111,
    116, 5, 101, 120, 105, 115, 116, 4, 102, 98, 105, 103, 5, 105, 108, 115, 101, 113, 10, 105,
    110, 112, 114, 111, 103, 114, 101, 115, 115, 4, 105, 110, 116, 114, 5, 105, 110, 118, 97, 108,
    2, 105, 111, 5, 105, 115, 100, 105, 114, 4, 108, 111, 111, 112, 5, 109, 108, 105, 110, 107, 7,
    109, 115, 103, 115, 105, 122, 101, 11, 110, 97, 109, 101, 116, 111, 111, 108, 111, 110, 103, 5,
    110, 111, 100, 101, 118, 5, 110, 111, 101, 110, 116, 5, 110, 111, 108, 99, 107, 5, 110, 111,
    109, 101, 109, 5, 110, 111, 115, 112, 99, 5, 110, 111, 115, 121, 115, 6, 110, 111, 116, 100,
    105, 114, 8, 110, 111, 116, 101, 109, 112, 116, 121, 14, 110, 111, 116, 114, 101, 99, 111, 118,
    101, 114, 97, 98, 108, 101, 6, 110, 111, 116, 115, 117, 112, 5, 110, 111, 116, 116, 121, 4,
    110, 120, 105, 111, 8, 111, 118, 101, 114, 102, 108, 111, 119, 4, 112, 101, 114, 109, 4, 112,
    105, 112, 101, 4, 114, 111, 102, 115, 5, 115, 112, 105, 112, 101, 6, 116, 120, 116, 98, 115,
    121, 4, 120, 100, 101, 118, 4, 5, 101, 114, 114, 110, 111, 0, 3, 0, 20, 1, 121, 4, 16, 100,
    105, 114, 45, 101, 110, 116, 114, 121, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 22, 1, 119, 4,
    6, 100, 101, 118, 105, 99, 101, 0, 3, 0, 24, 1, 109, 8, 7, 117, 110, 107, 110, 111, 119, 110,
    12, 98, 108, 111, 99, 107, 45, 100, 101, 118, 105, 99, 101, 16, 99, 104, 97, 114, 97, 99, 116,
    101, 114, 45, 100, 101, 118, 105, 99, 101, 9, 100, 105, 114, 101, 99, 116, 111, 114, 121, 4,
    102, 105, 102, 111, 13, 115, 121, 109, 98, 111, 108, 105, 99, 45, 108, 105, 110, 107, 12, 114,
    101, 103, 117, 108, 97, 114, 45, 102, 105, 108, 101, 6, 115, 111, 99, 107, 101, 116, 4, 15,
    100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 45, 116, 121, 112, 101, 0, 3, 0, 26, 1, 107,
    15, 1, 114, 3, 3, 105, 110, 111, 28, 4, 116, 121, 112, 101, 27, 4, 110, 97, 109, 101, 115, 4,
    9, 100, 105, 114, 45, 101, 110, 116, 114, 121, 0, 3, 0, 29, 1, 110, 6, 4, 114, 101, 97, 100, 5,
    119, 114, 105, 116, 101, 5, 100, 115, 121, 110, 99, 8, 110, 111, 110, 98, 108, 111, 99, 107, 5,
    114, 115, 121, 110, 99, 4, 115, 121, 110, 99, 4, 16, 100, 101, 115, 99, 114, 105, 112, 116,
    111, 114, 45, 102, 108, 97, 103, 115, 0, 3, 0, 31, 1, 121, 4, 10, 100, 101, 115, 99, 114, 105,
    112, 116, 111, 114, 0, 3, 0, 33, 1, 113, 3, 9, 110, 111, 45, 99, 104, 97, 110, 103, 101, 0, 0,
    3, 110, 111, 119, 0, 0, 9, 116, 105, 109, 101, 115, 116, 97, 109, 112, 1, 5, 0, 4, 13, 110,
    101, 119, 45, 116, 105, 109, 101, 115, 116, 97, 109, 112, 0, 3, 0, 35, 1, 114, 8, 3, 100, 101,
    118, 25, 3, 105, 110, 111, 15, 4, 116, 121, 112, 101, 27, 5, 110, 108, 105, 110, 107, 13, 4,
    115, 105, 122, 101, 17, 4, 97, 116, 105, 109, 5, 4, 109, 116, 105, 109, 5, 4, 99, 116, 105,
    109, 5, 4, 15, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 45, 115, 116, 97, 116, 0, 3, 0,
    37, 1, 110, 1, 14, 115, 121, 109, 108, 105, 110, 107, 45, 102, 111, 108, 108, 111, 119, 4, 8,
    97, 116, 45, 102, 108, 97, 103, 115, 0, 3, 0, 39, 1, 109, 6, 6, 110, 111, 114, 109, 97, 108,
    10, 115, 101, 113, 117, 101, 110, 116, 105, 97, 108, 6, 114, 97, 110, 100, 111, 109, 9, 119,
    105, 108, 108, 45, 110, 101, 101, 100, 9, 100, 111, 110, 116, 45, 110, 101, 101, 100, 8, 110,
    111, 45, 114, 101, 117, 115, 101, 4, 6, 97, 100, 118, 105, 99, 101, 0, 3, 0, 41, 1, 106, 0, 1,
    21, 1, 64, 4, 2, 102, 100, 34, 6, 111, 102, 102, 115, 101, 116, 17, 3, 108, 101, 110, 17, 6,
    97, 100, 118, 105, 99, 101, 42, 0, 43, 4, 7, 102, 97, 100, 118, 105, 115, 101, 0, 1, 44, 1, 64,
    1, 2, 102, 100, 34, 0, 43, 4, 8, 100, 97, 116, 97, 115, 121, 110, 99, 0, 1, 45, 1, 106, 1, 32,
    1, 21, 1, 64, 1, 2, 102, 100, 34, 0, 46, 4, 5, 102, 108, 97, 103, 115, 0, 1, 47, 1, 106, 1, 27,
    1, 21, 1, 64, 1, 2, 102, 100, 34, 0, 48, 4, 9, 116, 111, 100, 111, 45, 116, 121, 112, 101, 0,
    1, 49, 1, 64, 2, 2, 102, 100, 34, 5, 102, 108, 97, 103, 115, 32, 0, 43, 4, 9, 115, 101, 116,
    45, 102, 108, 97, 103, 115, 0, 1, 50, 1, 64, 2, 2, 102, 100, 34, 4, 115, 105, 122, 101, 17, 0,
    43, 4, 8, 115, 101, 116, 45, 115, 105, 122, 101, 0, 1, 51, 1, 64, 3, 2, 102, 100, 34, 4, 97,
    116, 105, 109, 36, 4, 109, 116, 105, 109, 36, 0, 43, 4, 9, 115, 101, 116, 45, 116, 105, 109,
    101, 115, 0, 1, 52, 1, 106, 1, 1, 1, 21, 1, 64, 2, 2, 102, 100, 34, 6, 111, 102, 102, 115, 101,
    116, 17, 0, 53, 4, 15, 114, 101, 97, 100, 45, 118, 105, 97, 45, 115, 116, 114, 101, 97, 109, 0,
    1, 54, 1, 106, 1, 3, 1, 21, 1, 64, 2, 2, 102, 100, 34, 6, 111, 102, 102, 115, 101, 116, 17, 0,
    55, 4, 16, 119, 114, 105, 116, 101, 45, 118, 105, 97, 45, 115, 116, 114, 101, 97, 109, 0, 1,
    56, 1, 64, 1, 2, 102, 100, 34, 0, 55, 4, 17, 97, 112, 112, 101, 110, 100, 45, 118, 105, 97, 45,
    115, 116, 114, 101, 97, 109, 0, 1, 57, 1, 112, 125, 1, 111, 2, 58, 127, 1, 106, 1, 59, 1, 21,
    1, 64, 3, 2, 102, 100, 34, 3, 108, 101, 110, 7, 6, 111, 102, 102, 115, 101, 116, 17, 0, 60, 4,
    5, 112, 114, 101, 97, 100, 0, 1, 61, 1, 106, 1, 7, 1, 21, 1, 64, 3, 2, 102, 100, 34, 3, 98,
    117, 102, 58, 6, 111, 102, 102, 115, 101, 116, 17, 0, 62, 4, 6, 112, 119, 114, 105, 116, 101,
    0, 1, 63, 1, 106, 1, 23, 1, 21, 1, 64, 1, 2, 102, 100, 34, 0, 192, 0, 4, 7, 114, 101, 97, 100,
    100, 105, 114, 0, 1, 65, 1, 64, 1, 1, 115, 23, 1, 0, 4, 22, 99, 108, 111, 115, 101, 45, 100,
    105, 114, 45, 101, 110, 116, 114, 121, 45, 115, 116, 114, 101, 97, 109, 0, 1, 66, 1, 107, 30,
    1, 106, 1, 195, 0, 1, 21, 1, 64, 1, 10, 100, 105, 114, 45, 115, 116, 114, 101, 97, 109, 23, 0,
    196, 0, 4, 14, 114, 101, 97, 100, 45, 100, 105, 114, 45, 101, 110, 116, 114, 121, 0, 1, 69, 4,
    4, 115, 121, 110, 99, 0, 1, 45, 1, 64, 2, 2, 102, 100, 34, 4, 112, 97, 116, 104, 115, 0, 43, 4,
    19, 99, 114, 101, 97, 116, 101, 45, 100, 105, 114, 101, 99, 116, 111, 114, 121, 45, 97, 116, 0,
    1, 70, 1, 106, 1, 38, 1, 21, 1, 64, 1, 2, 102, 100, 34, 0, 199, 0, 4, 4, 115, 116, 97, 116, 0,
    1, 72, 1, 64, 3, 2, 102, 100, 34, 8, 97, 116, 45, 102, 108, 97, 103, 115, 40, 4, 112, 97, 116,
    104, 115, 0, 199, 0, 4, 7, 115, 116, 97, 116, 45, 97, 116, 0, 1, 73, 1, 64, 5, 2, 102, 100, 34,
    8, 97, 116, 45, 102, 108, 97, 103, 115, 40, 4, 112, 97, 116, 104, 115, 4, 97, 116, 105, 109,
    36, 4, 109, 116, 105, 109, 36, 0, 43, 4, 12, 115, 101, 116, 45, 116, 105, 109, 101, 115, 45,
    97, 116, 0, 1, 74, 1, 64, 5, 2, 102, 100, 34, 12, 111, 108, 100, 45, 97, 116, 45, 102, 108, 97,
    103, 115, 40, 8, 111, 108, 100, 45, 112, 97, 116, 104, 115, 14, 110, 101, 119, 45, 100, 101,
    115, 99, 114, 105, 112, 116, 111, 114, 34, 8, 110, 101, 119, 45, 112, 97, 116, 104, 115, 0, 43,
    4, 7, 108, 105, 110, 107, 45, 97, 116, 0, 1, 75, 1, 106, 1, 34, 1, 21, 1, 64, 6, 2, 102, 100,
    34, 8, 97, 116, 45, 102, 108, 97, 103, 115, 40, 4, 112, 97, 116, 104, 115, 7, 111, 45, 102,
    108, 97, 103, 115, 9, 5, 102, 108, 97, 103, 115, 32, 4, 109, 111, 100, 101, 11, 0, 204, 0, 4,
    7, 111, 112, 101, 110, 45, 97, 116, 0, 1, 77, 1, 64, 1, 2, 102, 100, 34, 1, 0, 4, 5, 99, 108,
    111, 115, 101, 0, 1, 78, 1, 106, 1, 115, 1, 21, 1, 64, 2, 2, 102, 100, 34, 4, 112, 97, 116,
    104, 115, 0, 207, 0, 4, 11, 114, 101, 97, 100, 108, 105, 110, 107, 45, 97, 116, 0, 1, 80, 4,
    19, 114, 101, 109, 111, 118, 101, 45, 100, 105, 114, 101, 99, 116, 111, 114, 121, 45, 97, 116,
    0, 1, 70, 1, 64, 4, 2, 102, 100, 34, 8, 111, 108, 100, 45, 112, 97, 116, 104, 115, 14, 110,
    101, 119, 45, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 34, 8, 110, 101, 119, 45, 112,
    97, 116, 104, 115, 0, 43, 4, 9, 114, 101, 110, 97, 109, 101, 45, 97, 116, 0, 1, 81, 1, 64, 3,
    2, 102, 100, 34, 8, 111, 108, 100, 45, 112, 97, 116, 104, 115, 8, 110, 101, 119, 45, 112, 97,
    116, 104, 115, 0, 43, 4, 10, 115, 121, 109, 108, 105, 110, 107, 45, 97, 116, 0, 1, 82, 4, 14,
    117, 110, 108, 105, 110, 107, 45, 102, 105, 108, 101, 45, 97, 116, 0, 1, 70, 1, 64, 4, 2, 102,
    100, 34, 8, 97, 116, 45, 102, 108, 97, 103, 115, 40, 4, 112, 97, 116, 104, 115, 4, 109, 111,
    100, 101, 11, 0, 43, 4, 26, 99, 104, 97, 110, 103, 101, 45, 102, 105, 108, 101, 45, 112, 101,
    114, 109, 105, 115, 115, 105, 111, 110, 115, 45, 97, 116, 0, 1, 83, 4, 31, 99, 104, 97, 110,
    103, 101, 45, 100, 105, 114, 101, 99, 116, 111, 114, 121, 45, 112, 101, 114, 109, 105, 115,
    115, 105, 111, 110, 115, 45, 97, 116, 0, 1, 83, 4, 11, 108, 111, 99, 107, 45, 115, 104, 97,
    114, 101, 100, 0, 1, 45, 4, 14, 108, 111, 99, 107, 45, 101, 120, 99, 108, 117, 115, 105, 118,
    101, 0, 1, 45, 4, 15, 116, 114, 121, 45, 108, 111, 99, 107, 45, 115, 104, 97, 114, 101, 100, 0,
    1, 45, 4, 18, 116, 114, 121, 45, 108, 111, 99, 107, 45, 101, 120, 99, 108, 117, 115, 105, 118,
    101, 0, 1, 45, 4, 6, 117, 110, 108, 111, 99, 107, 0, 1, 45, 3, 15, 119, 97, 115, 105, 45, 102,
    105, 108, 101, 115, 121, 115, 116, 101, 109, 36, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45,
    102, 105, 108, 101, 115, 121, 115, 116, 101, 109, 47, 119, 97, 115, 105, 45, 102, 105, 108,
    101, 115, 121, 115, 116, 101, 109, 5, 10, 1, 66, 5, 1, 112, 125, 1, 64, 1, 3, 108, 101, 110,
    121, 0, 0, 4, 16, 103, 101, 116, 45, 114, 97, 110, 100, 111, 109, 45, 98, 121, 116, 101, 115,
    0, 1, 1, 1, 64, 0, 0, 119, 4, 14, 103, 101, 116, 45, 114, 97, 110, 100, 111, 109, 45, 117, 54,
    52, 0, 1, 2, 3, 11, 119, 97, 115, 105, 45, 114, 97, 110, 100, 111, 109, 28, 112, 107, 103, 58,
    47, 119, 97, 115, 105, 45, 114, 97, 110, 100, 111, 109, 47, 119, 97, 115, 105, 45, 114, 97,
    110, 100, 111, 109, 5, 11, 2, 3, 0, 0, 7, 105, 110, 115, 116, 97, 110, 116, 1, 66, 26, 2, 3, 2,
    1, 2, 4, 10, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 3, 0, 0, 2, 3, 2, 1, 1, 4, 15,
    109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 3, 0, 2, 2, 3, 2, 1,
    9, 4, 8, 100, 97, 116, 101, 116, 105, 109, 101, 0, 3, 0, 4, 2, 3, 2, 1, 12, 4, 7, 105, 110,
    115, 116, 97, 110, 116, 0, 3, 0, 6, 2, 3, 2, 1, 7, 4, 12, 105, 110, 112, 117, 116, 45, 115,
    116, 114, 101, 97, 109, 0, 3, 0, 8, 2, 3, 2, 1, 8, 4, 13, 111, 117, 116, 112, 117, 116, 45,
    115, 116, 114, 101, 97, 109, 0, 3, 0, 10, 1, 121, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101,
    0, 3, 0, 12, 1, 64, 1, 1, 102, 13, 1, 0, 4, 13, 100, 114, 111, 112, 45, 112, 111, 108, 108, 97,
    98, 108, 101, 0, 1, 14, 1, 64, 1, 1, 115, 9, 0, 13, 4, 14, 115, 117, 98, 115, 99, 114, 105, 98,
    101, 45, 114, 101, 97, 100, 0, 1, 15, 1, 64, 1, 1, 115, 11, 0, 13, 4, 15, 115, 117, 98, 115,
    99, 114, 105, 98, 101, 45, 119, 114, 105, 116, 101, 0, 1, 16, 1, 64, 3, 5, 99, 108, 111, 99,
    107, 3, 4, 119, 104, 101, 110, 7, 8, 97, 98, 115, 111, 108, 117, 116, 101, 127, 0, 13, 4, 25,
    115, 117, 98, 115, 99, 114, 105, 98, 101, 45, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45,
    99, 108, 111, 99, 107, 0, 1, 17, 1, 112, 13, 1, 112, 125, 1, 64, 1, 2, 105, 110, 18, 0, 19, 4,
    11, 112, 111, 108, 108, 45, 111, 110, 101, 111, 102, 102, 0, 1, 20, 3, 9, 119, 97, 115, 105,
    45, 112, 111, 108, 108, 24, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108,
    47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 5, 13, 1, 66, 2, 1, 121, 4, 7, 110, 101, 116,
    119, 111, 114, 107, 0, 3, 0, 0, 3, 8, 119, 97, 115, 105, 45, 110, 101, 116, 22, 112, 107, 103,
    58, 47, 119, 97, 115, 105, 45, 110, 101, 116, 47, 119, 97, 115, 105, 45, 110, 101, 116, 5, 14,
    2, 3, 0, 7, 8, 112, 111, 108, 108, 97, 98, 108, 101, 2, 3, 0, 8, 7, 110, 101, 116, 119, 111,
    114, 107, 1, 66, 76, 2, 3, 2, 1, 15, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 2,
    3, 2, 1, 7, 4, 12, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 2, 2, 3,
    2, 1, 8, 4, 13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 4, 2,
    3, 2, 1, 16, 4, 7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 6, 1, 121, 4, 12, 116, 99, 112,
    45, 108, 105, 115, 116, 101, 110, 101, 114, 0, 3, 0, 8, 1, 110, 1, 8, 110, 111, 110, 98, 108,
    111, 99, 107, 4, 14, 108, 105, 115, 116, 101, 110, 101, 114, 45, 102, 108, 97, 103, 115, 0, 3,
    0, 10, 1, 121, 4, 8, 108, 105, 115, 116, 101, 110, 101, 114, 0, 3, 0, 12, 1, 111, 8, 123, 123,
    123, 123, 123, 123, 123, 123, 4, 12, 105, 112, 118, 54, 45, 97, 100, 100, 114, 101, 115, 115,
    0, 3, 0, 14, 1, 114, 4, 7, 97, 100, 100, 114, 101, 115, 115, 15, 4, 112, 111, 114, 116, 123, 9,
    102, 108, 111, 119, 45, 105, 110, 102, 111, 121, 8, 115, 99, 111, 112, 101, 45, 105, 100, 121,
    4, 19, 105, 112, 118, 54, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115,
    115, 0, 3, 0, 16, 1, 111, 4, 125, 125, 125, 125, 4, 12, 105, 112, 118, 52, 45, 97, 100, 100,
    114, 101, 115, 115, 0, 3, 0, 18, 1, 114, 2, 7, 97, 100, 100, 114, 101, 115, 115, 19, 4, 112,
    111, 114, 116, 123, 4, 19, 105, 112, 118, 52, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100,
    100, 114, 101, 115, 115, 0, 3, 0, 20, 1, 113, 2, 4, 105, 112, 118, 52, 1, 21, 0, 4, 105, 112,
    118, 54, 1, 17, 0, 4, 17, 105, 112, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114,
    101, 115, 115, 0, 3, 0, 22, 1, 119, 4, 7, 105, 111, 45, 115, 105, 122, 101, 0, 3, 0, 24, 1,
    109, 36, 6, 97, 99, 99, 101, 115, 115, 9, 97, 100, 100, 114, 105, 110, 117, 115, 101, 12, 97,
    100, 100, 114, 110, 111, 116, 97, 118, 97, 105, 108, 11, 97, 102, 110, 111, 115, 117, 112, 112,
    111, 114, 116, 5, 97, 103, 97, 105, 110, 7, 97, 108, 114, 101, 97, 100, 121, 4, 98, 97, 100,
    102, 4, 98, 117, 115, 121, 18, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 45, 97, 98, 111,
    114, 116, 101, 100, 18, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 45, 114, 101, 102, 117,
    115, 101, 100, 16, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 45, 114, 101, 115, 101, 116,
    8, 100, 101, 97, 100, 108, 111, 99, 107, 11, 100, 101, 115, 116, 97, 100, 100, 114, 114, 101,
    113, 16, 104, 111, 115, 116, 45, 117, 110, 114, 101, 97, 99, 104, 97, 98, 108, 101, 5, 105,
    108, 115, 101, 113, 10, 105, 110, 112, 114, 111, 103, 114, 101, 115, 115, 4, 105, 110, 116,
    114, 5, 105, 110, 118, 97, 108, 2, 105, 111, 6, 105, 115, 99, 111, 110, 110, 7, 109, 115, 103,
    115, 105, 122, 101, 8, 109, 117, 108, 116, 105, 104, 111, 112, 11, 110, 97, 109, 101, 116, 111,
    111, 108, 111, 110, 103, 12, 110, 101, 116, 119, 111, 114, 107, 45, 100, 111, 119, 110, 13,
    110, 101, 116, 119, 111, 114, 107, 45, 114, 101, 115, 101, 116, 19, 110, 101, 116, 119, 111,
    114, 107, 45, 117, 110, 114, 101, 97, 99, 104, 97, 98, 108, 101, 6, 110, 111, 98, 117, 102,
    115, 5, 110, 111, 101, 110, 116, 5, 110, 111, 109, 101, 109, 10, 110, 111, 112, 114, 111, 116,
    111, 111, 112, 116, 5, 110, 111, 115, 121, 115, 14, 110, 111, 116, 114, 101, 99, 111, 118, 101,
    114, 97, 98, 108, 101, 6, 110, 111, 116, 115, 117, 112, 8, 111, 118, 101, 114, 102, 108, 111,
    119, 4, 112, 101, 114, 109, 8, 116, 105, 109, 101, 100, 111, 117, 116, 4, 5, 101, 114, 114,
    110, 111, 0, 3, 0, 26, 1, 110, 3, 9, 107, 101, 101, 112, 97, 108, 105, 118, 101, 8, 110, 111,
    110, 98, 108, 111, 99, 107, 7, 110, 111, 100, 101, 108, 97, 121, 4, 16, 99, 111, 110, 110, 101,
    99, 116, 105, 111, 110, 45, 102, 108, 97, 103, 115, 0, 3, 0, 28, 1, 121, 4, 10, 99, 111, 110,
    110, 101, 99, 116, 105, 111, 110, 0, 3, 0, 30, 1, 107, 121, 1, 106, 1, 9, 1, 27, 1, 64, 4, 7,
    110, 101, 116, 119, 111, 114, 107, 7, 7, 97, 100, 100, 114, 101, 115, 115, 23, 7, 98, 97, 99,
    107, 108, 111, 103, 32, 5, 102, 108, 97, 103, 115, 11, 0, 33, 4, 6, 108, 105, 115, 116, 101,
    110, 0, 1, 34, 1, 111, 3, 31, 3, 5, 1, 106, 1, 35, 1, 27, 1, 64, 2, 8, 108, 105, 115, 116, 101,
    110, 101, 114, 13, 5, 102, 108, 97, 103, 115, 29, 0, 36, 4, 6, 97, 99, 99, 101, 112, 116, 0, 1,
    37, 1, 111, 4, 31, 3, 5, 23, 1, 106, 1, 38, 1, 27, 1, 64, 2, 8, 108, 105, 115, 116, 101, 110,
    101, 114, 9, 5, 102, 108, 97, 103, 115, 29, 0, 39, 4, 10, 97, 99, 99, 101, 112, 116, 45, 116,
    99, 112, 0, 1, 40, 1, 64, 4, 7, 110, 101, 116, 119, 111, 114, 107, 7, 13, 108, 111, 99, 97,
    108, 45, 97, 100, 100, 114, 101, 115, 115, 23, 14, 114, 101, 109, 111, 116, 101, 45, 97, 100,
    100, 114, 101, 115, 115, 23, 5, 102, 108, 97, 103, 115, 29, 0, 36, 4, 7, 99, 111, 110, 110,
    101, 99, 116, 0, 1, 41, 1, 112, 125, 1, 106, 1, 25, 1, 27, 1, 64, 2, 10, 99, 111, 110, 110,
    101, 99, 116, 105, 111, 110, 31, 5, 98, 121, 116, 101, 115, 42, 0, 43, 4, 4, 115, 101, 110,
    100, 0, 1, 44, 1, 111, 2, 42, 127, 1, 106, 1, 45, 1, 27, 1, 64, 2, 10, 99, 111, 110, 110, 101,
    99, 116, 105, 111, 110, 31, 6, 108, 101, 110, 103, 116, 104, 25, 0, 46, 4, 7, 114, 101, 99,
    101, 105, 118, 101, 0, 1, 47, 1, 106, 1, 29, 1, 27, 1, 64, 1, 10, 99, 111, 110, 110, 101, 99,
    116, 105, 111, 110, 31, 0, 48, 4, 9, 103, 101, 116, 45, 102, 108, 97, 103, 115, 0, 1, 49, 1,
    106, 0, 1, 27, 1, 64, 2, 10, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 31, 5, 102, 108,
    97, 103, 115, 29, 0, 50, 4, 9, 115, 101, 116, 45, 102, 108, 97, 103, 115, 0, 1, 51, 1, 64, 1,
    10, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 31, 0, 43, 4, 23, 103, 101, 116, 45, 114,
    101, 99, 101, 105, 118, 101, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 52,
    1, 64, 2, 10, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 31, 5, 118, 97, 108, 117, 101,
    25, 0, 50, 4, 23, 115, 101, 116, 45, 114, 101, 99, 101, 105, 118, 101, 45, 98, 117, 102, 102,
    101, 114, 45, 115, 105, 122, 101, 0, 1, 53, 4, 20, 103, 101, 116, 45, 115, 101, 110, 100, 45,
    98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 52, 4, 20, 115, 101, 116, 45, 115,
    101, 110, 100, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 53, 1, 111, 2,
    25, 127, 1, 106, 1, 54, 1, 27, 1, 64, 1, 1, 115, 31, 0, 55, 4, 14, 98, 121, 116, 101, 115, 45,
    114, 101, 97, 100, 97, 98, 108, 101, 0, 1, 56, 4, 14, 98, 121, 116, 101, 115, 45, 119, 114,
    105, 116, 97, 98, 108, 101, 0, 1, 56, 1, 64, 1, 10, 99, 111, 110, 110, 101, 99, 116, 105, 111,
    110, 31, 0, 127, 4, 12, 105, 115, 45, 99, 111, 110, 110, 101, 99, 116, 101, 100, 0, 1, 57, 1,
    64, 1, 8, 108, 105, 115, 116, 101, 110, 101, 114, 9, 1, 0, 4, 18, 99, 108, 111, 115, 101, 45,
    116, 99, 112, 45, 108, 105, 115, 116, 101, 110, 101, 114, 0, 1, 58, 4, 16, 99, 108, 111, 115,
    101, 45, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 0, 1, 58, 3, 8, 119, 97, 115, 105, 45,
    116, 99, 112, 22, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 116, 99, 112, 47, 119, 97, 115,
    105, 45, 116, 99, 112, 5, 17, 1, 66, 14, 1, 111, 8, 123, 123, 123, 123, 123, 123, 123, 123, 4,
    12, 105, 112, 118, 54, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 0, 1, 114, 4, 7, 97, 100,
    100, 114, 101, 115, 115, 1, 4, 112, 111, 114, 116, 123, 9, 102, 108, 111, 119, 45, 105, 110,
    102, 111, 121, 8, 115, 99, 111, 112, 101, 45, 105, 100, 121, 4, 19, 105, 112, 118, 54, 45, 115,
    111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 2, 1, 111, 4, 125, 125,
    125, 125, 4, 12, 105, 112, 118, 52, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 4, 1, 114,
    2, 7, 97, 100, 100, 114, 101, 115, 115, 5, 4, 112, 111, 114, 116, 123, 4, 19, 105, 112, 118,
    52, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 6, 1, 113,
    2, 4, 105, 112, 118, 52, 1, 7, 0, 4, 105, 112, 118, 54, 1, 3, 0, 4, 17, 105, 112, 45, 115, 111,
    99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 8, 1, 109, 2, 4, 105, 112,
    118, 52, 4, 105, 112, 118, 54, 4, 17, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102,
    97, 109, 105, 108, 121, 0, 3, 0, 10, 1, 113, 2, 4, 105, 112, 118, 52, 1, 5, 0, 4, 105, 112,
    118, 54, 1, 1, 0, 4, 10, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 12, 3, 7,
    119, 97, 115, 105, 45, 105, 112, 20, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 105, 112,
    47, 119, 97, 115, 105, 45, 105, 112, 5, 18, 1, 66, 28, 1, 110, 1, 8, 110, 111, 110, 98, 108,
    111, 99, 107, 4, 14, 114, 101, 115, 111, 108, 118, 101, 114, 45, 102, 108, 97, 103, 115, 0, 3,
    0, 0, 1, 121, 4, 8, 114, 101, 115, 111, 108, 118, 101, 114, 0, 3, 0, 2, 1, 109, 1, 12, 105,
    110, 118, 97, 108, 105, 100, 45, 110, 97, 109, 101, 4, 18, 114, 101, 115, 111, 108, 118, 101,
    45, 110, 97, 109, 101, 45, 101, 114, 114, 111, 114, 0, 3, 0, 4, 1, 109, 2, 11, 119, 111, 117,
    108, 100, 45, 98, 108, 111, 99, 107, 15, 100, 110, 115, 45, 117, 110, 97, 118, 97, 105, 108,
    97, 98, 108, 101, 4, 13, 114, 101, 115, 111, 108, 118, 101, 45, 101, 114, 114, 111, 114, 0, 3,
    0, 6, 1, 121, 4, 7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 8, 1, 111, 8, 123, 123, 123,
    123, 123, 123, 123, 123, 4, 12, 105, 112, 118, 54, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3,
    0, 10, 1, 111, 4, 125, 125, 125, 125, 4, 12, 105, 112, 118, 52, 45, 97, 100, 100, 114, 101,
    115, 115, 0, 3, 0, 12, 1, 109, 2, 4, 105, 112, 118, 52, 4, 105, 112, 118, 54, 4, 17, 105, 112,
    45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 3, 0, 14, 1, 113, 2,
    4, 105, 112, 118, 52, 1, 13, 0, 4, 105, 112, 118, 54, 1, 11, 0, 4, 10, 105, 112, 45, 97, 100,
    100, 114, 101, 115, 115, 0, 3, 0, 16, 1, 107, 15, 1, 106, 1, 3, 1, 5, 1, 64, 4, 7, 110, 101,
    116, 119, 111, 114, 107, 9, 4, 110, 97, 109, 101, 115, 14, 97, 100, 100, 114, 101, 115, 115,
    45, 102, 97, 109, 105, 108, 121, 18, 5, 102, 108, 97, 103, 115, 1, 0, 19, 4, 12, 114, 101, 115,
    111, 108, 118, 101, 45, 110, 97, 109, 101, 0, 1, 20, 1, 107, 17, 1, 106, 1, 21, 1, 7, 1, 64, 1,
    8, 114, 101, 115, 111, 108, 118, 101, 114, 3, 0, 22, 4, 12, 114, 101, 115, 111, 108, 118, 101,
    45, 110, 101, 120, 116, 0, 1, 23, 1, 64, 1, 8, 114, 101, 115, 111, 108, 118, 101, 114, 3, 1, 0,
    4, 14, 99, 108, 111, 115, 101, 45, 114, 101, 115, 111, 108, 118, 101, 114, 0, 1, 24, 3, 8, 119,
    97, 115, 105, 45, 100, 110, 115, 22, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 100, 110,
    115, 47, 119, 97, 115, 105, 45, 100, 110, 115, 5, 19, 1, 66, 3, 1, 106, 0, 0, 1, 64, 1, 6, 115,
    116, 97, 116, 117, 115, 0, 1, 0, 4, 4, 101, 120, 105, 116, 0, 1, 1, 3, 9, 119, 97, 115, 105,
    45, 101, 120, 105, 116, 24, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 101, 120, 105, 116,
    47, 119, 97, 115, 105, 45, 101, 120, 105, 116, 5, 20, 1, 112, 115, 1, 111, 2, 115, 115, 1, 112,
    22, 1, 111, 2, 121, 115, 1, 112, 24, 1, 106, 0, 0, 1, 64, 5, 5, 115, 116, 100, 105, 110, 121,
    6, 115, 116, 100, 111, 117, 116, 121, 4, 97, 114, 103, 115, 21, 8, 101, 110, 118, 45, 118, 97,
    114, 115, 23, 8, 112, 114, 101, 111, 112, 101, 110, 115, 25, 0, 26, 4, 7, 99, 111, 109, 109,
    97, 110, 100, 0, 1, 27, 4, 12, 119, 97, 115, 105, 45, 99, 111, 109, 109, 97, 110, 100, 30, 112,
    107, 103, 58, 47, 119, 97, 115, 105, 45, 99, 111, 109, 109, 97, 110, 100, 47, 119, 97, 115,
    105, 45, 99, 111, 109, 109, 97, 110, 100, 4, 0, 11, 35, 1, 12, 119, 97, 115, 105, 45, 99, 111,
    109, 109, 97, 110, 100, 17, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 99, 111, 109, 109,
    97, 110, 100, 3, 24, 0, 7, 154, 50, 1, 65, 2, 1, 65, 34, 1, 66, 20, 1, 121, 4, 10, 119, 97,
    108, 108, 45, 99, 108, 111, 99, 107, 0, 3, 0, 0, 1, 121, 4, 8, 112, 111, 108, 108, 97, 98, 108,
    101, 0, 3, 0, 2, 1, 121, 4, 15, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111,
    99, 107, 0, 3, 0, 4, 1, 119, 4, 7, 105, 110, 115, 116, 97, 110, 116, 0, 3, 0, 6, 1, 114, 2, 7,
    115, 101, 99, 111, 110, 100, 115, 119, 11, 110, 97, 110, 111, 115, 101, 99, 111, 110, 100, 115,
    121, 4, 8, 100, 97, 116, 101, 116, 105, 109, 101, 0, 3, 0, 8, 1, 64, 1, 5, 99, 108, 111, 99,
    107, 5, 0, 7, 4, 19, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 45,
    110, 111, 119, 0, 1, 10, 4, 26, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111,
    99, 107, 45, 114, 101, 115, 111, 108, 117, 116, 105, 111, 110, 0, 1, 10, 1, 64, 1, 5, 99, 108,
    111, 99, 107, 1, 0, 9, 4, 14, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 45, 110, 111, 119,
    0, 1, 11, 4, 21, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 45, 114, 101, 115, 111, 108,
    117, 116, 105, 111, 110, 0, 1, 11, 1, 64, 1, 5, 99, 108, 111, 99, 107, 5, 1, 0, 4, 21, 99, 108,
    111, 115, 101, 45, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 1,
    12, 1, 64, 1, 5, 99, 108, 111, 99, 107, 1, 1, 0, 4, 16, 99, 108, 111, 115, 101, 45, 119, 97,
    108, 108, 45, 99, 108, 111, 99, 107, 0, 1, 13, 3, 11, 119, 97, 115, 105, 45, 99, 108, 111, 99,
    107, 115, 28, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 99, 108, 111, 99, 107, 115, 47,
    119, 97, 115, 105, 45, 99, 108, 111, 99, 107, 115, 5, 0, 2, 3, 0, 0, 15, 109, 111, 110, 111,
    116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 2, 3, 0, 0, 10, 119, 97, 108, 108, 45, 99,
    108, 111, 99, 107, 1, 66, 8, 2, 3, 2, 1, 1, 4, 15, 109, 111, 110, 111, 116, 111, 110, 105, 99,
    45, 99, 108, 111, 99, 107, 0, 3, 0, 0, 2, 3, 2, 1, 2, 4, 10, 119, 97, 108, 108, 45, 99, 108,
    111, 99, 107, 0, 3, 0, 2, 1, 64, 0, 0, 1, 4, 23, 100, 101, 102, 97, 117, 108, 116, 45, 109,
    111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 1, 4, 1, 64, 0, 0, 3, 4,
    18, 100, 101, 102, 97, 117, 108, 116, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 1,
    5, 3, 19, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 99, 108, 111, 99, 107,
    115, 36, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 99, 108, 111, 99, 107, 115, 47, 119, 97,
    115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 99, 108, 111, 99, 107, 115, 5, 3, 1, 66, 4,
    1, 109, 5, 5, 116, 114, 97, 99, 101, 5, 100, 101, 98, 117, 103, 4, 105, 110, 102, 111, 4, 119,
    97, 114, 110, 5, 101, 114, 114, 111, 114, 4, 5, 108, 101, 118, 101, 108, 0, 3, 0, 0, 1, 64, 3,
    5, 108, 101, 118, 101, 108, 1, 7, 99, 111, 110, 116, 101, 120, 116, 115, 7, 109, 101, 115, 115,
    97, 103, 101, 115, 1, 0, 4, 3, 108, 111, 103, 0, 1, 2, 3, 12, 119, 97, 115, 105, 45, 108, 111,
    103, 103, 105, 110, 103, 30, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 108, 111, 103, 103,
    105, 110, 103, 47, 119, 97, 115, 105, 45, 108, 111, 103, 103, 105, 110, 103, 5, 4, 1, 66, 7, 1,
    64, 1, 7, 109, 101, 115, 115, 97, 103, 101, 115, 1, 0, 4, 5, 112, 114, 105, 110, 116, 0, 1, 0,
    1, 64, 0, 0, 127, 4, 11, 105, 115, 45, 116, 101, 114, 109, 105, 110, 97, 108, 0, 1, 1, 1, 107,
    123, 1, 64, 0, 0, 2, 4, 11, 110, 117, 109, 45, 99, 111, 108, 117, 109, 110, 115, 0, 1, 3, 3,
    11, 119, 97, 115, 105, 45, 115, 116, 100, 101, 114, 114, 28, 112, 107, 103, 58, 47, 119, 97,
    115, 105, 45, 115, 116, 100, 101, 114, 114, 47, 119, 97, 115, 105, 45, 115, 116, 100, 101, 114,
    114, 5, 5, 1, 66, 28, 1, 114, 0, 4, 12, 115, 116, 114, 101, 97, 109, 45, 101, 114, 114, 111,
    114, 0, 3, 0, 0, 1, 121, 4, 13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109,
    0, 3, 0, 2, 1, 121, 4, 12, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0,
    4, 1, 112, 125, 1, 111, 2, 6, 127, 1, 106, 1, 7, 1, 1, 1, 64, 2, 3, 115, 114, 99, 5, 3, 108,
    101, 110, 119, 0, 8, 4, 4, 114, 101, 97, 100, 0, 1, 9, 1, 111, 2, 119, 127, 1, 106, 1, 10, 1,
    1, 1, 64, 2, 3, 115, 114, 99, 5, 3, 108, 101, 110, 119, 0, 11, 4, 4, 115, 107, 105, 112, 0, 1,
    12, 1, 106, 1, 119, 1, 1, 1, 64, 2, 3, 100, 115, 116, 3, 3, 98, 117, 102, 6, 0, 13, 4, 5, 119,
    114, 105, 116, 101, 0, 1, 14, 1, 64, 3, 3, 100, 115, 116, 3, 4, 98, 121, 116, 101, 125, 3, 108,
    101, 110, 119, 0, 13, 4, 14, 119, 114, 105, 116, 101, 45, 114, 101, 112, 101, 97, 116, 101,
    100, 0, 1, 15, 1, 64, 3, 3, 100, 115, 116, 3, 3, 115, 114, 99, 5, 3, 108, 101, 110, 119, 0, 11,
    4, 6, 115, 112, 108, 105, 99, 101, 0, 1, 16, 1, 64, 2, 3, 100, 115, 116, 3, 3, 115, 114, 99, 5,
    0, 13, 4, 7, 102, 111, 114, 119, 97, 114, 100, 0, 1, 17, 1, 64, 1, 1, 102, 5, 1, 0, 4, 17, 100,
    114, 111, 112, 45, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 1, 18, 1, 64,
    1, 1, 102, 3, 1, 0, 4, 18, 100, 114, 111, 112, 45, 111, 117, 116, 112, 117, 116, 45, 115, 116,
    114, 101, 97, 109, 0, 1, 19, 3, 7, 119, 97, 115, 105, 45, 105, 111, 20, 112, 107, 103, 58, 47,
    119, 97, 115, 105, 45, 105, 111, 47, 119, 97, 115, 105, 45, 105, 111, 5, 6, 2, 3, 0, 4, 12,
    105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 2, 3, 0, 4, 13, 111, 117, 116, 112,
    117, 116, 45, 115, 116, 114, 101, 97, 109, 2, 3, 0, 0, 8, 100, 97, 116, 101, 116, 105, 109,
    101, 1, 66, 119, 2, 3, 2, 1, 7, 4, 12, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97,
    109, 0, 3, 0, 0, 2, 3, 2, 1, 8, 4, 13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101,
    97, 109, 0, 3, 0, 2, 2, 3, 2, 1, 9, 4, 8, 100, 97, 116, 101, 116, 105, 109, 101, 0, 3, 0, 4, 1,
    121, 4, 4, 115, 105, 122, 101, 0, 3, 0, 6, 1, 110, 4, 6, 99, 114, 101, 97, 116, 101, 9, 100,
    105, 114, 101, 99, 116, 111, 114, 121, 4, 101, 120, 99, 108, 5, 116, 114, 117, 110, 99, 4, 7,
    111, 45, 102, 108, 97, 103, 115, 0, 3, 0, 8, 1, 110, 3, 8, 114, 101, 97, 100, 97, 98, 108, 101,
    9, 119, 114, 105, 116, 101, 97, 98, 108, 101, 10, 101, 120, 101, 99, 117, 116, 97, 98, 108,
    101, 4, 4, 109, 111, 100, 101, 0, 3, 0, 10, 1, 119, 4, 9, 108, 105, 110, 107, 99, 111, 117,
    110, 116, 0, 3, 0, 12, 1, 119, 4, 5, 105, 110, 111, 100, 101, 0, 3, 0, 14, 1, 119, 4, 8, 102,
    105, 108, 101, 115, 105, 122, 101, 0, 3, 0, 16, 1, 120, 4, 9, 102, 105, 108, 101, 100, 101,
    108, 116, 97, 0, 3, 0, 18, 1, 109, 38, 6, 97, 99, 99, 101, 115, 115, 5, 97, 103, 97, 105, 110,
    7, 97, 108, 114, 101, 97, 100, 121, 4, 98, 97, 100, 102, 4, 98, 117, 115, 121, 6, 100, 101, 97,
    100, 108, 107, 5, 100, 113, 117, 111, 116, 5, 101, 120, 105, 115, 116, 4, 102, 98, 105, 103, 5,
    105, 108, 115, 101, 113, 10, 105, 110, 112, 114, 111, 103, 114, 101, 115, 115, 4, 105, 110,
    116, 114, 5, 105, 110, 118, 97, 108, 2, 105, 111, 5, 105, 115, 100, 105, 114, 4, 108, 111, 111,
    112, 5, 109, 108, 105, 110, 107, 7, 109, 115, 103, 115, 105, 122, 101, 11, 110, 97, 109, 101,
    116, 111, 111, 108, 111, 110, 103, 5, 110, 111, 100, 101, 118, 5, 110, 111, 101, 110, 116, 5,
    110, 111, 108, 99, 107, 5, 110, 111, 109, 101, 109, 5, 110, 111, 115, 112, 99, 5, 110, 111,
    115, 121, 115, 6, 110, 111, 116, 100, 105, 114, 8, 110, 111, 116, 101, 109, 112, 116, 121, 14,
    110, 111, 116, 114, 101, 99, 111, 118, 101, 114, 97, 98, 108, 101, 6, 110, 111, 116, 115, 117,
    112, 5, 110, 111, 116, 116, 121, 4, 110, 120, 105, 111, 8, 111, 118, 101, 114, 102, 108, 111,
    119, 4, 112, 101, 114, 109, 4, 112, 105, 112, 101, 4, 114, 111, 102, 115, 5, 115, 112, 105,
    112, 101, 6, 116, 120, 116, 98, 115, 121, 4, 120, 100, 101, 118, 4, 5, 101, 114, 114, 110, 111,
    0, 3, 0, 20, 1, 121, 4, 16, 100, 105, 114, 45, 101, 110, 116, 114, 121, 45, 115, 116, 114, 101,
    97, 109, 0, 3, 0, 22, 1, 119, 4, 6, 100, 101, 118, 105, 99, 101, 0, 3, 0, 24, 1, 109, 8, 7,
    117, 110, 107, 110, 111, 119, 110, 12, 98, 108, 111, 99, 107, 45, 100, 101, 118, 105, 99, 101,
    16, 99, 104, 97, 114, 97, 99, 116, 101, 114, 45, 100, 101, 118, 105, 99, 101, 9, 100, 105, 114,
    101, 99, 116, 111, 114, 121, 4, 102, 105, 102, 111, 13, 115, 121, 109, 98, 111, 108, 105, 99,
    45, 108, 105, 110, 107, 12, 114, 101, 103, 117, 108, 97, 114, 45, 102, 105, 108, 101, 6, 115,
    111, 99, 107, 101, 116, 4, 15, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 45, 116, 121,
    112, 101, 0, 3, 0, 26, 1, 107, 15, 1, 114, 3, 3, 105, 110, 111, 28, 4, 116, 121, 112, 101, 27,
    4, 110, 97, 109, 101, 115, 4, 9, 100, 105, 114, 45, 101, 110, 116, 114, 121, 0, 3, 0, 29, 1,
    110, 6, 4, 114, 101, 97, 100, 5, 119, 114, 105, 116, 101, 5, 100, 115, 121, 110, 99, 8, 110,
    111, 110, 98, 108, 111, 99, 107, 5, 114, 115, 121, 110, 99, 4, 115, 121, 110, 99, 4, 16, 100,
    101, 115, 99, 114, 105, 112, 116, 111, 114, 45, 102, 108, 97, 103, 115, 0, 3, 0, 31, 1, 121, 4,
    10, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 0, 3, 0, 33, 1, 113, 3, 9, 110, 111, 45,
    99, 104, 97, 110, 103, 101, 0, 0, 3, 110, 111, 119, 0, 0, 9, 116, 105, 109, 101, 115, 116, 97,
    109, 112, 1, 5, 0, 4, 13, 110, 101, 119, 45, 116, 105, 109, 101, 115, 116, 97, 109, 112, 0, 3,
    0, 35, 1, 114, 8, 3, 100, 101, 118, 25, 3, 105, 110, 111, 15, 4, 116, 121, 112, 101, 27, 5,
    110, 108, 105, 110, 107, 13, 4, 115, 105, 122, 101, 17, 4, 97, 116, 105, 109, 5, 4, 109, 116,
    105, 109, 5, 4, 99, 116, 105, 109, 5, 4, 15, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114,
    45, 115, 116, 97, 116, 0, 3, 0, 37, 1, 110, 1, 14, 115, 121, 109, 108, 105, 110, 107, 45, 102,
    111, 108, 108, 111, 119, 4, 8, 97, 116, 45, 102, 108, 97, 103, 115, 0, 3, 0, 39, 1, 109, 6, 6,
    110, 111, 114, 109, 97, 108, 10, 115, 101, 113, 117, 101, 110, 116, 105, 97, 108, 6, 114, 97,
    110, 100, 111, 109, 9, 119, 105, 108, 108, 45, 110, 101, 101, 100, 9, 100, 111, 110, 116, 45,
    110, 101, 101, 100, 8, 110, 111, 45, 114, 101, 117, 115, 101, 4, 6, 97, 100, 118, 105, 99, 101,
    0, 3, 0, 41, 1, 106, 0, 1, 21, 1, 64, 4, 2, 102, 100, 34, 6, 111, 102, 102, 115, 101, 116, 17,
    3, 108, 101, 110, 17, 6, 97, 100, 118, 105, 99, 101, 42, 0, 43, 4, 7, 102, 97, 100, 118, 105,
    115, 101, 0, 1, 44, 1, 64, 1, 2, 102, 100, 34, 0, 43, 4, 8, 100, 97, 116, 97, 115, 121, 110,
    99, 0, 1, 45, 1, 106, 1, 32, 1, 21, 1, 64, 1, 2, 102, 100, 34, 0, 46, 4, 5, 102, 108, 97, 103,
    115, 0, 1, 47, 1, 106, 1, 27, 1, 21, 1, 64, 1, 2, 102, 100, 34, 0, 48, 4, 9, 116, 111, 100,
    111, 45, 116, 121, 112, 101, 0, 1, 49, 1, 64, 2, 2, 102, 100, 34, 5, 102, 108, 97, 103, 115,
    32, 0, 43, 4, 9, 115, 101, 116, 45, 102, 108, 97, 103, 115, 0, 1, 50, 1, 64, 2, 2, 102, 100,
    34, 4, 115, 105, 122, 101, 17, 0, 43, 4, 8, 115, 101, 116, 45, 115, 105, 122, 101, 0, 1, 51, 1,
    64, 3, 2, 102, 100, 34, 4, 97, 116, 105, 109, 36, 4, 109, 116, 105, 109, 36, 0, 43, 4, 9, 115,
    101, 116, 45, 116, 105, 109, 101, 115, 0, 1, 52, 1, 106, 1, 1, 1, 21, 1, 64, 2, 2, 102, 100,
    34, 6, 111, 102, 102, 115, 101, 116, 17, 0, 53, 4, 15, 114, 101, 97, 100, 45, 118, 105, 97, 45,
    115, 116, 114, 101, 97, 109, 0, 1, 54, 1, 106, 1, 3, 1, 21, 1, 64, 2, 2, 102, 100, 34, 6, 111,
    102, 102, 115, 101, 116, 17, 0, 55, 4, 16, 119, 114, 105, 116, 101, 45, 118, 105, 97, 45, 115,
    116, 114, 101, 97, 109, 0, 1, 56, 1, 64, 1, 2, 102, 100, 34, 0, 55, 4, 17, 97, 112, 112, 101,
    110, 100, 45, 118, 105, 97, 45, 115, 116, 114, 101, 97, 109, 0, 1, 57, 1, 112, 125, 1, 111, 2,
    58, 127, 1, 106, 1, 59, 1, 21, 1, 64, 3, 2, 102, 100, 34, 3, 108, 101, 110, 7, 6, 111, 102,
    102, 115, 101, 116, 17, 0, 60, 4, 5, 112, 114, 101, 97, 100, 0, 1, 61, 1, 106, 1, 7, 1, 21, 1,
    64, 3, 2, 102, 100, 34, 3, 98, 117, 102, 58, 6, 111, 102, 102, 115, 101, 116, 17, 0, 62, 4, 6,
    112, 119, 114, 105, 116, 101, 0, 1, 63, 1, 106, 1, 23, 1, 21, 1, 64, 1, 2, 102, 100, 34, 0,
    192, 0, 4, 7, 114, 101, 97, 100, 100, 105, 114, 0, 1, 65, 1, 64, 1, 1, 115, 23, 1, 0, 4, 22,
    99, 108, 111, 115, 101, 45, 100, 105, 114, 45, 101, 110, 116, 114, 121, 45, 115, 116, 114, 101,
    97, 109, 0, 1, 66, 1, 107, 30, 1, 106, 1, 195, 0, 1, 21, 1, 64, 1, 10, 100, 105, 114, 45, 115,
    116, 114, 101, 97, 109, 23, 0, 196, 0, 4, 14, 114, 101, 97, 100, 45, 100, 105, 114, 45, 101,
    110, 116, 114, 121, 0, 1, 69, 4, 4, 115, 121, 110, 99, 0, 1, 45, 1, 64, 2, 2, 102, 100, 34, 4,
    112, 97, 116, 104, 115, 0, 43, 4, 19, 99, 114, 101, 97, 116, 101, 45, 100, 105, 114, 101, 99,
    116, 111, 114, 121, 45, 97, 116, 0, 1, 70, 1, 106, 1, 38, 1, 21, 1, 64, 1, 2, 102, 100, 34, 0,
    199, 0, 4, 4, 115, 116, 97, 116, 0, 1, 72, 1, 64, 3, 2, 102, 100, 34, 8, 97, 116, 45, 102, 108,
    97, 103, 115, 40, 4, 112, 97, 116, 104, 115, 0, 199, 0, 4, 7, 115, 116, 97, 116, 45, 97, 116,
    0, 1, 73, 1, 64, 5, 2, 102, 100, 34, 8, 97, 116, 45, 102, 108, 97, 103, 115, 40, 4, 112, 97,
    116, 104, 115, 4, 97, 116, 105, 109, 36, 4, 109, 116, 105, 109, 36, 0, 43, 4, 12, 115, 101,
    116, 45, 116, 105, 109, 101, 115, 45, 97, 116, 0, 1, 74, 1, 64, 5, 2, 102, 100, 34, 12, 111,
    108, 100, 45, 97, 116, 45, 102, 108, 97, 103, 115, 40, 8, 111, 108, 100, 45, 112, 97, 116, 104,
    115, 14, 110, 101, 119, 45, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 34, 8, 110, 101,
    119, 45, 112, 97, 116, 104, 115, 0, 43, 4, 7, 108, 105, 110, 107, 45, 97, 116, 0, 1, 75, 1,
    106, 1, 34, 1, 21, 1, 64, 6, 2, 102, 100, 34, 8, 97, 116, 45, 102, 108, 97, 103, 115, 40, 4,
    112, 97, 116, 104, 115, 7, 111, 45, 102, 108, 97, 103, 115, 9, 5, 102, 108, 97, 103, 115, 32,
    4, 109, 111, 100, 101, 11, 0, 204, 0, 4, 7, 111, 112, 101, 110, 45, 97, 116, 0, 1, 77, 1, 64,
    1, 2, 102, 100, 34, 1, 0, 4, 5, 99, 108, 111, 115, 101, 0, 1, 78, 1, 106, 1, 115, 1, 21, 1, 64,
    2, 2, 102, 100, 34, 4, 112, 97, 116, 104, 115, 0, 207, 0, 4, 11, 114, 101, 97, 100, 108, 105,
    110, 107, 45, 97, 116, 0, 1, 80, 4, 19, 114, 101, 109, 111, 118, 101, 45, 100, 105, 114, 101,
    99, 116, 111, 114, 121, 45, 97, 116, 0, 1, 70, 1, 64, 4, 2, 102, 100, 34, 8, 111, 108, 100, 45,
    112, 97, 116, 104, 115, 14, 110, 101, 119, 45, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114,
    34, 8, 110, 101, 119, 45, 112, 97, 116, 104, 115, 0, 43, 4, 9, 114, 101, 110, 97, 109, 101, 45,
    97, 116, 0, 1, 81, 1, 64, 3, 2, 102, 100, 34, 8, 111, 108, 100, 45, 112, 97, 116, 104, 115, 8,
    110, 101, 119, 45, 112, 97, 116, 104, 115, 0, 43, 4, 10, 115, 121, 109, 108, 105, 110, 107, 45,
    97, 116, 0, 1, 82, 4, 14, 117, 110, 108, 105, 110, 107, 45, 102, 105, 108, 101, 45, 97, 116, 0,
    1, 70, 1, 64, 4, 2, 102, 100, 34, 8, 97, 116, 45, 102, 108, 97, 103, 115, 40, 4, 112, 97, 116,
    104, 115, 4, 109, 111, 100, 101, 11, 0, 43, 4, 26, 99, 104, 97, 110, 103, 101, 45, 102, 105,
    108, 101, 45, 112, 101, 114, 109, 105, 115, 115, 105, 111, 110, 115, 45, 97, 116, 0, 1, 83, 4,
    31, 99, 104, 97, 110, 103, 101, 45, 100, 105, 114, 101, 99, 116, 111, 114, 121, 45, 112, 101,
    114, 109, 105, 115, 115, 105, 111, 110, 115, 45, 97, 116, 0, 1, 83, 4, 11, 108, 111, 99, 107,
    45, 115, 104, 97, 114, 101, 100, 0, 1, 45, 4, 14, 108, 111, 99, 107, 45, 101, 120, 99, 108,
    117, 115, 105, 118, 101, 0, 1, 45, 4, 15, 116, 114, 121, 45, 108, 111, 99, 107, 45, 115, 104,
    97, 114, 101, 100, 0, 1, 45, 4, 18, 116, 114, 121, 45, 108, 111, 99, 107, 45, 101, 120, 99,
    108, 117, 115, 105, 118, 101, 0, 1, 45, 4, 6, 117, 110, 108, 111, 99, 107, 0, 1, 45, 3, 15,
    119, 97, 115, 105, 45, 102, 105, 108, 101, 115, 121, 115, 116, 101, 109, 36, 112, 107, 103, 58,
    47, 119, 97, 115, 105, 45, 102, 105, 108, 101, 115, 121, 115, 116, 101, 109, 47, 119, 97, 115,
    105, 45, 102, 105, 108, 101, 115, 121, 115, 116, 101, 109, 5, 10, 1, 66, 5, 1, 112, 125, 1, 64,
    1, 3, 108, 101, 110, 121, 0, 0, 4, 16, 103, 101, 116, 45, 114, 97, 110, 100, 111, 109, 45, 98,
    121, 116, 101, 115, 0, 1, 1, 1, 64, 0, 0, 119, 4, 14, 103, 101, 116, 45, 114, 97, 110, 100,
    111, 109, 45, 117, 54, 52, 0, 1, 2, 3, 11, 119, 97, 115, 105, 45, 114, 97, 110, 100, 111, 109,
    28, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 114, 97, 110, 100, 111, 109, 47, 119, 97,
    115, 105, 45, 114, 97, 110, 100, 111, 109, 5, 11, 2, 3, 0, 0, 7, 105, 110, 115, 116, 97, 110,
    116, 1, 66, 26, 2, 3, 2, 1, 2, 4, 10, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 3, 0, 0,
    2, 3, 2, 1, 1, 4, 15, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 0,
    3, 0, 2, 2, 3, 2, 1, 9, 4, 8, 100, 97, 116, 101, 116, 105, 109, 101, 0, 3, 0, 4, 2, 3, 2, 1,
    12, 4, 7, 105, 110, 115, 116, 97, 110, 116, 0, 3, 0, 6, 2, 3, 2, 1, 7, 4, 12, 105, 110, 112,
    117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 8, 2, 3, 2, 1, 8, 4, 13, 111, 117, 116,
    112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 10, 1, 121, 4, 8, 112, 111, 108, 108,
    97, 98, 108, 101, 0, 3, 0, 12, 1, 64, 1, 1, 102, 13, 1, 0, 4, 13, 100, 114, 111, 112, 45, 112,
    111, 108, 108, 97, 98, 108, 101, 0, 1, 14, 1, 64, 1, 1, 115, 9, 0, 13, 4, 14, 115, 117, 98,
    115, 99, 114, 105, 98, 101, 45, 114, 101, 97, 100, 0, 1, 15, 1, 64, 1, 1, 115, 11, 0, 13, 4,
    15, 115, 117, 98, 115, 99, 114, 105, 98, 101, 45, 119, 114, 105, 116, 101, 0, 1, 16, 1, 64, 3,
    5, 99, 108, 111, 99, 107, 3, 4, 119, 104, 101, 110, 7, 8, 97, 98, 115, 111, 108, 117, 116, 101,
    127, 0, 13, 4, 25, 115, 117, 98, 115, 99, 114, 105, 98, 101, 45, 109, 111, 110, 111, 116, 111,
    110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 1, 17, 1, 112, 13, 1, 112, 125, 1, 64, 1, 2, 105,
    110, 18, 0, 19, 4, 11, 112, 111, 108, 108, 45, 111, 110, 101, 111, 102, 102, 0, 1, 20, 3, 9,
    119, 97, 115, 105, 45, 112, 111, 108, 108, 24, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45,
    112, 111, 108, 108, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 5, 13, 1, 66, 2, 1, 121, 4,
    7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 0, 3, 8, 119, 97, 115, 105, 45, 110, 101, 116,
    22, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 110, 101, 116, 47, 119, 97, 115, 105, 45,
    110, 101, 116, 5, 14, 2, 3, 0, 7, 8, 112, 111, 108, 108, 97, 98, 108, 101, 2, 3, 0, 8, 7, 110,
    101, 116, 119, 111, 114, 107, 1, 66, 76, 2, 3, 2, 1, 15, 4, 8, 112, 111, 108, 108, 97, 98, 108,
    101, 0, 3, 0, 0, 2, 3, 2, 1, 7, 4, 12, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97,
    109, 0, 3, 0, 2, 2, 3, 2, 1, 8, 4, 13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101,
    97, 109, 0, 3, 0, 4, 2, 3, 2, 1, 16, 4, 7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 6, 1,
    121, 4, 12, 116, 99, 112, 45, 108, 105, 115, 116, 101, 110, 101, 114, 0, 3, 0, 8, 1, 110, 1, 8,
    110, 111, 110, 98, 108, 111, 99, 107, 4, 14, 108, 105, 115, 116, 101, 110, 101, 114, 45, 102,
    108, 97, 103, 115, 0, 3, 0, 10, 1, 121, 4, 8, 108, 105, 115, 116, 101, 110, 101, 114, 0, 3, 0,
    12, 1, 111, 8, 123, 123, 123, 123, 123, 123, 123, 123, 4, 12, 105, 112, 118, 54, 45, 97, 100,
    100, 114, 101, 115, 115, 0, 3, 0, 14, 1, 114, 4, 7, 97, 100, 100, 114, 101, 115, 115, 15, 4,
    112, 111, 114, 116, 123, 9, 102, 108, 111, 119, 45, 105, 110, 102, 111, 121, 8, 115, 99, 111,
    112, 101, 45, 105, 100, 121, 4, 19, 105, 112, 118, 54, 45, 115, 111, 99, 107, 101, 116, 45, 97,
    100, 100, 114, 101, 115, 115, 0, 3, 0, 16, 1, 111, 4, 125, 125, 125, 125, 4, 12, 105, 112, 118,
    52, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 18, 1, 114, 2, 7, 97, 100, 100, 114, 101,
    115, 115, 19, 4, 112, 111, 114, 116, 123, 4, 19, 105, 112, 118, 52, 45, 115, 111, 99, 107, 101,
    116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 20, 1, 113, 2, 4, 105, 112, 118, 52, 1, 21,
    0, 4, 105, 112, 118, 54, 1, 17, 0, 4, 17, 105, 112, 45, 115, 111, 99, 107, 101, 116, 45, 97,
    100, 100, 114, 101, 115, 115, 0, 3, 0, 22, 1, 119, 4, 7, 105, 111, 45, 115, 105, 122, 101, 0,
    3, 0, 24, 1, 109, 36, 6, 97, 99, 99, 101, 115, 115, 9, 97, 100, 100, 114, 105, 110, 117, 115,
    101, 12, 97, 100, 100, 114, 110, 111, 116, 97, 118, 97, 105, 108, 11, 97, 102, 110, 111, 115,
    117, 112, 112, 111, 114, 116, 5, 97, 103, 97, 105, 110, 7, 97, 108, 114, 101, 97, 100, 121, 4,
    98, 97, 100, 102, 4, 98, 117, 115, 121, 18, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 45,
    97, 98, 111, 114, 116, 101, 100, 18, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 45, 114,
    101, 102, 117, 115, 101, 100, 16, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 45, 114, 101,
    115, 101, 116, 8, 100, 101, 97, 100, 108, 111, 99, 107, 11, 100, 101, 115, 116, 97, 100, 100,
    114, 114, 101, 113, 16, 104, 111, 115, 116, 45, 117, 110, 114, 101, 97, 99, 104, 97, 98, 108,
    101, 5, 105, 108, 115, 101, 113, 10, 105, 110, 112, 114, 111, 103, 114, 101, 115, 115, 4, 105,
    110, 116, 114, 5, 105, 110, 118, 97, 108, 2, 105, 111, 6, 105, 115, 99, 111, 110, 110, 7, 109,
    115, 103, 115, 105, 122, 101, 8, 109, 117, 108, 116, 105, 104, 111, 112, 11, 110, 97, 109, 101,
    116, 111, 111, 108, 111, 110, 103, 12, 110, 101, 116, 119, 111, 114, 107, 45, 100, 111, 119,
    110, 13, 110, 101, 116, 119, 111, 114, 107, 45, 114, 101, 115, 101, 116, 19, 110, 101, 116,
    119, 111, 114, 107, 45, 117, 110, 114, 101, 97, 99, 104, 97, 98, 108, 101, 6, 110, 111, 98,
    117, 102, 115, 5, 110, 111, 101, 110, 116, 5, 110, 111, 109, 101, 109, 10, 110, 111, 112, 114,
    111, 116, 111, 111, 112, 116, 5, 110, 111, 115, 121, 115, 14, 110, 111, 116, 114, 101, 99, 111,
    118, 101, 114, 97, 98, 108, 101, 6, 110, 111, 116, 115, 117, 112, 8, 111, 118, 101, 114, 102,
    108, 111, 119, 4, 112, 101, 114, 109, 8, 116, 105, 109, 101, 100, 111, 117, 116, 4, 5, 101,
    114, 114, 110, 111, 0, 3, 0, 26, 1, 110, 3, 9, 107, 101, 101, 112, 97, 108, 105, 118, 101, 8,
    110, 111, 110, 98, 108, 111, 99, 107, 7, 110, 111, 100, 101, 108, 97, 121, 4, 16, 99, 111, 110,
    110, 101, 99, 116, 105, 111, 110, 45, 102, 108, 97, 103, 115, 0, 3, 0, 28, 1, 121, 4, 10, 99,
    111, 110, 110, 101, 99, 116, 105, 111, 110, 0, 3, 0, 30, 1, 107, 121, 1, 106, 1, 9, 1, 27, 1,
    64, 4, 7, 110, 101, 116, 119, 111, 114, 107, 7, 7, 97, 100, 100, 114, 101, 115, 115, 23, 7, 98,
    97, 99, 107, 108, 111, 103, 32, 5, 102, 108, 97, 103, 115, 11, 0, 33, 4, 6, 108, 105, 115, 116,
    101, 110, 0, 1, 34, 1, 111, 3, 31, 3, 5, 1, 106, 1, 35, 1, 27, 1, 64, 2, 8, 108, 105, 115, 116,
    101, 110, 101, 114, 13, 5, 102, 108, 97, 103, 115, 29, 0, 36, 4, 6, 97, 99, 99, 101, 112, 116,
    0, 1, 37, 1, 111, 4, 31, 3, 5, 23, 1, 106, 1, 38, 1, 27, 1, 64, 2, 8, 108, 105, 115, 116, 101,
    110, 101, 114, 9, 5, 102, 108, 97, 103, 115, 29, 0, 39, 4, 10, 97, 99, 99, 101, 112, 116, 45,
    116, 99, 112, 0, 1, 40, 1, 64, 4, 7, 110, 101, 116, 119, 111, 114, 107, 7, 13, 108, 111, 99,
    97, 108, 45, 97, 100, 100, 114, 101, 115, 115, 23, 14, 114, 101, 109, 111, 116, 101, 45, 97,
    100, 100, 114, 101, 115, 115, 23, 5, 102, 108, 97, 103, 115, 29, 0, 36, 4, 7, 99, 111, 110,
    110, 101, 99, 116, 0, 1, 41, 1, 112, 125, 1, 106, 1, 25, 1, 27, 1, 64, 2, 10, 99, 111, 110,
    110, 101, 99, 116, 105, 111, 110, 31, 5, 98, 121, 116, 101, 115, 42, 0, 43, 4, 4, 115, 101,
    110, 100, 0, 1, 44, 1, 111, 2, 42, 127, 1, 106, 1, 45, 1, 27, 1, 64, 2, 10, 99, 111, 110, 110,
    101, 99, 116, 105, 111, 110, 31, 6, 108, 101, 110, 103, 116, 104, 25, 0, 46, 4, 7, 114, 101,
    99, 101, 105, 118, 101, 0, 1, 47, 1, 106, 1, 29, 1, 27, 1, 64, 1, 10, 99, 111, 110, 110, 101,
    99, 116, 105, 111, 110, 31, 0, 48, 4, 9, 103, 101, 116, 45, 102, 108, 97, 103, 115, 0, 1, 49,
    1, 106, 0, 1, 27, 1, 64, 2, 10, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 31, 5, 102,
    108, 97, 103, 115, 29, 0, 50, 4, 9, 115, 101, 116, 45, 102, 108, 97, 103, 115, 0, 1, 51, 1, 64,
    1, 10, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 31, 0, 43, 4, 23, 103, 101, 116, 45,
    114, 101, 99, 101, 105, 118, 101, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0,
    1, 52, 1, 64, 2, 10, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 31, 5, 118, 97, 108, 117,
    101, 25, 0, 50, 4, 23, 115, 101, 116, 45, 114, 101, 99, 101, 105, 118, 101, 45, 98, 117, 102,
    102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 53, 4, 20, 103, 101, 116, 45, 115, 101, 110, 100,
    45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 52, 4, 20, 115, 101, 116, 45,
    115, 101, 110, 100, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 53, 1, 111,
    2, 25, 127, 1, 106, 1, 54, 1, 27, 1, 64, 1, 1, 115, 31, 0, 55, 4, 14, 98, 121, 116, 101, 115,
    45, 114, 101, 97, 100, 97, 98, 108, 101, 0, 1, 56, 4, 14, 98, 121, 116, 101, 115, 45, 119, 114,
    105, 116, 97, 98, 108, 101, 0, 1, 56, 1, 64, 1, 10, 99, 111, 110, 110, 101, 99, 116, 105, 111,
    110, 31, 0, 127, 4, 12, 105, 115, 45, 99, 111, 110, 110, 101, 99, 116, 101, 100, 0, 1, 57, 1,
    64, 1, 8, 108, 105, 115, 116, 101, 110, 101, 114, 9, 1, 0, 4, 18, 99, 108, 111, 115, 101, 45,
    116, 99, 112, 45, 108, 105, 115, 116, 101, 110, 101, 114, 0, 1, 58, 4, 16, 99, 108, 111, 115,
    101, 45, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 0, 1, 58, 3, 8, 119, 97, 115, 105, 45,
    116, 99, 112, 22, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 116, 99, 112, 47, 119, 97, 115,
    105, 45, 116, 99, 112, 5, 17, 1, 66, 14, 1, 111, 8, 123, 123, 123, 123, 123, 123, 123, 123, 4,
    12, 105, 112, 118, 54, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 0, 1, 114, 4, 7, 97, 100,
    100, 114, 101, 115, 115, 1, 4, 112, 111, 114, 116, 123, 9, 102, 108, 111, 119, 45, 105, 110,
    102, 111, 121, 8, 115, 99, 111, 112, 101, 45, 105, 100, 121, 4, 19, 105, 112, 118, 54, 45, 115,
    111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 2, 1, 111, 4, 125, 125,
    125, 125, 4, 12, 105, 112, 118, 52, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 4, 1, 114,
    2, 7, 97, 100, 100, 114, 101, 115, 115, 5, 4, 112, 111, 114, 116, 123, 4, 19, 105, 112, 118,
    52, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 6, 1, 113,
    2, 4, 105, 112, 118, 52, 1, 7, 0, 4, 105, 112, 118, 54, 1, 3, 0, 4, 17, 105, 112, 45, 115, 111,
    99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 8, 1, 109, 2, 4, 105, 112,
    118, 52, 4, 105, 112, 118, 54, 4, 17, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102,
    97, 109, 105, 108, 121, 0, 3, 0, 10, 1, 113, 2, 4, 105, 112, 118, 52, 1, 5, 0, 4, 105, 112,
    118, 54, 1, 1, 0, 4, 10, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 12, 3, 7,
    119, 97, 115, 105, 45, 105, 112, 20, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 105, 112,
    47, 119, 97, 115, 105, 45, 105, 112, 5, 18, 1, 66, 28, 1, 110, 1, 8, 110, 111, 110, 98, 108,
    111, 99, 107, 4, 14, 114, 101, 115, 111, 108, 118, 101, 114, 45, 102, 108, 97, 103, 115, 0, 3,
    0, 0, 1, 121, 4, 8, 114, 101, 115, 111, 108, 118, 101, 114, 0, 3, 0, 2, 1, 109, 1, 12, 105,
    110, 118, 97, 108, 105, 100, 45, 110, 97, 109, 101, 4, 18, 114, 101, 115, 111, 108, 118, 101,
    45, 110, 97, 109, 101, 45, 101, 114, 114, 111, 114, 0, 3, 0, 4, 1, 109, 2, 11, 119, 111, 117,
    108, 100, 45, 98, 108, 111, 99, 107, 15, 100, 110, 115, 45, 117, 110, 97, 118, 97, 105, 108,
    97, 98, 108, 101, 4, 13, 114, 101, 115, 111, 108, 118, 101, 45, 101, 114, 114, 111, 114, 0, 3,
    0, 6, 1, 121, 4, 7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 8, 1, 111, 8, 123, 123, 123,
    123, 123, 123, 123, 123, 4, 12, 105, 112, 118, 54, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3,
    0, 10, 1, 111, 4, 125, 125, 125, 125, 4, 12, 105, 112, 118, 52, 45, 97, 100, 100, 114, 101,
    115, 115, 0, 3, 0, 12, 1, 109, 2, 4, 105, 112, 118, 52, 4, 105, 112, 118, 54, 4, 17, 105, 112,
    45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 3, 0, 14, 1, 113, 2,
    4, 105, 112, 118, 52, 1, 13, 0, 4, 105, 112, 118, 54, 1, 11, 0, 4, 10, 105, 112, 45, 97, 100,
    100, 114, 101, 115, 115, 0, 3, 0, 16, 1, 107, 15, 1, 106, 1, 3, 1, 5, 1, 64, 4, 7, 110, 101,
    116, 119, 111, 114, 107, 9, 4, 110, 97, 109, 101, 115, 14, 97, 100, 100, 114, 101, 115, 115,
    45, 102, 97, 109, 105, 108, 121, 18, 5, 102, 108, 97, 103, 115, 1, 0, 19, 4, 12, 114, 101, 115,
    111, 108, 118, 101, 45, 110, 97, 109, 101, 0, 1, 20, 1, 107, 17, 1, 106, 1, 21, 1, 7, 1, 64, 1,
    8, 114, 101, 115, 111, 108, 118, 101, 114, 3, 0, 22, 4, 12, 114, 101, 115, 111, 108, 118, 101,
    45, 110, 101, 120, 116, 0, 1, 23, 1, 64, 1, 8, 114, 101, 115, 111, 108, 118, 101, 114, 3, 1, 0,
    4, 14, 99, 108, 111, 115, 101, 45, 114, 101, 115, 111, 108, 118, 101, 114, 0, 1, 24, 3, 8, 119,
    97, 115, 105, 45, 100, 110, 115, 22, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 100, 110,
    115, 47, 119, 97, 115, 105, 45, 100, 110, 115, 5, 19, 1, 66, 3, 1, 106, 0, 0, 1, 64, 1, 6, 115,
    116, 97, 116, 117, 115, 0, 1, 0, 4, 4, 101, 120, 105, 116, 0, 1, 1, 3, 9, 119, 97, 115, 105,
    45, 101, 120, 105, 116, 24, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 101, 120, 105, 116,
    47, 119, 97, 115, 105, 45, 101, 120, 105, 116, 5, 20, 4, 4, 119, 97, 115, 105, 14, 112, 107,
    103, 58, 47, 119, 97, 115, 105, 47, 119, 97, 115, 105, 4, 0, 0, 45, 9, 112, 114, 111, 100, 117,
    99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 1, 13, 119,
    105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 5, 48, 46, 54, 46, 48, 11, 19, 1, 4,
    119, 97, 115, 105, 9, 112, 107, 103, 58, 47, 119, 97, 115, 105, 3, 26, 0,
];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
