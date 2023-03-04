
#[allow(clippy::all)]
pub mod wasi_poll{
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
  /// `pollable` lifetimes are not automatically managed. Users must ensure
  /// that they do not outlive the resource they reference.
  pub type Pollable = u32;
  #[allow(clippy::all)]
  /// Dispose of the specified `pollable`, after which it may no longer be used.
  pub fn drop_pollable(this: Pollable,) -> (){
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
    unsafe {
      
      #[link(wasm_import_module = "wasi-poll")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "drop-pollable")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-poll_drop-pollable")]
        fn wit_import(
        _: i32, );
      }
      wit_import(wit_bindgen::rt::as_i32(this));
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
  pub fn poll_oneoff(in_: &[Pollable],) -> wit_bindgen::rt::vec::Vec::<u8>{
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
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
        fn wit_import(
        _: i32, _: i32, _: i32, );
      }
      wit_import(ptr0, len0, ptr1);
      let len2 = *((ptr1 + 4) as *const i32) as usize;
      Vec::from_raw_parts(*((ptr1 + 0) as *const i32) as *mut _, len2, len2)
    }
  }
  
}


#[allow(clippy::all)]
pub mod wasi_monotonic_clock{
  pub type Pollable = super::wasi_poll::Pollable;
  /// A monotonic clock is a clock which has an unspecified initial value, and
  /// successive reads of the clock will produce non-decreasing values.
  /// 
  /// It is intended for measuring elapsed time.
  pub type MonotonicClock = u32;
  /// A timestamp in nanoseconds.
  pub type Instant = u64;
  #[allow(clippy::all)]
  /// Read the current value of the clock.
  /// 
  /// The clock is monotonic, therefore calling this function repeatedly will produce
  /// a sequence of non-decreasing values.
  pub fn now(this: MonotonicClock,) -> Instant{
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
    unsafe {
      
      #[link(wasm_import_module = "wasi-monotonic-clock")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "now")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-monotonic-clock_now")]
        fn wit_import(
        _: i32, ) -> i64;
      }
      let ret = wit_import(wit_bindgen::rt::as_i32(this));
      ret as u64
    }
  }
  #[allow(clippy::all)]
  /// Query the resolution of the clock.
  pub fn resolution(this: MonotonicClock,) -> Instant{
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
    unsafe {
      
      #[link(wasm_import_module = "wasi-monotonic-clock")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "resolution")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-monotonic-clock_resolution")]
        fn wit_import(
        _: i32, ) -> i64;
      }
      let ret = wit_import(wit_bindgen::rt::as_i32(this));
      ret as u64
    }
  }
  #[allow(clippy::all)]
  /// Create a `pollable` which will resolve once the specified time has been reached.
  pub fn subscribe(this: MonotonicClock,when: Instant,absolute: bool,) -> Pollable{
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
    unsafe {
      
      #[link(wasm_import_module = "wasi-monotonic-clock")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "subscribe")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-monotonic-clock_subscribe")]
        fn wit_import(
        _: i32, _: i64, _: i32, ) -> i32;
      }
      let ret = wit_import(wit_bindgen::rt::as_i32(this), wit_bindgen::rt::as_i64(when), match absolute { true => 1, false => 0 });
      ret as u32
    }
  }
  #[allow(clippy::all)]
  /// Dispose of the specified `monotonic-clock`, after which it may no longer
  /// be used.
  pub fn drop_monotonic_clock(this: MonotonicClock,) -> (){
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
    unsafe {
      
      #[link(wasm_import_module = "wasi-monotonic-clock")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "drop-monotonic-clock")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-monotonic-clock_drop-monotonic-clock")]
        fn wit_import(
        _: i32, );
      }
      wit_import(wit_bindgen::rt::as_i32(this));
    }
  }
  
}


#[allow(clippy::all)]
pub mod wasi_wall_clock{
  /// A wall clock is a clock which measures the date and time according to some
  /// external reference.
  /// 
  /// External references may be reset, so this clock is not necessarily
  /// monotonic, making it unsuitable for measuring elapsed time.
  /// 
  /// It is intended for reporting the current date and time for humans.
  pub type WallClock = u32;
  /// A time and date in seconds plus nanoseconds.
  #[repr(C)]
  #[derive(Copy, Clone)]
  pub struct Datetime {
    pub seconds: u64,
    pub nanoseconds: u32,
  }
  impl core::fmt::Debug for Datetime {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      f.debug_struct("Datetime").field("seconds", &self.seconds).field("nanoseconds", &self.nanoseconds).finish()
    }
  }
  #[allow(clippy::all)]
  /// Read the current value of the clock.
  /// 
  /// This clock is not monotonic, therefore calling this function repeatedly will
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
  pub fn now(this: WallClock,) -> Datetime{
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
    unsafe {
      
      #[repr(align(8))]
      struct RetArea([u8; 16]);
      let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
      let ptr0 = ret_area.as_mut_ptr() as i32;
      #[link(wasm_import_module = "wasi-wall-clock")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "now")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-wall-clock_now")]
        fn wit_import(
        _: i32, _: i32, );
      }
      wit_import(wit_bindgen::rt::as_i32(this), ptr0);
      Datetime{seconds:*((ptr0 + 0) as *const i64) as u64, nanoseconds:*((ptr0 + 8) as *const i32) as u32, }
    }
  }
  #[allow(clippy::all)]
  /// Query the resolution of the clock.
  /// 
  /// The nanoseconds field of the output is always less than 1000000000.
  pub fn resolution(this: WallClock,) -> Datetime{
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
    unsafe {
      
      #[repr(align(8))]
      struct RetArea([u8; 16]);
      let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
      let ptr0 = ret_area.as_mut_ptr() as i32;
      #[link(wasm_import_module = "wasi-wall-clock")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "resolution")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-wall-clock_resolution")]
        fn wit_import(
        _: i32, _: i32, );
      }
      wit_import(wit_bindgen::rt::as_i32(this), ptr0);
      Datetime{seconds:*((ptr0 + 0) as *const i64) as u64, nanoseconds:*((ptr0 + 8) as *const i32) as u32, }
    }
  }
  #[allow(clippy::all)]
  /// Dispose of the specified `wall-clock`, after which it may no longer
  /// be used.
  pub fn drop_wall_clock(this: WallClock,) -> (){
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
    unsafe {
      
      #[link(wasm_import_module = "wasi-wall-clock")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "drop-wall-clock")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-wall-clock_drop-wall-clock")]
        fn wit_import(
        _: i32, );
      }
      wit_import(wit_bindgen::rt::as_i32(this));
    }
  }
  
}


#[allow(clippy::all)]
pub mod wasi_timezone{
  /// Information useful for displaying the timezone of a specific `datetime`.
  /// 
  /// This information may vary within a single `timezone` to reflect daylight
  /// saving time adjustments.
  #[derive(Clone)]
  pub struct TimezoneDisplay {
    /// The number of seconds difference between UTC time and the local time of
    /// the timezone.
    /// 
    /// The returned value will always be less than 86400 which is the number of
    /// seconds in a day (24*60*60).
    /// 
    /// In implementations that do not expose an actual time zone, this should
    /// return 0.
    pub utc_offset: i32,
    /// The abbreviated name of the timezone to display to a user. The name `UTC`
    /// indicates Coordinated Universal Time. Otherwise, this should reference
    /// local standards for the name of the time zone.
    /// 
    /// In implementations that do not expose an actual time zone, this should be
    /// the string `UTC`.
    /// 
    /// In time zones that do not have an applicable name, a formatted
    /// representation of the UTC offset may be returned, such as `-04:00`.
    pub name: wit_bindgen::rt::string::String,
    /// Whether daylight saving time is active.
    /// 
    /// In implementations that do not expose an actual time zone, this should
    /// return false.
    pub in_daylight_saving_time: bool,
  }
  impl core::fmt::Debug for TimezoneDisplay {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      f.debug_struct("TimezoneDisplay").field("utc-offset", &self.utc_offset).field("name", &self.name).field("in-daylight-saving-time", &self.in_daylight_saving_time).finish()
    }
  }
  /// A timezone.
  /// 
  /// In timezones that recognize daylight saving time, also known as daylight
  /// time and summer time, the information returned from the functions varies
  /// over time to reflect these adjustments.
  pub type Timezone = u32;
  /// A time and date in seconds plus nanoseconds.
  /// 
  /// TODO: Use the definition from the monotonic clock API instead of defining our own copy.
  #[repr(C)]
  #[derive(Copy, Clone)]
  pub struct Datetime {
    pub seconds: u64,
    pub nanoseconds: u32,
  }
  impl core::fmt::Debug for Datetime {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      f.debug_struct("Datetime").field("seconds", &self.seconds).field("nanoseconds", &self.nanoseconds).finish()
    }
  }
  #[allow(clippy::all)]
  /// Return information needed to display the given `datetime`. This includes
  /// the UTC offset, the time zone name, and a flag indicating whether
  /// daylight saving time is active.
  /// 
  /// If the timezone cannot be determined for the given `datetime`, return a
  /// `timezone-display` for `UTC` with a `utc-offset` of 0 and no daylight
  /// saving time.
  pub fn display(this: Timezone,when: Datetime,) -> TimezoneDisplay{
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
    unsafe {
      
      #[repr(align(4))]
      struct RetArea([u8; 16]);
      let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
      let Datetime{ seconds:seconds0, nanoseconds:nanoseconds0, } = when;
      let ptr1 = ret_area.as_mut_ptr() as i32;
      #[link(wasm_import_module = "wasi-timezone")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "display")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-timezone_display")]
        fn wit_import(
        _: i32, _: i64, _: i32, _: i32, );
      }
      wit_import(wit_bindgen::rt::as_i32(this), wit_bindgen::rt::as_i64(seconds0), wit_bindgen::rt::as_i32(nanoseconds0), ptr1);
      let len2 = *((ptr1 + 8) as *const i32) as usize;
      TimezoneDisplay{utc_offset:*((ptr1 + 0) as *const i32), name:String::from_utf8(Vec::from_raw_parts(*((ptr1 + 4) as *const i32) as *mut _, len2, len2)).unwrap(), in_daylight_saving_time:match i32::from(*((ptr1 + 12) as *const u8)) {
        0 => false,
        1 => true,
        _ => panic!("invalid bool discriminant"),
      }, }
    }
  }
  #[allow(clippy::all)]
  /// The same as `display`, but only return the UTC offset.
  pub fn utc_offset(this: Timezone,when: Datetime,) -> i32{
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
    unsafe {
      let Datetime{ seconds:seconds0, nanoseconds:nanoseconds0, } = when;
      
      #[link(wasm_import_module = "wasi-timezone")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "utc-offset")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-timezone_utc-offset")]
        fn wit_import(
        _: i32, _: i64, _: i32, ) -> i32;
      }
      let ret = wit_import(wit_bindgen::rt::as_i32(this), wit_bindgen::rt::as_i64(seconds0), wit_bindgen::rt::as_i32(nanoseconds0));
      ret
    }
  }
  #[allow(clippy::all)]
  /// Dispose of the specified input-stream, after which it may no longer
  /// be used.
  pub fn drop_timezone(this: Timezone,) -> (){
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
    unsafe {
      
      #[link(wasm_import_module = "wasi-timezone")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "drop-timezone")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-timezone_drop-timezone")]
        fn wit_import(
        _: i32, );
      }
      wit_import(wit_bindgen::rt::as_i32(this));
    }
  }
  
}


#[allow(clippy::all)]
pub mod wasi_default_clocks{
  pub type MonotonicClock = super::wasi_monotonic_clock::MonotonicClock;
  pub type WallClock = super::wasi_wall_clock::WallClock;
  #[allow(clippy::all)]
  /// Return a default monotonic clock, suitable for general-purpose application
  /// needs.
  /// 
  /// This allocates a new handle, so applications with frequent need of a clock
  /// handle should call this function once and reuse the handle instead of
  /// calling this function each time.
  pub fn default_monotonic_clock() -> MonotonicClock{
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
    unsafe {
      
      #[link(wasm_import_module = "wasi-default-clocks")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "default-monotonic-clock")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-default-clocks_default-monotonic-clock")]
        fn wit_import(
        ) -> i32;
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
  /// calling this function each time.
  pub fn default_wall_clock() -> WallClock{
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
    unsafe {
      
      #[link(wasm_import_module = "wasi-default-clocks")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "default-wall-clock")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-default-clocks_default-wall-clock")]
        fn wit_import(
        ) -> i32;
      }
      let ret = wit_import();
      ret as u32
    }
  }
  
}


#[allow(clippy::all)]
pub mod wasi_logging{
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
        Level::Trace => {
          f.debug_tuple("Level::Trace").finish()
        }
        Level::Debug => {
          f.debug_tuple("Level::Debug").finish()
        }
        Level::Info => {
          f.debug_tuple("Level::Info").finish()
        }
        Level::Warn => {
          f.debug_tuple("Level::Warn").finish()
        }
        Level::Error => {
          f.debug_tuple("Level::Error").finish()
        }
      }
    }
  }
  #[allow(clippy::all)]
  /// Emit a log message.
  /// 
  /// A log message has a `level` describing what kind of message is being sent,
  /// a context, which is an uninterpreted string meant to help consumers group
  /// similar messages, and a string containing the message text.
  pub fn log(level: Level,context: &str,message: &str,) -> (){
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
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
        fn wit_import(
        _: i32, _: i32, _: i32, _: i32, _: i32, );
      }
      wit_import(match level {
        Level::Trace => 0,
        Level::Debug => 1,
        Level::Info => 2,
        Level::Warn => 3,
        Level::Error => 4,
      }, ptr0, len0, ptr1, len1);
    }
  }
  
}


#[allow(clippy::all)]
pub mod wasi_stderr{
  #[allow(clippy::all)]
  /// Print text to stderr.
  pub fn print(message: &str,) -> (){
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
    unsafe {
      let vec0 = message;
      let ptr0 = vec0.as_ptr() as i32;
      let len0 = vec0.len() as i32;
      
      #[link(wasm_import_module = "wasi-stderr")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "print")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-stderr_print")]
        fn wit_import(
        _: i32, _: i32, );
      }
      wit_import(ptr0, len0);
    }
  }
  #[allow(clippy::all)]
  /// Test whether stderr is known to be a terminal.
  /// 
  /// This is similar to `isatty` in POSIX.
  pub fn is_terminal() -> bool{
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
    unsafe {
      
      #[link(wasm_import_module = "wasi-stderr")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "is-terminal")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-stderr_is-terminal")]
        fn wit_import(
        ) -> i32;
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
  pub fn num_columns() -> Option<u16>{
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
    unsafe {
      
      #[repr(align(2))]
      struct RetArea([u8; 4]);
      let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
      let ptr0 = ret_area.as_mut_ptr() as i32;
      #[link(wasm_import_module = "wasi-stderr")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "num-columns")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-stderr_num-columns")]
        fn wit_import(
        _: i32, );
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
pub mod wasi_io{
  pub type Pollable = super::wasi_poll::Pollable;
  /// An error type returned from a stream operation. Currently this
  /// doesn't provide any additional information.
  #[repr(C)]
  #[derive(Copy, Clone)]
  pub struct StreamError {
  }
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
  impl std::error::Error for StreamError{}
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
  pub fn read(this: InputStream,len: u64,) -> Result<(wit_bindgen::rt::vec::Vec::<u8>,bool,),StreamError>{
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
    unsafe {
      
      #[repr(align(4))]
      struct RetArea([u8; 16]);
      let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
      let ptr0 = ret_area.as_mut_ptr() as i32;
      #[link(wasm_import_module = "wasi-io")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "read")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-io_read")]
        fn wit_import(
        _: i32, _: i64, _: i32, );
      }
      wit_import(wit_bindgen::rt::as_i32(this), wit_bindgen::rt::as_i64(len), ptr0);
      match i32::from(*((ptr0 + 0) as *const u8)) {
        0 => Ok({
          let len1 = *((ptr0 + 8) as *const i32) as usize;
          
          (Vec::from_raw_parts(*((ptr0 + 4) as *const i32) as *mut _, len1, len1), match i32::from(*((ptr0 + 12) as *const u8)) {
            0 => false,
            1 => true,
            _ => panic!("invalid bool discriminant"),
          })
        }),
        1 => Err(StreamError{}),
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
  pub fn skip(this: InputStream,len: u64,) -> Result<(u64,bool,),StreamError>{
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
    unsafe {
      
      #[repr(align(8))]
      struct RetArea([u8; 24]);
      let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
      let ptr0 = ret_area.as_mut_ptr() as i32;
      #[link(wasm_import_module = "wasi-io")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "skip")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-io_skip")]
        fn wit_import(
        _: i32, _: i64, _: i32, );
      }
      wit_import(wit_bindgen::rt::as_i32(this), wit_bindgen::rt::as_i64(len), ptr0);
      match i32::from(*((ptr0 + 0) as *const u8)) {
        0 => Ok((*((ptr0 + 8) as *const i64) as u64, match i32::from(*((ptr0 + 16) as *const u8)) {
          0 => false,
          1 => true,
          _ => panic!("invalid bool discriminant"),
        })),
        1 => Err(StreamError{}),
        _ => panic!("invalid enum discriminant"),
      }
    }
  }
  #[allow(clippy::all)]
  /// Create a `pollable` which will resolve once either the specified stream has bytes
  /// available to read or the other end of the stream has been closed.
  pub fn subscribe_read(this: InputStream,) -> Pollable{
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
    unsafe {
      
      #[link(wasm_import_module = "wasi-io")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "subscribe-read")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-io_subscribe-read")]
        fn wit_import(
        _: i32, ) -> i32;
      }
      let ret = wit_import(wit_bindgen::rt::as_i32(this));
      ret as u32
    }
  }
  #[allow(clippy::all)]
  /// Dispose of the specified `input-stream`, after which it may no longer
  /// be used.
  pub fn drop_input_stream(this: InputStream,) -> (){
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
    unsafe {
      
      #[link(wasm_import_module = "wasi-io")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "drop-input-stream")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-io_drop-input-stream")]
        fn wit_import(
        _: i32, );
      }
      wit_import(wit_bindgen::rt::as_i32(this));
    }
  }
  #[allow(clippy::all)]
  /// Write bytes to a stream.
  /// 
  /// This function returns a `u64` indicating the number of bytes from
  /// `buf` that were written; it may be less than the full list.
  pub fn write(this: OutputStream,buf: &[u8],) -> Result<u64,StreamError>{
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
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
        fn wit_import(
        _: i32, _: i32, _: i32, _: i32, );
      }
      wit_import(wit_bindgen::rt::as_i32(this), ptr0, len0, ptr1);
      match i32::from(*((ptr1 + 0) as *const u8)) {
        0 => Ok(*((ptr1 + 8) as *const i64) as u64),
        1 => Err(StreamError{}),
        _ => panic!("invalid enum discriminant"),
      }
    }
  }
  #[allow(clippy::all)]
  /// Write multiple zero bytes to a stream.
  /// 
  /// This function returns a `u64` indicating the number of zero bytes
  /// that were written; it may be less than `len`.
  pub fn write_zeroes(this: OutputStream,len: u64,) -> Result<u64,StreamError>{
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
    unsafe {
      
      #[repr(align(8))]
      struct RetArea([u8; 16]);
      let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
      let ptr0 = ret_area.as_mut_ptr() as i32;
      #[link(wasm_import_module = "wasi-io")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "write-zeroes")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-io_write-zeroes")]
        fn wit_import(
        _: i32, _: i64, _: i32, );
      }
      wit_import(wit_bindgen::rt::as_i32(this), wit_bindgen::rt::as_i64(len), ptr0);
      match i32::from(*((ptr0 + 0) as *const u8)) {
        0 => Ok(*((ptr0 + 8) as *const i64) as u64),
        1 => Err(StreamError{}),
        _ => panic!("invalid enum discriminant"),
      }
    }
  }
  #[allow(clippy::all)]
  /// Read from one stream and write to another.
  /// 
  /// This function returns the number of bytes transferred; it may be less
  /// than `len`.
  pub fn splice(this: OutputStream,src: InputStream,len: u64,) -> Result<(u64,bool,),StreamError>{
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
    unsafe {
      
      #[repr(align(8))]
      struct RetArea([u8; 24]);
      let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
      let ptr0 = ret_area.as_mut_ptr() as i32;
      #[link(wasm_import_module = "wasi-io")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "splice")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-io_splice")]
        fn wit_import(
        _: i32, _: i32, _: i64, _: i32, );
      }
      wit_import(wit_bindgen::rt::as_i32(this), wit_bindgen::rt::as_i32(src), wit_bindgen::rt::as_i64(len), ptr0);
      match i32::from(*((ptr0 + 0) as *const u8)) {
        0 => Ok((*((ptr0 + 8) as *const i64) as u64, match i32::from(*((ptr0 + 16) as *const u8)) {
          0 => false,
          1 => true,
          _ => panic!("invalid bool discriminant"),
        })),
        1 => Err(StreamError{}),
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
  pub fn forward(this: OutputStream,src: InputStream,) -> Result<u64,StreamError>{
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
    unsafe {
      
      #[repr(align(8))]
      struct RetArea([u8; 16]);
      let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
      let ptr0 = ret_area.as_mut_ptr() as i32;
      #[link(wasm_import_module = "wasi-io")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "forward")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-io_forward")]
        fn wit_import(
        _: i32, _: i32, _: i32, );
      }
      wit_import(wit_bindgen::rt::as_i32(this), wit_bindgen::rt::as_i32(src), ptr0);
      match i32::from(*((ptr0 + 0) as *const u8)) {
        0 => Ok(*((ptr0 + 8) as *const i64) as u64),
        1 => Err(StreamError{}),
        _ => panic!("invalid enum discriminant"),
      }
    }
  }
  #[allow(clippy::all)]
  /// Create a `pollable` which will resolve once either the specified stream is ready
  /// to accept bytes or the other end of the stream has been closed.
  pub fn subscribe(this: OutputStream,) -> Pollable{
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
    unsafe {
      
      #[link(wasm_import_module = "wasi-io")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "subscribe")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-io_subscribe")]
        fn wit_import(
        _: i32, ) -> i32;
      }
      let ret = wit_import(wit_bindgen::rt::as_i32(this));
      ret as u32
    }
  }
  #[allow(clippy::all)]
  /// Dispose of the specified `output-stream`, after which it may no longer
  /// be used.
  pub fn drop_output_stream(this: OutputStream,) -> (){
    
    #[allow(unused_imports)]
    use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
    unsafe {
      
      #[link(wasm_import_module = "wasi-io")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "drop-output-stream")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-io_drop-output-stream")]
        fn wit_import(
        _: i32, );
      }
      wit_import(wit_bindgen::rt::as_i32(this));
    }
  }
  
}


#[allow(clippy::all)]
pub mod wasi_filesystem{
  pub type InputStream = super::wasi_io::InputStream;
  pub type OutputStream = super::wasi_io::OutputStream;
  pub type Datetime = super::wasi_wall_clock::Datetime;
  wit_bindgen::bitflags::bitflags! {
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
  wit_bindgen::bitflags::bitflags! {
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
  /// File size or length of a region within a file.
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
  impl Errno{
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
  impl core::fmt::Debug for Errno{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      f.debug_struct("Errno")
      .field("code", &(*self as i32))
      .field("name", &self.name())
      .field("message", &self.message())
      .finish()
    }
  }
  impl core::fmt::Display for Errno{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      write!(f, "{} (error {})", self.name(), *self as i32)}
    }
    
    impl std::error::Error for Errno{}
    /// A stream of directory entries.
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
          DescriptorType::Unknown => {
            f.debug_tuple("DescriptorType::Unknown").finish()
          }
          DescriptorType::BlockDevice => {
            f.debug_tuple("DescriptorType::BlockDevice").finish()
          }
          DescriptorType::CharacterDevice => {
            f.debug_tuple("DescriptorType::CharacterDevice").finish()
          }
          DescriptorType::Directory => {
            f.debug_tuple("DescriptorType::Directory").finish()
          }
          DescriptorType::Fifo => {
            f.debug_tuple("DescriptorType::Fifo").finish()
          }
          DescriptorType::SymbolicLink => {
            f.debug_tuple("DescriptorType::SymbolicLink").finish()
          }
          DescriptorType::RegularFile => {
            f.debug_tuple("DescriptorType::RegularFile").finish()
          }
          DescriptorType::Socket => {
            f.debug_tuple("DescriptorType::Socket").finish()
          }
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
      pub name: wit_bindgen::rt::string::String,
    }
    impl core::fmt::Debug for DirEntry {
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DirEntry").field("ino", &self.ino).field("type", &self.type_).field("name", &self.name).finish()
      }
    }
    wit_bindgen::bitflags::bitflags! {
      /// Descriptor flags.
      /// 
      /// Note: This was called `fdflags` in earlier versions of WASI.
      pub struct DescriptorFlags: u8 {
        /// Read mode: Data can be read.
        const READ = 1 << 0;
        /// Write mode: Data can be written to.
        const WRITE = 1 << 1;
        /// Requests non-blocking operation.
        /// 
        /// When this flag is enabled, functions may return immediately with an
        /// `errno::again` error code in situations where they would otherwise
        /// block. However, this non-blocking behavior is not required.
        /// Implementations are permitted to ignore this flag and block.
        const NONBLOCK = 1 << 2;
        /// Request that writes be performed according to synchronized I/O file
        /// integrity completion. The data stored in the file and the file's
        /// metadata are synchronized.
        /// 
        /// The precise semantics of this operation have not yet been defined for
        /// WASI. At this time, it should be interpreted as a request, and not a
        /// requirement.
        const SYNC = 1 << 3;
        /// Request that writes be performed according to synchronized I/O data
        /// integrity completion. Only the data stored in the file is
        /// synchronized.
        /// 
        /// The precise semantics of this operation have not yet been defined for
        /// WASI. At this time, it should be interpreted as a request, and not a
        /// requirement.
        const DSYNC = 1 << 4;
        /// Requests that reads be performed at the same level of integrety
        /// requested for writes.
        /// 
        /// The precise semantics of this operation have not yet been defined for
        /// WASI. At this time, it should be interpreted as a request, and not a
        /// requirement.
        const RSYNC = 1 << 5;
        /// Mutating directories mode: Directory contents may be mutated.
        /// 
        /// When this flag is unset on a descriptor, operations using the
        /// descriptor which would create, rename, delete, modify the data or
        /// metadata of filesystem objects, or obtain another handle which
        /// would permit any of those, shall fail with `errno::rofs` if
        /// they would otherwise succeed.
        /// 
        /// This may only be set on directories.
        const MUTATE_DIRECTORY = 1 << 6;
      }
    }
    impl DescriptorFlags {
      /// Convert from a raw integer, preserving any unknown bits. See
      /// <https://github.com/bitflags/bitflags/issues/263#issuecomment-957088321>
      pub fn from_bits_preserve(bits: u8) -> Self {
        Self { bits }
      }
    }
    /// A descriptor is a reference to a filesystem object, which may be a file,
    /// directory, named pipe, special file, or other object on which filesystem
    /// calls may be made.
    pub type Descriptor = u32;
    /// When setting a timestamp, this gives the value to set it to.
    #[derive(Clone, Copy)]
    pub enum NewTimestamp{
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
          NewTimestamp::NoChange => {
            f.debug_tuple("NewTimestamp::NoChange").finish()
          }
          NewTimestamp::Now => {
            f.debug_tuple("NewTimestamp::Now").finish()
          }
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
        f.debug_struct("DescriptorStat").field("dev", &self.dev).field("ino", &self.ino).field("type", &self.type_).field("nlink", &self.nlink).field("size", &self.size).field("atim", &self.atim).field("mtim", &self.mtim).field("ctim", &self.ctim).finish()
      }
    }
    wit_bindgen::bitflags::bitflags! {
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
          Advice::Normal => {
            f.debug_tuple("Advice::Normal").finish()
          }
          Advice::Sequential => {
            f.debug_tuple("Advice::Sequential").finish()
          }
          Advice::Random => {
            f.debug_tuple("Advice::Random").finish()
          }
          Advice::WillNeed => {
            f.debug_tuple("Advice::WillNeed").finish()
          }
          Advice::DontNeed => {
            f.debug_tuple("Advice::DontNeed").finish()
          }
          Advice::NoReuse => {
            f.debug_tuple("Advice::NoReuse").finish()
          }
        }
      }
    }
    #[allow(clippy::all)]
    /// Get preopened file descriptors.
    /// 
    /// Provided by the environment as a pair of a path name and a descriptor
    /// of a directory.
    /// 
    /// Morally, these are a value import, but until value imports are available
    /// in the component model, this import function should be called at most
    /// once, and subsequent calls should trap.
    pub fn get_preopens() -> wit_bindgen::rt::vec::Vec::<(Descriptor,wit_bindgen::rt::string::String,)>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[repr(align(4))]
        struct RetArea([u8; 8]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[link(wasm_import_module = "wasi-filesystem")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "get-preopens")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_get-preopens")]
          fn wit_import(
          _: i32, );
        }
        wit_import(ptr0);
        let base2 = *((ptr0 + 0) as *const i32);
        let len2 = *((ptr0 + 4) as *const i32);
        let mut result2 = Vec::with_capacity(len2 as usize);
        for i in 0..len2 {
          let base = base2 + i *12;
          result2.push({
            let len1 = *((base + 8) as *const i32) as usize;
            
            (*((base + 0) as *const i32) as u32, String::from_utf8(Vec::from_raw_parts(*((base + 4) as *const i32) as *mut _, len1, len1)).unwrap())
          });
        }
        wit_bindgen::rt::dealloc(base2, (len2 as usize) * 12, 4);
        result2
      }
    }
    #[allow(clippy::all)]
    /// Return a stream for reading from a file.
    /// 
    /// Multiple read, write, and append streams may be active on the same open
    /// file and they do not interfere with each other.
    /// 
    /// Note: This allows using `read-stream`, which is similar to `read` in POSIX.
    pub fn read_via_stream(this: Descriptor,offset: Filesize,) -> Result<InputStream,Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[repr(align(4))]
        struct RetArea([u8; 8]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[link(wasm_import_module = "wasi-filesystem")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "read-via-stream")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_read-via-stream")]
          fn wit_import(
          _: i32, _: i64, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), wit_bindgen::rt::as_i64(offset), ptr0);
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
    pub fn write_via_stream(this: Descriptor,offset: Filesize,) -> Result<OutputStream,Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[repr(align(4))]
        struct RetArea([u8; 8]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[link(wasm_import_module = "wasi-filesystem")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "write-via-stream")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_write-via-stream")]
          fn wit_import(
          _: i32, _: i64, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), wit_bindgen::rt::as_i64(offset), ptr0);
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
    pub fn append_via_stream(this: Descriptor,) -> Result<OutputStream,Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[repr(align(4))]
        struct RetArea([u8; 8]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[link(wasm_import_module = "wasi-filesystem")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "append-via-stream")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_append-via-stream")]
          fn wit_import(
          _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), ptr0);
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
    /// Provide file advisory information on a descriptor.
    /// 
    /// This is similar to `posix_fadvise` in POSIX.
    pub fn fadvise(this: Descriptor,offset: Filesize,len: Filesize,advice: Advice,) -> Result<(),Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[repr(align(1))]
        struct RetArea([u8; 2]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[link(wasm_import_module = "wasi-filesystem")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "fadvise")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_fadvise")]
          fn wit_import(
          _: i32, _: i64, _: i64, _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), wit_bindgen::rt::as_i64(offset), wit_bindgen::rt::as_i64(len), match advice {
          Advice::Normal => 0,
          Advice::Sequential => 1,
          Advice::Random => 2,
          Advice::WillNeed => 3,
          Advice::DontNeed => 4,
          Advice::NoReuse => 5,
        }, ptr0);
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
    /// This function succeeds with no effect if the file descriptor is not
    /// opened for writing.
    /// 
    /// Note: This is similar to `fdatasync` in POSIX.
    pub fn datasync(this: Descriptor,) -> Result<(),Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[repr(align(1))]
        struct RetArea([u8; 2]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[link(wasm_import_module = "wasi-filesystem")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "datasync")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_datasync")]
          fn wit_import(
          _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), ptr0);
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
    pub fn flags(this: Descriptor,) -> Result<DescriptorFlags,Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[repr(align(1))]
        struct RetArea([u8; 2]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[link(wasm_import_module = "wasi-filesystem")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "flags")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_flags")]
          fn wit_import(
          _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), ptr0);
        match i32::from(*((ptr0 + 0) as *const u8)) {
          0 => Ok(DescriptorFlags::empty() | DescriptorFlags::from_bits_preserve(((i32::from(*((ptr0 + 1) as *const u8)) as u8) << 0) as _)),
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
    /// Note: This returns the same value as the `type` field of the `fd-stat`
    /// returned by `stat`, `stat-at` and similar.
    /// 
    /// Note: This returns similar flags to the `st_mode & S_IFMT` value provided
    /// by `fstat` in POSIX.
    /// 
    /// Note: This returns the value that was the `fs_filetype` value returned
    /// from `fdstat_get` in earlier versions of WASI.
    pub fn todo_type(this: Descriptor,) -> Result<DescriptorType,Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[repr(align(1))]
        struct RetArea([u8; 2]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[link(wasm_import_module = "wasi-filesystem")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "todo-type")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_todo-type")]
          fn wit_import(
          _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), ptr0);
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
    /// Set status flags associated with a descriptor.
    /// 
    /// This function may only change the `nonblock` flag.
    /// 
    /// Note: This is similar to `fcntl(fd, F_SETFL, flags)` in POSIX.
    /// 
    /// Note: This was called `fd_fdstat_set_flags` in earlier versions of WASI.
    pub fn set_flags(this: Descriptor,flags: DescriptorFlags,) -> Result<(),Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
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
          fn wit_import(
          _: i32, _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), (flags0.bits() >> 0) as i32, ptr1);
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
    pub fn set_size(this: Descriptor,size: Filesize,) -> Result<(),Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[repr(align(1))]
        struct RetArea([u8; 2]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[link(wasm_import_module = "wasi-filesystem")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "set-size")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_set-size")]
          fn wit_import(
          _: i32, _: i64, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), wit_bindgen::rt::as_i64(size), ptr0);
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
    pub fn set_times(this: Descriptor,atim: NewTimestamp,mtim: NewTimestamp,) -> Result<(),Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[repr(align(1))]
        struct RetArea([u8; 2]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let (result1_0,result1_1,result1_2,) = match atim {
          NewTimestamp::NoChange=> {
            (0i32, 0i64, 0i32)
          }
          NewTimestamp::Now=> {
            (1i32, 0i64, 0i32)
          }
          NewTimestamp::Timestamp(e) => {
            let Datetime{ seconds:seconds0, nanoseconds:nanoseconds0, } = e;
            
            (2i32, wit_bindgen::rt::as_i64(seconds0), wit_bindgen::rt::as_i32(nanoseconds0))
          },
        };
        let (result3_0,result3_1,result3_2,) = match mtim {
          NewTimestamp::NoChange=> {
            (0i32, 0i64, 0i32)
          }
          NewTimestamp::Now=> {
            (1i32, 0i64, 0i32)
          }
          NewTimestamp::Timestamp(e) => {
            let Datetime{ seconds:seconds2, nanoseconds:nanoseconds2, } = e;
            
            (2i32, wit_bindgen::rt::as_i64(seconds2), wit_bindgen::rt::as_i32(nanoseconds2))
          },
        };
        let ptr4 = ret_area.as_mut_ptr() as i32;
        #[link(wasm_import_module = "wasi-filesystem")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "set-times")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_set-times")]
          fn wit_import(
          _: i32, _: i32, _: i64, _: i32, _: i32, _: i64, _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), result1_0, result1_1, result1_2, result3_0, result3_1, result3_2, ptr4);
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
    /// Read from a descriptor, without using and updating the descriptor's offset.
    /// 
    /// This function returns a list of bytes containing the data that was
    /// read, along with a bool indicating whether the end of the file
    /// was reached. The returned list will contain up to `len` bytes; it
    /// may return fewer than requested, but not more.
    /// 
    /// Note: This is similar to `pread` in POSIX.
    pub fn pread(this: Descriptor,len: Filesize,offset: Filesize,) -> Result<(wit_bindgen::rt::vec::Vec::<u8>,bool,),Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[repr(align(4))]
        struct RetArea([u8; 16]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[link(wasm_import_module = "wasi-filesystem")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "pread")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_pread")]
          fn wit_import(
          _: i32, _: i64, _: i64, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), wit_bindgen::rt::as_i64(len), wit_bindgen::rt::as_i64(offset), ptr0);
        match i32::from(*((ptr0 + 0) as *const u8)) {
          0 => Ok({
            let len1 = *((ptr0 + 8) as *const i32) as usize;
            
            (Vec::from_raw_parts(*((ptr0 + 4) as *const i32) as *mut _, len1, len1), match i32::from(*((ptr0 + 12) as *const u8)) {
              0 => false,
              1 => true,
              _ => panic!("invalid bool discriminant"),
            })
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
    /// Write to a descriptor, without using and updating the descriptor's offset.
    /// 
    /// It is valid to write past the end of a file; the file is extended to the
    /// extent of the write, with bytes between the previous end and the start of
    /// the write set to zero.
    /// 
    /// Note: This is similar to `pwrite` in POSIX.
    pub fn pwrite(this: Descriptor,buf: &[u8],offset: Filesize,) -> Result<Filesize,Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[repr(align(8))]
        struct RetArea([u8; 16]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let vec0 = buf;
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let ptr1 = ret_area.as_mut_ptr() as i32;
        #[link(wasm_import_module = "wasi-filesystem")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "pwrite")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_pwrite")]
          fn wit_import(
          _: i32, _: i32, _: i32, _: i64, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), ptr0, len0, wit_bindgen::rt::as_i64(offset), ptr1);
        match i32::from(*((ptr1 + 0) as *const u8)) {
          0 => Ok(*((ptr1 + 8) as *const i64) as u64),
          1 => Err(match i32::from(*((ptr1 + 8) as *const u8)) {
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
    /// On filesystems where directories contain entries referring to themselves
    /// and their parents, often named `.` and `..` respectively, these entries
    /// are omitted.
    /// 
    /// This always returns a new stream which starts at the beginning of the
    /// directory. Multiple streams may be active on the same directory, and they
    /// do not interfere with each other.
    pub fn readdir(this: Descriptor,) -> Result<DirEntryStream,Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[repr(align(4))]
        struct RetArea([u8; 8]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[link(wasm_import_module = "wasi-filesystem")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "readdir")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_readdir")]
          fn wit_import(
          _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), ptr0);
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
    /// Synchronize the data and metadata of a file to disk.
    /// 
    /// This function succeeds with no effect if the file descriptor is not
    /// opened for writing.
    /// 
    /// Note: This is similar to `fsync` in POSIX.
    pub fn sync(this: Descriptor,) -> Result<(),Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[repr(align(1))]
        struct RetArea([u8; 2]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[link(wasm_import_module = "wasi-filesystem")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "sync")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_sync")]
          fn wit_import(
          _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), ptr0);
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
    pub fn create_directory_at(this: Descriptor,path: &str,) -> Result<(),Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
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
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_create-directory-at")]
          fn wit_import(
          _: i32, _: i32, _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), ptr0, len0, ptr1);
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
    pub fn stat(this: Descriptor,) -> Result<DescriptorStat,Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[repr(align(8))]
        struct RetArea([u8; 96]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[link(wasm_import_module = "wasi-filesystem")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "stat")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_stat")]
          fn wit_import(
          _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), ptr0);
        match i32::from(*((ptr0 + 0) as *const u8)) {
          0 => Ok(DescriptorStat{dev:*((ptr0 + 8) as *const i64) as u64, ino:*((ptr0 + 16) as *const i64) as u64, type_:match i32::from(*((ptr0 + 24) as *const u8)) {
            0 => DescriptorType::Unknown,
            1 => DescriptorType::BlockDevice,
            2 => DescriptorType::CharacterDevice,
            3 => DescriptorType::Directory,
            4 => DescriptorType::Fifo,
            5 => DescriptorType::SymbolicLink,
            6 => DescriptorType::RegularFile,
            7 => DescriptorType::Socket,
            _ => panic!("invalid enum discriminant"),
          }, nlink:*((ptr0 + 32) as *const i64) as u64, size:*((ptr0 + 40) as *const i64) as u64, atim:Datetime{seconds:*((ptr0 + 48) as *const i64) as u64, nanoseconds:*((ptr0 + 56) as *const i32) as u32, }, mtim:Datetime{seconds:*((ptr0 + 64) as *const i64) as u64, nanoseconds:*((ptr0 + 72) as *const i32) as u32, }, ctim:Datetime{seconds:*((ptr0 + 80) as *const i64) as u64, nanoseconds:*((ptr0 + 88) as *const i32) as u32, }, }),
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
    /// Note: This was called `path_filestat_get` in earlier versions of WASI.
    pub fn stat_at(this: Descriptor,at_flags: AtFlags,path: &str,) -> Result<DescriptorStat,Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
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
          fn wit_import(
          _: i32, _: i32, _: i32, _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), (flags0.bits() >> 0) as i32, ptr1, len1, ptr2);
        match i32::from(*((ptr2 + 0) as *const u8)) {
          0 => Ok(DescriptorStat{dev:*((ptr2 + 8) as *const i64) as u64, ino:*((ptr2 + 16) as *const i64) as u64, type_:match i32::from(*((ptr2 + 24) as *const u8)) {
            0 => DescriptorType::Unknown,
            1 => DescriptorType::BlockDevice,
            2 => DescriptorType::CharacterDevice,
            3 => DescriptorType::Directory,
            4 => DescriptorType::Fifo,
            5 => DescriptorType::SymbolicLink,
            6 => DescriptorType::RegularFile,
            7 => DescriptorType::Socket,
            _ => panic!("invalid enum discriminant"),
          }, nlink:*((ptr2 + 32) as *const i64) as u64, size:*((ptr2 + 40) as *const i64) as u64, atim:Datetime{seconds:*((ptr2 + 48) as *const i64) as u64, nanoseconds:*((ptr2 + 56) as *const i32) as u32, }, mtim:Datetime{seconds:*((ptr2 + 64) as *const i64) as u64, nanoseconds:*((ptr2 + 72) as *const i32) as u32, }, ctim:Datetime{seconds:*((ptr2 + 80) as *const i64) as u64, nanoseconds:*((ptr2 + 88) as *const i32) as u32, }, }),
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
    pub fn set_times_at(this: Descriptor,at_flags: AtFlags,path: &str,atim: NewTimestamp,mtim: NewTimestamp,) -> Result<(),Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[repr(align(1))]
        struct RetArea([u8; 2]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let flags0 = at_flags;
        let vec1 = path;
        let ptr1 = vec1.as_ptr() as i32;
        let len1 = vec1.len() as i32;
        let (result3_0,result3_1,result3_2,) = match atim {
          NewTimestamp::NoChange=> {
            (0i32, 0i64, 0i32)
          }
          NewTimestamp::Now=> {
            (1i32, 0i64, 0i32)
          }
          NewTimestamp::Timestamp(e) => {
            let Datetime{ seconds:seconds2, nanoseconds:nanoseconds2, } = e;
            
            (2i32, wit_bindgen::rt::as_i64(seconds2), wit_bindgen::rt::as_i32(nanoseconds2))
          },
        };
        let (result5_0,result5_1,result5_2,) = match mtim {
          NewTimestamp::NoChange=> {
            (0i32, 0i64, 0i32)
          }
          NewTimestamp::Now=> {
            (1i32, 0i64, 0i32)
          }
          NewTimestamp::Timestamp(e) => {
            let Datetime{ seconds:seconds4, nanoseconds:nanoseconds4, } = e;
            
            (2i32, wit_bindgen::rt::as_i64(seconds4), wit_bindgen::rt::as_i32(nanoseconds4))
          },
        };
        let ptr6 = ret_area.as_mut_ptr() as i32;
        #[link(wasm_import_module = "wasi-filesystem")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "set-times-at")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_set-times-at")]
          fn wit_import(
          _: i32, _: i32, _: i32, _: i32, _: i32, _: i64, _: i32, _: i32, _: i64, _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), (flags0.bits() >> 0) as i32, ptr1, len1, result3_0, result3_1, result3_2, result5_0, result5_1, result5_2, ptr6);
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
    pub fn link_at(this: Descriptor,old_at_flags: AtFlags,old_path: &str,new_descriptor: Descriptor,new_path: &str,) -> Result<(),Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
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
          fn wit_import(
          _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), (flags0.bits() >> 0) as i32, ptr1, len1, wit_bindgen::rt::as_i32(new_descriptor), ptr2, len2, ptr3);
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
    /// If `flags` contains `descriptor-flags::mutate-directory`, and the base
    /// descriptor doesn't have `descriptor-flags::mutate-directory` set,
    /// `open-at` fails with `errno::rofs`.
    /// 
    /// If `flags` contains `write` or `mutate-directory`, or `o-flags` contains
    /// `trunc` or `create`, and the base descriptor doesn't have
    /// `descriptor-flags::mutate-directory` set, `open-at` fails with
    /// `errno::rofs`.
    /// 
    /// Note: This is similar to `openat` in POSIX.
    pub fn open_at(this: Descriptor,at_flags: AtFlags,path: &str,o_flags: OFlags,flags: DescriptorFlags,mode: Mode,) -> Result<Descriptor,Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
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
          fn wit_import(
          _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), (flags0.bits() >> 0) as i32, ptr1, len1, (flags2.bits() >> 0) as i32, (flags3.bits() >> 0) as i32, (flags4.bits() >> 0) as i32, ptr5);
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
    /// Read the contents of a symbolic link.
    /// 
    /// If the contents contain an absolute or rooted path in the underlying
    /// filesystem, this function fails with `errno::perm`.
    /// 
    /// Note: This is similar to `readlinkat` in POSIX.
    pub fn readlink_at(this: Descriptor,path: &str,) -> Result<wit_bindgen::rt::string::String,Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
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
          fn wit_import(
          _: i32, _: i32, _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), ptr0, len0, ptr1);
        match i32::from(*((ptr1 + 0) as *const u8)) {
          0 => Ok({
            let len2 = *((ptr1 + 8) as *const i32) as usize;
            
            String::from_utf8(Vec::from_raw_parts(*((ptr1 + 4) as *const i32) as *mut _, len2, len2)).unwrap()
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
    pub fn remove_directory_at(this: Descriptor,path: &str,) -> Result<(),Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
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
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_remove-directory-at")]
          fn wit_import(
          _: i32, _: i32, _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), ptr0, len0, ptr1);
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
    pub fn rename_at(this: Descriptor,old_path: &str,new_descriptor: Descriptor,new_path: &str,) -> Result<(),Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
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
          fn wit_import(
          _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), ptr0, len0, wit_bindgen::rt::as_i32(new_descriptor), ptr1, len1, ptr2);
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
    /// Create a symbolic link (also known as a "symlink").
    /// 
    /// Note: This is similar to `symlinkat` in POSIX.
    pub fn symlink_at(this: Descriptor,old_path: &str,new_path: &str,) -> Result<(),Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
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
          fn wit_import(
          _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), ptr0, len0, ptr1, len1, ptr2);
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
    pub fn unlink_file_at(this: Descriptor,path: &str,) -> Result<(),Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
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
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_unlink-file-at")]
          fn wit_import(
          _: i32, _: i32, _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), ptr0, len0, ptr1);
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
    pub fn change_file_permissions_at(this: Descriptor,at_flags: AtFlags,path: &str,mode: Mode,) -> Result<(),Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
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
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_change-file-permissions-at")]
          fn wit_import(
          _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), (flags0.bits() >> 0) as i32, ptr1, len1, (flags2.bits() >> 0) as i32, ptr3);
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
    pub fn change_directory_permissions_at(this: Descriptor,at_flags: AtFlags,path: &str,mode: Mode,) -> Result<(),Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
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
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_change-directory-permissions-at")]
          fn wit_import(
          _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), (flags0.bits() >> 0) as i32, ptr1, len1, (flags2.bits() >> 0) as i32, ptr3);
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
    pub fn lock_shared(this: Descriptor,) -> Result<(),Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[repr(align(1))]
        struct RetArea([u8; 2]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[link(wasm_import_module = "wasi-filesystem")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "lock-shared")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_lock-shared")]
          fn wit_import(
          _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), ptr0);
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
    /// for the file, this function upgrades the lock to an exclusive lock. If the
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
    pub fn lock_exclusive(this: Descriptor,) -> Result<(),Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[repr(align(1))]
        struct RetArea([u8; 2]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[link(wasm_import_module = "wasi-filesystem")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "lock-exclusive")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_lock-exclusive")]
          fn wit_import(
          _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), ptr0);
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
    /// This function returns `errno::again` if the lock cannot be acquired.
    /// 
    /// Not all filesystems support locking; on filesystems which don't support
    /// locking, this function returns `errno::notsup`.
    /// 
    /// Note: This is similar to `flock(fd, LOCK_SH | LOCK_NB)` in Unix.
    pub fn try_lock_shared(this: Descriptor,) -> Result<(),Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[repr(align(1))]
        struct RetArea([u8; 2]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[link(wasm_import_module = "wasi-filesystem")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "try-lock-shared")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_try-lock-shared")]
          fn wit_import(
          _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), ptr0);
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
    /// for the file, this function upgrades the lock to an exclusive lock. If the
    /// open file already has an exclusive lock, this function has no effect.
    /// 
    /// This requests an *advisory* lock, meaning that the file could be accessed
    /// by other programs that don't hold the lock.
    /// 
    /// It is unspecified whether this function succeeds if the file descriptor
    /// is not opened for writing. It is unspecified how exclusive locks interact
    /// with locks acquired by non-WASI programs.
    /// 
    /// This function returns `errno::again` if the lock cannot be acquired.
    /// 
    /// Not all filesystems support locking; on filesystems which don't support
    /// locking, this function returns `errno::notsup`.
    /// 
    /// Note: This is similar to `flock(fd, LOCK_EX | LOCK_NB)` in Unix.
    pub fn try_lock_exclusive(this: Descriptor,) -> Result<(),Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[repr(align(1))]
        struct RetArea([u8; 2]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[link(wasm_import_module = "wasi-filesystem")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "try-lock-exclusive")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_try-lock-exclusive")]
          fn wit_import(
          _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), ptr0);
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
    pub fn unlock(this: Descriptor,) -> Result<(),Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[repr(align(1))]
        struct RetArea([u8; 2]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[link(wasm_import_module = "wasi-filesystem")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "unlock")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_unlock")]
          fn wit_import(
          _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), ptr0);
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
    /// Dispose of the specified `descriptor`, after which it may no longer
    /// be used.
    pub fn drop_descriptor(this: Descriptor,) -> (){
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[link(wasm_import_module = "wasi-filesystem")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "drop-descriptor")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_drop-descriptor")]
          fn wit_import(
          _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this));
      }
    }
    #[allow(clippy::all)]
    /// Read a single directory entry from a `dir-entry-stream`.
    pub fn read_dir_entry(this: DirEntryStream,) -> Result<Option<DirEntry>,Errno>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[repr(align(8))]
        struct RetArea([u8; 48]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[link(wasm_import_module = "wasi-filesystem")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "read-dir-entry")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_read-dir-entry")]
          fn wit_import(
          _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this), ptr0);
        match i32::from(*((ptr0 + 0) as *const u8)) {
          0 => Ok(match i32::from(*((ptr0 + 8) as *const u8)) {
            0 => None,
            1 => Some({
              let len1 = *((ptr0 + 40) as *const i32) as usize;
              
              DirEntry{ino:match i32::from(*((ptr0 + 16) as *const u8)) {
                0 => None,
                1 => Some(*((ptr0 + 24) as *const i64) as u64),
                _ => panic!("invalid enum discriminant"),
              }, type_:match i32::from(*((ptr0 + 32) as *const u8)) {
                0 => DescriptorType::Unknown,
                1 => DescriptorType::BlockDevice,
                2 => DescriptorType::CharacterDevice,
                3 => DescriptorType::Directory,
                4 => DescriptorType::Fifo,
                5 => DescriptorType::SymbolicLink,
                6 => DescriptorType::RegularFile,
                7 => DescriptorType::Socket,
                _ => panic!("invalid enum discriminant"),
              }, name:String::from_utf8(Vec::from_raw_parts(*((ptr0 + 36) as *const i32) as *mut _, len1, len1)).unwrap(), }
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
    /// Dispose of the specified `dir-entry-stream`, after which it may no longer
    /// be used.
    pub fn drop_dir_entry_stream(this: DirEntryStream,) -> (){
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[link(wasm_import_module = "wasi-filesystem")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "drop-dir-entry-stream")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-filesystem_drop-dir-entry-stream")]
          fn wit_import(
          _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(this));
      }
    }
    
  }
  
  
  #[allow(clippy::all)]
  pub mod wasi_random{
    #[allow(clippy::all)]
    /// Return `len` random bytes.
    /// 
    /// This function must produce data from an adaquately seeded CSPRNG, so it
    /// must not block, and the returned data is always unpredictable.
    /// 
    /// Deterministic environments must omit this function, rather than
    /// implementing it with deterministic data.
    pub fn get_random_bytes(len: u32,) -> wit_bindgen::rt::vec::Vec::<u8>{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[repr(align(4))]
        struct RetArea([u8; 8]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[link(wasm_import_module = "wasi-random")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "get-random-bytes")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-random_get-random-bytes")]
          fn wit_import(
          _: i32, _: i32, );
        }
        wit_import(wit_bindgen::rt::as_i32(len), ptr0);
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
    pub fn get_random_u64() -> u64{
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
      unsafe {
        
        #[link(wasm_import_module = "wasi-random")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "get-random-u64")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-random_get-random-u64")]
          fn wit_import(
          ) -> i64;
        }
        let ret = wit_import();
        ret as u64
      }
    }
    
  }
  
  
  #[allow(clippy::all)]
  pub mod wasi_network{
    /// An opaque resource that represents access to (a subset of) the network.
    /// This enables context-based security for networking.
    /// There is no need for this to map 1:1 to a physical network interface.
    /// 
    /// FYI, In the future this will be replaced by handle types.
    pub type Network = u32;
    pub type Ipv6Address = (u16,u16,u16,u16,u16,u16,u16,u16,);
    pub type Ipv4Address = (u8,u8,u8,u8,);
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
          IpAddressFamily::Ipv4 => {
            f.debug_tuple("IpAddressFamily::Ipv4").finish()
          }
          IpAddressFamily::Ipv6 => {
            f.debug_tuple("IpAddressFamily::Ipv6").finish()
          }
        }
      }
    }
    #[derive(Clone, Copy)]
    pub enum IpAddress{
      Ipv4(Ipv4Address),
      Ipv6(Ipv6Address),
    }
    impl core::fmt::Debug for IpAddress {
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
          IpAddress::Ipv4(e) => {
            f.debug_tuple("IpAddress::Ipv4").field(e).finish()
          }
          IpAddress::Ipv6(e) => {
            f.debug_tuple("IpAddress::Ipv6").field(e).finish()
          }
        }
      }
    }
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Error {
      Unknown,
      Again,
    }
    impl Error{
      pub fn name(&self) -> &'static str {
        match self {
          Error::Unknown => "unknown",
          Error::Again => "again",
        }
      }
      pub fn message(&self) -> &'static str {
        match self {
          Error::Unknown => "",
          Error::Again => "",
        }
      }
    }
    impl core::fmt::Debug for Error{
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Error")
        .field("code", &(*self as i32))
        .field("name", &self.name())
        .field("message", &self.message())
        .finish()
      }
    }
    impl core::fmt::Display for Error{
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} (error {})", self.name(), *self as i32)}
      }
      
      impl std::error::Error for Error{}
      #[allow(clippy::all)]
      /// Dispose of the specified `network`, after which it may no longer be used.
      pub fn drop_network(this: Network,) -> (){
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[link(wasm_import_module = "wasi-network")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "drop-network")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-network_drop-network")]
            fn wit_import(
            _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this));
        }
      }
      
    }
    
    
    #[allow(clippy::all)]
    pub mod wasi_default_network{
      pub type Network = super::wasi_network::Network;
      #[allow(clippy::all)]
      /// Get a handle to the default network.
      pub fn default_network() -> Network{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[link(wasm_import_module = "wasi-default-network")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "default-network")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-default-network_default-network")]
            fn wit_import(
            ) -> i32;
          }
          let ret = wit_import();
          ret as u32
        }
      }
      
    }
    
    
    #[allow(clippy::all)]
    pub mod wasi_ip_name_lookup{
      pub type Pollable = super::wasi_poll::Pollable;
      pub type Network = super::wasi_network::Network;
      pub type Error = super::wasi_network::Error;
      pub type IpAddress = super::wasi_network::IpAddress;
      pub type IpAddressFamily = super::wasi_network::IpAddressFamily;
      pub type ResolveAddressStream = u32;
      #[allow(clippy::all)]
      /// Resolve an internet host name to a list of IP addresses.
      /// 
      /// See the wasi-socket proposal README.md for a comparison with getaddrinfo.
      /// 
      /// Parameters:
      /// - `name`: The name to look up. IP addresses are not allowed. Unicode domain names are automatically converted
      /// to ASCII using IDNA encoding.
      /// - `address-family`: If provided, limit the results to addresses of this specific address family.
      /// - `include-unavailable`: When set to true, this function will also return addresses of which the runtime
      /// thinks (or knows) can't be connected to at the moment. For example, this will return IPv6 addresses on
      /// systems without an active IPv6 interface. Notes:
      /// - Even when no public IPv6 interfaces are present or active, names like "localhost" can still resolve to an IPv6 address.
      /// - Whatever is "available" or "unavailable" is volatile and can change everytime a network cable is unplugged.
      /// 
      /// This function never blocks. It either immediately returns successfully with a `resolve-address-stream`
      /// that can be used to (asynchronously) fetch the results.
      /// Or it immediately fails whenever `name` is:
      /// - empty
      /// - an IP address
      /// - a syntactically invalid domain name in another way
      /// 
      /// References:
      /// - https://pubs.opengroup.org/onlinepubs/9699919799/functions/getaddrinfo.html
      /// - https://man7.org/linux/man-pages/man3/getaddrinfo.3.html
      pub fn resolve_addresses(network: Network,name: &str,address_family: Option<IpAddressFamily>,include_unavailable: bool,) -> Result<ResolveAddressStream,Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(4))]
          struct RetArea([u8; 8]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let vec0 = name;
          let ptr0 = vec0.as_ptr() as i32;
          let len0 = vec0.len() as i32;
          let (result1_0,result1_1,) = match address_family {
            Some(e) => (1i32, match e {
              IpAddressFamily::Ipv4 => 0,
              IpAddressFamily::Ipv6 => 1,
            }),
            None => {
              (0i32, 0i32)
            },
          };let ptr2 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-ip-name-lookup")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "resolve-addresses")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-ip-name-lookup_resolve-addresses")]
            fn wit_import(
            _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(network), ptr0, len0, result1_0, result1_1, match include_unavailable { true => 1, false => 0 }, ptr2);
          match i32::from(*((ptr2 + 0) as *const u8)) {
            0 => Ok(*((ptr2 + 4) as *const i32) as u32),
            1 => Err(match i32::from(*((ptr2 + 4) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Returns the next address from the resolver.
      /// 
      /// This function should be called multiple times. On each call, it will
      /// return the next address in connection order preference. If all
      /// addresses have been exhausted, this function returns `none`.
      /// After which, you should release the stream with `drop-resolve-address-stream`.
      /// 
      /// This function never returns IPv4-mapped IPv6 addresses.
      pub fn resolve_next_address(this: ResolveAddressStream,) -> Result<Option<IpAddress>,Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(2))]
          struct RetArea([u8; 22]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-ip-name-lookup")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "resolve-next-address")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-ip-name-lookup_resolve-next-address")]
            fn wit_import(
            _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(match i32::from(*((ptr0 + 2) as *const u8)) {
              0 => None,
              1 => Some(match i32::from(*((ptr0 + 4) as *const u8)) {
                0 => IpAddress::Ipv4((i32::from(*((ptr0 + 6) as *const u8)) as u8, i32::from(*((ptr0 + 7) as *const u8)) as u8, i32::from(*((ptr0 + 8) as *const u8)) as u8, i32::from(*((ptr0 + 9) as *const u8)) as u8)),
                1 => IpAddress::Ipv6((i32::from(*((ptr0 + 6) as *const u16)) as u16, i32::from(*((ptr0 + 8) as *const u16)) as u16, i32::from(*((ptr0 + 10) as *const u16)) as u16, i32::from(*((ptr0 + 12) as *const u16)) as u16, i32::from(*((ptr0 + 14) as *const u16)) as u16, i32::from(*((ptr0 + 16) as *const u16)) as u16, i32::from(*((ptr0 + 18) as *const u16)) as u16, i32::from(*((ptr0 + 20) as *const u16)) as u16)),
                _ => panic!("invalid enum discriminant"),
              }),
              _ => panic!("invalid enum discriminant"),
            }),
            1 => Err(match i32::from(*((ptr0 + 2) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Dispose of the specified `resolve-address-stream`, after which it may no longer be used.
      pub fn drop_resolve_address_stream(this: ResolveAddressStream,) -> (){
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[link(wasm_import_module = "wasi-ip-name-lookup")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "drop-resolve-address-stream")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-ip-name-lookup_drop-resolve-address-stream")]
            fn wit_import(
            _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this));
        }
      }
      #[allow(clippy::all)]
      /// Get/set the blocking mode of the stream.
      /// 
      /// By default a stream is in "blocking" mode, meaning that any function blocks and waits for its completion.
      /// When switched to "non-blocking" mode, operations that would block return an `again` error. After which
      /// the API consumer is expected to call `subscribe` and wait for completion using the wasi-poll module.
      /// 
      /// Note: these functions are here for WASI Preview2 only.
      /// They're planned to be removed when `async` is natively supported in Preview3.
      pub fn non_blocking(this: ResolveAddressStream,) -> Result<bool,Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-ip-name-lookup")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "non-blocking")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-ip-name-lookup_non-blocking")]
            fn wit_import(
            _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => false,
              1 => true,
              _ => panic!("invalid bool discriminant"),
            }),
            1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      pub fn set_non_blocking(this: ResolveAddressStream,value: bool,) -> Result<(),Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-ip-name-lookup")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "set-non-blocking")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-ip-name-lookup_set-non-blocking")]
            fn wit_import(
            _: i32, _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), match value { true => 1, false => 0 }, ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(()),
            1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Create a `pollable` which will resolve once the stream is ready for I/O.
      /// 
      /// Note: this function is here for WASI Preview2 only.
      /// It's planned to be removed when `async` is natively supported in Preview3.
      pub fn subscribe(this: ResolveAddressStream,) -> Pollable{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[link(wasm_import_module = "wasi-ip-name-lookup")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "subscribe")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-ip-name-lookup_subscribe")]
            fn wit_import(
            _: i32, ) -> i32;
          }
          let ret = wit_import(wit_bindgen::rt::as_i32(this));
          ret as u32
        }
      }
      
    }
    
    
    #[allow(clippy::all)]
    pub mod wasi_tcp{
      pub type InputStream = super::wasi_io::InputStream;
      pub type OutputStream = super::wasi_io::OutputStream;
      pub type Pollable = super::wasi_poll::Pollable;
      pub type Network = super::wasi_network::Network;
      pub type Error = super::wasi_network::Error;
      pub type IpAddressFamily = super::wasi_network::IpAddressFamily;
      /// A TCP socket handle.
      pub type TcpSocket = u32;
      #[repr(u8)]
      #[derive(Clone, Copy, PartialEq, Eq)]
      pub enum ShutdownType {
        /// Similar to `SHUT_RD` in POSIX.
        Receive,
        /// Similar to `SHUT_WR` in POSIX.
        Send,
        /// Similar to `SHUT_RDWR` in POSIX.
        Both,
      }
      impl core::fmt::Debug for ShutdownType {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          match self {
            ShutdownType::Receive => {
              f.debug_tuple("ShutdownType::Receive").finish()
            }
            ShutdownType::Send => {
              f.debug_tuple("ShutdownType::Send").finish()
            }
            ShutdownType::Both => {
              f.debug_tuple("ShutdownType::Both").finish()
            }
          }
        }
      }
      pub type Ipv6Address = (u16,u16,u16,u16,u16,u16,u16,u16,);
      #[repr(C)]
      #[derive(Copy, Clone)]
      pub struct Ipv6SocketAddress {
        pub port: u16,
        pub flow_info: u32,
        pub address: Ipv6Address,
        pub scope_id: u32,
      }
      impl core::fmt::Debug for Ipv6SocketAddress {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          f.debug_struct("Ipv6SocketAddress").field("port", &self.port).field("flow-info", &self.flow_info).field("address", &self.address).field("scope-id", &self.scope_id).finish()
        }
      }
      pub type Ipv4Address = (u8,u8,u8,u8,);
      #[repr(C)]
      #[derive(Copy, Clone)]
      pub struct Ipv4SocketAddress {
        pub port: u16,
        pub address: Ipv4Address,
      }
      impl core::fmt::Debug for Ipv4SocketAddress {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          f.debug_struct("Ipv4SocketAddress").field("port", &self.port).field("address", &self.address).finish()
        }
      }
      #[derive(Clone, Copy)]
      pub enum IpSocketAddress{
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
      #[allow(clippy::all)]
      /// Create a new TCP socket.
      /// 
      /// Similar to `socket(AF_INET or AF_INET6, SOCK_STREAM, IPPROTO_TCP)` in POSIX.
      /// 
      /// References:
      /// - https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html
      /// - https://man7.org/linux/man-pages/man2/socket.2.html
      pub fn create_tcp_socket(network: Network,address_family: IpAddressFamily,) -> Result<TcpSocket,Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(4))]
          struct RetArea([u8; 8]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-tcp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "create-tcp-socket")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_create-tcp-socket")]
            fn wit_import(
            _: i32, _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(network), match address_family {
            IpAddressFamily::Ipv4 => 0,
            IpAddressFamily::Ipv6 => 1,
          }, ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(*((ptr0 + 4) as *const i32) as u32),
            1 => Err(match i32::from(*((ptr0 + 4) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Dispose of the specified `tcp-socket`, after which it may no longer be used.
      pub fn drop_tcp_socket(this: TcpSocket,) -> (){
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[link(wasm_import_module = "wasi-tcp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "drop-tcp-socket")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_drop-tcp-socket")]
            fn wit_import(
            _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this));
        }
      }
      #[allow(clippy::all)]
      /// Bind the socket to a specific IP address and port.
      /// 
      /// If the IP address is zero (`0.0.0.0` in IPv4, `::` in IPv6), it is left to the implementation to decide which
      /// network interface(s) to bind to.
      /// If the TCP/UDP port is zero, the socket will be bound to a random free port.
      /// 
      /// When a socket is not explicitly bound, the first invocation to a listen or connect operation will
      /// implicitly bind the socket.
      /// 
      /// Returns an error if the socket is already bound.
      /// 
      /// References
      /// - https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html
      /// - https://man7.org/linux/man-pages/man2/bind.2.html
      pub fn bind(this: TcpSocket,local_address: IpSocketAddress,) -> Result<(),Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let (result4_0,result4_1,result4_2,result4_3,result4_4,result4_5,result4_6,result4_7,result4_8,result4_9,result4_10,result4_11,) = match local_address {
            IpSocketAddress::Ipv4(e) => {
              let Ipv4SocketAddress{ port:port0, address:address0, } = e;
              let (t1_0, t1_1, t1_2, t1_3, ) = address0;
              
              (0i32, wit_bindgen::rt::as_i32(port0), wit_bindgen::rt::as_i32(t1_0), wit_bindgen::rt::as_i32(t1_1), wit_bindgen::rt::as_i32(t1_2), wit_bindgen::rt::as_i32(t1_3), 0i32, 0i32, 0i32, 0i32, 0i32, 0i32)
            },
            IpSocketAddress::Ipv6(e) => {
              let Ipv6SocketAddress{ port:port2, flow_info:flow_info2, address:address2, scope_id:scope_id2, } = e;
              let (t3_0, t3_1, t3_2, t3_3, t3_4, t3_5, t3_6, t3_7, ) = address2;
              
              (1i32, wit_bindgen::rt::as_i32(port2), wit_bindgen::rt::as_i32(flow_info2), wit_bindgen::rt::as_i32(t3_0), wit_bindgen::rt::as_i32(t3_1), wit_bindgen::rt::as_i32(t3_2), wit_bindgen::rt::as_i32(t3_3), wit_bindgen::rt::as_i32(t3_4), wit_bindgen::rt::as_i32(t3_5), wit_bindgen::rt::as_i32(t3_6), wit_bindgen::rt::as_i32(t3_7), wit_bindgen::rt::as_i32(scope_id2))
            },
          };
          let ptr5 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-tcp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "bind")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_bind")]
            fn wit_import(
            _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), result4_0, result4_1, result4_2, result4_3, result4_4, result4_5, result4_6, result4_7, result4_8, result4_9, result4_10, result4_11, ptr5);
          match i32::from(*((ptr5 + 0) as *const u8)) {
            0 => Ok(()),
            1 => Err(match i32::from(*((ptr5 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Get the current bound address.
      /// 
      /// Returns an error if the socket is not bound.
      /// 
      /// References
      /// - https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockname.html
      /// - https://man7.org/linux/man-pages/man2/getsockname.2.html
      pub fn local_address(this: TcpSocket,) -> Result<IpSocketAddress,Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(4))]
          struct RetArea([u8; 36]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-tcp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "local-address")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_local-address")]
            fn wit_import(
            _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(match i32::from(*((ptr0 + 4) as *const u8)) {
              0 => IpSocketAddress::Ipv4(Ipv4SocketAddress{port:i32::from(*((ptr0 + 8) as *const u16)) as u16, address:(i32::from(*((ptr0 + 10) as *const u8)) as u8, i32::from(*((ptr0 + 11) as *const u8)) as u8, i32::from(*((ptr0 + 12) as *const u8)) as u8, i32::from(*((ptr0 + 13) as *const u8)) as u8), }),
              1 => IpSocketAddress::Ipv6(Ipv6SocketAddress{port:i32::from(*((ptr0 + 8) as *const u16)) as u16, flow_info:*((ptr0 + 12) as *const i32) as u32, address:(i32::from(*((ptr0 + 16) as *const u16)) as u16, i32::from(*((ptr0 + 18) as *const u16)) as u16, i32::from(*((ptr0 + 20) as *const u16)) as u16, i32::from(*((ptr0 + 22) as *const u16)) as u16, i32::from(*((ptr0 + 24) as *const u16)) as u16, i32::from(*((ptr0 + 26) as *const u16)) as u16, i32::from(*((ptr0 + 28) as *const u16)) as u16, i32::from(*((ptr0 + 30) as *const u16)) as u16), scope_id:*((ptr0 + 32) as *const i32) as u32, }),
              _ => panic!("invalid enum discriminant"),
            }),
            1 => Err(match i32::from(*((ptr0 + 4) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Connect to a remote endpoint.
      /// 
      /// Transitions the socket into the Connection state.
      /// Fails when the socket is already in the Connection or Listener state.
      /// 
      /// On success, this function returns a pair of streams that can be used to read & write to the connection.
      /// 
      /// References
      /// - https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html
      /// - https://man7.org/linux/man-pages/man2/connect.2.html
      pub fn connect(this: TcpSocket,remote_address: IpSocketAddress,) -> Result<(InputStream,OutputStream,),Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(4))]
          struct RetArea([u8; 12]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let (result4_0,result4_1,result4_2,result4_3,result4_4,result4_5,result4_6,result4_7,result4_8,result4_9,result4_10,result4_11,) = match remote_address {
            IpSocketAddress::Ipv4(e) => {
              let Ipv4SocketAddress{ port:port0, address:address0, } = e;
              let (t1_0, t1_1, t1_2, t1_3, ) = address0;
              
              (0i32, wit_bindgen::rt::as_i32(port0), wit_bindgen::rt::as_i32(t1_0), wit_bindgen::rt::as_i32(t1_1), wit_bindgen::rt::as_i32(t1_2), wit_bindgen::rt::as_i32(t1_3), 0i32, 0i32, 0i32, 0i32, 0i32, 0i32)
            },
            IpSocketAddress::Ipv6(e) => {
              let Ipv6SocketAddress{ port:port2, flow_info:flow_info2, address:address2, scope_id:scope_id2, } = e;
              let (t3_0, t3_1, t3_2, t3_3, t3_4, t3_5, t3_6, t3_7, ) = address2;
              
              (1i32, wit_bindgen::rt::as_i32(port2), wit_bindgen::rt::as_i32(flow_info2), wit_bindgen::rt::as_i32(t3_0), wit_bindgen::rt::as_i32(t3_1), wit_bindgen::rt::as_i32(t3_2), wit_bindgen::rt::as_i32(t3_3), wit_bindgen::rt::as_i32(t3_4), wit_bindgen::rt::as_i32(t3_5), wit_bindgen::rt::as_i32(t3_6), wit_bindgen::rt::as_i32(t3_7), wit_bindgen::rt::as_i32(scope_id2))
            },
          };
          let ptr5 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-tcp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "connect")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_connect")]
            fn wit_import(
            _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), result4_0, result4_1, result4_2, result4_3, result4_4, result4_5, result4_6, result4_7, result4_8, result4_9, result4_10, result4_11, ptr5);
          match i32::from(*((ptr5 + 0) as *const u8)) {
            0 => Ok((*((ptr5 + 4) as *const i32) as u32, *((ptr5 + 8) as *const i32) as u32)),
            1 => Err(match i32::from(*((ptr5 + 4) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Gracefully shut down the connection.
      /// 
      /// - receive: the socket is not expecting to receive any more data from the peer. All subsequent read
      /// operations on the `input-stream` associated with this socket will return an End Of Stream indication.
      /// Any data still in the receive queue at time of calling `shutdown` will be discarded.
      /// - send: the socket is not expecting to send any more data to the peer. All subsequent write
      /// operations on the `output-stream` associated with this socket will return an error.
      /// - both: same effect as receive & send combined.
      /// 
      /// The shutdown function does not close the socket.
      /// 
      /// Fails when the socket is not in the Connection state.
      /// 
      /// References
      /// - https://pubs.opengroup.org/onlinepubs/9699919799/functions/shutdown.html
      /// - https://man7.org/linux/man-pages/man2/shutdown.2.html
      pub fn shutdown(this: TcpSocket,shutdown_type: ShutdownType,) -> Result<(),Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-tcp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "shutdown")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_shutdown")]
            fn wit_import(
            _: i32, _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), match shutdown_type {
            ShutdownType::Receive => 0,
            ShutdownType::Send => 1,
            ShutdownType::Both => 2,
          }, ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(()),
            1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Start listening for new connections.
      /// 
      /// Transitions the socket into the Listener state.
      /// Fails when the socket is already in the Connection or Listener state.
      /// 
      /// References
      /// - https://pubs.opengroup.org/onlinepubs/9699919799/functions/listen.html
      /// - https://man7.org/linux/man-pages/man2/listen.2.html
      pub fn listen(this: TcpSocket,backlog_size_hint: Option<u64>,) -> Result<(),Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let (result0_0,result0_1,) = match backlog_size_hint {
            Some(e) => (1i32, wit_bindgen::rt::as_i64(e)),
            None => {
              (0i32, 0i64)
            },
          };let ptr1 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-tcp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "listen")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_listen")]
            fn wit_import(
            _: i32, _: i32, _: i64, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), result0_0, result0_1, ptr1);
          match i32::from(*((ptr1 + 0) as *const u8)) {
            0 => Ok(()),
            1 => Err(match i32::from(*((ptr1 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Fails when the socket is not in the Connection state.
      /// 
      /// References
      /// - https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpeername.html
      /// - https://man7.org/linux/man-pages/man2/getpeername.2.html
      pub fn remote_address(this: TcpSocket,) -> Result<IpSocketAddress,Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(4))]
          struct RetArea([u8; 36]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-tcp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "remote-address")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_remote-address")]
            fn wit_import(
            _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(match i32::from(*((ptr0 + 4) as *const u8)) {
              0 => IpSocketAddress::Ipv4(Ipv4SocketAddress{port:i32::from(*((ptr0 + 8) as *const u16)) as u16, address:(i32::from(*((ptr0 + 10) as *const u8)) as u8, i32::from(*((ptr0 + 11) as *const u8)) as u8, i32::from(*((ptr0 + 12) as *const u8)) as u8, i32::from(*((ptr0 + 13) as *const u8)) as u8), }),
              1 => IpSocketAddress::Ipv6(Ipv6SocketAddress{port:i32::from(*((ptr0 + 8) as *const u16)) as u16, flow_info:*((ptr0 + 12) as *const i32) as u32, address:(i32::from(*((ptr0 + 16) as *const u16)) as u16, i32::from(*((ptr0 + 18) as *const u16)) as u16, i32::from(*((ptr0 + 20) as *const u16)) as u16, i32::from(*((ptr0 + 22) as *const u16)) as u16, i32::from(*((ptr0 + 24) as *const u16)) as u16, i32::from(*((ptr0 + 26) as *const u16)) as u16, i32::from(*((ptr0 + 28) as *const u16)) as u16, i32::from(*((ptr0 + 30) as *const u16)) as u16), scope_id:*((ptr0 + 32) as *const i32) as u32, }),
              _ => panic!("invalid enum discriminant"),
            }),
            1 => Err(match i32::from(*((ptr0 + 4) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Accept a new client socket.
      /// 
      /// The returned socket is bound and in the Connection state.
      /// 
      /// On success, this function returns the newly accepted client socket along with
      /// a pair of streams that can be used to read & write to the connection.
      /// 
      /// Fails when this socket is not in the Listening state.
      /// 
      /// References:
      /// - https://pubs.opengroup.org/onlinepubs/9699919799/functions/accept.html
      /// - https://man7.org/linux/man-pages/man2/accept.2.html
      pub fn accept(this: TcpSocket,) -> Result<(TcpSocket,InputStream,OutputStream,),Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(4))]
          struct RetArea([u8; 16]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-tcp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "accept")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_accept")]
            fn wit_import(
            _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok((*((ptr0 + 4) as *const i32) as u32, *((ptr0 + 8) as *const i32) as u32, *((ptr0 + 12) as *const i32) as u32)),
            1 => Err(match i32::from(*((ptr0 + 4) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Equivalent to the SO_KEEPALIVE socket option.
      pub fn keep_alive(this: TcpSocket,) -> Result<bool,Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-tcp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "keep-alive")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_keep-alive")]
            fn wit_import(
            _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => false,
              1 => true,
              _ => panic!("invalid bool discriminant"),
            }),
            1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      pub fn set_keep_alive(this: TcpSocket,value: bool,) -> Result<(),Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-tcp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "set-keep-alive")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_set-keep-alive")]
            fn wit_import(
            _: i32, _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), match value { true => 1, false => 0 }, ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(()),
            1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Equivalent to the TCP_NODELAY socket option.
      pub fn no_delay(this: TcpSocket,) -> Result<bool,Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-tcp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "no-delay")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_no-delay")]
            fn wit_import(
            _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => false,
              1 => true,
              _ => panic!("invalid bool discriminant"),
            }),
            1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      pub fn set_no_delay(this: TcpSocket,value: bool,) -> Result<(),Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-tcp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "set-no-delay")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_set-no-delay")]
            fn wit_import(
            _: i32, _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), match value { true => 1, false => 0 }, ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(()),
            1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// The kernel buffer space reserved for sends/receives on this socket.
      /// 
      /// Note #1: an implementation may choose to cap or round the buffer size when setting the value.
      /// In other words, after setting a value, reading the same setting back may return a different value.
      /// 
      /// Note #2: there is not necessarily a direct relationship between the kernel buffer size and the bytes of
      /// actual data to be sent/received by the application, because the kernel might also use the buffer space
      /// for internal metadata structures.
      /// 
      /// Fails when this socket is in the Listening state.
      /// 
      /// Equivalent to the SO_RCVBUF and SO_SNDBUF socket options.
      pub fn receive_buffer_size(this: TcpSocket,) -> Result<u64,Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(8))]
          struct RetArea([u8; 16]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-tcp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "receive-buffer-size")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_receive-buffer-size")]
            fn wit_import(
            _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(*((ptr0 + 8) as *const i64) as u64),
            1 => Err(match i32::from(*((ptr0 + 8) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      pub fn set_receive_buffer_size(this: TcpSocket,value: u64,) -> Result<(),Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-tcp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "set-receive-buffer-size")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_set-receive-buffer-size")]
            fn wit_import(
            _: i32, _: i64, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), wit_bindgen::rt::as_i64(value), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(()),
            1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      pub fn send_buffer_size(this: TcpSocket,) -> Result<u64,Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(8))]
          struct RetArea([u8; 16]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-tcp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "send-buffer-size")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_send-buffer-size")]
            fn wit_import(
            _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(*((ptr0 + 8) as *const i64) as u64),
            1 => Err(match i32::from(*((ptr0 + 8) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      pub fn set_send_buffer_size(this: TcpSocket,value: u64,) -> Result<(),Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-tcp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "set-send-buffer-size")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_set-send-buffer-size")]
            fn wit_import(
            _: i32, _: i64, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), wit_bindgen::rt::as_i64(value), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(()),
            1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Whether this is a IPv4 or IPv6 socket.
      /// 
      /// Equivalent to the SO_DOMAIN socket option.
      pub fn address_family(this: TcpSocket,) -> IpAddressFamily{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[link(wasm_import_module = "wasi-tcp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "address-family")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_address-family")]
            fn wit_import(
            _: i32, ) -> i32;
          }
          let ret = wit_import(wit_bindgen::rt::as_i32(this));
          match ret {
            0 => IpAddressFamily::Ipv4,
            1 => IpAddressFamily::Ipv6,
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Equivalent to the IP_TTL & IPV6_UNICAST_HOPS socket options.
      pub fn unicast_hop_limit(this: TcpSocket,) -> Result<u8,Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-tcp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "unicast-hop-limit")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_unicast-hop-limit")]
            fn wit_import(
            _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(i32::from(*((ptr0 + 1) as *const u8)) as u8),
            1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      pub fn set_unicast_hop_limit(this: TcpSocket,value: u8,) -> Result<(),Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-tcp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "set-unicast-hop-limit")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_set-unicast-hop-limit")]
            fn wit_import(
            _: i32, _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), wit_bindgen::rt::as_i32(value), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(()),
            1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Whether IPv4 compatibility (dual-stack) mode is disabled or not.
      /// Implementations are not required to support dual-stack mode, so calling `set-ipv6-only(false)` might fail.
      /// 
      /// Fails when called on an IPv4 socket.
      /// 
      /// Equivalent to the IPV6_V6ONLY socket option.
      pub fn ipv6_only(this: TcpSocket,) -> Result<bool,Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-tcp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "ipv6-only")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_ipv6-only")]
            fn wit_import(
            _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => false,
              1 => true,
              _ => panic!("invalid bool discriminant"),
            }),
            1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      pub fn set_ipv6_only(this: TcpSocket,value: bool,) -> Result<(),Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-tcp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "set-ipv6-only")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_set-ipv6-only")]
            fn wit_import(
            _: i32, _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), match value { true => 1, false => 0 }, ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(()),
            1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Get/set the blocking mode of the socket.
      /// 
      /// By default a socket is in "blocking" mode, meaning that any function blocks and waits for its completion.
      /// When switched to "non-blocking" mode, operations that would block return an `again` error. After which
      /// the API consumer is expected to call `subscribe` and wait for completion using the wasi-poll module.
      /// 
      /// Note: these functions are here for WASI Preview2 only.
      /// They're planned to be removed when `async` is natively supported in Preview3.
      pub fn non_blocking(this: TcpSocket,) -> Result<bool,Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-tcp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "non-blocking")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_non-blocking")]
            fn wit_import(
            _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => false,
              1 => true,
              _ => panic!("invalid bool discriminant"),
            }),
            1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      pub fn set_non_blocking(this: TcpSocket,value: bool,) -> Result<(),Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-tcp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "set-non-blocking")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_set-non-blocking")]
            fn wit_import(
            _: i32, _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), match value { true => 1, false => 0 }, ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(()),
            1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Create a `pollable` which will resolve once the socket is ready for I/O.
      /// 
      /// Note: this function is here for WASI Preview2 only.
      /// It's planned to be removed when `async` is natively supported in Preview3.
      pub fn subscribe(this: TcpSocket,) -> Pollable{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[link(wasm_import_module = "wasi-tcp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "subscribe")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-tcp_subscribe")]
            fn wit_import(
            _: i32, ) -> i32;
          }
          let ret = wit_import(wit_bindgen::rt::as_i32(this));
          ret as u32
        }
      }
      
    }
    
    
    #[allow(clippy::all)]
    pub mod wasi_udp{
      pub type Pollable = super::wasi_poll::Pollable;
      pub type Network = super::wasi_network::Network;
      pub type Error = super::wasi_network::Error;
      pub type IpAddressFamily = super::wasi_network::IpAddressFamily;
      /// A UDP socket handle.
      pub type UdpSocket = u32;
      pub type Ipv6Address = (u16,u16,u16,u16,u16,u16,u16,u16,);
      #[repr(C)]
      #[derive(Copy, Clone)]
      pub struct Ipv6SocketAddress {
        pub port: u16,
        pub flow_info: u32,
        pub address: Ipv6Address,
        pub scope_id: u32,
      }
      impl core::fmt::Debug for Ipv6SocketAddress {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          f.debug_struct("Ipv6SocketAddress").field("port", &self.port).field("flow-info", &self.flow_info).field("address", &self.address).field("scope-id", &self.scope_id).finish()
        }
      }
      pub type Ipv4Address = (u8,u8,u8,u8,);
      #[repr(C)]
      #[derive(Copy, Clone)]
      pub struct Ipv4SocketAddress {
        pub port: u16,
        pub address: Ipv4Address,
      }
      impl core::fmt::Debug for Ipv4SocketAddress {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          f.debug_struct("Ipv4SocketAddress").field("port", &self.port).field("address", &self.address).finish()
        }
      }
      #[derive(Clone, Copy)]
      pub enum IpSocketAddress{
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
      #[derive(Clone)]
      pub struct DatagramParam<'a,> {
        pub data: &'a [u8],
        pub remote_address: IpSocketAddress,
      }
      impl<'a,> core::fmt::Debug for DatagramParam<'a,> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          f.debug_struct("DatagramParam").field("data", &self.data).field("remote-address", &self.remote_address).finish()
        }
      }
      #[derive(Clone)]
      pub struct DatagramResult {
        pub data: wit_bindgen::rt::vec::Vec::<u8>,
        pub remote_address: IpSocketAddress,
      }
      impl core::fmt::Debug for DatagramResult {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          f.debug_struct("DatagramResult").field("data", &self.data).field("remote-address", &self.remote_address).finish()
        }
      }
      #[allow(clippy::all)]
      /// Create a new UDP socket.
      /// 
      /// Similar to `socket(AF_INET or AF_INET6, SOCK_DGRAM, IPPROTO_UDP)` in POSIX.
      /// 
      /// References:
      /// - https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html
      /// - https://man7.org/linux/man-pages/man2/socket.2.html
      pub fn create_udp_socket(network: Network,address_family: IpAddressFamily,) -> Result<UdpSocket,Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(4))]
          struct RetArea([u8; 8]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-udp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "create-udp-socket")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-udp_create-udp-socket")]
            fn wit_import(
            _: i32, _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(network), match address_family {
            IpAddressFamily::Ipv4 => 0,
            IpAddressFamily::Ipv6 => 1,
          }, ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(*((ptr0 + 4) as *const i32) as u32),
            1 => Err(match i32::from(*((ptr0 + 4) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Dispose of the specified `udp-socket`, after which it may no longer be used.
      pub fn drop_udp_socket(this: UdpSocket,) -> (){
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[link(wasm_import_module = "wasi-udp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "drop-udp-socket")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-udp_drop-udp-socket")]
            fn wit_import(
            _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this));
        }
      }
      #[allow(clippy::all)]
      /// Bind the socket to a specific IP address and port.
      /// 
      /// If the IP address is zero (`0.0.0.0` in IPv4, `::` in IPv6), it is left to the implementation to decide which
      /// network interface(s) to bind to.
      /// If the TCP/UDP port is zero, the socket will be bound to a random free port.
      /// 
      /// When a socket is not explicitly bound, the first invocation to a send or receive operation will
      /// implicitly bind the socket.
      /// 
      /// Returns an error if the socket is already bound.
      /// 
      /// TODO: disallow wildcard binds as long as there isn't a way to pass the local address to send & receive?
      /// - https://blog.cloudflare.com/everything-you-ever-wanted-to-know-about-udp-sockets-but-were-afraid-to-ask-part-1/#sourcing-packets-from-a-wildcard-socket
      /// - https://blog.powerdns.com/2012/10/08/on-binding-datagram-udp-sockets-to-the-any-addresses/
      /// 
      /// References
      /// - https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html
      /// - https://man7.org/linux/man-pages/man2/bind.2.html
      pub fn bind(this: UdpSocket,local_address: IpSocketAddress,) -> Result<(),Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let (result4_0,result4_1,result4_2,result4_3,result4_4,result4_5,result4_6,result4_7,result4_8,result4_9,result4_10,result4_11,) = match local_address {
            IpSocketAddress::Ipv4(e) => {
              let Ipv4SocketAddress{ port:port0, address:address0, } = e;
              let (t1_0, t1_1, t1_2, t1_3, ) = address0;
              
              (0i32, wit_bindgen::rt::as_i32(port0), wit_bindgen::rt::as_i32(t1_0), wit_bindgen::rt::as_i32(t1_1), wit_bindgen::rt::as_i32(t1_2), wit_bindgen::rt::as_i32(t1_3), 0i32, 0i32, 0i32, 0i32, 0i32, 0i32)
            },
            IpSocketAddress::Ipv6(e) => {
              let Ipv6SocketAddress{ port:port2, flow_info:flow_info2, address:address2, scope_id:scope_id2, } = e;
              let (t3_0, t3_1, t3_2, t3_3, t3_4, t3_5, t3_6, t3_7, ) = address2;
              
              (1i32, wit_bindgen::rt::as_i32(port2), wit_bindgen::rt::as_i32(flow_info2), wit_bindgen::rt::as_i32(t3_0), wit_bindgen::rt::as_i32(t3_1), wit_bindgen::rt::as_i32(t3_2), wit_bindgen::rt::as_i32(t3_3), wit_bindgen::rt::as_i32(t3_4), wit_bindgen::rt::as_i32(t3_5), wit_bindgen::rt::as_i32(t3_6), wit_bindgen::rt::as_i32(t3_7), wit_bindgen::rt::as_i32(scope_id2))
            },
          };
          let ptr5 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-udp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "bind")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-udp_bind")]
            fn wit_import(
            _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), result4_0, result4_1, result4_2, result4_3, result4_4, result4_5, result4_6, result4_7, result4_8, result4_9, result4_10, result4_11, ptr5);
          match i32::from(*((ptr5 + 0) as *const u8)) {
            0 => Ok(()),
            1 => Err(match i32::from(*((ptr5 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Get the current bound address.
      /// 
      /// Returns an error if the socket is not bound.
      /// 
      /// References
      /// - https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockname.html
      /// - https://man7.org/linux/man-pages/man2/getsockname.2.html
      pub fn local_address(this: UdpSocket,) -> Result<IpSocketAddress,Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(4))]
          struct RetArea([u8; 36]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-udp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "local-address")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-udp_local-address")]
            fn wit_import(
            _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(match i32::from(*((ptr0 + 4) as *const u8)) {
              0 => IpSocketAddress::Ipv4(Ipv4SocketAddress{port:i32::from(*((ptr0 + 8) as *const u16)) as u16, address:(i32::from(*((ptr0 + 10) as *const u8)) as u8, i32::from(*((ptr0 + 11) as *const u8)) as u8, i32::from(*((ptr0 + 12) as *const u8)) as u8, i32::from(*((ptr0 + 13) as *const u8)) as u8), }),
              1 => IpSocketAddress::Ipv6(Ipv6SocketAddress{port:i32::from(*((ptr0 + 8) as *const u16)) as u16, flow_info:*((ptr0 + 12) as *const i32) as u32, address:(i32::from(*((ptr0 + 16) as *const u16)) as u16, i32::from(*((ptr0 + 18) as *const u16)) as u16, i32::from(*((ptr0 + 20) as *const u16)) as u16, i32::from(*((ptr0 + 22) as *const u16)) as u16, i32::from(*((ptr0 + 24) as *const u16)) as u16, i32::from(*((ptr0 + 26) as *const u16)) as u16, i32::from(*((ptr0 + 28) as *const u16)) as u16, i32::from(*((ptr0 + 30) as *const u16)) as u16), scope_id:*((ptr0 + 32) as *const i32) as u32, }),
              _ => panic!("invalid enum discriminant"),
            }),
            1 => Err(match i32::from(*((ptr0 + 4) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// receive a message.
      /// 
      /// Returns:
      /// - The sender address of the datagram
      /// - The number of bytes read.
      /// - When the received datagram is larger than the provided buffers,
      /// the excess data is lost and the `truncated` flag will be set.
      /// 
      /// References
      /// - https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvfrom.html
      /// - https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvmsg.html
      /// - https://man7.org/linux/man-pages/man2/recv.2.html
      pub fn receive(this: UdpSocket,) -> Result<DatagramResult,Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(4))]
          struct RetArea([u8; 44]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-udp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "receive")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-udp_receive")]
            fn wit_import(
            _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok({
              let len1 = *((ptr0 + 8) as *const i32) as usize;
              
              DatagramResult{data:Vec::from_raw_parts(*((ptr0 + 4) as *const i32) as *mut _, len1, len1), remote_address:match i32::from(*((ptr0 + 12) as *const u8)) {
                0 => IpSocketAddress::Ipv4(Ipv4SocketAddress{port:i32::from(*((ptr0 + 16) as *const u16)) as u16, address:(i32::from(*((ptr0 + 18) as *const u8)) as u8, i32::from(*((ptr0 + 19) as *const u8)) as u8, i32::from(*((ptr0 + 20) as *const u8)) as u8, i32::from(*((ptr0 + 21) as *const u8)) as u8), }),
                1 => IpSocketAddress::Ipv6(Ipv6SocketAddress{port:i32::from(*((ptr0 + 16) as *const u16)) as u16, flow_info:*((ptr0 + 20) as *const i32) as u32, address:(i32::from(*((ptr0 + 24) as *const u16)) as u16, i32::from(*((ptr0 + 26) as *const u16)) as u16, i32::from(*((ptr0 + 28) as *const u16)) as u16, i32::from(*((ptr0 + 30) as *const u16)) as u16, i32::from(*((ptr0 + 32) as *const u16)) as u16, i32::from(*((ptr0 + 34) as *const u16)) as u16, i32::from(*((ptr0 + 36) as *const u16)) as u16, i32::from(*((ptr0 + 38) as *const u16)) as u16), scope_id:*((ptr0 + 40) as *const i32) as u32, }),
                _ => panic!("invalid enum discriminant"),
              }, }
            }),
            1 => Err(match i32::from(*((ptr0 + 4) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// send a message to a specific destination address.
      /// 
      /// The remote address option is required. To send a message to the "connected" peer,
      /// call `remote-address` to get their address.
      /// 
      /// References
      /// - https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendto.html
      /// - https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendmsg.html
      /// - https://man7.org/linux/man-pages/man2/send.2.html
      pub fn send(this: UdpSocket,datagram: DatagramParam<'_,>,) -> Result<(),Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let DatagramParam{ data:data0, remote_address:remote_address0, } = datagram;
          let vec1 = data0;
          let ptr1 = vec1.as_ptr() as i32;
          let len1 = vec1.len() as i32;
          let (result6_0,result6_1,result6_2,result6_3,result6_4,result6_5,result6_6,result6_7,result6_8,result6_9,result6_10,result6_11,) = match remote_address0 {
            IpSocketAddress::Ipv4(e) => {
              let Ipv4SocketAddress{ port:port2, address:address2, } = e;
              let (t3_0, t3_1, t3_2, t3_3, ) = address2;
              
              (0i32, wit_bindgen::rt::as_i32(port2), wit_bindgen::rt::as_i32(t3_0), wit_bindgen::rt::as_i32(t3_1), wit_bindgen::rt::as_i32(t3_2), wit_bindgen::rt::as_i32(t3_3), 0i32, 0i32, 0i32, 0i32, 0i32, 0i32)
            },
            IpSocketAddress::Ipv6(e) => {
              let Ipv6SocketAddress{ port:port4, flow_info:flow_info4, address:address4, scope_id:scope_id4, } = e;
              let (t5_0, t5_1, t5_2, t5_3, t5_4, t5_5, t5_6, t5_7, ) = address4;
              
              (1i32, wit_bindgen::rt::as_i32(port4), wit_bindgen::rt::as_i32(flow_info4), wit_bindgen::rt::as_i32(t5_0), wit_bindgen::rt::as_i32(t5_1), wit_bindgen::rt::as_i32(t5_2), wit_bindgen::rt::as_i32(t5_3), wit_bindgen::rt::as_i32(t5_4), wit_bindgen::rt::as_i32(t5_5), wit_bindgen::rt::as_i32(t5_6), wit_bindgen::rt::as_i32(t5_7), wit_bindgen::rt::as_i32(scope_id4))
            },
          };
          let ptr7 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-udp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "send")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-udp_send")]
            fn wit_import(
            _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), ptr1, len1, result6_0, result6_1, result6_2, result6_3, result6_4, result6_5, result6_6, result6_7, result6_8, result6_9, result6_10, result6_11, ptr7);
          match i32::from(*((ptr7 + 0) as *const u8)) {
            0 => Ok(()),
            1 => Err(match i32::from(*((ptr7 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Set the destination address.
      /// 
      /// When a destination address is set:
      /// - all receive operations will only return datagrams sent from the provided `remote-address`.
      /// - the `send` function can still be used to send to any other destination, however you can't receive their response.
      /// 
      /// Similar to `connect(sock, ...)` in POSIX.
      /// 
      /// Note that this function does not generate any network traffic and the peer is not aware of this "connection".
      /// 
      /// TODO: "connect" is a rather odd name for this function because it doesn't reflect what's actually happening.
      /// Feels like it was chosen just to shoehorn UDP into the existing Socket interface.
      /// Do we have to keep this name?
      /// 
      /// TODO: add unconnect ability.
      /// 
      /// References
      /// - https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html
      /// - https://man7.org/linux/man-pages/man2/connect.2.html
      pub fn connect(this: UdpSocket,remote_address: IpSocketAddress,) -> Result<(),Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let (result4_0,result4_1,result4_2,result4_3,result4_4,result4_5,result4_6,result4_7,result4_8,result4_9,result4_10,result4_11,) = match remote_address {
            IpSocketAddress::Ipv4(e) => {
              let Ipv4SocketAddress{ port:port0, address:address0, } = e;
              let (t1_0, t1_1, t1_2, t1_3, ) = address0;
              
              (0i32, wit_bindgen::rt::as_i32(port0), wit_bindgen::rt::as_i32(t1_0), wit_bindgen::rt::as_i32(t1_1), wit_bindgen::rt::as_i32(t1_2), wit_bindgen::rt::as_i32(t1_3), 0i32, 0i32, 0i32, 0i32, 0i32, 0i32)
            },
            IpSocketAddress::Ipv6(e) => {
              let Ipv6SocketAddress{ port:port2, flow_info:flow_info2, address:address2, scope_id:scope_id2, } = e;
              let (t3_0, t3_1, t3_2, t3_3, t3_4, t3_5, t3_6, t3_7, ) = address2;
              
              (1i32, wit_bindgen::rt::as_i32(port2), wit_bindgen::rt::as_i32(flow_info2), wit_bindgen::rt::as_i32(t3_0), wit_bindgen::rt::as_i32(t3_1), wit_bindgen::rt::as_i32(t3_2), wit_bindgen::rt::as_i32(t3_3), wit_bindgen::rt::as_i32(t3_4), wit_bindgen::rt::as_i32(t3_5), wit_bindgen::rt::as_i32(t3_6), wit_bindgen::rt::as_i32(t3_7), wit_bindgen::rt::as_i32(scope_id2))
            },
          };
          let ptr5 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-udp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "connect")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-udp_connect")]
            fn wit_import(
            _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), result4_0, result4_1, result4_2, result4_3, result4_4, result4_5, result4_6, result4_7, result4_8, result4_9, result4_10, result4_11, ptr5);
          match i32::from(*((ptr5 + 0) as *const u8)) {
            0 => Ok(()),
            1 => Err(match i32::from(*((ptr5 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Get the address set with `connect`.
      /// 
      /// References
      /// - https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpeername.html
      /// - https://man7.org/linux/man-pages/man2/getpeername.2.html
      pub fn remote_address(this: UdpSocket,) -> Result<IpSocketAddress,Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(4))]
          struct RetArea([u8; 36]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-udp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "remote-address")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-udp_remote-address")]
            fn wit_import(
            _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(match i32::from(*((ptr0 + 4) as *const u8)) {
              0 => IpSocketAddress::Ipv4(Ipv4SocketAddress{port:i32::from(*((ptr0 + 8) as *const u16)) as u16, address:(i32::from(*((ptr0 + 10) as *const u8)) as u8, i32::from(*((ptr0 + 11) as *const u8)) as u8, i32::from(*((ptr0 + 12) as *const u8)) as u8, i32::from(*((ptr0 + 13) as *const u8)) as u8), }),
              1 => IpSocketAddress::Ipv6(Ipv6SocketAddress{port:i32::from(*((ptr0 + 8) as *const u16)) as u16, flow_info:*((ptr0 + 12) as *const i32) as u32, address:(i32::from(*((ptr0 + 16) as *const u16)) as u16, i32::from(*((ptr0 + 18) as *const u16)) as u16, i32::from(*((ptr0 + 20) as *const u16)) as u16, i32::from(*((ptr0 + 22) as *const u16)) as u16, i32::from(*((ptr0 + 24) as *const u16)) as u16, i32::from(*((ptr0 + 26) as *const u16)) as u16, i32::from(*((ptr0 + 28) as *const u16)) as u16, i32::from(*((ptr0 + 30) as *const u16)) as u16), scope_id:*((ptr0 + 32) as *const i32) as u32, }),
              _ => panic!("invalid enum discriminant"),
            }),
            1 => Err(match i32::from(*((ptr0 + 4) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// The kernel buffer space reserved for sends/receives on this socket.
      /// 
      /// Note #1: an implementation may choose to cap or round the buffer size when setting the value.
      /// In other words, after setting a value, reading the same setting back may return a different value.
      /// 
      /// Note #2: there is not necessarily a direct relationship between the kernel buffer size and the bytes of
      /// actual data to be sent/received by the application, because the kernel might also use the buffer space
      /// for internal metadata structures.
      /// 
      /// Fails when this socket is in the Listening state.
      /// 
      /// Equivalent to the SO_RCVBUF and SO_SNDBUF socket options.
      pub fn receive_buffer_size(this: UdpSocket,) -> Result<u64,Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(8))]
          struct RetArea([u8; 16]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-udp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "receive-buffer-size")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-udp_receive-buffer-size")]
            fn wit_import(
            _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(*((ptr0 + 8) as *const i64) as u64),
            1 => Err(match i32::from(*((ptr0 + 8) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      pub fn set_receive_buffer_size(this: UdpSocket,value: u64,) -> Result<(),Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-udp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "set-receive-buffer-size")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-udp_set-receive-buffer-size")]
            fn wit_import(
            _: i32, _: i64, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), wit_bindgen::rt::as_i64(value), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(()),
            1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      pub fn send_buffer_size(this: UdpSocket,) -> Result<u64,Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(8))]
          struct RetArea([u8; 16]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-udp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "send-buffer-size")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-udp_send-buffer-size")]
            fn wit_import(
            _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(*((ptr0 + 8) as *const i64) as u64),
            1 => Err(match i32::from(*((ptr0 + 8) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      pub fn set_send_buffer_size(this: UdpSocket,value: u64,) -> Result<(),Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-udp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "set-send-buffer-size")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-udp_set-send-buffer-size")]
            fn wit_import(
            _: i32, _: i64, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), wit_bindgen::rt::as_i64(value), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(()),
            1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Whether this is a IPv4 or IPv6 socket.
      /// 
      /// Equivalent to the SO_DOMAIN socket option.
      pub fn address_family(this: UdpSocket,) -> IpAddressFamily{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[link(wasm_import_module = "wasi-udp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "address-family")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-udp_address-family")]
            fn wit_import(
            _: i32, ) -> i32;
          }
          let ret = wit_import(wit_bindgen::rt::as_i32(this));
          match ret {
            0 => IpAddressFamily::Ipv4,
            1 => IpAddressFamily::Ipv6,
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Equivalent to the IP_TTL & IPV6_UNICAST_HOPS socket options.
      pub fn unicast_hop_limit(this: UdpSocket,) -> Result<u8,Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-udp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "unicast-hop-limit")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-udp_unicast-hop-limit")]
            fn wit_import(
            _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(i32::from(*((ptr0 + 1) as *const u8)) as u8),
            1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      pub fn set_unicast_hop_limit(this: UdpSocket,value: u8,) -> Result<(),Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-udp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "set-unicast-hop-limit")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-udp_set-unicast-hop-limit")]
            fn wit_import(
            _: i32, _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), wit_bindgen::rt::as_i32(value), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(()),
            1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Whether IPv4 compatibility (dual-stack) mode is disabled or not.
      /// Implementations are not required to support dual-stack mode, so calling `set-ipv6-only(false)` might fail.
      /// 
      /// Fails when called on an IPv4 socket.
      /// 
      /// Equivalent to the IPV6_V6ONLY socket option.
      pub fn ipv6_only(this: UdpSocket,) -> Result<bool,Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-udp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "ipv6-only")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-udp_ipv6-only")]
            fn wit_import(
            _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => false,
              1 => true,
              _ => panic!("invalid bool discriminant"),
            }),
            1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      pub fn set_ipv6_only(this: UdpSocket,value: bool,) -> Result<(),Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-udp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "set-ipv6-only")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-udp_set-ipv6-only")]
            fn wit_import(
            _: i32, _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), match value { true => 1, false => 0 }, ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(()),
            1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Get/set the blocking mode of the socket.
      /// 
      /// By default a socket is in "blocking" mode, meaning that any function blocks and waits for its completion.
      /// When switched to "non-blocking" mode, operations that would block return an `again` error. After which
      /// the API consumer is expected to call `subscribe` and wait for completion using the wasi-poll module.
      /// 
      /// Note: these functions are here for WASI Preview2 only.
      /// They're planned to be removed when `async` is natively supported in Preview3.
      pub fn non_blocking(this: UdpSocket,) -> Result<bool,Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-udp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "non-blocking")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-udp_non-blocking")]
            fn wit_import(
            _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => false,
              1 => true,
              _ => panic!("invalid bool discriminant"),
            }),
            1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      pub fn set_non_blocking(this: UdpSocket,value: bool,) -> Result<(),Error>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(1))]
          struct RetArea([u8; 2]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-udp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "set-non-blocking")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-udp_set-non-blocking")]
            fn wit_import(
            _: i32, _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(this), match value { true => 1, false => 0 }, ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok(()),
            1 => Err(match i32::from(*((ptr0 + 1) as *const u8)) {
              0 => Error::Unknown,
              1 => Error::Again,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      /// Create a `pollable` which will resolve once the socket is ready for I/O.
      /// 
      /// Note: this function is here for WASI Preview2 only.
      /// It's planned to be removed when `async` is natively supported in Preview3.
      pub fn subscribe(this: UdpSocket,) -> Pollable{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[link(wasm_import_module = "wasi-udp")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "subscribe")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-udp_subscribe")]
            fn wit_import(
            _: i32, ) -> i32;
          }
          let ret = wit_import(wit_bindgen::rt::as_i32(this));
          ret as u32
        }
      }
      
    }
    
    
    #[allow(clippy::all)]
    pub mod wasi_exit{
      #[allow(clippy::all)]
      /// Exit the curerent instance and any linked instances.
      pub fn exit(status: Result<(),()>,) -> (){
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          let result0 = match status {
            Ok(_) => { 0i32 },
            Err(_) => { 1i32 },
          };
          #[link(wasm_import_module = "wasi-exit")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "exit")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-exit_exit")]
            fn wit_import(
            _: i32, );
          }
          wit_import(result0);
        }
      }
      
    }
    
    
    #[allow(clippy::all)]
    pub mod wasi_environment{
      #[allow(clippy::all)]
      /// Get the POSIX-style environment variables.
      /// 
      /// Each environment variable is provided as a pair of string variable names
      /// and string value.
      /// 
      /// Morally, these are a value import, but until value imports are available
      /// in the component model, this import function should return the same
      /// values each time it is called.
      pub fn get_environment() -> wit_bindgen::rt::vec::Vec::<(wit_bindgen::rt::string::String,wit_bindgen::rt::string::String,)>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(4))]
          struct RetArea([u8; 8]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "wasi-environment")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "get-environment")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasi-environment_get-environment")]
            fn wit_import(
            _: i32, );
          }
          wit_import(ptr0);
          let base3 = *((ptr0 + 0) as *const i32);
          let len3 = *((ptr0 + 4) as *const i32);
          let mut result3 = Vec::with_capacity(len3 as usize);
          for i in 0..len3 {
            let base = base3 + i *16;
            result3.push({
              let len1 = *((base + 4) as *const i32) as usize;
              let len2 = *((base + 12) as *const i32) as usize;
              
              (String::from_utf8(Vec::from_raw_parts(*((base + 0) as *const i32) as *mut _, len1, len1)).unwrap(), String::from_utf8(Vec::from_raw_parts(*((base + 8) as *const i32) as *mut _, len2, len2)).unwrap())
            });
          }
          wit_bindgen::rt::dealloc(base3, (len3 as usize) * 16, 4);
          result3
        }
      }
      
    }
    
    pub trait WasiCommand {
      fn command(stdin: u32,stdout: u32,args: wit_bindgen::rt::vec::Vec::<wit_bindgen::rt::string::String>,) -> Result<(),()>;
    }
    
    #[doc(hidden)]
    pub unsafe fn call_command<T: WasiCommand>(arg0: i32,arg1: i32,arg2: i32,arg3: i32,) -> i32 {
      
      #[allow(unused_imports)]
      use wit_bindgen::rt::{alloc, vec::Vec, string::String};
      let base1 = arg2;
      let len1 = arg3;
      let mut result1 = Vec::with_capacity(len1 as usize);
      for i in 0..len1 {
        let base = base1 + i *8;
        result1.push({
          let len0 = *((base + 4) as *const i32) as usize;
          
          String::from_utf8(Vec::from_raw_parts(*((base + 0) as *const i32) as *mut _, len0, len0)).unwrap()
        });
      }
      wit_bindgen::rt::dealloc(base1, (len1 as usize) * 8, 4);
      let result2 = T::command(arg0 as u32, arg1 as u32, result1);
      let result3 = match result2 {
        Ok(_) => { 0i32 },
        Err(_) => { 1i32 },
      };result3
    }
    
    /// Declares the export of the component's world for the
    /// given type.
    #[macro_export]
    macro_rules! export_wasi_command(($t:ident) => {
      const _: () = {
        
        #[doc(hidden)]
        #[export_name = "command"]
        #[allow(non_snake_case)]
        unsafe extern "C" fn __export_wasi_command_command(arg0: i32,arg1: i32,arg2: i32,arg3: i32,) -> i32 {
          call_command::<$t>(arg0,arg1,arg2,arg3,)
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
    pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 26957] = [2, 0, 3, 119, 105, 116, 12, 119, 97, 115, 105, 45, 99, 111, 109, 109, 97, 110, 100, 12, 119, 97, 115, 105, 45, 99, 111, 109, 109, 97, 110, 100, 0, 97, 115, 109, 12, 0, 1, 0, 7, 184, 1, 1, 65, 2, 1, 66, 9, 1, 121, 4, 10, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 3, 0, 0, 1, 114, 2, 7, 115, 101, 99, 111, 110, 100, 115, 119, 11, 110, 97, 110, 111, 115, 101, 99, 111, 110, 100, 115, 121, 4, 8, 100, 97, 116, 101, 116, 105, 109, 101, 0, 3, 0, 2, 1, 64, 1, 4, 116, 104, 105, 115, 1, 0, 3, 4, 3, 110, 111, 119, 0, 1, 4, 4, 10, 114, 101, 115, 111, 108, 117, 116, 105, 111, 110, 0, 1, 4, 1, 64, 1, 4, 116, 104, 105, 115, 1, 1, 0, 4, 15, 100, 114, 111, 112, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 1, 5, 4, 15, 119, 97, 115, 105, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 36, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 47, 119, 97, 115, 105, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 5, 0, 11, 41, 1, 15, 119, 97, 115, 105, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 20, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 3, 0, 0, 7, 141, 2, 1, 65, 2, 1, 66, 12, 1, 114, 3, 10, 117, 116, 99, 45, 111, 102, 102, 115, 101, 116, 122, 4, 110, 97, 109, 101, 115, 23, 105, 110, 45, 100, 97, 121, 108, 105, 103, 104, 116, 45, 115, 97, 118, 105, 110, 103, 45, 116, 105, 109, 101, 127, 4, 16, 116, 105, 109, 101, 122, 111, 110, 101, 45, 100, 105, 115, 112, 108, 97, 121, 0, 3, 0, 0, 1, 121, 4, 8, 116, 105, 109, 101, 122, 111, 110, 101, 0, 3, 0, 2, 1, 114, 2, 7, 115, 101, 99, 111, 110, 100, 115, 119, 11, 110, 97, 110, 111, 115, 101, 99, 111, 110, 100, 115, 121, 4, 8, 100, 97, 116, 101, 116, 105, 109, 101, 0, 3, 0, 4, 1, 64, 2, 4, 116, 104, 105, 115, 3, 4, 119, 104, 101, 110, 5, 0, 1, 4, 7, 100, 105, 115, 112, 108, 97, 121, 0, 1, 6, 1, 64, 2, 4, 116, 104, 105, 115, 3, 4, 119, 104, 101, 110, 5, 0, 122, 4, 10, 117, 116, 99, 45, 111, 102, 102, 115, 101, 116, 0, 1, 7, 1, 64, 1, 4, 116, 104, 105, 115, 3, 1, 0, 4, 13, 100, 114, 111, 112, 45, 116, 105, 109, 101, 122, 111, 110, 101, 0, 1, 8, 4, 13, 119, 97, 115, 105, 45, 116, 105, 109, 101, 122, 111, 110, 101, 32, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 116, 105, 109, 101, 122, 111, 110, 101, 47, 119, 97, 115, 105, 45, 116, 105, 109, 101, 122, 111, 110, 101, 5, 0, 11, 37, 1, 13, 119, 97, 115, 105, 45, 116, 105, 109, 101, 122, 111, 110, 101, 18, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 116, 105, 109, 101, 122, 111, 110, 101, 3, 2, 0, 7, 119, 1, 65, 2, 1, 66, 7, 1, 64, 1, 7, 109, 101, 115, 115, 97, 103, 101, 115, 1, 0, 4, 5, 112, 114, 105, 110, 116, 0, 1, 0, 1, 64, 0, 0, 127, 4, 11, 105, 115, 45, 116, 101, 114, 109, 105, 110, 97, 108, 0, 1, 1, 1, 107, 123, 1, 64, 0, 0, 2, 4, 11, 110, 117, 109, 45, 99, 111, 108, 117, 109, 110, 115, 0, 1, 3, 4, 11, 119, 97, 115, 105, 45, 115, 116, 100, 101, 114, 114, 28, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 115, 116, 100, 101, 114, 114, 47, 119, 97, 115, 105, 45, 115, 116, 100, 101, 114, 114, 5, 0, 11, 33, 1, 11, 119, 97, 115, 105, 45, 115, 116, 100, 101, 114, 114, 16, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 115, 116, 100, 101, 114, 114, 3, 4, 0, 7, 108, 1, 65, 2, 1, 66, 5, 1, 112, 125, 1, 64, 1, 3, 108, 101, 110, 121, 0, 0, 4, 16, 103, 101, 116, 45, 114, 97, 110, 100, 111, 109, 45, 98, 121, 116, 101, 115, 0, 1, 1, 1, 64, 0, 0, 119, 4, 14, 103, 101, 116, 45, 114, 97, 110, 100, 111, 109, 45, 117, 54, 52, 0, 1, 2, 4, 11, 119, 97, 115, 105, 45, 114, 97, 110, 100, 111, 109, 28, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 114, 97, 110, 100, 111, 109, 47, 119, 97, 115, 105, 45, 114, 97, 110, 100, 111, 109, 5, 0, 11, 33, 1, 11, 119, 97, 115, 105, 45, 114, 97, 110, 100, 111, 109, 16, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 114, 97, 110, 100, 111, 109, 3, 6, 0, 7, 120, 1, 65, 2, 1, 66, 8, 1, 121, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 1, 64, 1, 4, 116, 104, 105, 115, 1, 1, 0, 4, 13, 100, 114, 111, 112, 45, 112, 111, 108, 108, 97, 98, 108, 101, 0, 1, 2, 1, 112, 1, 1, 112, 125, 1, 64, 1, 2, 105, 110, 3, 0, 4, 4, 11, 112, 111, 108, 108, 45, 111, 110, 101, 111, 102, 102, 0, 1, 5, 4, 9, 119, 97, 115, 105, 45, 112, 111, 108, 108, 24, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 5, 0, 11, 29, 1, 9, 119, 97, 115, 105, 45, 112, 111, 108, 108, 14, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 3, 8, 0, 7, 142, 3, 1, 65, 2, 1, 66, 20, 1, 121, 4, 7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 0, 1, 111, 8, 123, 123, 123, 123, 123, 123, 123, 123, 4, 12, 105, 112, 118, 54, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 2, 1, 114, 4, 4, 112, 111, 114, 116, 123, 9, 102, 108, 111, 119, 45, 105, 110, 102, 111, 121, 7, 97, 100, 100, 114, 101, 115, 115, 3, 8, 115, 99, 111, 112, 101, 45, 105, 100, 121, 4, 19, 105, 112, 118, 54, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 4, 1, 111, 4, 125, 125, 125, 125, 4, 12, 105, 112, 118, 52, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 6, 1, 114, 2, 4, 112, 111, 114, 116, 123, 7, 97, 100, 100, 114, 101, 115, 115, 7, 4, 19, 105, 112, 118, 52, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 8, 1, 113, 2, 4, 105, 112, 118, 52, 1, 9, 0, 4, 105, 112, 118, 54, 1, 5, 0, 4, 17, 105, 112, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 10, 1, 109, 2, 4, 105, 112, 118, 52, 4, 105, 112, 118, 54, 4, 17, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 3, 0, 12, 1, 113, 2, 4, 105, 112, 118, 52, 1, 7, 0, 4, 105, 112, 118, 54, 1, 3, 0, 4, 10, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 14, 1, 109, 2, 7, 117, 110, 107, 110, 111, 119, 110, 5, 97, 103, 97, 105, 110, 4, 5, 101, 114, 114, 111, 114, 0, 3, 0, 16, 1, 64, 1, 4, 116, 104, 105, 115, 1, 1, 0, 4, 12, 100, 114, 111, 112, 45, 110, 101, 116, 119, 111, 114, 107, 0, 1, 18, 4, 12, 119, 97, 115, 105, 45, 110, 101, 116, 119, 111, 114, 107, 30, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 110, 101, 116, 119, 111, 114, 107, 47, 119, 97, 115, 105, 45, 110, 101, 116, 119, 111, 114, 107, 5, 0, 11, 35, 1, 12, 119, 97, 115, 105, 45, 110, 101, 116, 119, 111, 114, 107, 17, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 110, 101, 116, 119, 111, 114, 107, 3, 10, 0, 7, 150, 12, 1, 65, 10, 1, 66, 2, 1, 121, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 3, 9, 119, 97, 115, 105, 45, 112, 111, 108, 108, 24, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 5, 0, 1, 66, 18, 1, 121, 4, 7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 0, 1, 111, 8, 123, 123, 123, 123, 123, 123, 123, 123, 4, 12, 105, 112, 118, 54, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 2, 1, 114, 4, 4, 112, 111, 114, 116, 123, 9, 102, 108, 111, 119, 45, 105, 110, 102, 111, 121, 7, 97, 100, 100, 114, 101, 115, 115, 3, 8, 115, 99, 111, 112, 101, 45, 105, 100, 121, 4, 19, 105, 112, 118, 54, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 4, 1, 111, 4, 125, 125, 125, 125, 4, 12, 105, 112, 118, 52, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 6, 1, 114, 2, 4, 112, 111, 114, 116, 123, 7, 97, 100, 100, 114, 101, 115, 115, 7, 4, 19, 105, 112, 118, 52, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 8, 1, 113, 2, 4, 105, 112, 118, 52, 1, 9, 0, 4, 105, 112, 118, 54, 1, 5, 0, 4, 17, 105, 112, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 10, 1, 109, 2, 4, 105, 112, 118, 52, 4, 105, 112, 118, 54, 4, 17, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 3, 0, 12, 1, 113, 2, 4, 105, 112, 118, 52, 1, 7, 0, 4, 105, 112, 118, 54, 1, 3, 0, 4, 10, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 14, 1, 109, 2, 7, 117, 110, 107, 110, 111, 119, 110, 5, 97, 103, 97, 105, 110, 4, 5, 101, 114, 114, 111, 114, 0, 3, 0, 16, 3, 12, 119, 97, 115, 105, 45, 110, 101, 116, 119, 111, 114, 107, 30, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 110, 101, 116, 119, 111, 114, 107, 47, 119, 97, 115, 105, 45, 110, 101, 116, 119, 111, 114, 107, 5, 1, 2, 3, 0, 0, 8, 112, 111, 108, 108, 97, 98, 108, 101, 2, 3, 0, 1, 7, 110, 101, 116, 119, 111, 114, 107, 2, 3, 0, 1, 5, 101, 114, 114, 111, 114, 2, 3, 0, 1, 17, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 1, 66, 67, 2, 3, 2, 1, 2, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 2, 3, 2, 1, 3, 4, 7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 2, 2, 3, 2, 1, 4, 4, 5, 101, 114, 114, 111, 114, 0, 3, 0, 4, 2, 3, 2, 1, 5, 4, 17, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 3, 0, 6, 1, 121, 4, 10, 117, 100, 112, 45, 115, 111, 99, 107, 101, 116, 0, 3, 0, 8, 1, 111, 8, 123, 123, 123, 123, 123, 123, 123, 123, 4, 12, 105, 112, 118, 54, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 10, 1, 114, 4, 4, 112, 111, 114, 116, 123, 9, 102, 108, 111, 119, 45, 105, 110, 102, 111, 121, 7, 97, 100, 100, 114, 101, 115, 115, 11, 8, 115, 99, 111, 112, 101, 45, 105, 100, 121, 4, 19, 105, 112, 118, 54, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 12, 1, 111, 4, 125, 125, 125, 125, 4, 12, 105, 112, 118, 52, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 14, 1, 114, 2, 4, 112, 111, 114, 116, 123, 7, 97, 100, 100, 114, 101, 115, 115, 15, 4, 19, 105, 112, 118, 52, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 16, 1, 113, 2, 4, 105, 112, 118, 52, 1, 17, 0, 4, 105, 112, 118, 54, 1, 13, 0, 4, 17, 105, 112, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 18, 1, 113, 2, 4, 105, 112, 118, 52, 1, 15, 0, 4, 105, 112, 118, 54, 1, 11, 0, 4, 10, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 20, 1, 112, 125, 1, 114, 2, 4, 100, 97, 116, 97, 22, 14, 114, 101, 109, 111, 116, 101, 45, 97, 100, 100, 114, 101, 115, 115, 19, 4, 8, 100, 97, 116, 97, 103, 114, 97, 109, 0, 3, 0, 23, 1, 106, 1, 9, 1, 5, 1, 64, 2, 7, 110, 101, 116, 119, 111, 114, 107, 3, 14, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 7, 0, 25, 4, 17, 99, 114, 101, 97, 116, 101, 45, 117, 100, 112, 45, 115, 111, 99, 107, 101, 116, 0, 1, 26, 1, 64, 1, 4, 116, 104, 105, 115, 9, 1, 0, 4, 15, 100, 114, 111, 112, 45, 117, 100, 112, 45, 115, 111, 99, 107, 101, 116, 0, 1, 27, 1, 106, 0, 1, 5, 1, 64, 2, 4, 116, 104, 105, 115, 9, 13, 108, 111, 99, 97, 108, 45, 97, 100, 100, 114, 101, 115, 115, 19, 0, 28, 4, 4, 98, 105, 110, 100, 0, 1, 29, 1, 106, 1, 19, 1, 5, 1, 64, 1, 4, 116, 104, 105, 115, 9, 0, 30, 4, 13, 108, 111, 99, 97, 108, 45, 97, 100, 100, 114, 101, 115, 115, 0, 1, 31, 1, 106, 1, 24, 1, 5, 1, 64, 1, 4, 116, 104, 105, 115, 9, 0, 32, 4, 7, 114, 101, 99, 101, 105, 118, 101, 0, 1, 33, 1, 64, 2, 4, 116, 104, 105, 115, 9, 8, 100, 97, 116, 97, 103, 114, 97, 109, 24, 0, 28, 4, 4, 115, 101, 110, 100, 0, 1, 34, 1, 64, 2, 4, 116, 104, 105, 115, 9, 14, 114, 101, 109, 111, 116, 101, 45, 97, 100, 100, 114, 101, 115, 115, 19, 0, 28, 4, 7, 99, 111, 110, 110, 101, 99, 116, 0, 1, 35, 4, 14, 114, 101, 109, 111, 116, 101, 45, 97, 100, 100, 114, 101, 115, 115, 0, 1, 31, 1, 106, 1, 119, 1, 5, 1, 64, 1, 4, 116, 104, 105, 115, 9, 0, 36, 4, 19, 114, 101, 99, 101, 105, 118, 101, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 37, 1, 64, 2, 4, 116, 104, 105, 115, 9, 5, 118, 97, 108, 117, 101, 119, 0, 28, 4, 23, 115, 101, 116, 45, 114, 101, 99, 101, 105, 118, 101, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 38, 4, 16, 115, 101, 110, 100, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 37, 4, 20, 115, 101, 116, 45, 115, 101, 110, 100, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 38, 1, 64, 1, 4, 116, 104, 105, 115, 9, 0, 7, 4, 14, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 1, 39, 1, 106, 1, 125, 1, 5, 1, 64, 1, 4, 116, 104, 105, 115, 9, 0, 40, 4, 17, 117, 110, 105, 99, 97, 115, 116, 45, 104, 111, 112, 45, 108, 105, 109, 105, 116, 0, 1, 41, 1, 64, 2, 4, 116, 104, 105, 115, 9, 5, 118, 97, 108, 117, 101, 125, 0, 28, 4, 21, 115, 101, 116, 45, 117, 110, 105, 99, 97, 115, 116, 45, 104, 111, 112, 45, 108, 105, 109, 105, 116, 0, 1, 42, 1, 106, 1, 127, 1, 5, 1, 64, 1, 4, 116, 104, 105, 115, 9, 0, 43, 4, 9, 105, 112, 118, 54, 45, 111, 110, 108, 121, 0, 1, 44, 1, 64, 2, 4, 116, 104, 105, 115, 9, 5, 118, 97, 108, 117, 101, 127, 0, 28, 4, 13, 115, 101, 116, 45, 105, 112, 118, 54, 45, 111, 110, 108, 121, 0, 1, 45, 4, 12, 110, 111, 110, 45, 98, 108, 111, 99, 107, 105, 110, 103, 0, 1, 44, 4, 16, 115, 101, 116, 45, 110, 111, 110, 45, 98, 108, 111, 99, 107, 105, 110, 103, 0, 1, 45, 1, 64, 1, 4, 116, 104, 105, 115, 9, 0, 1, 4, 9, 115, 117, 98, 115, 99, 114, 105, 98, 101, 0, 1, 46, 4, 8, 119, 97, 115, 105, 45, 117, 100, 112, 22, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 117, 100, 112, 47, 119, 97, 115, 105, 45, 117, 100, 112, 5, 6, 11, 27, 1, 8, 119, 97, 115, 105, 45, 117, 100, 112, 13, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 117, 100, 112, 3, 12, 0, 7, 187, 2, 1, 65, 5, 1, 66, 2, 1, 121, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 3, 9, 119, 97, 115, 105, 45, 112, 111, 108, 108, 24, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 5, 0, 2, 3, 0, 0, 8, 112, 111, 108, 108, 97, 98, 108, 101, 1, 66, 13, 2, 3, 2, 1, 1, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 1, 121, 4, 15, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 3, 0, 2, 1, 119, 4, 7, 105, 110, 115, 116, 97, 110, 116, 0, 3, 0, 4, 1, 64, 1, 4, 116, 104, 105, 115, 3, 0, 5, 4, 3, 110, 111, 119, 0, 1, 6, 4, 10, 114, 101, 115, 111, 108, 117, 116, 105, 111, 110, 0, 1, 6, 1, 64, 3, 4, 116, 104, 105, 115, 3, 4, 119, 104, 101, 110, 5, 8, 97, 98, 115, 111, 108, 117, 116, 101, 127, 0, 1, 4, 9, 115, 117, 98, 115, 99, 114, 105, 98, 101, 0, 1, 7, 1, 64, 1, 4, 116, 104, 105, 115, 3, 1, 0, 4, 20, 100, 114, 111, 112, 45, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 1, 8, 4, 20, 119, 97, 115, 105, 45, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 46, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 47, 119, 97, 115, 105, 45, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 5, 2, 11, 51, 1, 20, 119, 97, 115, 105, 45, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 25, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 3, 14, 0, 7, 133, 1, 1, 65, 2, 1, 66, 4, 1, 109, 5, 5, 116, 114, 97, 99, 101, 5, 100, 101, 98, 117, 103, 4, 105, 110, 102, 111, 4, 119, 97, 114, 110, 5, 101, 114, 114, 111, 114, 4, 5, 108, 101, 118, 101, 108, 0, 3, 0, 0, 1, 64, 3, 5, 108, 101, 118, 101, 108, 1, 7, 99, 111, 110, 116, 101, 120, 116, 115, 7, 109, 101, 115, 115, 97, 103, 101, 115, 1, 0, 4, 3, 108, 111, 103, 0, 1, 2, 4, 12, 119, 97, 115, 105, 45, 108, 111, 103, 103, 105, 110, 103, 30, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 108, 111, 103, 103, 105, 110, 103, 47, 119, 97, 115, 105, 45, 108, 111, 103, 103, 105, 110, 103, 5, 0, 11, 35, 1, 12, 119, 97, 115, 105, 45, 108, 111, 103, 103, 105, 110, 103, 17, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 108, 111, 103, 103, 105, 110, 103, 3, 16, 0, 7, 213, 7, 1, 65, 11, 1, 66, 2, 1, 121, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 3, 9, 119, 97, 115, 105, 45, 112, 111, 108, 108, 24, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 5, 0, 1, 66, 18, 1, 121, 4, 7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 0, 1, 111, 8, 123, 123, 123, 123, 123, 123, 123, 123, 4, 12, 105, 112, 118, 54, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 2, 1, 114, 4, 4, 112, 111, 114, 116, 123, 9, 102, 108, 111, 119, 45, 105, 110, 102, 111, 121, 7, 97, 100, 100, 114, 101, 115, 115, 3, 8, 115, 99, 111, 112, 101, 45, 105, 100, 121, 4, 19, 105, 112, 118, 54, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 4, 1, 111, 4, 125, 125, 125, 125, 4, 12, 105, 112, 118, 52, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 6, 1, 114, 2, 4, 112, 111, 114, 116, 123, 7, 97, 100, 100, 114, 101, 115, 115, 7, 4, 19, 105, 112, 118, 52, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 8, 1, 113, 2, 4, 105, 112, 118, 52, 1, 9, 0, 4, 105, 112, 118, 54, 1, 5, 0, 4, 17, 105, 112, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 10, 1, 109, 2, 4, 105, 112, 118, 52, 4, 105, 112, 118, 54, 4, 17, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 3, 0, 12, 1, 113, 2, 4, 105, 112, 118, 52, 1, 7, 0, 4, 105, 112, 118, 54, 1, 3, 0, 4, 10, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 14, 1, 109, 2, 7, 117, 110, 107, 110, 111, 119, 110, 5, 97, 103, 97, 105, 110, 4, 5, 101, 114, 114, 111, 114, 0, 3, 0, 16, 3, 12, 119, 97, 115, 105, 45, 110, 101, 116, 119, 111, 114, 107, 30, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 110, 101, 116, 119, 111, 114, 107, 47, 119, 97, 115, 105, 45, 110, 101, 116, 119, 111, 114, 107, 5, 1, 2, 3, 0, 0, 8, 112, 111, 108, 108, 97, 98, 108, 101, 2, 3, 0, 1, 7, 110, 101, 116, 119, 111, 114, 107, 2, 3, 0, 1, 5, 101, 114, 114, 111, 114, 2, 3, 0, 1, 10, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 2, 3, 0, 1, 17, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 1, 66, 30, 2, 3, 2, 1, 2, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 2, 3, 2, 1, 3, 4, 7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 2, 2, 3, 2, 1, 4, 4, 5, 101, 114, 114, 111, 114, 0, 3, 0, 4, 2, 3, 2, 1, 5, 4, 10, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 6, 2, 3, 2, 1, 6, 4, 17, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 3, 0, 8, 1, 121, 4, 22, 114, 101, 115, 111, 108, 118, 101, 45, 97, 100, 100, 114, 101, 115, 115, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 10, 1, 107, 9, 1, 106, 1, 11, 1, 5, 1, 64, 4, 7, 110, 101, 116, 119, 111, 114, 107, 3, 4, 110, 97, 109, 101, 115, 14, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 12, 19, 105, 110, 99, 108, 117, 100, 101, 45, 117, 110, 97, 118, 97, 105, 108, 97, 98, 108, 101, 127, 0, 13, 4, 17, 114, 101, 115, 111, 108, 118, 101, 45, 97, 100, 100, 114, 101, 115, 115, 101, 115, 0, 1, 14, 1, 107, 7, 1, 106, 1, 15, 1, 5, 1, 64, 1, 4, 116, 104, 105, 115, 11, 0, 16, 4, 20, 114, 101, 115, 111, 108, 118, 101, 45, 110, 101, 120, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 1, 17, 1, 64, 1, 4, 116, 104, 105, 115, 11, 1, 0, 4, 27, 100, 114, 111, 112, 45, 114, 101, 115, 111, 108, 118, 101, 45, 97, 100, 100, 114, 101, 115, 115, 45, 115, 116, 114, 101, 97, 109, 0, 1, 18, 1, 106, 1, 127, 1, 5, 1, 64, 1, 4, 116, 104, 105, 115, 11, 0, 19, 4, 12, 110, 111, 110, 45, 98, 108, 111, 99, 107, 105, 110, 103, 0, 1, 20, 1, 106, 0, 1, 5, 1, 64, 2, 4, 116, 104, 105, 115, 11, 5, 118, 97, 108, 117, 101, 127, 0, 21, 4, 16, 115, 101, 116, 45, 110, 111, 110, 45, 98, 108, 111, 99, 107, 105, 110, 103, 0, 1, 22, 1, 64, 1, 4, 116, 104, 105, 115, 11, 0, 1, 4, 9, 115, 117, 98, 115, 99, 114, 105, 98, 101, 0, 1, 23, 4, 19, 119, 97, 115, 105, 45, 105, 112, 45, 110, 97, 109, 101, 45, 108, 111, 111, 107, 117, 112, 44, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 105, 112, 45, 110, 97, 109, 101, 45, 108, 111, 111, 107, 117, 112, 47, 119, 97, 115, 105, 45, 105, 112, 45, 110, 97, 109, 101, 45, 108, 111, 111, 107, 117, 112, 5, 7, 11, 49, 1, 19, 119, 97, 115, 105, 45, 105, 112, 45, 110, 97, 109, 101, 45, 108, 111, 111, 107, 117, 112, 24, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 105, 112, 45, 110, 97, 109, 101, 45, 108, 111, 111, 107, 117, 112, 3, 18, 0, 7, 255, 3, 1, 65, 5, 1, 66, 2, 1, 121, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 3, 9, 119, 97, 115, 105, 45, 112, 111, 108, 108, 24, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 5, 0, 2, 3, 0, 0, 8, 112, 111, 108, 108, 97, 98, 108, 101, 1, 66, 34, 2, 3, 2, 1, 1, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 1, 114, 0, 4, 12, 115, 116, 114, 101, 97, 109, 45, 101, 114, 114, 111, 114, 0, 3, 0, 2, 1, 121, 4, 13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 4, 1, 121, 4, 12, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 6, 1, 112, 125, 1, 111, 2, 8, 127, 1, 106, 1, 9, 1, 3, 1, 64, 2, 4, 116, 104, 105, 115, 7, 3, 108, 101, 110, 119, 0, 10, 4, 4, 114, 101, 97, 100, 0, 1, 11, 1, 111, 2, 119, 127, 1, 106, 1, 12, 1, 3, 1, 64, 2, 4, 116, 104, 105, 115, 7, 3, 108, 101, 110, 119, 0, 13, 4, 4, 115, 107, 105, 112, 0, 1, 14, 1, 64, 1, 4, 116, 104, 105, 115, 7, 0, 1, 4, 14, 115, 117, 98, 115, 99, 114, 105, 98, 101, 45, 114, 101, 97, 100, 0, 1, 15, 1, 64, 1, 4, 116, 104, 105, 115, 7, 1, 0, 4, 17, 100, 114, 111, 112, 45, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 1, 16, 1, 106, 1, 119, 1, 3, 1, 64, 2, 4, 116, 104, 105, 115, 5, 3, 98, 117, 102, 8, 0, 17, 4, 5, 119, 114, 105, 116, 101, 0, 1, 18, 1, 64, 2, 4, 116, 104, 105, 115, 5, 3, 108, 101, 110, 119, 0, 17, 4, 12, 119, 114, 105, 116, 101, 45, 122, 101, 114, 111, 101, 115, 0, 1, 19, 1, 64, 3, 4, 116, 104, 105, 115, 5, 3, 115, 114, 99, 7, 3, 108, 101, 110, 119, 0, 13, 4, 6, 115, 112, 108, 105, 99, 101, 0, 1, 20, 1, 64, 2, 4, 116, 104, 105, 115, 5, 3, 115, 114, 99, 7, 0, 17, 4, 7, 102, 111, 114, 119, 97, 114, 100, 0, 1, 21, 1, 64, 1, 4, 116, 104, 105, 115, 5, 0, 1, 4, 9, 115, 117, 98, 115, 99, 114, 105, 98, 101, 0, 1, 22, 1, 64, 1, 4, 116, 104, 105, 115, 5, 1, 0, 4, 18, 100, 114, 111, 112, 45, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 1, 23, 4, 7, 119, 97, 115, 105, 45, 105, 111, 20, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 105, 111, 47, 119, 97, 115, 105, 45, 105, 111, 5, 2, 11, 25, 1, 7, 119, 97, 115, 105, 45, 105, 111, 12, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 105, 111, 3, 20, 0, 7, 223, 14, 1, 65, 14, 1, 66, 2, 1, 121, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 3, 9, 119, 97, 115, 105, 45, 112, 111, 108, 108, 24, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 5, 0, 2, 3, 0, 0, 8, 112, 111, 108, 108, 97, 98, 108, 101, 1, 66, 8, 2, 3, 2, 1, 1, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 1, 114, 0, 4, 12, 115, 116, 114, 101, 97, 109, 45, 101, 114, 114, 111, 114, 0, 3, 0, 2, 1, 121, 4, 13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 4, 1, 121, 4, 12, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 6, 3, 7, 119, 97, 115, 105, 45, 105, 111, 20, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 105, 111, 47, 119, 97, 115, 105, 45, 105, 111, 5, 2, 1, 66, 18, 1, 121, 4, 7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 0, 1, 111, 8, 123, 123, 123, 123, 123, 123, 123, 123, 4, 12, 105, 112, 118, 54, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 2, 1, 114, 4, 4, 112, 111, 114, 116, 123, 9, 102, 108, 111, 119, 45, 105, 110, 102, 111, 121, 7, 97, 100, 100, 114, 101, 115, 115, 3, 8, 115, 99, 111, 112, 101, 45, 105, 100, 121, 4, 19, 105, 112, 118, 54, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 4, 1, 111, 4, 125, 125, 125, 125, 4, 12, 105, 112, 118, 52, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 6, 1, 114, 2, 4, 112, 111, 114, 116, 123, 7, 97, 100, 100, 114, 101, 115, 115, 7, 4, 19, 105, 112, 118, 52, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 8, 1, 113, 2, 4, 105, 112, 118, 52, 1, 9, 0, 4, 105, 112, 118, 54, 1, 5, 0, 4, 17, 105, 112, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 10, 1, 109, 2, 4, 105, 112, 118, 52, 4, 105, 112, 118, 54, 4, 17, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 3, 0, 12, 1, 113, 2, 4, 105, 112, 118, 52, 1, 7, 0, 4, 105, 112, 118, 54, 1, 3, 0, 4, 10, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 14, 1, 109, 2, 7, 117, 110, 107, 110, 111, 119, 110, 5, 97, 103, 97, 105, 110, 4, 5, 101, 114, 114, 111, 114, 0, 3, 0, 16, 3, 12, 119, 97, 115, 105, 45, 110, 101, 116, 119, 111, 114, 107, 30, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 110, 101, 116, 119, 111, 114, 107, 47, 119, 97, 115, 105, 45, 110, 101, 116, 119, 111, 114, 107, 5, 3, 2, 3, 0, 1, 12, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 2, 3, 0, 1, 13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 2, 3, 0, 2, 7, 110, 101, 116, 119, 111, 114, 107, 2, 3, 0, 2, 5, 101, 114, 114, 111, 114, 2, 3, 0, 2, 17, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 1, 66, 80, 2, 3, 2, 1, 4, 4, 12, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 0, 2, 3, 2, 1, 5, 4, 13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 2, 2, 3, 2, 1, 1, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 4, 2, 3, 2, 1, 6, 4, 7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 6, 2, 3, 2, 1, 7, 4, 5, 101, 114, 114, 111, 114, 0, 3, 0, 8, 2, 3, 2, 1, 8, 4, 17, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 3, 0, 10, 1, 121, 4, 10, 116, 99, 112, 45, 115, 111, 99, 107, 101, 116, 0, 3, 0, 12, 1, 109, 3, 7, 114, 101, 99, 101, 105, 118, 101, 4, 115, 101, 110, 100, 4, 98, 111, 116, 104, 4, 13, 115, 104, 117, 116, 100, 111, 119, 110, 45, 116, 121, 112, 101, 0, 3, 0, 14, 1, 111, 8, 123, 123, 123, 123, 123, 123, 123, 123, 4, 12, 105, 112, 118, 54, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 16, 1, 114, 4, 4, 112, 111, 114, 116, 123, 9, 102, 108, 111, 119, 45, 105, 110, 102, 111, 121, 7, 97, 100, 100, 114, 101, 115, 115, 17, 8, 115, 99, 111, 112, 101, 45, 105, 100, 121, 4, 19, 105, 112, 118, 54, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 18, 1, 111, 4, 125, 125, 125, 125, 4, 12, 105, 112, 118, 52, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 20, 1, 114, 2, 4, 112, 111, 114, 116, 123, 7, 97, 100, 100, 114, 101, 115, 115, 21, 4, 19, 105, 112, 118, 52, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 22, 1, 113, 2, 4, 105, 112, 118, 52, 1, 23, 0, 4, 105, 112, 118, 54, 1, 19, 0, 4, 17, 105, 112, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 24, 1, 113, 2, 4, 105, 112, 118, 52, 1, 21, 0, 4, 105, 112, 118, 54, 1, 17, 0, 4, 10, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 26, 1, 106, 1, 13, 1, 9, 1, 64, 2, 7, 110, 101, 116, 119, 111, 114, 107, 7, 14, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 11, 0, 28, 4, 17, 99, 114, 101, 97, 116, 101, 45, 116, 99, 112, 45, 115, 111, 99, 107, 101, 116, 0, 1, 29, 1, 64, 1, 4, 116, 104, 105, 115, 13, 1, 0, 4, 15, 100, 114, 111, 112, 45, 116, 99, 112, 45, 115, 111, 99, 107, 101, 116, 0, 1, 30, 1, 106, 0, 1, 9, 1, 64, 2, 4, 116, 104, 105, 115, 13, 13, 108, 111, 99, 97, 108, 45, 97, 100, 100, 114, 101, 115, 115, 25, 0, 31, 4, 4, 98, 105, 110, 100, 0, 1, 32, 1, 106, 1, 25, 1, 9, 1, 64, 1, 4, 116, 104, 105, 115, 13, 0, 33, 4, 13, 108, 111, 99, 97, 108, 45, 97, 100, 100, 114, 101, 115, 115, 0, 1, 34, 1, 111, 2, 1, 3, 1, 106, 1, 35, 1, 9, 1, 64, 2, 4, 116, 104, 105, 115, 13, 14, 114, 101, 109, 111, 116, 101, 45, 97, 100, 100, 114, 101, 115, 115, 25, 0, 36, 4, 7, 99, 111, 110, 110, 101, 99, 116, 0, 1, 37, 1, 64, 2, 4, 116, 104, 105, 115, 13, 13, 115, 104, 117, 116, 100, 111, 119, 110, 45, 116, 121, 112, 101, 15, 0, 31, 4, 8, 115, 104, 117, 116, 100, 111, 119, 110, 0, 1, 38, 1, 107, 119, 1, 64, 2, 4, 116, 104, 105, 115, 13, 17, 98, 97, 99, 107, 108, 111, 103, 45, 115, 105, 122, 101, 45, 104, 105, 110, 116, 39, 0, 31, 4, 6, 108, 105, 115, 116, 101, 110, 0, 1, 40, 4, 14, 114, 101, 109, 111, 116, 101, 45, 97, 100, 100, 114, 101, 115, 115, 0, 1, 34, 1, 111, 3, 13, 1, 3, 1, 106, 1, 41, 1, 9, 1, 64, 1, 4, 116, 104, 105, 115, 13, 0, 42, 4, 6, 97, 99, 99, 101, 112, 116, 0, 1, 43, 1, 106, 1, 127, 1, 9, 1, 64, 1, 4, 116, 104, 105, 115, 13, 0, 44, 4, 10, 107, 101, 101, 112, 45, 97, 108, 105, 118, 101, 0, 1, 45, 1, 64, 2, 4, 116, 104, 105, 115, 13, 5, 118, 97, 108, 117, 101, 127, 0, 31, 4, 14, 115, 101, 116, 45, 107, 101, 101, 112, 45, 97, 108, 105, 118, 101, 0, 1, 46, 4, 8, 110, 111, 45, 100, 101, 108, 97, 121, 0, 1, 45, 4, 12, 115, 101, 116, 45, 110, 111, 45, 100, 101, 108, 97, 121, 0, 1, 46, 1, 106, 1, 119, 1, 9, 1, 64, 1, 4, 116, 104, 105, 115, 13, 0, 47, 4, 19, 114, 101, 99, 101, 105, 118, 101, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 48, 1, 64, 2, 4, 116, 104, 105, 115, 13, 5, 118, 97, 108, 117, 101, 119, 0, 31, 4, 23, 115, 101, 116, 45, 114, 101, 99, 101, 105, 118, 101, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 49, 4, 16, 115, 101, 110, 100, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 48, 4, 20, 115, 101, 116, 45, 115, 101, 110, 100, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 49, 1, 64, 1, 4, 116, 104, 105, 115, 13, 0, 11, 4, 14, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 1, 50, 1, 106, 1, 125, 1, 9, 1, 64, 1, 4, 116, 104, 105, 115, 13, 0, 51, 4, 17, 117, 110, 105, 99, 97, 115, 116, 45, 104, 111, 112, 45, 108, 105, 109, 105, 116, 0, 1, 52, 1, 64, 2, 4, 116, 104, 105, 115, 13, 5, 118, 97, 108, 117, 101, 125, 0, 31, 4, 21, 115, 101, 116, 45, 117, 110, 105, 99, 97, 115, 116, 45, 104, 111, 112, 45, 108, 105, 109, 105, 116, 0, 1, 53, 4, 9, 105, 112, 118, 54, 45, 111, 110, 108, 121, 0, 1, 45, 4, 13, 115, 101, 116, 45, 105, 112, 118, 54, 45, 111, 110, 108, 121, 0, 1, 46, 4, 12, 110, 111, 110, 45, 98, 108, 111, 99, 107, 105, 110, 103, 0, 1, 45, 4, 16, 115, 101, 116, 45, 110, 111, 110, 45, 98, 108, 111, 99, 107, 105, 110, 103, 0, 1, 46, 1, 64, 1, 4, 116, 104, 105, 115, 13, 0, 5, 4, 9, 115, 117, 98, 115, 99, 114, 105, 98, 101, 0, 1, 54, 4, 8, 119, 97, 115, 105, 45, 116, 99, 112, 22, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 116, 99, 112, 47, 119, 97, 115, 105, 45, 116, 99, 112, 5, 9, 11, 27, 1, 8, 119, 97, 115, 105, 45, 116, 99, 112, 13, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 116, 99, 112, 3, 22, 0, 7, 128, 21, 1, 65, 12, 1, 66, 2, 1, 121, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 3, 9, 119, 97, 115, 105, 45, 112, 111, 108, 108, 24, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 5, 0, 2, 3, 0, 0, 8, 112, 111, 108, 108, 97, 98, 108, 101, 1, 66, 8, 2, 3, 2, 1, 1, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 1, 114, 0, 4, 12, 115, 116, 114, 101, 97, 109, 45, 101, 114, 114, 111, 114, 0, 3, 0, 2, 1, 121, 4, 13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 4, 1, 121, 4, 12, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 6, 3, 7, 119, 97, 115, 105, 45, 105, 111, 20, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 105, 111, 47, 119, 97, 115, 105, 45, 105, 111, 5, 2, 1, 66, 4, 1, 121, 4, 10, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 3, 0, 0, 1, 114, 2, 7, 115, 101, 99, 111, 110, 100, 115, 119, 11, 110, 97, 110, 111, 115, 101, 99, 111, 110, 100, 115, 121, 4, 8, 100, 97, 116, 101, 116, 105, 109, 101, 0, 3, 0, 2, 3, 15, 119, 97, 115, 105, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 36, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 47, 119, 97, 115, 105, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 5, 3, 2, 3, 0, 1, 12, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 2, 3, 0, 1, 13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 2, 3, 0, 2, 8, 100, 97, 116, 101, 116, 105, 109, 101, 1, 66, 119, 2, 3, 2, 1, 4, 4, 12, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 0, 2, 3, 2, 1, 5, 4, 13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 2, 2, 3, 2, 1, 6, 4, 8, 100, 97, 116, 101, 116, 105, 109, 101, 0, 3, 0, 4, 1, 110, 4, 6, 99, 114, 101, 97, 116, 101, 9, 100, 105, 114, 101, 99, 116, 111, 114, 121, 4, 101, 120, 99, 108, 5, 116, 114, 117, 110, 99, 4, 7, 111, 45, 102, 108, 97, 103, 115, 0, 3, 0, 6, 1, 110, 3, 8, 114, 101, 97, 100, 97, 98, 108, 101, 9, 119, 114, 105, 116, 101, 97, 98, 108, 101, 10, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 4, 4, 109, 111, 100, 101, 0, 3, 0, 8, 1, 119, 4, 9, 108, 105, 110, 107, 99, 111, 117, 110, 116, 0, 3, 0, 10, 1, 119, 4, 5, 105, 110, 111, 100, 101, 0, 3, 0, 12, 1, 119, 4, 8, 102, 105, 108, 101, 115, 105, 122, 101, 0, 3, 0, 14, 1, 109, 38, 6, 97, 99, 99, 101, 115, 115, 5, 97, 103, 97, 105, 110, 7, 97, 108, 114, 101, 97, 100, 121, 4, 98, 97, 100, 102, 4, 98, 117, 115, 121, 6, 100, 101, 97, 100, 108, 107, 5, 100, 113, 117, 111, 116, 5, 101, 120, 105, 115, 116, 4, 102, 98, 105, 103, 5, 105, 108, 115, 101, 113, 10, 105, 110, 112, 114, 111, 103, 114, 101, 115, 115, 4, 105, 110, 116, 114, 5, 105, 110, 118, 97, 108, 2, 105, 111, 5, 105, 115, 100, 105, 114, 4, 108, 111, 111, 112, 5, 109, 108, 105, 110, 107, 7, 109, 115, 103, 115, 105, 122, 101, 11, 110, 97, 109, 101, 116, 111, 111, 108, 111, 110, 103, 5, 110, 111, 100, 101, 118, 5, 110, 111, 101, 110, 116, 5, 110, 111, 108, 99, 107, 5, 110, 111, 109, 101, 109, 5, 110, 111, 115, 112, 99, 5, 110, 111, 115, 121, 115, 6, 110, 111, 116, 100, 105, 114, 8, 110, 111, 116, 101, 109, 112, 116, 121, 14, 110, 111, 116, 114, 101, 99, 111, 118, 101, 114, 97, 98, 108, 101, 6, 110, 111, 116, 115, 117, 112, 5, 110, 111, 116, 116, 121, 4, 110, 120, 105, 111, 8, 111, 118, 101, 114, 102, 108, 111, 119, 4, 112, 101, 114, 109, 4, 112, 105, 112, 101, 4, 114, 111, 102, 115, 5, 115, 112, 105, 112, 101, 6, 116, 120, 116, 98, 115, 121, 4, 120, 100, 101, 118, 4, 5, 101, 114, 114, 110, 111, 0, 3, 0, 16, 1, 121, 4, 16, 100, 105, 114, 45, 101, 110, 116, 114, 121, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 18, 1, 119, 4, 6, 100, 101, 118, 105, 99, 101, 0, 3, 0, 20, 1, 109, 8, 7, 117, 110, 107, 110, 111, 119, 110, 12, 98, 108, 111, 99, 107, 45, 100, 101, 118, 105, 99, 101, 16, 99, 104, 97, 114, 97, 99, 116, 101, 114, 45, 100, 101, 118, 105, 99, 101, 9, 100, 105, 114, 101, 99, 116, 111, 114, 121, 4, 102, 105, 102, 111, 13, 115, 121, 109, 98, 111, 108, 105, 99, 45, 108, 105, 110, 107, 12, 114, 101, 103, 117, 108, 97, 114, 45, 102, 105, 108, 101, 6, 115, 111, 99, 107, 101, 116, 4, 15, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 45, 116, 121, 112, 101, 0, 3, 0, 22, 1, 107, 13, 1, 114, 3, 3, 105, 110, 111, 24, 4, 116, 121, 112, 101, 23, 4, 110, 97, 109, 101, 115, 4, 9, 100, 105, 114, 45, 101, 110, 116, 114, 121, 0, 3, 0, 25, 1, 110, 7, 4, 114, 101, 97, 100, 5, 119, 114, 105, 116, 101, 8, 110, 111, 110, 98, 108, 111, 99, 107, 4, 115, 121, 110, 99, 5, 100, 115, 121, 110, 99, 5, 114, 115, 121, 110, 99, 16, 109, 117, 116, 97, 116, 101, 45, 100, 105, 114, 101, 99, 116, 111, 114, 121, 4, 16, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 45, 102, 108, 97, 103, 115, 0, 3, 0, 27, 1, 121, 4, 10, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 0, 3, 0, 29, 1, 113, 3, 9, 110, 111, 45, 99, 104, 97, 110, 103, 101, 0, 0, 3, 110, 111, 119, 0, 0, 9, 116, 105, 109, 101, 115, 116, 97, 109, 112, 1, 5, 0, 4, 13, 110, 101, 119, 45, 116, 105, 109, 101, 115, 116, 97, 109, 112, 0, 3, 0, 31, 1, 114, 8, 3, 100, 101, 118, 21, 3, 105, 110, 111, 13, 4, 116, 121, 112, 101, 23, 5, 110, 108, 105, 110, 107, 11, 4, 115, 105, 122, 101, 15, 4, 97, 116, 105, 109, 5, 4, 109, 116, 105, 109, 5, 4, 99, 116, 105, 109, 5, 4, 15, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 45, 115, 116, 97, 116, 0, 3, 0, 33, 1, 110, 1, 14, 115, 121, 109, 108, 105, 110, 107, 45, 102, 111, 108, 108, 111, 119, 4, 8, 97, 116, 45, 102, 108, 97, 103, 115, 0, 3, 0, 35, 1, 109, 6, 6, 110, 111, 114, 109, 97, 108, 10, 115, 101, 113, 117, 101, 110, 116, 105, 97, 108, 6, 114, 97, 110, 100, 111, 109, 9, 119, 105, 108, 108, 45, 110, 101, 101, 100, 9, 100, 111, 110, 116, 45, 110, 101, 101, 100, 8, 110, 111, 45, 114, 101, 117, 115, 101, 4, 6, 97, 100, 118, 105, 99, 101, 0, 3, 0, 37, 1, 111, 2, 30, 115, 1, 112, 39, 1, 64, 0, 0, 40, 4, 12, 103, 101, 116, 45, 112, 114, 101, 111, 112, 101, 110, 115, 0, 1, 41, 1, 106, 1, 1, 1, 17, 1, 64, 2, 4, 116, 104, 105, 115, 30, 6, 111, 102, 102, 115, 101, 116, 15, 0, 42, 4, 15, 114, 101, 97, 100, 45, 118, 105, 97, 45, 115, 116, 114, 101, 97, 109, 0, 1, 43, 1, 106, 1, 3, 1, 17, 1, 64, 2, 4, 116, 104, 105, 115, 30, 6, 111, 102, 102, 115, 101, 116, 15, 0, 44, 4, 16, 119, 114, 105, 116, 101, 45, 118, 105, 97, 45, 115, 116, 114, 101, 97, 109, 0, 1, 45, 1, 64, 1, 4, 116, 104, 105, 115, 30, 0, 44, 4, 17, 97, 112, 112, 101, 110, 100, 45, 118, 105, 97, 45, 115, 116, 114, 101, 97, 109, 0, 1, 46, 1, 106, 0, 1, 17, 1, 64, 4, 4, 116, 104, 105, 115, 30, 6, 111, 102, 102, 115, 101, 116, 15, 3, 108, 101, 110, 15, 6, 97, 100, 118, 105, 99, 101, 38, 0, 47, 4, 7, 102, 97, 100, 118, 105, 115, 101, 0, 1, 48, 1, 64, 1, 4, 116, 104, 105, 115, 30, 0, 47, 4, 8, 100, 97, 116, 97, 115, 121, 110, 99, 0, 1, 49, 1, 106, 1, 28, 1, 17, 1, 64, 1, 4, 116, 104, 105, 115, 30, 0, 50, 4, 5, 102, 108, 97, 103, 115, 0, 1, 51, 1, 106, 1, 23, 1, 17, 1, 64, 1, 4, 116, 104, 105, 115, 30, 0, 52, 4, 9, 116, 111, 100, 111, 45, 116, 121, 112, 101, 0, 1, 53, 1, 64, 2, 4, 116, 104, 105, 115, 30, 5, 102, 108, 97, 103, 115, 28, 0, 47, 4, 9, 115, 101, 116, 45, 102, 108, 97, 103, 115, 0, 1, 54, 1, 64, 2, 4, 116, 104, 105, 115, 30, 4, 115, 105, 122, 101, 15, 0, 47, 4, 8, 115, 101, 116, 45, 115, 105, 122, 101, 0, 1, 55, 1, 64, 3, 4, 116, 104, 105, 115, 30, 4, 97, 116, 105, 109, 32, 4, 109, 116, 105, 109, 32, 0, 47, 4, 9, 115, 101, 116, 45, 116, 105, 109, 101, 115, 0, 1, 56, 1, 112, 125, 1, 111, 2, 57, 127, 1, 106, 1, 58, 1, 17, 1, 64, 3, 4, 116, 104, 105, 115, 30, 3, 108, 101, 110, 15, 6, 111, 102, 102, 115, 101, 116, 15, 0, 59, 4, 5, 112, 114, 101, 97, 100, 0, 1, 60, 1, 106, 1, 15, 1, 17, 1, 64, 3, 4, 116, 104, 105, 115, 30, 3, 98, 117, 102, 57, 6, 111, 102, 102, 115, 101, 116, 15, 0, 61, 4, 6, 112, 119, 114, 105, 116, 101, 0, 1, 62, 1, 106, 1, 19, 1, 17, 1, 64, 1, 4, 116, 104, 105, 115, 30, 0, 63, 4, 7, 114, 101, 97, 100, 100, 105, 114, 0, 1, 64, 4, 4, 115, 121, 110, 99, 0, 1, 49, 1, 64, 2, 4, 116, 104, 105, 115, 30, 4, 112, 97, 116, 104, 115, 0, 47, 4, 19, 99, 114, 101, 97, 116, 101, 45, 100, 105, 114, 101, 99, 116, 111, 114, 121, 45, 97, 116, 0, 1, 65, 1, 106, 1, 34, 1, 17, 1, 64, 1, 4, 116, 104, 105, 115, 30, 0, 194, 0, 4, 4, 115, 116, 97, 116, 0, 1, 67, 1, 64, 3, 4, 116, 104, 105, 115, 30, 8, 97, 116, 45, 102, 108, 97, 103, 115, 36, 4, 112, 97, 116, 104, 115, 0, 194, 0, 4, 7, 115, 116, 97, 116, 45, 97, 116, 0, 1, 68, 1, 64, 5, 4, 116, 104, 105, 115, 30, 8, 97, 116, 45, 102, 108, 97, 103, 115, 36, 4, 112, 97, 116, 104, 115, 4, 97, 116, 105, 109, 32, 4, 109, 116, 105, 109, 32, 0, 47, 4, 12, 115, 101, 116, 45, 116, 105, 109, 101, 115, 45, 97, 116, 0, 1, 69, 1, 64, 5, 4, 116, 104, 105, 115, 30, 12, 111, 108, 100, 45, 97, 116, 45, 102, 108, 97, 103, 115, 36, 8, 111, 108, 100, 45, 112, 97, 116, 104, 115, 14, 110, 101, 119, 45, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 30, 8, 110, 101, 119, 45, 112, 97, 116, 104, 115, 0, 47, 4, 7, 108, 105, 110, 107, 45, 97, 116, 0, 1, 70, 1, 106, 1, 30, 1, 17, 1, 64, 6, 4, 116, 104, 105, 115, 30, 8, 97, 116, 45, 102, 108, 97, 103, 115, 36, 4, 112, 97, 116, 104, 115, 7, 111, 45, 102, 108, 97, 103, 115, 7, 5, 102, 108, 97, 103, 115, 28, 4, 109, 111, 100, 101, 9, 0, 199, 0, 4, 7, 111, 112, 101, 110, 45, 97, 116, 0, 1, 72, 1, 106, 1, 115, 1, 17, 1, 64, 2, 4, 116, 104, 105, 115, 30, 4, 112, 97, 116, 104, 115, 0, 201, 0, 4, 11, 114, 101, 97, 100, 108, 105, 110, 107, 45, 97, 116, 0, 1, 74, 4, 19, 114, 101, 109, 111, 118, 101, 45, 100, 105, 114, 101, 99, 116, 111, 114, 121, 45, 97, 116, 0, 1, 65, 1, 64, 4, 4, 116, 104, 105, 115, 30, 8, 111, 108, 100, 45, 112, 97, 116, 104, 115, 14, 110, 101, 119, 45, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 30, 8, 110, 101, 119, 45, 112, 97, 116, 104, 115, 0, 47, 4, 9, 114, 101, 110, 97, 109, 101, 45, 97, 116, 0, 1, 75, 1, 64, 3, 4, 116, 104, 105, 115, 30, 8, 111, 108, 100, 45, 112, 97, 116, 104, 115, 8, 110, 101, 119, 45, 112, 97, 116, 104, 115, 0, 47, 4, 10, 115, 121, 109, 108, 105, 110, 107, 45, 97, 116, 0, 1, 76, 4, 14, 117, 110, 108, 105, 110, 107, 45, 102, 105, 108, 101, 45, 97, 116, 0, 1, 65, 1, 64, 4, 4, 116, 104, 105, 115, 30, 8, 97, 116, 45, 102, 108, 97, 103, 115, 36, 4, 112, 97, 116, 104, 115, 4, 109, 111, 100, 101, 9, 0, 47, 4, 26, 99, 104, 97, 110, 103, 101, 45, 102, 105, 108, 101, 45, 112, 101, 114, 109, 105, 115, 115, 105, 111, 110, 115, 45, 97, 116, 0, 1, 77, 4, 31, 99, 104, 97, 110, 103, 101, 45, 100, 105, 114, 101, 99, 116, 111, 114, 121, 45, 112, 101, 114, 109, 105, 115, 115, 105, 111, 110, 115, 45, 97, 116, 0, 1, 77, 4, 11, 108, 111, 99, 107, 45, 115, 104, 97, 114, 101, 100, 0, 1, 49, 4, 14, 108, 111, 99, 107, 45, 101, 120, 99, 108, 117, 115, 105, 118, 101, 0, 1, 49, 4, 15, 116, 114, 121, 45, 108, 111, 99, 107, 45, 115, 104, 97, 114, 101, 100, 0, 1, 49, 4, 18, 116, 114, 121, 45, 108, 111, 99, 107, 45, 101, 120, 99, 108, 117, 115, 105, 118, 101, 0, 1, 49, 4, 6, 117, 110, 108, 111, 99, 107, 0, 1, 49, 1, 64, 1, 4, 116, 104, 105, 115, 30, 1, 0, 4, 15, 100, 114, 111, 112, 45, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 0, 1, 78, 1, 107, 26, 1, 106, 1, 207, 0, 1, 17, 1, 64, 1, 4, 116, 104, 105, 115, 19, 0, 208, 0, 4, 14, 114, 101, 97, 100, 45, 100, 105, 114, 45, 101, 110, 116, 114, 121, 0, 1, 81, 1, 64, 1, 4, 116, 104, 105, 115, 19, 1, 0, 4, 21, 100, 114, 111, 112, 45, 100, 105, 114, 45, 101, 110, 116, 114, 121, 45, 115, 116, 114, 101, 97, 109, 0, 1, 82, 4, 15, 119, 97, 115, 105, 45, 102, 105, 108, 101, 115, 121, 115, 116, 101, 109, 36, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 102, 105, 108, 101, 115, 121, 115, 116, 101, 109, 47, 119, 97, 115, 105, 45, 102, 105, 108, 101, 115, 121, 115, 116, 101, 109, 5, 7, 11, 41, 1, 15, 119, 97, 115, 105, 45, 102, 105, 108, 101, 115, 121, 115, 116, 101, 109, 20, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 102, 105, 108, 101, 115, 121, 115, 116, 101, 109, 3, 24, 0, 7, 70, 1, 65, 2, 1, 66, 3, 1, 106, 0, 0, 1, 64, 1, 6, 115, 116, 97, 116, 117, 115, 0, 1, 0, 4, 4, 101, 120, 105, 116, 0, 1, 1, 4, 9, 119, 97, 115, 105, 45, 101, 120, 105, 116, 24, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 101, 120, 105, 116, 47, 119, 97, 115, 105, 45, 101, 120, 105, 116, 5, 0, 11, 29, 1, 9, 119, 97, 115, 105, 45, 101, 120, 105, 116, 14, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 101, 120, 105, 116, 3, 26, 0, 7, 98, 1, 65, 2, 1, 66, 4, 1, 111, 2, 115, 115, 1, 112, 0, 1, 64, 0, 0, 1, 4, 15, 103, 101, 116, 45, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 0, 1, 2, 4, 16, 119, 97, 115, 105, 45, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 38, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 47, 119, 97, 115, 105, 45, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 5, 0, 11, 43, 1, 16, 119, 97, 115, 105, 45, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 21, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 3, 28, 0, 7, 243, 3, 1, 65, 5, 1, 66, 18, 1, 121, 4, 7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 0, 1, 111, 8, 123, 123, 123, 123, 123, 123, 123, 123, 4, 12, 105, 112, 118, 54, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 2, 1, 114, 4, 4, 112, 111, 114, 116, 123, 9, 102, 108, 111, 119, 45, 105, 110, 102, 111, 121, 7, 97, 100, 100, 114, 101, 115, 115, 3, 8, 115, 99, 111, 112, 101, 45, 105, 100, 121, 4, 19, 105, 112, 118, 54, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 4, 1, 111, 4, 125, 125, 125, 125, 4, 12, 105, 112, 118, 52, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 6, 1, 114, 2, 4, 112, 111, 114, 116, 123, 7, 97, 100, 100, 114, 101, 115, 115, 7, 4, 19, 105, 112, 118, 52, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 8, 1, 113, 2, 4, 105, 112, 118, 52, 1, 9, 0, 4, 105, 112, 118, 54, 1, 5, 0, 4, 17, 105, 112, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 10, 1, 109, 2, 4, 105, 112, 118, 52, 4, 105, 112, 118, 54, 4, 17, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 3, 0, 12, 1, 113, 2, 4, 105, 112, 118, 52, 1, 7, 0, 4, 105, 112, 118, 54, 1, 3, 0, 4, 10, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 14, 1, 109, 2, 7, 117, 110, 107, 110, 111, 119, 110, 5, 97, 103, 97, 105, 110, 4, 5, 101, 114, 114, 111, 114, 0, 3, 0, 16, 3, 12, 119, 97, 115, 105, 45, 110, 101, 116, 119, 111, 114, 107, 30, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 110, 101, 116, 119, 111, 114, 107, 47, 119, 97, 115, 105, 45, 110, 101, 116, 119, 111, 114, 107, 5, 0, 2, 3, 0, 0, 7, 110, 101, 116, 119, 111, 114, 107, 1, 66, 4, 2, 3, 2, 1, 1, 4, 7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 0, 1, 64, 0, 0, 1, 4, 15, 100, 101, 102, 97, 117, 108, 116, 45, 110, 101, 116, 119, 111, 114, 107, 0, 1, 2, 4, 20, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 110, 101, 116, 119, 111, 114, 107, 46, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 110, 101, 116, 119, 111, 114, 107, 47, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 110, 101, 116, 119, 111, 114, 107, 5, 2, 11, 51, 1, 20, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 110, 101, 116, 119, 111, 114, 107, 25, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 110, 101, 116, 119, 111, 114, 107, 3, 30, 0, 7, 150, 4, 1, 65, 11, 1, 66, 2, 1, 121, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 3, 9, 119, 97, 115, 105, 45, 112, 111, 108, 108, 24, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 5, 0, 2, 3, 0, 0, 8, 112, 111, 108, 108, 97, 98, 108, 101, 1, 66, 6, 2, 3, 2, 1, 1, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 1, 121, 4, 15, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 3, 0, 2, 1, 119, 4, 7, 105, 110, 115, 116, 97, 110, 116, 0, 3, 0, 4, 3, 20, 119, 97, 115, 105, 45, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 46, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 47, 119, 97, 115, 105, 45, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 5, 2, 1, 66, 4, 1, 121, 4, 10, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 3, 0, 0, 1, 114, 2, 7, 115, 101, 99, 111, 110, 100, 115, 119, 11, 110, 97, 110, 111, 115, 101, 99, 111, 110, 100, 115, 121, 4, 8, 100, 97, 116, 101, 116, 105, 109, 101, 0, 3, 0, 2, 3, 15, 119, 97, 115, 105, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 36, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 47, 119, 97, 115, 105, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 5, 3, 2, 3, 0, 1, 15, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 2, 3, 0, 2, 10, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 1, 66, 8, 2, 3, 2, 1, 4, 4, 15, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 3, 0, 0, 2, 3, 2, 1, 5, 4, 10, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 3, 0, 2, 1, 64, 0, 0, 1, 4, 23, 100, 101, 102, 97, 117, 108, 116, 45, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 1, 4, 1, 64, 0, 0, 3, 4, 18, 100, 101, 102, 97, 117, 108, 116, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 1, 5, 4, 19, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 99, 108, 111, 99, 107, 115, 44, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 99, 108, 111, 99, 107, 115, 47, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 99, 108, 111, 99, 107, 115, 5, 6, 11, 49, 1, 19, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 99, 108, 111, 99, 107, 115, 24, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 99, 108, 111, 99, 107, 115, 3, 32, 0, 7, 164, 61, 1, 65, 2, 1, 65, 48, 1, 66, 8, 1, 121, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 1, 64, 1, 4, 116, 104, 105, 115, 1, 1, 0, 4, 13, 100, 114, 111, 112, 45, 112, 111, 108, 108, 97, 98, 108, 101, 0, 1, 2, 1, 112, 1, 1, 112, 125, 1, 64, 1, 2, 105, 110, 3, 0, 4, 4, 11, 112, 111, 108, 108, 45, 111, 110, 101, 111, 102, 102, 0, 1, 5, 3, 9, 119, 97, 115, 105, 45, 112, 111, 108, 108, 24, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 5, 0, 2, 3, 0, 0, 8, 112, 111, 108, 108, 97, 98, 108, 101, 1, 66, 13, 2, 3, 2, 1, 1, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 1, 121, 4, 15, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 3, 0, 2, 1, 119, 4, 7, 105, 110, 115, 116, 97, 110, 116, 0, 3, 0, 4, 1, 64, 1, 4, 116, 104, 105, 115, 3, 0, 5, 4, 3, 110, 111, 119, 0, 1, 6, 4, 10, 114, 101, 115, 111, 108, 117, 116, 105, 111, 110, 0, 1, 6, 1, 64, 3, 4, 116, 104, 105, 115, 3, 4, 119, 104, 101, 110, 5, 8, 97, 98, 115, 111, 108, 117, 116, 101, 127, 0, 1, 4, 9, 115, 117, 98, 115, 99, 114, 105, 98, 101, 0, 1, 7, 1, 64, 1, 4, 116, 104, 105, 115, 3, 1, 0, 4, 20, 100, 114, 111, 112, 45, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 1, 8, 3, 20, 119, 97, 115, 105, 45, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 46, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 47, 119, 97, 115, 105, 45, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 5, 2, 1, 66, 9, 1, 121, 4, 10, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 3, 0, 0, 1, 114, 2, 7, 115, 101, 99, 111, 110, 100, 115, 119, 11, 110, 97, 110, 111, 115, 101, 99, 111, 110, 100, 115, 121, 4, 8, 100, 97, 116, 101, 116, 105, 109, 101, 0, 3, 0, 2, 1, 64, 1, 4, 116, 104, 105, 115, 1, 0, 3, 4, 3, 110, 111, 119, 0, 1, 4, 4, 10, 114, 101, 115, 111, 108, 117, 116, 105, 111, 110, 0, 1, 4, 1, 64, 1, 4, 116, 104, 105, 115, 1, 1, 0, 4, 15, 100, 114, 111, 112, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 1, 5, 3, 15, 119, 97, 115, 105, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 36, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 47, 119, 97, 115, 105, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 5, 3, 1, 66, 12, 1, 114, 3, 10, 117, 116, 99, 45, 111, 102, 102, 115, 101, 116, 122, 4, 110, 97, 109, 101, 115, 23, 105, 110, 45, 100, 97, 121, 108, 105, 103, 104, 116, 45, 115, 97, 118, 105, 110, 103, 45, 116, 105, 109, 101, 127, 4, 16, 116, 105, 109, 101, 122, 111, 110, 101, 45, 100, 105, 115, 112, 108, 97, 121, 0, 3, 0, 0, 1, 121, 4, 8, 116, 105, 109, 101, 122, 111, 110, 101, 0, 3, 0, 2, 1, 114, 2, 7, 115, 101, 99, 111, 110, 100, 115, 119, 11, 110, 97, 110, 111, 115, 101, 99, 111, 110, 100, 115, 121, 4, 8, 100, 97, 116, 101, 116, 105, 109, 101, 0, 3, 0, 4, 1, 64, 2, 4, 116, 104, 105, 115, 3, 4, 119, 104, 101, 110, 5, 0, 1, 4, 7, 100, 105, 115, 112, 108, 97, 121, 0, 1, 6, 1, 64, 2, 4, 116, 104, 105, 115, 3, 4, 119, 104, 101, 110, 5, 0, 122, 4, 10, 117, 116, 99, 45, 111, 102, 102, 115, 101, 116, 0, 1, 7, 1, 64, 1, 4, 116, 104, 105, 115, 3, 1, 0, 4, 13, 100, 114, 111, 112, 45, 116, 105, 109, 101, 122, 111, 110, 101, 0, 1, 8, 3, 13, 119, 97, 115, 105, 45, 116, 105, 109, 101, 122, 111, 110, 101, 32, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 116, 105, 109, 101, 122, 111, 110, 101, 47, 119, 97, 115, 105, 45, 116, 105, 109, 101, 122, 111, 110, 101, 5, 4, 2, 3, 0, 1, 15, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 2, 3, 0, 2, 10, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 1, 66, 8, 2, 3, 2, 1, 5, 4, 15, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 3, 0, 0, 2, 3, 2, 1, 6, 4, 10, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 3, 0, 2, 1, 64, 0, 0, 1, 4, 23, 100, 101, 102, 97, 117, 108, 116, 45, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 1, 4, 1, 64, 0, 0, 3, 4, 18, 100, 101, 102, 97, 117, 108, 116, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 1, 5, 3, 19, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 99, 108, 111, 99, 107, 115, 44, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 99, 108, 111, 99, 107, 115, 47, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 99, 108, 111, 99, 107, 115, 5, 7, 1, 66, 4, 1, 109, 5, 5, 116, 114, 97, 99, 101, 5, 100, 101, 98, 117, 103, 4, 105, 110, 102, 111, 4, 119, 97, 114, 110, 5, 101, 114, 114, 111, 114, 4, 5, 108, 101, 118, 101, 108, 0, 3, 0, 0, 1, 64, 3, 5, 108, 101, 118, 101, 108, 1, 7, 99, 111, 110, 116, 101, 120, 116, 115, 7, 109, 101, 115, 115, 97, 103, 101, 115, 1, 0, 4, 3, 108, 111, 103, 0, 1, 2, 3, 12, 119, 97, 115, 105, 45, 108, 111, 103, 103, 105, 110, 103, 30, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 108, 111, 103, 103, 105, 110, 103, 47, 119, 97, 115, 105, 45, 108, 111, 103, 103, 105, 110, 103, 5, 8, 1, 66, 7, 1, 64, 1, 7, 109, 101, 115, 115, 97, 103, 101, 115, 1, 0, 4, 5, 112, 114, 105, 110, 116, 0, 1, 0, 1, 64, 0, 0, 127, 4, 11, 105, 115, 45, 116, 101, 114, 109, 105, 110, 97, 108, 0, 1, 1, 1, 107, 123, 1, 64, 0, 0, 2, 4, 11, 110, 117, 109, 45, 99, 111, 108, 117, 109, 110, 115, 0, 1, 3, 3, 11, 119, 97, 115, 105, 45, 115, 116, 100, 101, 114, 114, 28, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 115, 116, 100, 101, 114, 114, 47, 119, 97, 115, 105, 45, 115, 116, 100, 101, 114, 114, 5, 9, 1, 66, 34, 2, 3, 2, 1, 1, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 1, 114, 0, 4, 12, 115, 116, 114, 101, 97, 109, 45, 101, 114, 114, 111, 114, 0, 3, 0, 2, 1, 121, 4, 13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 4, 1, 121, 4, 12, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 6, 1, 112, 125, 1, 111, 2, 8, 127, 1, 106, 1, 9, 1, 3, 1, 64, 2, 4, 116, 104, 105, 115, 7, 3, 108, 101, 110, 119, 0, 10, 4, 4, 114, 101, 97, 100, 0, 1, 11, 1, 111, 2, 119, 127, 1, 106, 1, 12, 1, 3, 1, 64, 2, 4, 116, 104, 105, 115, 7, 3, 108, 101, 110, 119, 0, 13, 4, 4, 115, 107, 105, 112, 0, 1, 14, 1, 64, 1, 4, 116, 104, 105, 115, 7, 0, 1, 4, 14, 115, 117, 98, 115, 99, 114, 105, 98, 101, 45, 114, 101, 97, 100, 0, 1, 15, 1, 64, 1, 4, 116, 104, 105, 115, 7, 1, 0, 4, 17, 100, 114, 111, 112, 45, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 1, 16, 1, 106, 1, 119, 1, 3, 1, 64, 2, 4, 116, 104, 105, 115, 5, 3, 98, 117, 102, 8, 0, 17, 4, 5, 119, 114, 105, 116, 101, 0, 1, 18, 1, 64, 2, 4, 116, 104, 105, 115, 5, 3, 108, 101, 110, 119, 0, 17, 4, 12, 119, 114, 105, 116, 101, 45, 122, 101, 114, 111, 101, 115, 0, 1, 19, 1, 64, 3, 4, 116, 104, 105, 115, 5, 3, 115, 114, 99, 7, 3, 108, 101, 110, 119, 0, 13, 4, 6, 115, 112, 108, 105, 99, 101, 0, 1, 20, 1, 64, 2, 4, 116, 104, 105, 115, 5, 3, 115, 114, 99, 7, 0, 17, 4, 7, 102, 111, 114, 119, 97, 114, 100, 0, 1, 21, 1, 64, 1, 4, 116, 104, 105, 115, 5, 0, 1, 4, 9, 115, 117, 98, 115, 99, 114, 105, 98, 101, 0, 1, 22, 1, 64, 1, 4, 116, 104, 105, 115, 5, 1, 0, 4, 18, 100, 114, 111, 112, 45, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 1, 23, 3, 7, 119, 97, 115, 105, 45, 105, 111, 20, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 105, 111, 47, 119, 97, 115, 105, 45, 105, 111, 5, 10, 2, 3, 0, 7, 12, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 2, 3, 0, 7, 13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 2, 3, 0, 2, 8, 100, 97, 116, 101, 116, 105, 109, 101, 1, 66, 119, 2, 3, 2, 1, 11, 4, 12, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 0, 2, 3, 2, 1, 12, 4, 13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 2, 2, 3, 2, 1, 13, 4, 8, 100, 97, 116, 101, 116, 105, 109, 101, 0, 3, 0, 4, 1, 110, 4, 6, 99, 114, 101, 97, 116, 101, 9, 100, 105, 114, 101, 99, 116, 111, 114, 121, 4, 101, 120, 99, 108, 5, 116, 114, 117, 110, 99, 4, 7, 111, 45, 102, 108, 97, 103, 115, 0, 3, 0, 6, 1, 110, 3, 8, 114, 101, 97, 100, 97, 98, 108, 101, 9, 119, 114, 105, 116, 101, 97, 98, 108, 101, 10, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 4, 4, 109, 111, 100, 101, 0, 3, 0, 8, 1, 119, 4, 9, 108, 105, 110, 107, 99, 111, 117, 110, 116, 0, 3, 0, 10, 1, 119, 4, 5, 105, 110, 111, 100, 101, 0, 3, 0, 12, 1, 119, 4, 8, 102, 105, 108, 101, 115, 105, 122, 101, 0, 3, 0, 14, 1, 109, 38, 6, 97, 99, 99, 101, 115, 115, 5, 97, 103, 97, 105, 110, 7, 97, 108, 114, 101, 97, 100, 121, 4, 98, 97, 100, 102, 4, 98, 117, 115, 121, 6, 100, 101, 97, 100, 108, 107, 5, 100, 113, 117, 111, 116, 5, 101, 120, 105, 115, 116, 4, 102, 98, 105, 103, 5, 105, 108, 115, 101, 113, 10, 105, 110, 112, 114, 111, 103, 114, 101, 115, 115, 4, 105, 110, 116, 114, 5, 105, 110, 118, 97, 108, 2, 105, 111, 5, 105, 115, 100, 105, 114, 4, 108, 111, 111, 112, 5, 109, 108, 105, 110, 107, 7, 109, 115, 103, 115, 105, 122, 101, 11, 110, 97, 109, 101, 116, 111, 111, 108, 111, 110, 103, 5, 110, 111, 100, 101, 118, 5, 110, 111, 101, 110, 116, 5, 110, 111, 108, 99, 107, 5, 110, 111, 109, 101, 109, 5, 110, 111, 115, 112, 99, 5, 110, 111, 115, 121, 115, 6, 110, 111, 116, 100, 105, 114, 8, 110, 111, 116, 101, 109, 112, 116, 121, 14, 110, 111, 116, 114, 101, 99, 111, 118, 101, 114, 97, 98, 108, 101, 6, 110, 111, 116, 115, 117, 112, 5, 110, 111, 116, 116, 121, 4, 110, 120, 105, 111, 8, 111, 118, 101, 114, 102, 108, 111, 119, 4, 112, 101, 114, 109, 4, 112, 105, 112, 101, 4, 114, 111, 102, 115, 5, 115, 112, 105, 112, 101, 6, 116, 120, 116, 98, 115, 121, 4, 120, 100, 101, 118, 4, 5, 101, 114, 114, 110, 111, 0, 3, 0, 16, 1, 121, 4, 16, 100, 105, 114, 45, 101, 110, 116, 114, 121, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 18, 1, 119, 4, 6, 100, 101, 118, 105, 99, 101, 0, 3, 0, 20, 1, 109, 8, 7, 117, 110, 107, 110, 111, 119, 110, 12, 98, 108, 111, 99, 107, 45, 100, 101, 118, 105, 99, 101, 16, 99, 104, 97, 114, 97, 99, 116, 101, 114, 45, 100, 101, 118, 105, 99, 101, 9, 100, 105, 114, 101, 99, 116, 111, 114, 121, 4, 102, 105, 102, 111, 13, 115, 121, 109, 98, 111, 108, 105, 99, 45, 108, 105, 110, 107, 12, 114, 101, 103, 117, 108, 97, 114, 45, 102, 105, 108, 101, 6, 115, 111, 99, 107, 101, 116, 4, 15, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 45, 116, 121, 112, 101, 0, 3, 0, 22, 1, 107, 13, 1, 114, 3, 3, 105, 110, 111, 24, 4, 116, 121, 112, 101, 23, 4, 110, 97, 109, 101, 115, 4, 9, 100, 105, 114, 45, 101, 110, 116, 114, 121, 0, 3, 0, 25, 1, 110, 7, 4, 114, 101, 97, 100, 5, 119, 114, 105, 116, 101, 8, 110, 111, 110, 98, 108, 111, 99, 107, 4, 115, 121, 110, 99, 5, 100, 115, 121, 110, 99, 5, 114, 115, 121, 110, 99, 16, 109, 117, 116, 97, 116, 101, 45, 100, 105, 114, 101, 99, 116, 111, 114, 121, 4, 16, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 45, 102, 108, 97, 103, 115, 0, 3, 0, 27, 1, 121, 4, 10, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 0, 3, 0, 29, 1, 113, 3, 9, 110, 111, 45, 99, 104, 97, 110, 103, 101, 0, 0, 3, 110, 111, 119, 0, 0, 9, 116, 105, 109, 101, 115, 116, 97, 109, 112, 1, 5, 0, 4, 13, 110, 101, 119, 45, 116, 105, 109, 101, 115, 116, 97, 109, 112, 0, 3, 0, 31, 1, 114, 8, 3, 100, 101, 118, 21, 3, 105, 110, 111, 13, 4, 116, 121, 112, 101, 23, 5, 110, 108, 105, 110, 107, 11, 4, 115, 105, 122, 101, 15, 4, 97, 116, 105, 109, 5, 4, 109, 116, 105, 109, 5, 4, 99, 116, 105, 109, 5, 4, 15, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 45, 115, 116, 97, 116, 0, 3, 0, 33, 1, 110, 1, 14, 115, 121, 109, 108, 105, 110, 107, 45, 102, 111, 108, 108, 111, 119, 4, 8, 97, 116, 45, 102, 108, 97, 103, 115, 0, 3, 0, 35, 1, 109, 6, 6, 110, 111, 114, 109, 97, 108, 10, 115, 101, 113, 117, 101, 110, 116, 105, 97, 108, 6, 114, 97, 110, 100, 111, 109, 9, 119, 105, 108, 108, 45, 110, 101, 101, 100, 9, 100, 111, 110, 116, 45, 110, 101, 101, 100, 8, 110, 111, 45, 114, 101, 117, 115, 101, 4, 6, 97, 100, 118, 105, 99, 101, 0, 3, 0, 37, 1, 111, 2, 30, 115, 1, 112, 39, 1, 64, 0, 0, 40, 4, 12, 103, 101, 116, 45, 112, 114, 101, 111, 112, 101, 110, 115, 0, 1, 41, 1, 106, 1, 1, 1, 17, 1, 64, 2, 4, 116, 104, 105, 115, 30, 6, 111, 102, 102, 115, 101, 116, 15, 0, 42, 4, 15, 114, 101, 97, 100, 45, 118, 105, 97, 45, 115, 116, 114, 101, 97, 109, 0, 1, 43, 1, 106, 1, 3, 1, 17, 1, 64, 2, 4, 116, 104, 105, 115, 30, 6, 111, 102, 102, 115, 101, 116, 15, 0, 44, 4, 16, 119, 114, 105, 116, 101, 45, 118, 105, 97, 45, 115, 116, 114, 101, 97, 109, 0, 1, 45, 1, 64, 1, 4, 116, 104, 105, 115, 30, 0, 44, 4, 17, 97, 112, 112, 101, 110, 100, 45, 118, 105, 97, 45, 115, 116, 114, 101, 97, 109, 0, 1, 46, 1, 106, 0, 1, 17, 1, 64, 4, 4, 116, 104, 105, 115, 30, 6, 111, 102, 102, 115, 101, 116, 15, 3, 108, 101, 110, 15, 6, 97, 100, 118, 105, 99, 101, 38, 0, 47, 4, 7, 102, 97, 100, 118, 105, 115, 101, 0, 1, 48, 1, 64, 1, 4, 116, 104, 105, 115, 30, 0, 47, 4, 8, 100, 97, 116, 97, 115, 121, 110, 99, 0, 1, 49, 1, 106, 1, 28, 1, 17, 1, 64, 1, 4, 116, 104, 105, 115, 30, 0, 50, 4, 5, 102, 108, 97, 103, 115, 0, 1, 51, 1, 106, 1, 23, 1, 17, 1, 64, 1, 4, 116, 104, 105, 115, 30, 0, 52, 4, 9, 116, 111, 100, 111, 45, 116, 121, 112, 101, 0, 1, 53, 1, 64, 2, 4, 116, 104, 105, 115, 30, 5, 102, 108, 97, 103, 115, 28, 0, 47, 4, 9, 115, 101, 116, 45, 102, 108, 97, 103, 115, 0, 1, 54, 1, 64, 2, 4, 116, 104, 105, 115, 30, 4, 115, 105, 122, 101, 15, 0, 47, 4, 8, 115, 101, 116, 45, 115, 105, 122, 101, 0, 1, 55, 1, 64, 3, 4, 116, 104, 105, 115, 30, 4, 97, 116, 105, 109, 32, 4, 109, 116, 105, 109, 32, 0, 47, 4, 9, 115, 101, 116, 45, 116, 105, 109, 101, 115, 0, 1, 56, 1, 112, 125, 1, 111, 2, 57, 127, 1, 106, 1, 58, 1, 17, 1, 64, 3, 4, 116, 104, 105, 115, 30, 3, 108, 101, 110, 15, 6, 111, 102, 102, 115, 101, 116, 15, 0, 59, 4, 5, 112, 114, 101, 97, 100, 0, 1, 60, 1, 106, 1, 15, 1, 17, 1, 64, 3, 4, 116, 104, 105, 115, 30, 3, 98, 117, 102, 57, 6, 111, 102, 102, 115, 101, 116, 15, 0, 61, 4, 6, 112, 119, 114, 105, 116, 101, 0, 1, 62, 1, 106, 1, 19, 1, 17, 1, 64, 1, 4, 116, 104, 105, 115, 30, 0, 63, 4, 7, 114, 101, 97, 100, 100, 105, 114, 0, 1, 64, 4, 4, 115, 121, 110, 99, 0, 1, 49, 1, 64, 2, 4, 116, 104, 105, 115, 30, 4, 112, 97, 116, 104, 115, 0, 47, 4, 19, 99, 114, 101, 97, 116, 101, 45, 100, 105, 114, 101, 99, 116, 111, 114, 121, 45, 97, 116, 0, 1, 65, 1, 106, 1, 34, 1, 17, 1, 64, 1, 4, 116, 104, 105, 115, 30, 0, 194, 0, 4, 4, 115, 116, 97, 116, 0, 1, 67, 1, 64, 3, 4, 116, 104, 105, 115, 30, 8, 97, 116, 45, 102, 108, 97, 103, 115, 36, 4, 112, 97, 116, 104, 115, 0, 194, 0, 4, 7, 115, 116, 97, 116, 45, 97, 116, 0, 1, 68, 1, 64, 5, 4, 116, 104, 105, 115, 30, 8, 97, 116, 45, 102, 108, 97, 103, 115, 36, 4, 112, 97, 116, 104, 115, 4, 97, 116, 105, 109, 32, 4, 109, 116, 105, 109, 32, 0, 47, 4, 12, 115, 101, 116, 45, 116, 105, 109, 101, 115, 45, 97, 116, 0, 1, 69, 1, 64, 5, 4, 116, 104, 105, 115, 30, 12, 111, 108, 100, 45, 97, 116, 45, 102, 108, 97, 103, 115, 36, 8, 111, 108, 100, 45, 112, 97, 116, 104, 115, 14, 110, 101, 119, 45, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 30, 8, 110, 101, 119, 45, 112, 97, 116, 104, 115, 0, 47, 4, 7, 108, 105, 110, 107, 45, 97, 116, 0, 1, 70, 1, 106, 1, 30, 1, 17, 1, 64, 6, 4, 116, 104, 105, 115, 30, 8, 97, 116, 45, 102, 108, 97, 103, 115, 36, 4, 112, 97, 116, 104, 115, 7, 111, 45, 102, 108, 97, 103, 115, 7, 5, 102, 108, 97, 103, 115, 28, 4, 109, 111, 100, 101, 9, 0, 199, 0, 4, 7, 111, 112, 101, 110, 45, 97, 116, 0, 1, 72, 1, 106, 1, 115, 1, 17, 1, 64, 2, 4, 116, 104, 105, 115, 30, 4, 112, 97, 116, 104, 115, 0, 201, 0, 4, 11, 114, 101, 97, 100, 108, 105, 110, 107, 45, 97, 116, 0, 1, 74, 4, 19, 114, 101, 109, 111, 118, 101, 45, 100, 105, 114, 101, 99, 116, 111, 114, 121, 45, 97, 116, 0, 1, 65, 1, 64, 4, 4, 116, 104, 105, 115, 30, 8, 111, 108, 100, 45, 112, 97, 116, 104, 115, 14, 110, 101, 119, 45, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 30, 8, 110, 101, 119, 45, 112, 97, 116, 104, 115, 0, 47, 4, 9, 114, 101, 110, 97, 109, 101, 45, 97, 116, 0, 1, 75, 1, 64, 3, 4, 116, 104, 105, 115, 30, 8, 111, 108, 100, 45, 112, 97, 116, 104, 115, 8, 110, 101, 119, 45, 112, 97, 116, 104, 115, 0, 47, 4, 10, 115, 121, 109, 108, 105, 110, 107, 45, 97, 116, 0, 1, 76, 4, 14, 117, 110, 108, 105, 110, 107, 45, 102, 105, 108, 101, 45, 97, 116, 0, 1, 65, 1, 64, 4, 4, 116, 104, 105, 115, 30, 8, 97, 116, 45, 102, 108, 97, 103, 115, 36, 4, 112, 97, 116, 104, 115, 4, 109, 111, 100, 101, 9, 0, 47, 4, 26, 99, 104, 97, 110, 103, 101, 45, 102, 105, 108, 101, 45, 112, 101, 114, 109, 105, 115, 115, 105, 111, 110, 115, 45, 97, 116, 0, 1, 77, 4, 31, 99, 104, 97, 110, 103, 101, 45, 100, 105, 114, 101, 99, 116, 111, 114, 121, 45, 112, 101, 114, 109, 105, 115, 115, 105, 111, 110, 115, 45, 97, 116, 0, 1, 77, 4, 11, 108, 111, 99, 107, 45, 115, 104, 97, 114, 101, 100, 0, 1, 49, 4, 14, 108, 111, 99, 107, 45, 101, 120, 99, 108, 117, 115, 105, 118, 101, 0, 1, 49, 4, 15, 116, 114, 121, 45, 108, 111, 99, 107, 45, 115, 104, 97, 114, 101, 100, 0, 1, 49, 4, 18, 116, 114, 121, 45, 108, 111, 99, 107, 45, 101, 120, 99, 108, 117, 115, 105, 118, 101, 0, 1, 49, 4, 6, 117, 110, 108, 111, 99, 107, 0, 1, 49, 1, 64, 1, 4, 116, 104, 105, 115, 30, 1, 0, 4, 15, 100, 114, 111, 112, 45, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 0, 1, 78, 1, 107, 26, 1, 106, 1, 207, 0, 1, 17, 1, 64, 1, 4, 116, 104, 105, 115, 19, 0, 208, 0, 4, 14, 114, 101, 97, 100, 45, 100, 105, 114, 45, 101, 110, 116, 114, 121, 0, 1, 81, 1, 64, 1, 4, 116, 104, 105, 115, 19, 1, 0, 4, 21, 100, 114, 111, 112, 45, 100, 105, 114, 45, 101, 110, 116, 114, 121, 45, 115, 116, 114, 101, 97, 109, 0, 1, 82, 3, 15, 119, 97, 115, 105, 45, 102, 105, 108, 101, 115, 121, 115, 116, 101, 109, 36, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 102, 105, 108, 101, 115, 121, 115, 116, 101, 109, 47, 119, 97, 115, 105, 45, 102, 105, 108, 101, 115, 121, 115, 116, 101, 109, 5, 14, 1, 66, 5, 1, 112, 125, 1, 64, 1, 3, 108, 101, 110, 121, 0, 0, 4, 16, 103, 101, 116, 45, 114, 97, 110, 100, 111, 109, 45, 98, 121, 116, 101, 115, 0, 1, 1, 1, 64, 0, 0, 119, 4, 14, 103, 101, 116, 45, 114, 97, 110, 100, 111, 109, 45, 117, 54, 52, 0, 1, 2, 3, 11, 119, 97, 115, 105, 45, 114, 97, 110, 100, 111, 109, 28, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 114, 97, 110, 100, 111, 109, 47, 119, 97, 115, 105, 45, 114, 97, 110, 100, 111, 109, 5, 15, 1, 66, 20, 1, 121, 4, 7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 0, 1, 111, 8, 123, 123, 123, 123, 123, 123, 123, 123, 4, 12, 105, 112, 118, 54, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 2, 1, 114, 4, 4, 112, 111, 114, 116, 123, 9, 102, 108, 111, 119, 45, 105, 110, 102, 111, 121, 7, 97, 100, 100, 114, 101, 115, 115, 3, 8, 115, 99, 111, 112, 101, 45, 105, 100, 121, 4, 19, 105, 112, 118, 54, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 4, 1, 111, 4, 125, 125, 125, 125, 4, 12, 105, 112, 118, 52, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 6, 1, 114, 2, 4, 112, 111, 114, 116, 123, 7, 97, 100, 100, 114, 101, 115, 115, 7, 4, 19, 105, 112, 118, 52, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 8, 1, 113, 2, 4, 105, 112, 118, 52, 1, 9, 0, 4, 105, 112, 118, 54, 1, 5, 0, 4, 17, 105, 112, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 10, 1, 109, 2, 4, 105, 112, 118, 52, 4, 105, 112, 118, 54, 4, 17, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 3, 0, 12, 1, 113, 2, 4, 105, 112, 118, 52, 1, 7, 0, 4, 105, 112, 118, 54, 1, 3, 0, 4, 10, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 14, 1, 109, 2, 7, 117, 110, 107, 110, 111, 119, 110, 5, 97, 103, 97, 105, 110, 4, 5, 101, 114, 114, 111, 114, 0, 3, 0, 16, 1, 64, 1, 4, 116, 104, 105, 115, 1, 1, 0, 4, 12, 100, 114, 111, 112, 45, 110, 101, 116, 119, 111, 114, 107, 0, 1, 18, 3, 12, 119, 97, 115, 105, 45, 110, 101, 116, 119, 111, 114, 107, 30, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 110, 101, 116, 119, 111, 114, 107, 47, 119, 97, 115, 105, 45, 110, 101, 116, 119, 111, 114, 107, 5, 16, 2, 3, 0, 10, 7, 110, 101, 116, 119, 111, 114, 107, 1, 66, 4, 2, 3, 2, 1, 17, 4, 7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 0, 1, 64, 0, 0, 1, 4, 15, 100, 101, 102, 97, 117, 108, 116, 45, 110, 101, 116, 119, 111, 114, 107, 0, 1, 2, 3, 20, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 110, 101, 116, 119, 111, 114, 107, 46, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 110, 101, 116, 119, 111, 114, 107, 47, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 110, 101, 116, 119, 111, 114, 107, 5, 18, 2, 3, 0, 10, 5, 101, 114, 114, 111, 114, 2, 3, 0, 10, 10, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 2, 3, 0, 10, 17, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 1, 66, 30, 2, 3, 2, 1, 1, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 2, 3, 2, 1, 17, 4, 7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 2, 2, 3, 2, 1, 19, 4, 5, 101, 114, 114, 111, 114, 0, 3, 0, 4, 2, 3, 2, 1, 20, 4, 10, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 6, 2, 3, 2, 1, 21, 4, 17, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 3, 0, 8, 1, 121, 4, 22, 114, 101, 115, 111, 108, 118, 101, 45, 97, 100, 100, 114, 101, 115, 115, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 10, 1, 107, 9, 1, 106, 1, 11, 1, 5, 1, 64, 4, 7, 110, 101, 116, 119, 111, 114, 107, 3, 4, 110, 97, 109, 101, 115, 14, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 12, 19, 105, 110, 99, 108, 117, 100, 101, 45, 117, 110, 97, 118, 97, 105, 108, 97, 98, 108, 101, 127, 0, 13, 4, 17, 114, 101, 115, 111, 108, 118, 101, 45, 97, 100, 100, 114, 101, 115, 115, 101, 115, 0, 1, 14, 1, 107, 7, 1, 106, 1, 15, 1, 5, 1, 64, 1, 4, 116, 104, 105, 115, 11, 0, 16, 4, 20, 114, 101, 115, 111, 108, 118, 101, 45, 110, 101, 120, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 1, 17, 1, 64, 1, 4, 116, 104, 105, 115, 11, 1, 0, 4, 27, 100, 114, 111, 112, 45, 114, 101, 115, 111, 108, 118, 101, 45, 97, 100, 100, 114, 101, 115, 115, 45, 115, 116, 114, 101, 97, 109, 0, 1, 18, 1, 106, 1, 127, 1, 5, 1, 64, 1, 4, 116, 104, 105, 115, 11, 0, 19, 4, 12, 110, 111, 110, 45, 98, 108, 111, 99, 107, 105, 110, 103, 0, 1, 20, 1, 106, 0, 1, 5, 1, 64, 2, 4, 116, 104, 105, 115, 11, 5, 118, 97, 108, 117, 101, 127, 0, 21, 4, 16, 115, 101, 116, 45, 110, 111, 110, 45, 98, 108, 111, 99, 107, 105, 110, 103, 0, 1, 22, 1, 64, 1, 4, 116, 104, 105, 115, 11, 0, 1, 4, 9, 115, 117, 98, 115, 99, 114, 105, 98, 101, 0, 1, 23, 3, 19, 119, 97, 115, 105, 45, 105, 112, 45, 110, 97, 109, 101, 45, 108, 111, 111, 107, 117, 112, 44, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 105, 112, 45, 110, 97, 109, 101, 45, 108, 111, 111, 107, 117, 112, 47, 119, 97, 115, 105, 45, 105, 112, 45, 110, 97, 109, 101, 45, 108, 111, 111, 107, 117, 112, 5, 22, 1, 66, 80, 2, 3, 2, 1, 11, 4, 12, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 0, 2, 3, 2, 1, 12, 4, 13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 2, 2, 3, 2, 1, 1, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 4, 2, 3, 2, 1, 17, 4, 7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 6, 2, 3, 2, 1, 19, 4, 5, 101, 114, 114, 111, 114, 0, 3, 0, 8, 2, 3, 2, 1, 21, 4, 17, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 3, 0, 10, 1, 121, 4, 10, 116, 99, 112, 45, 115, 111, 99, 107, 101, 116, 0, 3, 0, 12, 1, 109, 3, 7, 114, 101, 99, 101, 105, 118, 101, 4, 115, 101, 110, 100, 4, 98, 111, 116, 104, 4, 13, 115, 104, 117, 116, 100, 111, 119, 110, 45, 116, 121, 112, 101, 0, 3, 0, 14, 1, 111, 8, 123, 123, 123, 123, 123, 123, 123, 123, 4, 12, 105, 112, 118, 54, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 16, 1, 114, 4, 4, 112, 111, 114, 116, 123, 9, 102, 108, 111, 119, 45, 105, 110, 102, 111, 121, 7, 97, 100, 100, 114, 101, 115, 115, 17, 8, 115, 99, 111, 112, 101, 45, 105, 100, 121, 4, 19, 105, 112, 118, 54, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 18, 1, 111, 4, 125, 125, 125, 125, 4, 12, 105, 112, 118, 52, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 20, 1, 114, 2, 4, 112, 111, 114, 116, 123, 7, 97, 100, 100, 114, 101, 115, 115, 21, 4, 19, 105, 112, 118, 52, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 22, 1, 113, 2, 4, 105, 112, 118, 52, 1, 23, 0, 4, 105, 112, 118, 54, 1, 19, 0, 4, 17, 105, 112, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 24, 1, 113, 2, 4, 105, 112, 118, 52, 1, 21, 0, 4, 105, 112, 118, 54, 1, 17, 0, 4, 10, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 26, 1, 106, 1, 13, 1, 9, 1, 64, 2, 7, 110, 101, 116, 119, 111, 114, 107, 7, 14, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 11, 0, 28, 4, 17, 99, 114, 101, 97, 116, 101, 45, 116, 99, 112, 45, 115, 111, 99, 107, 101, 116, 0, 1, 29, 1, 64, 1, 4, 116, 104, 105, 115, 13, 1, 0, 4, 15, 100, 114, 111, 112, 45, 116, 99, 112, 45, 115, 111, 99, 107, 101, 116, 0, 1, 30, 1, 106, 0, 1, 9, 1, 64, 2, 4, 116, 104, 105, 115, 13, 13, 108, 111, 99, 97, 108, 45, 97, 100, 100, 114, 101, 115, 115, 25, 0, 31, 4, 4, 98, 105, 110, 100, 0, 1, 32, 1, 106, 1, 25, 1, 9, 1, 64, 1, 4, 116, 104, 105, 115, 13, 0, 33, 4, 13, 108, 111, 99, 97, 108, 45, 97, 100, 100, 114, 101, 115, 115, 0, 1, 34, 1, 111, 2, 1, 3, 1, 106, 1, 35, 1, 9, 1, 64, 2, 4, 116, 104, 105, 115, 13, 14, 114, 101, 109, 111, 116, 101, 45, 97, 100, 100, 114, 101, 115, 115, 25, 0, 36, 4, 7, 99, 111, 110, 110, 101, 99, 116, 0, 1, 37, 1, 64, 2, 4, 116, 104, 105, 115, 13, 13, 115, 104, 117, 116, 100, 111, 119, 110, 45, 116, 121, 112, 101, 15, 0, 31, 4, 8, 115, 104, 117, 116, 100, 111, 119, 110, 0, 1, 38, 1, 107, 119, 1, 64, 2, 4, 116, 104, 105, 115, 13, 17, 98, 97, 99, 107, 108, 111, 103, 45, 115, 105, 122, 101, 45, 104, 105, 110, 116, 39, 0, 31, 4, 6, 108, 105, 115, 116, 101, 110, 0, 1, 40, 4, 14, 114, 101, 109, 111, 116, 101, 45, 97, 100, 100, 114, 101, 115, 115, 0, 1, 34, 1, 111, 3, 13, 1, 3, 1, 106, 1, 41, 1, 9, 1, 64, 1, 4, 116, 104, 105, 115, 13, 0, 42, 4, 6, 97, 99, 99, 101, 112, 116, 0, 1, 43, 1, 106, 1, 127, 1, 9, 1, 64, 1, 4, 116, 104, 105, 115, 13, 0, 44, 4, 10, 107, 101, 101, 112, 45, 97, 108, 105, 118, 101, 0, 1, 45, 1, 64, 2, 4, 116, 104, 105, 115, 13, 5, 118, 97, 108, 117, 101, 127, 0, 31, 4, 14, 115, 101, 116, 45, 107, 101, 101, 112, 45, 97, 108, 105, 118, 101, 0, 1, 46, 4, 8, 110, 111, 45, 100, 101, 108, 97, 121, 0, 1, 45, 4, 12, 115, 101, 116, 45, 110, 111, 45, 100, 101, 108, 97, 121, 0, 1, 46, 1, 106, 1, 119, 1, 9, 1, 64, 1, 4, 116, 104, 105, 115, 13, 0, 47, 4, 19, 114, 101, 99, 101, 105, 118, 101, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 48, 1, 64, 2, 4, 116, 104, 105, 115, 13, 5, 118, 97, 108, 117, 101, 119, 0, 31, 4, 23, 115, 101, 116, 45, 114, 101, 99, 101, 105, 118, 101, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 49, 4, 16, 115, 101, 110, 100, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 48, 4, 20, 115, 101, 116, 45, 115, 101, 110, 100, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 49, 1, 64, 1, 4, 116, 104, 105, 115, 13, 0, 11, 4, 14, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 1, 50, 1, 106, 1, 125, 1, 9, 1, 64, 1, 4, 116, 104, 105, 115, 13, 0, 51, 4, 17, 117, 110, 105, 99, 97, 115, 116, 45, 104, 111, 112, 45, 108, 105, 109, 105, 116, 0, 1, 52, 1, 64, 2, 4, 116, 104, 105, 115, 13, 5, 118, 97, 108, 117, 101, 125, 0, 31, 4, 21, 115, 101, 116, 45, 117, 110, 105, 99, 97, 115, 116, 45, 104, 111, 112, 45, 108, 105, 109, 105, 116, 0, 1, 53, 4, 9, 105, 112, 118, 54, 45, 111, 110, 108, 121, 0, 1, 45, 4, 13, 115, 101, 116, 45, 105, 112, 118, 54, 45, 111, 110, 108, 121, 0, 1, 46, 4, 12, 110, 111, 110, 45, 98, 108, 111, 99, 107, 105, 110, 103, 0, 1, 45, 4, 16, 115, 101, 116, 45, 110, 111, 110, 45, 98, 108, 111, 99, 107, 105, 110, 103, 0, 1, 46, 1, 64, 1, 4, 116, 104, 105, 115, 13, 0, 5, 4, 9, 115, 117, 98, 115, 99, 114, 105, 98, 101, 0, 1, 54, 3, 8, 119, 97, 115, 105, 45, 116, 99, 112, 22, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 116, 99, 112, 47, 119, 97, 115, 105, 45, 116, 99, 112, 5, 23, 1, 66, 67, 2, 3, 2, 1, 1, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 2, 3, 2, 1, 17, 4, 7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 2, 2, 3, 2, 1, 19, 4, 5, 101, 114, 114, 111, 114, 0, 3, 0, 4, 2, 3, 2, 1, 21, 4, 17, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 3, 0, 6, 1, 121, 4, 10, 117, 100, 112, 45, 115, 111, 99, 107, 101, 116, 0, 3, 0, 8, 1, 111, 8, 123, 123, 123, 123, 123, 123, 123, 123, 4, 12, 105, 112, 118, 54, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 10, 1, 114, 4, 4, 112, 111, 114, 116, 123, 9, 102, 108, 111, 119, 45, 105, 110, 102, 111, 121, 7, 97, 100, 100, 114, 101, 115, 115, 11, 8, 115, 99, 111, 112, 101, 45, 105, 100, 121, 4, 19, 105, 112, 118, 54, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 12, 1, 111, 4, 125, 125, 125, 125, 4, 12, 105, 112, 118, 52, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 14, 1, 114, 2, 4, 112, 111, 114, 116, 123, 7, 97, 100, 100, 114, 101, 115, 115, 15, 4, 19, 105, 112, 118, 52, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 16, 1, 113, 2, 4, 105, 112, 118, 52, 1, 17, 0, 4, 105, 112, 118, 54, 1, 13, 0, 4, 17, 105, 112, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 18, 1, 113, 2, 4, 105, 112, 118, 52, 1, 15, 0, 4, 105, 112, 118, 54, 1, 11, 0, 4, 10, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 20, 1, 112, 125, 1, 114, 2, 4, 100, 97, 116, 97, 22, 14, 114, 101, 109, 111, 116, 101, 45, 97, 100, 100, 114, 101, 115, 115, 19, 4, 8, 100, 97, 116, 97, 103, 114, 97, 109, 0, 3, 0, 23, 1, 106, 1, 9, 1, 5, 1, 64, 2, 7, 110, 101, 116, 119, 111, 114, 107, 3, 14, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 7, 0, 25, 4, 17, 99, 114, 101, 97, 116, 101, 45, 117, 100, 112, 45, 115, 111, 99, 107, 101, 116, 0, 1, 26, 1, 64, 1, 4, 116, 104, 105, 115, 9, 1, 0, 4, 15, 100, 114, 111, 112, 45, 117, 100, 112, 45, 115, 111, 99, 107, 101, 116, 0, 1, 27, 1, 106, 0, 1, 5, 1, 64, 2, 4, 116, 104, 105, 115, 9, 13, 108, 111, 99, 97, 108, 45, 97, 100, 100, 114, 101, 115, 115, 19, 0, 28, 4, 4, 98, 105, 110, 100, 0, 1, 29, 1, 106, 1, 19, 1, 5, 1, 64, 1, 4, 116, 104, 105, 115, 9, 0, 30, 4, 13, 108, 111, 99, 97, 108, 45, 97, 100, 100, 114, 101, 115, 115, 0, 1, 31, 1, 106, 1, 24, 1, 5, 1, 64, 1, 4, 116, 104, 105, 115, 9, 0, 32, 4, 7, 114, 101, 99, 101, 105, 118, 101, 0, 1, 33, 1, 64, 2, 4, 116, 104, 105, 115, 9, 8, 100, 97, 116, 97, 103, 114, 97, 109, 24, 0, 28, 4, 4, 115, 101, 110, 100, 0, 1, 34, 1, 64, 2, 4, 116, 104, 105, 115, 9, 14, 114, 101, 109, 111, 116, 101, 45, 97, 100, 100, 114, 101, 115, 115, 19, 0, 28, 4, 7, 99, 111, 110, 110, 101, 99, 116, 0, 1, 35, 4, 14, 114, 101, 109, 111, 116, 101, 45, 97, 100, 100, 114, 101, 115, 115, 0, 1, 31, 1, 106, 1, 119, 1, 5, 1, 64, 1, 4, 116, 104, 105, 115, 9, 0, 36, 4, 19, 114, 101, 99, 101, 105, 118, 101, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 37, 1, 64, 2, 4, 116, 104, 105, 115, 9, 5, 118, 97, 108, 117, 101, 119, 0, 28, 4, 23, 115, 101, 116, 45, 114, 101, 99, 101, 105, 118, 101, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 38, 4, 16, 115, 101, 110, 100, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 37, 4, 20, 115, 101, 116, 45, 115, 101, 110, 100, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 38, 1, 64, 1, 4, 116, 104, 105, 115, 9, 0, 7, 4, 14, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 1, 39, 1, 106, 1, 125, 1, 5, 1, 64, 1, 4, 116, 104, 105, 115, 9, 0, 40, 4, 17, 117, 110, 105, 99, 97, 115, 116, 45, 104, 111, 112, 45, 108, 105, 109, 105, 116, 0, 1, 41, 1, 64, 2, 4, 116, 104, 105, 115, 9, 5, 118, 97, 108, 117, 101, 125, 0, 28, 4, 21, 115, 101, 116, 45, 117, 110, 105, 99, 97, 115, 116, 45, 104, 111, 112, 45, 108, 105, 109, 105, 116, 0, 1, 42, 1, 106, 1, 127, 1, 5, 1, 64, 1, 4, 116, 104, 105, 115, 9, 0, 43, 4, 9, 105, 112, 118, 54, 45, 111, 110, 108, 121, 0, 1, 44, 1, 64, 2, 4, 116, 104, 105, 115, 9, 5, 118, 97, 108, 117, 101, 127, 0, 28, 4, 13, 115, 101, 116, 45, 105, 112, 118, 54, 45, 111, 110, 108, 121, 0, 1, 45, 4, 12, 110, 111, 110, 45, 98, 108, 111, 99, 107, 105, 110, 103, 0, 1, 44, 4, 16, 115, 101, 116, 45, 110, 111, 110, 45, 98, 108, 111, 99, 107, 105, 110, 103, 0, 1, 45, 1, 64, 1, 4, 116, 104, 105, 115, 9, 0, 1, 4, 9, 115, 117, 98, 115, 99, 114, 105, 98, 101, 0, 1, 46, 3, 8, 119, 97, 115, 105, 45, 117, 100, 112, 22, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 117, 100, 112, 47, 119, 97, 115, 105, 45, 117, 100, 112, 5, 24, 1, 66, 3, 1, 106, 0, 0, 1, 64, 1, 6, 115, 116, 97, 116, 117, 115, 0, 1, 0, 4, 4, 101, 120, 105, 116, 0, 1, 1, 3, 9, 119, 97, 115, 105, 45, 101, 120, 105, 116, 24, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 101, 120, 105, 116, 47, 119, 97, 115, 105, 45, 101, 120, 105, 116, 5, 25, 1, 66, 4, 1, 111, 2, 115, 115, 1, 112, 0, 1, 64, 0, 0, 1, 4, 15, 103, 101, 116, 45, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 0, 1, 2, 3, 16, 119, 97, 115, 105, 45, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 38, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 47, 119, 97, 115, 105, 45, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 5, 26, 1, 112, 115, 1, 106, 0, 0, 1, 64, 3, 5, 115, 116, 100, 105, 110, 121, 6, 115, 116, 100, 111, 117, 116, 121, 4, 97, 114, 103, 115, 27, 0, 28, 4, 7, 99, 111, 109, 109, 97, 110, 100, 0, 1, 29, 4, 12, 119, 97, 115, 105, 45, 99, 111, 109, 109, 97, 110, 100, 30, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 99, 111, 109, 109, 97, 110, 100, 47, 119, 97, 115, 105, 45, 99, 111, 109, 109, 97, 110, 100, 4, 0, 11, 35, 1, 12, 119, 97, 115, 105, 45, 99, 111, 109, 109, 97, 110, 100, 17, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 99, 111, 109, 109, 97, 110, 100, 3, 34, 0, 7, 223, 60, 1, 65, 2, 1, 65, 44, 1, 66, 8, 1, 121, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 1, 64, 1, 4, 116, 104, 105, 115, 1, 1, 0, 4, 13, 100, 114, 111, 112, 45, 112, 111, 108, 108, 97, 98, 108, 101, 0, 1, 2, 1, 112, 1, 1, 112, 125, 1, 64, 1, 2, 105, 110, 3, 0, 4, 4, 11, 112, 111, 108, 108, 45, 111, 110, 101, 111, 102, 102, 0, 1, 5, 3, 9, 119, 97, 115, 105, 45, 112, 111, 108, 108, 24, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 47, 119, 97, 115, 105, 45, 112, 111, 108, 108, 5, 0, 2, 3, 0, 0, 8, 112, 111, 108, 108, 97, 98, 108, 101, 1, 66, 13, 2, 3, 2, 1, 1, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 1, 121, 4, 15, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 3, 0, 2, 1, 119, 4, 7, 105, 110, 115, 116, 97, 110, 116, 0, 3, 0, 4, 1, 64, 1, 4, 116, 104, 105, 115, 3, 0, 5, 4, 3, 110, 111, 119, 0, 1, 6, 4, 10, 114, 101, 115, 111, 108, 117, 116, 105, 111, 110, 0, 1, 6, 1, 64, 3, 4, 116, 104, 105, 115, 3, 4, 119, 104, 101, 110, 5, 8, 97, 98, 115, 111, 108, 117, 116, 101, 127, 0, 1, 4, 9, 115, 117, 98, 115, 99, 114, 105, 98, 101, 0, 1, 7, 1, 64, 1, 4, 116, 104, 105, 115, 3, 1, 0, 4, 20, 100, 114, 111, 112, 45, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 1, 8, 3, 20, 119, 97, 115, 105, 45, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 46, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 47, 119, 97, 115, 105, 45, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 5, 2, 1, 66, 9, 1, 121, 4, 10, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 3, 0, 0, 1, 114, 2, 7, 115, 101, 99, 111, 110, 100, 115, 119, 11, 110, 97, 110, 111, 115, 101, 99, 111, 110, 100, 115, 121, 4, 8, 100, 97, 116, 101, 116, 105, 109, 101, 0, 3, 0, 2, 1, 64, 1, 4, 116, 104, 105, 115, 1, 0, 3, 4, 3, 110, 111, 119, 0, 1, 4, 4, 10, 114, 101, 115, 111, 108, 117, 116, 105, 111, 110, 0, 1, 4, 1, 64, 1, 4, 116, 104, 105, 115, 1, 1, 0, 4, 15, 100, 114, 111, 112, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 1, 5, 3, 15, 119, 97, 115, 105, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 36, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 47, 119, 97, 115, 105, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 5, 3, 1, 66, 12, 1, 114, 3, 10, 117, 116, 99, 45, 111, 102, 102, 115, 101, 116, 122, 4, 110, 97, 109, 101, 115, 23, 105, 110, 45, 100, 97, 121, 108, 105, 103, 104, 116, 45, 115, 97, 118, 105, 110, 103, 45, 116, 105, 109, 101, 127, 4, 16, 116, 105, 109, 101, 122, 111, 110, 101, 45, 100, 105, 115, 112, 108, 97, 121, 0, 3, 0, 0, 1, 121, 4, 8, 116, 105, 109, 101, 122, 111, 110, 101, 0, 3, 0, 2, 1, 114, 2, 7, 115, 101, 99, 111, 110, 100, 115, 119, 11, 110, 97, 110, 111, 115, 101, 99, 111, 110, 100, 115, 121, 4, 8, 100, 97, 116, 101, 116, 105, 109, 101, 0, 3, 0, 4, 1, 64, 2, 4, 116, 104, 105, 115, 3, 4, 119, 104, 101, 110, 5, 0, 1, 4, 7, 100, 105, 115, 112, 108, 97, 121, 0, 1, 6, 1, 64, 2, 4, 116, 104, 105, 115, 3, 4, 119, 104, 101, 110, 5, 0, 122, 4, 10, 117, 116, 99, 45, 111, 102, 102, 115, 101, 116, 0, 1, 7, 1, 64, 1, 4, 116, 104, 105, 115, 3, 1, 0, 4, 13, 100, 114, 111, 112, 45, 116, 105, 109, 101, 122, 111, 110, 101, 0, 1, 8, 3, 13, 119, 97, 115, 105, 45, 116, 105, 109, 101, 122, 111, 110, 101, 32, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 116, 105, 109, 101, 122, 111, 110, 101, 47, 119, 97, 115, 105, 45, 116, 105, 109, 101, 122, 111, 110, 101, 5, 4, 2, 3, 0, 1, 15, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 2, 3, 0, 2, 10, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 1, 66, 8, 2, 3, 2, 1, 5, 4, 15, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 3, 0, 0, 2, 3, 2, 1, 6, 4, 10, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 3, 0, 2, 1, 64, 0, 0, 1, 4, 23, 100, 101, 102, 97, 117, 108, 116, 45, 109, 111, 110, 111, 116, 111, 110, 105, 99, 45, 99, 108, 111, 99, 107, 0, 1, 4, 1, 64, 0, 0, 3, 4, 18, 100, 101, 102, 97, 117, 108, 116, 45, 119, 97, 108, 108, 45, 99, 108, 111, 99, 107, 0, 1, 5, 3, 19, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 99, 108, 111, 99, 107, 115, 44, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 99, 108, 111, 99, 107, 115, 47, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 99, 108, 111, 99, 107, 115, 5, 7, 1, 66, 4, 1, 109, 5, 5, 116, 114, 97, 99, 101, 5, 100, 101, 98, 117, 103, 4, 105, 110, 102, 111, 4, 119, 97, 114, 110, 5, 101, 114, 114, 111, 114, 4, 5, 108, 101, 118, 101, 108, 0, 3, 0, 0, 1, 64, 3, 5, 108, 101, 118, 101, 108, 1, 7, 99, 111, 110, 116, 101, 120, 116, 115, 7, 109, 101, 115, 115, 97, 103, 101, 115, 1, 0, 4, 3, 108, 111, 103, 0, 1, 2, 3, 12, 119, 97, 115, 105, 45, 108, 111, 103, 103, 105, 110, 103, 30, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 108, 111, 103, 103, 105, 110, 103, 47, 119, 97, 115, 105, 45, 108, 111, 103, 103, 105, 110, 103, 5, 8, 1, 66, 7, 1, 64, 1, 7, 109, 101, 115, 115, 97, 103, 101, 115, 1, 0, 4, 5, 112, 114, 105, 110, 116, 0, 1, 0, 1, 64, 0, 0, 127, 4, 11, 105, 115, 45, 116, 101, 114, 109, 105, 110, 97, 108, 0, 1, 1, 1, 107, 123, 1, 64, 0, 0, 2, 4, 11, 110, 117, 109, 45, 99, 111, 108, 117, 109, 110, 115, 0, 1, 3, 3, 11, 119, 97, 115, 105, 45, 115, 116, 100, 101, 114, 114, 28, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 115, 116, 100, 101, 114, 114, 47, 119, 97, 115, 105, 45, 115, 116, 100, 101, 114, 114, 5, 9, 1, 66, 34, 2, 3, 2, 1, 1, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 1, 114, 0, 4, 12, 115, 116, 114, 101, 97, 109, 45, 101, 114, 114, 111, 114, 0, 3, 0, 2, 1, 121, 4, 13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 4, 1, 121, 4, 12, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 6, 1, 112, 125, 1, 111, 2, 8, 127, 1, 106, 1, 9, 1, 3, 1, 64, 2, 4, 116, 104, 105, 115, 7, 3, 108, 101, 110, 119, 0, 10, 4, 4, 114, 101, 97, 100, 0, 1, 11, 1, 111, 2, 119, 127, 1, 106, 1, 12, 1, 3, 1, 64, 2, 4, 116, 104, 105, 115, 7, 3, 108, 101, 110, 119, 0, 13, 4, 4, 115, 107, 105, 112, 0, 1, 14, 1, 64, 1, 4, 116, 104, 105, 115, 7, 0, 1, 4, 14, 115, 117, 98, 115, 99, 114, 105, 98, 101, 45, 114, 101, 97, 100, 0, 1, 15, 1, 64, 1, 4, 116, 104, 105, 115, 7, 1, 0, 4, 17, 100, 114, 111, 112, 45, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 1, 16, 1, 106, 1, 119, 1, 3, 1, 64, 2, 4, 116, 104, 105, 115, 5, 3, 98, 117, 102, 8, 0, 17, 4, 5, 119, 114, 105, 116, 101, 0, 1, 18, 1, 64, 2, 4, 116, 104, 105, 115, 5, 3, 108, 101, 110, 119, 0, 17, 4, 12, 119, 114, 105, 116, 101, 45, 122, 101, 114, 111, 101, 115, 0, 1, 19, 1, 64, 3, 4, 116, 104, 105, 115, 5, 3, 115, 114, 99, 7, 3, 108, 101, 110, 119, 0, 13, 4, 6, 115, 112, 108, 105, 99, 101, 0, 1, 20, 1, 64, 2, 4, 116, 104, 105, 115, 5, 3, 115, 114, 99, 7, 0, 17, 4, 7, 102, 111, 114, 119, 97, 114, 100, 0, 1, 21, 1, 64, 1, 4, 116, 104, 105, 115, 5, 0, 1, 4, 9, 115, 117, 98, 115, 99, 114, 105, 98, 101, 0, 1, 22, 1, 64, 1, 4, 116, 104, 105, 115, 5, 1, 0, 4, 18, 100, 114, 111, 112, 45, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 1, 23, 3, 7, 119, 97, 115, 105, 45, 105, 111, 20, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 105, 111, 47, 119, 97, 115, 105, 45, 105, 111, 5, 10, 2, 3, 0, 7, 12, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 2, 3, 0, 7, 13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 2, 3, 0, 2, 8, 100, 97, 116, 101, 116, 105, 109, 101, 1, 66, 119, 2, 3, 2, 1, 11, 4, 12, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 0, 2, 3, 2, 1, 12, 4, 13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 2, 2, 3, 2, 1, 13, 4, 8, 100, 97, 116, 101, 116, 105, 109, 101, 0, 3, 0, 4, 1, 110, 4, 6, 99, 114, 101, 97, 116, 101, 9, 100, 105, 114, 101, 99, 116, 111, 114, 121, 4, 101, 120, 99, 108, 5, 116, 114, 117, 110, 99, 4, 7, 111, 45, 102, 108, 97, 103, 115, 0, 3, 0, 6, 1, 110, 3, 8, 114, 101, 97, 100, 97, 98, 108, 101, 9, 119, 114, 105, 116, 101, 97, 98, 108, 101, 10, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 4, 4, 109, 111, 100, 101, 0, 3, 0, 8, 1, 119, 4, 9, 108, 105, 110, 107, 99, 111, 117, 110, 116, 0, 3, 0, 10, 1, 119, 4, 5, 105, 110, 111, 100, 101, 0, 3, 0, 12, 1, 119, 4, 8, 102, 105, 108, 101, 115, 105, 122, 101, 0, 3, 0, 14, 1, 109, 38, 6, 97, 99, 99, 101, 115, 115, 5, 97, 103, 97, 105, 110, 7, 97, 108, 114, 101, 97, 100, 121, 4, 98, 97, 100, 102, 4, 98, 117, 115, 121, 6, 100, 101, 97, 100, 108, 107, 5, 100, 113, 117, 111, 116, 5, 101, 120, 105, 115, 116, 4, 102, 98, 105, 103, 5, 105, 108, 115, 101, 113, 10, 105, 110, 112, 114, 111, 103, 114, 101, 115, 115, 4, 105, 110, 116, 114, 5, 105, 110, 118, 97, 108, 2, 105, 111, 5, 105, 115, 100, 105, 114, 4, 108, 111, 111, 112, 5, 109, 108, 105, 110, 107, 7, 109, 115, 103, 115, 105, 122, 101, 11, 110, 97, 109, 101, 116, 111, 111, 108, 111, 110, 103, 5, 110, 111, 100, 101, 118, 5, 110, 111, 101, 110, 116, 5, 110, 111, 108, 99, 107, 5, 110, 111, 109, 101, 109, 5, 110, 111, 115, 112, 99, 5, 110, 111, 115, 121, 115, 6, 110, 111, 116, 100, 105, 114, 8, 110, 111, 116, 101, 109, 112, 116, 121, 14, 110, 111, 116, 114, 101, 99, 111, 118, 101, 114, 97, 98, 108, 101, 6, 110, 111, 116, 115, 117, 112, 5, 110, 111, 116, 116, 121, 4, 110, 120, 105, 111, 8, 111, 118, 101, 114, 102, 108, 111, 119, 4, 112, 101, 114, 109, 4, 112, 105, 112, 101, 4, 114, 111, 102, 115, 5, 115, 112, 105, 112, 101, 6, 116, 120, 116, 98, 115, 121, 4, 120, 100, 101, 118, 4, 5, 101, 114, 114, 110, 111, 0, 3, 0, 16, 1, 121, 4, 16, 100, 105, 114, 45, 101, 110, 116, 114, 121, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 18, 1, 119, 4, 6, 100, 101, 118, 105, 99, 101, 0, 3, 0, 20, 1, 109, 8, 7, 117, 110, 107, 110, 111, 119, 110, 12, 98, 108, 111, 99, 107, 45, 100, 101, 118, 105, 99, 101, 16, 99, 104, 97, 114, 97, 99, 116, 101, 114, 45, 100, 101, 118, 105, 99, 101, 9, 100, 105, 114, 101, 99, 116, 111, 114, 121, 4, 102, 105, 102, 111, 13, 115, 121, 109, 98, 111, 108, 105, 99, 45, 108, 105, 110, 107, 12, 114, 101, 103, 117, 108, 97, 114, 45, 102, 105, 108, 101, 6, 115, 111, 99, 107, 101, 116, 4, 15, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 45, 116, 121, 112, 101, 0, 3, 0, 22, 1, 107, 13, 1, 114, 3, 3, 105, 110, 111, 24, 4, 116, 121, 112, 101, 23, 4, 110, 97, 109, 101, 115, 4, 9, 100, 105, 114, 45, 101, 110, 116, 114, 121, 0, 3, 0, 25, 1, 110, 7, 4, 114, 101, 97, 100, 5, 119, 114, 105, 116, 101, 8, 110, 111, 110, 98, 108, 111, 99, 107, 4, 115, 121, 110, 99, 5, 100, 115, 121, 110, 99, 5, 114, 115, 121, 110, 99, 16, 109, 117, 116, 97, 116, 101, 45, 100, 105, 114, 101, 99, 116, 111, 114, 121, 4, 16, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 45, 102, 108, 97, 103, 115, 0, 3, 0, 27, 1, 121, 4, 10, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 0, 3, 0, 29, 1, 113, 3, 9, 110, 111, 45, 99, 104, 97, 110, 103, 101, 0, 0, 3, 110, 111, 119, 0, 0, 9, 116, 105, 109, 101, 115, 116, 97, 109, 112, 1, 5, 0, 4, 13, 110, 101, 119, 45, 116, 105, 109, 101, 115, 116, 97, 109, 112, 0, 3, 0, 31, 1, 114, 8, 3, 100, 101, 118, 21, 3, 105, 110, 111, 13, 4, 116, 121, 112, 101, 23, 5, 110, 108, 105, 110, 107, 11, 4, 115, 105, 122, 101, 15, 4, 97, 116, 105, 109, 5, 4, 109, 116, 105, 109, 5, 4, 99, 116, 105, 109, 5, 4, 15, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 45, 115, 116, 97, 116, 0, 3, 0, 33, 1, 110, 1, 14, 115, 121, 109, 108, 105, 110, 107, 45, 102, 111, 108, 108, 111, 119, 4, 8, 97, 116, 45, 102, 108, 97, 103, 115, 0, 3, 0, 35, 1, 109, 6, 6, 110, 111, 114, 109, 97, 108, 10, 115, 101, 113, 117, 101, 110, 116, 105, 97, 108, 6, 114, 97, 110, 100, 111, 109, 9, 119, 105, 108, 108, 45, 110, 101, 101, 100, 9, 100, 111, 110, 116, 45, 110, 101, 101, 100, 8, 110, 111, 45, 114, 101, 117, 115, 101, 4, 6, 97, 100, 118, 105, 99, 101, 0, 3, 0, 37, 1, 111, 2, 30, 115, 1, 112, 39, 1, 64, 0, 0, 40, 4, 12, 103, 101, 116, 45, 112, 114, 101, 111, 112, 101, 110, 115, 0, 1, 41, 1, 106, 1, 1, 1, 17, 1, 64, 2, 4, 116, 104, 105, 115, 30, 6, 111, 102, 102, 115, 101, 116, 15, 0, 42, 4, 15, 114, 101, 97, 100, 45, 118, 105, 97, 45, 115, 116, 114, 101, 97, 109, 0, 1, 43, 1, 106, 1, 3, 1, 17, 1, 64, 2, 4, 116, 104, 105, 115, 30, 6, 111, 102, 102, 115, 101, 116, 15, 0, 44, 4, 16, 119, 114, 105, 116, 101, 45, 118, 105, 97, 45, 115, 116, 114, 101, 97, 109, 0, 1, 45, 1, 64, 1, 4, 116, 104, 105, 115, 30, 0, 44, 4, 17, 97, 112, 112, 101, 110, 100, 45, 118, 105, 97, 45, 115, 116, 114, 101, 97, 109, 0, 1, 46, 1, 106, 0, 1, 17, 1, 64, 4, 4, 116, 104, 105, 115, 30, 6, 111, 102, 102, 115, 101, 116, 15, 3, 108, 101, 110, 15, 6, 97, 100, 118, 105, 99, 101, 38, 0, 47, 4, 7, 102, 97, 100, 118, 105, 115, 101, 0, 1, 48, 1, 64, 1, 4, 116, 104, 105, 115, 30, 0, 47, 4, 8, 100, 97, 116, 97, 115, 121, 110, 99, 0, 1, 49, 1, 106, 1, 28, 1, 17, 1, 64, 1, 4, 116, 104, 105, 115, 30, 0, 50, 4, 5, 102, 108, 97, 103, 115, 0, 1, 51, 1, 106, 1, 23, 1, 17, 1, 64, 1, 4, 116, 104, 105, 115, 30, 0, 52, 4, 9, 116, 111, 100, 111, 45, 116, 121, 112, 101, 0, 1, 53, 1, 64, 2, 4, 116, 104, 105, 115, 30, 5, 102, 108, 97, 103, 115, 28, 0, 47, 4, 9, 115, 101, 116, 45, 102, 108, 97, 103, 115, 0, 1, 54, 1, 64, 2, 4, 116, 104, 105, 115, 30, 4, 115, 105, 122, 101, 15, 0, 47, 4, 8, 115, 101, 116, 45, 115, 105, 122, 101, 0, 1, 55, 1, 64, 3, 4, 116, 104, 105, 115, 30, 4, 97, 116, 105, 109, 32, 4, 109, 116, 105, 109, 32, 0, 47, 4, 9, 115, 101, 116, 45, 116, 105, 109, 101, 115, 0, 1, 56, 1, 112, 125, 1, 111, 2, 57, 127, 1, 106, 1, 58, 1, 17, 1, 64, 3, 4, 116, 104, 105, 115, 30, 3, 108, 101, 110, 15, 6, 111, 102, 102, 115, 101, 116, 15, 0, 59, 4, 5, 112, 114, 101, 97, 100, 0, 1, 60, 1, 106, 1, 15, 1, 17, 1, 64, 3, 4, 116, 104, 105, 115, 30, 3, 98, 117, 102, 57, 6, 111, 102, 102, 115, 101, 116, 15, 0, 61, 4, 6, 112, 119, 114, 105, 116, 101, 0, 1, 62, 1, 106, 1, 19, 1, 17, 1, 64, 1, 4, 116, 104, 105, 115, 30, 0, 63, 4, 7, 114, 101, 97, 100, 100, 105, 114, 0, 1, 64, 4, 4, 115, 121, 110, 99, 0, 1, 49, 1, 64, 2, 4, 116, 104, 105, 115, 30, 4, 112, 97, 116, 104, 115, 0, 47, 4, 19, 99, 114, 101, 97, 116, 101, 45, 100, 105, 114, 101, 99, 116, 111, 114, 121, 45, 97, 116, 0, 1, 65, 1, 106, 1, 34, 1, 17, 1, 64, 1, 4, 116, 104, 105, 115, 30, 0, 194, 0, 4, 4, 115, 116, 97, 116, 0, 1, 67, 1, 64, 3, 4, 116, 104, 105, 115, 30, 8, 97, 116, 45, 102, 108, 97, 103, 115, 36, 4, 112, 97, 116, 104, 115, 0, 194, 0, 4, 7, 115, 116, 97, 116, 45, 97, 116, 0, 1, 68, 1, 64, 5, 4, 116, 104, 105, 115, 30, 8, 97, 116, 45, 102, 108, 97, 103, 115, 36, 4, 112, 97, 116, 104, 115, 4, 97, 116, 105, 109, 32, 4, 109, 116, 105, 109, 32, 0, 47, 4, 12, 115, 101, 116, 45, 116, 105, 109, 101, 115, 45, 97, 116, 0, 1, 69, 1, 64, 5, 4, 116, 104, 105, 115, 30, 12, 111, 108, 100, 45, 97, 116, 45, 102, 108, 97, 103, 115, 36, 8, 111, 108, 100, 45, 112, 97, 116, 104, 115, 14, 110, 101, 119, 45, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 30, 8, 110, 101, 119, 45, 112, 97, 116, 104, 115, 0, 47, 4, 7, 108, 105, 110, 107, 45, 97, 116, 0, 1, 70, 1, 106, 1, 30, 1, 17, 1, 64, 6, 4, 116, 104, 105, 115, 30, 8, 97, 116, 45, 102, 108, 97, 103, 115, 36, 4, 112, 97, 116, 104, 115, 7, 111, 45, 102, 108, 97, 103, 115, 7, 5, 102, 108, 97, 103, 115, 28, 4, 109, 111, 100, 101, 9, 0, 199, 0, 4, 7, 111, 112, 101, 110, 45, 97, 116, 0, 1, 72, 1, 106, 1, 115, 1, 17, 1, 64, 2, 4, 116, 104, 105, 115, 30, 4, 112, 97, 116, 104, 115, 0, 201, 0, 4, 11, 114, 101, 97, 100, 108, 105, 110, 107, 45, 97, 116, 0, 1, 74, 4, 19, 114, 101, 109, 111, 118, 101, 45, 100, 105, 114, 101, 99, 116, 111, 114, 121, 45, 97, 116, 0, 1, 65, 1, 64, 4, 4, 116, 104, 105, 115, 30, 8, 111, 108, 100, 45, 112, 97, 116, 104, 115, 14, 110, 101, 119, 45, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 30, 8, 110, 101, 119, 45, 112, 97, 116, 104, 115, 0, 47, 4, 9, 114, 101, 110, 97, 109, 101, 45, 97, 116, 0, 1, 75, 1, 64, 3, 4, 116, 104, 105, 115, 30, 8, 111, 108, 100, 45, 112, 97, 116, 104, 115, 8, 110, 101, 119, 45, 112, 97, 116, 104, 115, 0, 47, 4, 10, 115, 121, 109, 108, 105, 110, 107, 45, 97, 116, 0, 1, 76, 4, 14, 117, 110, 108, 105, 110, 107, 45, 102, 105, 108, 101, 45, 97, 116, 0, 1, 65, 1, 64, 4, 4, 116, 104, 105, 115, 30, 8, 97, 116, 45, 102, 108, 97, 103, 115, 36, 4, 112, 97, 116, 104, 115, 4, 109, 111, 100, 101, 9, 0, 47, 4, 26, 99, 104, 97, 110, 103, 101, 45, 102, 105, 108, 101, 45, 112, 101, 114, 109, 105, 115, 115, 105, 111, 110, 115, 45, 97, 116, 0, 1, 77, 4, 31, 99, 104, 97, 110, 103, 101, 45, 100, 105, 114, 101, 99, 116, 111, 114, 121, 45, 112, 101, 114, 109, 105, 115, 115, 105, 111, 110, 115, 45, 97, 116, 0, 1, 77, 4, 11, 108, 111, 99, 107, 45, 115, 104, 97, 114, 101, 100, 0, 1, 49, 4, 14, 108, 111, 99, 107, 45, 101, 120, 99, 108, 117, 115, 105, 118, 101, 0, 1, 49, 4, 15, 116, 114, 121, 45, 108, 111, 99, 107, 45, 115, 104, 97, 114, 101, 100, 0, 1, 49, 4, 18, 116, 114, 121, 45, 108, 111, 99, 107, 45, 101, 120, 99, 108, 117, 115, 105, 118, 101, 0, 1, 49, 4, 6, 117, 110, 108, 111, 99, 107, 0, 1, 49, 1, 64, 1, 4, 116, 104, 105, 115, 30, 1, 0, 4, 15, 100, 114, 111, 112, 45, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 0, 1, 78, 1, 107, 26, 1, 106, 1, 207, 0, 1, 17, 1, 64, 1, 4, 116, 104, 105, 115, 19, 0, 208, 0, 4, 14, 114, 101, 97, 100, 45, 100, 105, 114, 45, 101, 110, 116, 114, 121, 0, 1, 81, 1, 64, 1, 4, 116, 104, 105, 115, 19, 1, 0, 4, 21, 100, 114, 111, 112, 45, 100, 105, 114, 45, 101, 110, 116, 114, 121, 45, 115, 116, 114, 101, 97, 109, 0, 1, 82, 3, 15, 119, 97, 115, 105, 45, 102, 105, 108, 101, 115, 121, 115, 116, 101, 109, 36, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 102, 105, 108, 101, 115, 121, 115, 116, 101, 109, 47, 119, 97, 115, 105, 45, 102, 105, 108, 101, 115, 121, 115, 116, 101, 109, 5, 14, 1, 66, 5, 1, 112, 125, 1, 64, 1, 3, 108, 101, 110, 121, 0, 0, 4, 16, 103, 101, 116, 45, 114, 97, 110, 100, 111, 109, 45, 98, 121, 116, 101, 115, 0, 1, 1, 1, 64, 0, 0, 119, 4, 14, 103, 101, 116, 45, 114, 97, 110, 100, 111, 109, 45, 117, 54, 52, 0, 1, 2, 3, 11, 119, 97, 115, 105, 45, 114, 97, 110, 100, 111, 109, 28, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 114, 97, 110, 100, 111, 109, 47, 119, 97, 115, 105, 45, 114, 97, 110, 100, 111, 109, 5, 15, 1, 66, 20, 1, 121, 4, 7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 0, 1, 111, 8, 123, 123, 123, 123, 123, 123, 123, 123, 4, 12, 105, 112, 118, 54, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 2, 1, 114, 4, 4, 112, 111, 114, 116, 123, 9, 102, 108, 111, 119, 45, 105, 110, 102, 111, 121, 7, 97, 100, 100, 114, 101, 115, 115, 3, 8, 115, 99, 111, 112, 101, 45, 105, 100, 121, 4, 19, 105, 112, 118, 54, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 4, 1, 111, 4, 125, 125, 125, 125, 4, 12, 105, 112, 118, 52, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 6, 1, 114, 2, 4, 112, 111, 114, 116, 123, 7, 97, 100, 100, 114, 101, 115, 115, 7, 4, 19, 105, 112, 118, 52, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 8, 1, 113, 2, 4, 105, 112, 118, 52, 1, 9, 0, 4, 105, 112, 118, 54, 1, 5, 0, 4, 17, 105, 112, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 10, 1, 109, 2, 4, 105, 112, 118, 52, 4, 105, 112, 118, 54, 4, 17, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 3, 0, 12, 1, 113, 2, 4, 105, 112, 118, 52, 1, 7, 0, 4, 105, 112, 118, 54, 1, 3, 0, 4, 10, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 14, 1, 109, 2, 7, 117, 110, 107, 110, 111, 119, 110, 5, 97, 103, 97, 105, 110, 4, 5, 101, 114, 114, 111, 114, 0, 3, 0, 16, 1, 64, 1, 4, 116, 104, 105, 115, 1, 1, 0, 4, 12, 100, 114, 111, 112, 45, 110, 101, 116, 119, 111, 114, 107, 0, 1, 18, 3, 12, 119, 97, 115, 105, 45, 110, 101, 116, 119, 111, 114, 107, 30, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 110, 101, 116, 119, 111, 114, 107, 47, 119, 97, 115, 105, 45, 110, 101, 116, 119, 111, 114, 107, 5, 16, 2, 3, 0, 10, 7, 110, 101, 116, 119, 111, 114, 107, 1, 66, 4, 2, 3, 2, 1, 17, 4, 7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 0, 1, 64, 0, 0, 1, 4, 15, 100, 101, 102, 97, 117, 108, 116, 45, 110, 101, 116, 119, 111, 114, 107, 0, 1, 2, 3, 20, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 110, 101, 116, 119, 111, 114, 107, 46, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 110, 101, 116, 119, 111, 114, 107, 47, 119, 97, 115, 105, 45, 100, 101, 102, 97, 117, 108, 116, 45, 110, 101, 116, 119, 111, 114, 107, 5, 18, 2, 3, 0, 10, 5, 101, 114, 114, 111, 114, 2, 3, 0, 10, 10, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 2, 3, 0, 10, 17, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 1, 66, 30, 2, 3, 2, 1, 1, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 2, 3, 2, 1, 17, 4, 7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 2, 2, 3, 2, 1, 19, 4, 5, 101, 114, 114, 111, 114, 0, 3, 0, 4, 2, 3, 2, 1, 20, 4, 10, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 6, 2, 3, 2, 1, 21, 4, 17, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 3, 0, 8, 1, 121, 4, 22, 114, 101, 115, 111, 108, 118, 101, 45, 97, 100, 100, 114, 101, 115, 115, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 10, 1, 107, 9, 1, 106, 1, 11, 1, 5, 1, 64, 4, 7, 110, 101, 116, 119, 111, 114, 107, 3, 4, 110, 97, 109, 101, 115, 14, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 12, 19, 105, 110, 99, 108, 117, 100, 101, 45, 117, 110, 97, 118, 97, 105, 108, 97, 98, 108, 101, 127, 0, 13, 4, 17, 114, 101, 115, 111, 108, 118, 101, 45, 97, 100, 100, 114, 101, 115, 115, 101, 115, 0, 1, 14, 1, 107, 7, 1, 106, 1, 15, 1, 5, 1, 64, 1, 4, 116, 104, 105, 115, 11, 0, 16, 4, 20, 114, 101, 115, 111, 108, 118, 101, 45, 110, 101, 120, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 1, 17, 1, 64, 1, 4, 116, 104, 105, 115, 11, 1, 0, 4, 27, 100, 114, 111, 112, 45, 114, 101, 115, 111, 108, 118, 101, 45, 97, 100, 100, 114, 101, 115, 115, 45, 115, 116, 114, 101, 97, 109, 0, 1, 18, 1, 106, 1, 127, 1, 5, 1, 64, 1, 4, 116, 104, 105, 115, 11, 0, 19, 4, 12, 110, 111, 110, 45, 98, 108, 111, 99, 107, 105, 110, 103, 0, 1, 20, 1, 106, 0, 1, 5, 1, 64, 2, 4, 116, 104, 105, 115, 11, 5, 118, 97, 108, 117, 101, 127, 0, 21, 4, 16, 115, 101, 116, 45, 110, 111, 110, 45, 98, 108, 111, 99, 107, 105, 110, 103, 0, 1, 22, 1, 64, 1, 4, 116, 104, 105, 115, 11, 0, 1, 4, 9, 115, 117, 98, 115, 99, 114, 105, 98, 101, 0, 1, 23, 3, 19, 119, 97, 115, 105, 45, 105, 112, 45, 110, 97, 109, 101, 45, 108, 111, 111, 107, 117, 112, 44, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 105, 112, 45, 110, 97, 109, 101, 45, 108, 111, 111, 107, 117, 112, 47, 119, 97, 115, 105, 45, 105, 112, 45, 110, 97, 109, 101, 45, 108, 111, 111, 107, 117, 112, 5, 22, 1, 66, 80, 2, 3, 2, 1, 11, 4, 12, 105, 110, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 0, 2, 3, 2, 1, 12, 4, 13, 111, 117, 116, 112, 117, 116, 45, 115, 116, 114, 101, 97, 109, 0, 3, 0, 2, 2, 3, 2, 1, 1, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 4, 2, 3, 2, 1, 17, 4, 7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 6, 2, 3, 2, 1, 19, 4, 5, 101, 114, 114, 111, 114, 0, 3, 0, 8, 2, 3, 2, 1, 21, 4, 17, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 3, 0, 10, 1, 121, 4, 10, 116, 99, 112, 45, 115, 111, 99, 107, 101, 116, 0, 3, 0, 12, 1, 109, 3, 7, 114, 101, 99, 101, 105, 118, 101, 4, 115, 101, 110, 100, 4, 98, 111, 116, 104, 4, 13, 115, 104, 117, 116, 100, 111, 119, 110, 45, 116, 121, 112, 101, 0, 3, 0, 14, 1, 111, 8, 123, 123, 123, 123, 123, 123, 123, 123, 4, 12, 105, 112, 118, 54, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 16, 1, 114, 4, 4, 112, 111, 114, 116, 123, 9, 102, 108, 111, 119, 45, 105, 110, 102, 111, 121, 7, 97, 100, 100, 114, 101, 115, 115, 17, 8, 115, 99, 111, 112, 101, 45, 105, 100, 121, 4, 19, 105, 112, 118, 54, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 18, 1, 111, 4, 125, 125, 125, 125, 4, 12, 105, 112, 118, 52, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 20, 1, 114, 2, 4, 112, 111, 114, 116, 123, 7, 97, 100, 100, 114, 101, 115, 115, 21, 4, 19, 105, 112, 118, 52, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 22, 1, 113, 2, 4, 105, 112, 118, 52, 1, 23, 0, 4, 105, 112, 118, 54, 1, 19, 0, 4, 17, 105, 112, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 24, 1, 113, 2, 4, 105, 112, 118, 52, 1, 21, 0, 4, 105, 112, 118, 54, 1, 17, 0, 4, 10, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 26, 1, 106, 1, 13, 1, 9, 1, 64, 2, 7, 110, 101, 116, 119, 111, 114, 107, 7, 14, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 11, 0, 28, 4, 17, 99, 114, 101, 97, 116, 101, 45, 116, 99, 112, 45, 115, 111, 99, 107, 101, 116, 0, 1, 29, 1, 64, 1, 4, 116, 104, 105, 115, 13, 1, 0, 4, 15, 100, 114, 111, 112, 45, 116, 99, 112, 45, 115, 111, 99, 107, 101, 116, 0, 1, 30, 1, 106, 0, 1, 9, 1, 64, 2, 4, 116, 104, 105, 115, 13, 13, 108, 111, 99, 97, 108, 45, 97, 100, 100, 114, 101, 115, 115, 25, 0, 31, 4, 4, 98, 105, 110, 100, 0, 1, 32, 1, 106, 1, 25, 1, 9, 1, 64, 1, 4, 116, 104, 105, 115, 13, 0, 33, 4, 13, 108, 111, 99, 97, 108, 45, 97, 100, 100, 114, 101, 115, 115, 0, 1, 34, 1, 111, 2, 1, 3, 1, 106, 1, 35, 1, 9, 1, 64, 2, 4, 116, 104, 105, 115, 13, 14, 114, 101, 109, 111, 116, 101, 45, 97, 100, 100, 114, 101, 115, 115, 25, 0, 36, 4, 7, 99, 111, 110, 110, 101, 99, 116, 0, 1, 37, 1, 64, 2, 4, 116, 104, 105, 115, 13, 13, 115, 104, 117, 116, 100, 111, 119, 110, 45, 116, 121, 112, 101, 15, 0, 31, 4, 8, 115, 104, 117, 116, 100, 111, 119, 110, 0, 1, 38, 1, 107, 119, 1, 64, 2, 4, 116, 104, 105, 115, 13, 17, 98, 97, 99, 107, 108, 111, 103, 45, 115, 105, 122, 101, 45, 104, 105, 110, 116, 39, 0, 31, 4, 6, 108, 105, 115, 116, 101, 110, 0, 1, 40, 4, 14, 114, 101, 109, 111, 116, 101, 45, 97, 100, 100, 114, 101, 115, 115, 0, 1, 34, 1, 111, 3, 13, 1, 3, 1, 106, 1, 41, 1, 9, 1, 64, 1, 4, 116, 104, 105, 115, 13, 0, 42, 4, 6, 97, 99, 99, 101, 112, 116, 0, 1, 43, 1, 106, 1, 127, 1, 9, 1, 64, 1, 4, 116, 104, 105, 115, 13, 0, 44, 4, 10, 107, 101, 101, 112, 45, 97, 108, 105, 118, 101, 0, 1, 45, 1, 64, 2, 4, 116, 104, 105, 115, 13, 5, 118, 97, 108, 117, 101, 127, 0, 31, 4, 14, 115, 101, 116, 45, 107, 101, 101, 112, 45, 97, 108, 105, 118, 101, 0, 1, 46, 4, 8, 110, 111, 45, 100, 101, 108, 97, 121, 0, 1, 45, 4, 12, 115, 101, 116, 45, 110, 111, 45, 100, 101, 108, 97, 121, 0, 1, 46, 1, 106, 1, 119, 1, 9, 1, 64, 1, 4, 116, 104, 105, 115, 13, 0, 47, 4, 19, 114, 101, 99, 101, 105, 118, 101, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 48, 1, 64, 2, 4, 116, 104, 105, 115, 13, 5, 118, 97, 108, 117, 101, 119, 0, 31, 4, 23, 115, 101, 116, 45, 114, 101, 99, 101, 105, 118, 101, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 49, 4, 16, 115, 101, 110, 100, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 48, 4, 20, 115, 101, 116, 45, 115, 101, 110, 100, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 49, 1, 64, 1, 4, 116, 104, 105, 115, 13, 0, 11, 4, 14, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 1, 50, 1, 106, 1, 125, 1, 9, 1, 64, 1, 4, 116, 104, 105, 115, 13, 0, 51, 4, 17, 117, 110, 105, 99, 97, 115, 116, 45, 104, 111, 112, 45, 108, 105, 109, 105, 116, 0, 1, 52, 1, 64, 2, 4, 116, 104, 105, 115, 13, 5, 118, 97, 108, 117, 101, 125, 0, 31, 4, 21, 115, 101, 116, 45, 117, 110, 105, 99, 97, 115, 116, 45, 104, 111, 112, 45, 108, 105, 109, 105, 116, 0, 1, 53, 4, 9, 105, 112, 118, 54, 45, 111, 110, 108, 121, 0, 1, 45, 4, 13, 115, 101, 116, 45, 105, 112, 118, 54, 45, 111, 110, 108, 121, 0, 1, 46, 4, 12, 110, 111, 110, 45, 98, 108, 111, 99, 107, 105, 110, 103, 0, 1, 45, 4, 16, 115, 101, 116, 45, 110, 111, 110, 45, 98, 108, 111, 99, 107, 105, 110, 103, 0, 1, 46, 1, 64, 1, 4, 116, 104, 105, 115, 13, 0, 5, 4, 9, 115, 117, 98, 115, 99, 114, 105, 98, 101, 0, 1, 54, 3, 8, 119, 97, 115, 105, 45, 116, 99, 112, 22, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 116, 99, 112, 47, 119, 97, 115, 105, 45, 116, 99, 112, 5, 23, 1, 66, 67, 2, 3, 2, 1, 1, 4, 8, 112, 111, 108, 108, 97, 98, 108, 101, 0, 3, 0, 0, 2, 3, 2, 1, 17, 4, 7, 110, 101, 116, 119, 111, 114, 107, 0, 3, 0, 2, 2, 3, 2, 1, 19, 4, 5, 101, 114, 114, 111, 114, 0, 3, 0, 4, 2, 3, 2, 1, 21, 4, 17, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 3, 0, 6, 1, 121, 4, 10, 117, 100, 112, 45, 115, 111, 99, 107, 101, 116, 0, 3, 0, 8, 1, 111, 8, 123, 123, 123, 123, 123, 123, 123, 123, 4, 12, 105, 112, 118, 54, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 10, 1, 114, 4, 4, 112, 111, 114, 116, 123, 9, 102, 108, 111, 119, 45, 105, 110, 102, 111, 121, 7, 97, 100, 100, 114, 101, 115, 115, 11, 8, 115, 99, 111, 112, 101, 45, 105, 100, 121, 4, 19, 105, 112, 118, 54, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 12, 1, 111, 4, 125, 125, 125, 125, 4, 12, 105, 112, 118, 52, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 14, 1, 114, 2, 4, 112, 111, 114, 116, 123, 7, 97, 100, 100, 114, 101, 115, 115, 15, 4, 19, 105, 112, 118, 52, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 16, 1, 113, 2, 4, 105, 112, 118, 52, 1, 17, 0, 4, 105, 112, 118, 54, 1, 13, 0, 4, 17, 105, 112, 45, 115, 111, 99, 107, 101, 116, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 18, 1, 113, 2, 4, 105, 112, 118, 52, 1, 15, 0, 4, 105, 112, 118, 54, 1, 11, 0, 4, 10, 105, 112, 45, 97, 100, 100, 114, 101, 115, 115, 0, 3, 0, 20, 1, 112, 125, 1, 114, 2, 4, 100, 97, 116, 97, 22, 14, 114, 101, 109, 111, 116, 101, 45, 97, 100, 100, 114, 101, 115, 115, 19, 4, 8, 100, 97, 116, 97, 103, 114, 97, 109, 0, 3, 0, 23, 1, 106, 1, 9, 1, 5, 1, 64, 2, 7, 110, 101, 116, 119, 111, 114, 107, 3, 14, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 7, 0, 25, 4, 17, 99, 114, 101, 97, 116, 101, 45, 117, 100, 112, 45, 115, 111, 99, 107, 101, 116, 0, 1, 26, 1, 64, 1, 4, 116, 104, 105, 115, 9, 1, 0, 4, 15, 100, 114, 111, 112, 45, 117, 100, 112, 45, 115, 111, 99, 107, 101, 116, 0, 1, 27, 1, 106, 0, 1, 5, 1, 64, 2, 4, 116, 104, 105, 115, 9, 13, 108, 111, 99, 97, 108, 45, 97, 100, 100, 114, 101, 115, 115, 19, 0, 28, 4, 4, 98, 105, 110, 100, 0, 1, 29, 1, 106, 1, 19, 1, 5, 1, 64, 1, 4, 116, 104, 105, 115, 9, 0, 30, 4, 13, 108, 111, 99, 97, 108, 45, 97, 100, 100, 114, 101, 115, 115, 0, 1, 31, 1, 106, 1, 24, 1, 5, 1, 64, 1, 4, 116, 104, 105, 115, 9, 0, 32, 4, 7, 114, 101, 99, 101, 105, 118, 101, 0, 1, 33, 1, 64, 2, 4, 116, 104, 105, 115, 9, 8, 100, 97, 116, 97, 103, 114, 97, 109, 24, 0, 28, 4, 4, 115, 101, 110, 100, 0, 1, 34, 1, 64, 2, 4, 116, 104, 105, 115, 9, 14, 114, 101, 109, 111, 116, 101, 45, 97, 100, 100, 114, 101, 115, 115, 19, 0, 28, 4, 7, 99, 111, 110, 110, 101, 99, 116, 0, 1, 35, 4, 14, 114, 101, 109, 111, 116, 101, 45, 97, 100, 100, 114, 101, 115, 115, 0, 1, 31, 1, 106, 1, 119, 1, 5, 1, 64, 1, 4, 116, 104, 105, 115, 9, 0, 36, 4, 19, 114, 101, 99, 101, 105, 118, 101, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 37, 1, 64, 2, 4, 116, 104, 105, 115, 9, 5, 118, 97, 108, 117, 101, 119, 0, 28, 4, 23, 115, 101, 116, 45, 114, 101, 99, 101, 105, 118, 101, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 38, 4, 16, 115, 101, 110, 100, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 37, 4, 20, 115, 101, 116, 45, 115, 101, 110, 100, 45, 98, 117, 102, 102, 101, 114, 45, 115, 105, 122, 101, 0, 1, 38, 1, 64, 1, 4, 116, 104, 105, 115, 9, 0, 7, 4, 14, 97, 100, 100, 114, 101, 115, 115, 45, 102, 97, 109, 105, 108, 121, 0, 1, 39, 1, 106, 1, 125, 1, 5, 1, 64, 1, 4, 116, 104, 105, 115, 9, 0, 40, 4, 17, 117, 110, 105, 99, 97, 115, 116, 45, 104, 111, 112, 45, 108, 105, 109, 105, 116, 0, 1, 41, 1, 64, 2, 4, 116, 104, 105, 115, 9, 5, 118, 97, 108, 117, 101, 125, 0, 28, 4, 21, 115, 101, 116, 45, 117, 110, 105, 99, 97, 115, 116, 45, 104, 111, 112, 45, 108, 105, 109, 105, 116, 0, 1, 42, 1, 106, 1, 127, 1, 5, 1, 64, 1, 4, 116, 104, 105, 115, 9, 0, 43, 4, 9, 105, 112, 118, 54, 45, 111, 110, 108, 121, 0, 1, 44, 1, 64, 2, 4, 116, 104, 105, 115, 9, 5, 118, 97, 108, 117, 101, 127, 0, 28, 4, 13, 115, 101, 116, 45, 105, 112, 118, 54, 45, 111, 110, 108, 121, 0, 1, 45, 4, 12, 110, 111, 110, 45, 98, 108, 111, 99, 107, 105, 110, 103, 0, 1, 44, 4, 16, 115, 101, 116, 45, 110, 111, 110, 45, 98, 108, 111, 99, 107, 105, 110, 103, 0, 1, 45, 1, 64, 1, 4, 116, 104, 105, 115, 9, 0, 1, 4, 9, 115, 117, 98, 115, 99, 114, 105, 98, 101, 0, 1, 46, 3, 8, 119, 97, 115, 105, 45, 117, 100, 112, 22, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 117, 100, 112, 47, 119, 97, 115, 105, 45, 117, 100, 112, 5, 24, 1, 66, 3, 1, 106, 0, 0, 1, 64, 1, 6, 115, 116, 97, 116, 117, 115, 0, 1, 0, 4, 4, 101, 120, 105, 116, 0, 1, 1, 3, 9, 119, 97, 115, 105, 45, 101, 120, 105, 116, 24, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 101, 120, 105, 116, 47, 119, 97, 115, 105, 45, 101, 120, 105, 116, 5, 25, 1, 66, 4, 1, 111, 2, 115, 115, 1, 112, 0, 1, 64, 0, 0, 1, 4, 15, 103, 101, 116, 45, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 0, 1, 2, 3, 16, 119, 97, 115, 105, 45, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 38, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 47, 119, 97, 115, 105, 45, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 5, 26, 4, 4, 119, 97, 115, 105, 14, 112, 107, 103, 58, 47, 119, 97, 115, 105, 47, 119, 97, 115, 105, 4, 0, 0, 45, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 1, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 5, 48, 46, 54, 46, 48, 11, 19, 1, 4, 119, 97, 115, 105, 9, 112, 107, 103, 58, 47, 119, 97, 115, 105, 3, 36, 0];
    
    #[inline(never)]
    #[doc(hidden)]
    #[cfg(target_arch = "wasm32")]
    pub fn __link_section() {}
    