
#[allow(clippy::all)]
pub mod asml_io{
  #[repr(u8)]
  #[derive(Clone, Copy, PartialEq, Eq)]
  pub enum PollError {
    NotReady,
    InvalidIoid,
  }
  impl PollError{
    pub fn name(&self) -> &'static str {
      match self {
        PollError::NotReady => "not-ready",
        PollError::InvalidIoid => "invalid-ioid",
      }
    }
    pub fn message(&self) -> &'static str {
      match self {
        PollError::NotReady => "",
        PollError::InvalidIoid => "",
      }
    }
  }
  impl core::fmt::Debug for PollError{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      f.debug_struct("PollError")
      .field("code", &(*self as i32))
      .field("name", &self.name())
      .field("message", &self.message())
      .finish()
    }
  }
  impl core::fmt::Display for PollError{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      write!(f, "{} (error {})", self.name(), *self as i32)}
    }
    
    impl std::error::Error for PollError{}
    pub type Ioid = u32;
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum IoError {
      CoordsNotFound,
      InvalidCoords,
      InvalidIoid,
    }
    impl IoError{
      pub fn name(&self) -> &'static str {
        match self {
          IoError::CoordsNotFound => "coords-not-found",
          IoError::InvalidCoords => "invalid-coords",
          IoError::InvalidIoid => "invalid-ioid",
        }
      }
      pub fn message(&self) -> &'static str {
        match self {
          IoError::CoordsNotFound => "",
          IoError::InvalidCoords => "",
          IoError::InvalidIoid => "",
        }
      }
    }
    impl core::fmt::Debug for IoError{
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IoError")
        .field("code", &(*self as i32))
        .field("name", &self.name())
        .field("message", &self.message())
        .finish()
      }
    }
    impl core::fmt::Display for IoError{
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} (error {})", self.name(), *self as i32)}
      }
      
      impl std::error::Error for IoError{}
      #[allow(clippy::all)]
      pub fn invoke(path: &str,input: &str,) -> Result<Ioid,IoError>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(4))]
          struct RetArea([u8; 8]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let vec0 = path;
          let ptr0 = vec0.as_ptr() as i32;
          let len0 = vec0.len() as i32;
          let vec1 = input;
          let ptr1 = vec1.as_ptr() as i32;
          let len1 = vec1.len() as i32;
          let ptr2 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "asml-io")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "invoke")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "asml-io_invoke")]
            fn wit_import(
            _: i32, _: i32, _: i32, _: i32, _: i32, );
          }
          wit_import(ptr0, len0, ptr1, len1, ptr2);
          match i32::from(*((ptr2 + 0) as *const u8)) {
            0 => Ok(*((ptr2 + 4) as *const i32) as u32),
            1 => Err(match i32::from(*((ptr2 + 4) as *const u8)) {
              0 => IoError::CoordsNotFound,
              1 => IoError::InvalidCoords,
              2 => IoError::InvalidIoid,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      #[allow(clippy::all)]
      pub fn poll(ioid: Ioid,) -> Result<wit_bindgen::rt::vec::Vec::<u8>,PollError>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(4))]
          struct RetArea([u8; 12]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "asml-io")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "poll")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "asml-io_poll")]
            fn wit_import(
            _: i32, _: i32, );
          }
          wit_import(wit_bindgen::rt::as_i32(ioid), ptr0);
          match i32::from(*((ptr0 + 0) as *const u8)) {
            0 => Ok({
              let len1 = *((ptr0 + 8) as *const i32) as usize;
              
              Vec::from_raw_parts(*((ptr0 + 4) as *const i32) as *mut _, len1, len1)
            }),
            1 => Err(match i32::from(*((ptr0 + 4) as *const u8)) {
              0 => PollError::NotReady,
              1 => PollError::InvalidIoid,
              _ => panic!("invalid enum discriminant"),
            }),
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      
    }
    
    
    #[allow(clippy::all)]
    pub mod asml_rt{
      pub type Bytes<'a,> = &'a [u8];
      #[allow(clippy::all)]
      pub fn success(response: Bytes<'_,>,) -> (){
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          let vec0 = response;
          let ptr0 = vec0.as_ptr() as i32;
          let len0 = vec0.len() as i32;
          
          #[link(wasm_import_module = "asml-rt")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "success")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "asml-rt_success")]
            fn wit_import(
            _: i32, _: i32, );
          }
          wit_import(ptr0, len0);
        }
      }
      #[allow(clippy::all)]
      pub fn failure(response: Bytes<'_,>,) -> (){
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          let vec0 = response;
          let ptr0 = vec0.as_ptr() as i32;
          let len0 = vec0.len() as i32;
          
          #[link(wasm_import_module = "asml-rt")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "failure")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "asml-rt_failure")]
            fn wit_import(
            _: i32, _: i32, );
          }
          wit_import(ptr0, len0);
        }
      }
      #[allow(clippy::all)]
      pub fn get_input() -> wit_bindgen::rt::vec::Vec::<u8>{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{{alloc, vec::Vec, string::String}};
        unsafe {
          
          #[repr(align(4))]
          struct RetArea([u8; 8]);
          let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
          let ptr0 = ret_area.as_mut_ptr() as i32;
          #[link(wasm_import_module = "asml-rt")]
          extern "C" {
            #[cfg_attr(target_arch = "wasm32", link_name = "get-input")]
            #[cfg_attr(not(target_arch = "wasm32"), link_name = "asml-rt_get-input")]
            fn wit_import(
            _: i32, );
          }
          wit_import(ptr0);
          let len1 = *((ptr0 + 4) as *const i32) as usize;
          Vec::from_raw_parts(*((ptr0 + 0) as *const i32) as *mut _, len1, len1)
        }
      }
      
    }
    
    
    #[cfg(target_arch = "wasm32")]
    #[link_section = "component-type:assemblylift"]
    pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 940] = [2, 0, 3, 119, 105, 116, 12, 97, 115, 115, 101, 109, 98, 108, 121, 108, 105, 102, 116, 12, 97, 115, 115, 101, 109, 98, 108, 121, 108, 105, 102, 116, 0, 97, 115, 109, 12, 0, 1, 0, 7, 173, 6, 1, 65, 6, 1, 66, 13, 1, 109, 2, 9, 110, 111, 116, 45, 114, 101, 97, 100, 121, 12, 105, 110, 118, 97, 108, 105, 100, 45, 105, 111, 105, 100, 4, 10, 112, 111, 108, 108, 45, 101, 114, 114, 111, 114, 0, 3, 0, 0, 1, 121, 4, 4, 105, 111, 105, 100, 0, 3, 0, 2, 1, 109, 3, 16, 99, 111, 111, 114, 100, 115, 45, 110, 111, 116, 45, 102, 111, 117, 110, 100, 14, 105, 110, 118, 97, 108, 105, 100, 45, 99, 111, 111, 114, 100, 115, 12, 105, 110, 118, 97, 108, 105, 100, 45, 105, 111, 105, 100, 4, 8, 105, 111, 45, 101, 114, 114, 111, 114, 0, 3, 0, 4, 1, 106, 1, 3, 1, 5, 1, 64, 2, 4, 112, 97, 116, 104, 115, 5, 105, 110, 112, 117, 116, 115, 0, 6, 4, 6, 105, 110, 118, 111, 107, 101, 0, 1, 7, 1, 112, 125, 1, 106, 1, 8, 1, 1, 1, 64, 1, 4, 105, 111, 105, 100, 3, 0, 9, 4, 4, 112, 111, 108, 108, 0, 1, 10, 4, 15, 97, 115, 115, 101, 109, 98, 108, 121, 108, 105, 102, 116, 45, 105, 111, 33, 112, 107, 103, 58, 47, 97, 115, 115, 101, 109, 98, 108, 121, 108, 105, 102, 116, 47, 97, 115, 115, 101, 109, 98, 108, 121, 108, 105, 102, 116, 45, 105, 111, 5, 0, 1, 66, 8, 1, 112, 125, 4, 5, 98, 121, 116, 101, 115, 0, 3, 0, 0, 1, 64, 1, 8, 114, 101, 115, 112, 111, 110, 115, 101, 1, 1, 0, 4, 7, 115, 117, 99, 99, 101, 115, 115, 0, 1, 2, 4, 7, 102, 97, 105, 108, 117, 114, 101, 0, 1, 2, 1, 112, 125, 1, 64, 0, 0, 3, 4, 9, 103, 101, 116, 45, 105, 110, 112, 117, 116, 0, 1, 4, 4, 29, 97, 115, 115, 101, 109, 98, 108, 121, 108, 105, 102, 116, 45, 102, 117, 110, 99, 116, 105, 111, 110, 45, 114, 117, 110, 116, 105, 109, 101, 47, 112, 107, 103, 58, 47, 97, 115, 115, 101, 109, 98, 108, 121, 108, 105, 102, 116, 47, 97, 115, 115, 101, 109, 98, 108, 121, 108, 105, 102, 116, 45, 102, 117, 110, 99, 116, 105, 111, 110, 45, 114, 117, 110, 116, 105, 109, 101, 5, 1, 1, 65, 4, 1, 66, 13, 1, 109, 2, 9, 110, 111, 116, 45, 114, 101, 97, 100, 121, 12, 105, 110, 118, 97, 108, 105, 100, 45, 105, 111, 105, 100, 4, 10, 112, 111, 108, 108, 45, 101, 114, 114, 111, 114, 0, 3, 0, 0, 1, 121, 4, 4, 105, 111, 105, 100, 0, 3, 0, 2, 1, 109, 3, 16, 99, 111, 111, 114, 100, 115, 45, 110, 111, 116, 45, 102, 111, 117, 110, 100, 14, 105, 110, 118, 97, 108, 105, 100, 45, 99, 111, 111, 114, 100, 115, 12, 105, 110, 118, 97, 108, 105, 100, 45, 105, 111, 105, 100, 4, 8, 105, 111, 45, 101, 114, 114, 111, 114, 0, 3, 0, 4, 1, 106, 1, 3, 1, 5, 1, 64, 2, 4, 112, 97, 116, 104, 115, 5, 105, 110, 112, 117, 116, 115, 0, 6, 4, 6, 105, 110, 118, 111, 107, 101, 0, 1, 7, 1, 112, 125, 1, 106, 1, 8, 1, 1, 1, 64, 1, 4, 105, 111, 105, 100, 3, 0, 9, 4, 4, 112, 111, 108, 108, 0, 1, 10, 3, 7, 97, 115, 109, 108, 45, 105, 111, 33, 112, 107, 103, 58, 47, 97, 115, 115, 101, 109, 98, 108, 121, 108, 105, 102, 116, 47, 97, 115, 115, 101, 109, 98, 108, 121, 108, 105, 102, 116, 45, 105, 111, 5, 0, 1, 66, 8, 1, 112, 125, 4, 5, 98, 121, 116, 101, 115, 0, 3, 0, 0, 1, 64, 1, 8, 114, 101, 115, 112, 111, 110, 115, 101, 1, 1, 0, 4, 7, 115, 117, 99, 99, 101, 115, 115, 0, 1, 2, 4, 7, 102, 97, 105, 108, 117, 114, 101, 0, 1, 2, 1, 112, 125, 1, 64, 0, 0, 3, 4, 9, 103, 101, 116, 45, 105, 110, 112, 117, 116, 0, 1, 4, 3, 7, 97, 115, 109, 108, 45, 114, 116, 47, 112, 107, 103, 58, 47, 97, 115, 115, 101, 109, 98, 108, 121, 108, 105, 102, 116, 47, 97, 115, 115, 101, 109, 98, 108, 121, 108, 105, 102, 116, 45, 102, 117, 110, 99, 116, 105, 111, 110, 45, 114, 117, 110, 116, 105, 109, 101, 5, 1, 4, 12, 97, 115, 115, 101, 109, 98, 108, 121, 108, 105, 102, 116, 30, 112, 107, 103, 58, 47, 97, 115, 115, 101, 109, 98, 108, 121, 108, 105, 102, 116, 47, 97, 115, 115, 101, 109, 98, 108, 121, 108, 105, 102, 116, 4, 2, 0, 45, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 1, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 5, 48, 46, 54, 46, 48, 11, 35, 1, 12, 97, 115, 115, 101, 109, 98, 108, 121, 108, 105, 102, 116, 17, 112, 107, 103, 58, 47, 97, 115, 115, 101, 109, 98, 108, 121, 108, 105, 102, 116, 3, 0, 0];
    
    #[inline(never)]
    #[doc(hidden)]
    #[cfg(target_arch = "wasm32")]
    pub fn __link_section() {}
    