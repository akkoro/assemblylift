#[allow(clippy::all)]
pub mod secrets {
    #[derive(Clone)]
    pub struct Secret {
        pub id: wit_bindgen::rt::string::String,
        pub value: Option<wit_bindgen::rt::vec::Vec<u8>>,
    }
    impl core::fmt::Debug for Secret {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Secret")
                .field("id", &self.id)
                .field("value", &self.value)
                .finish()
        }
    }
    pub type Key<'a> = &'a str;
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Error {
        Success,
        InvalidArgument,
        Forbidden,
    }
    impl Error {
        pub fn name(&self) -> &'static str {
            match self {
                Error::Success => "success",
                Error::InvalidArgument => "invalid-argument",
                Error::Forbidden => "forbidden",
            }
        }
        pub fn message(&self) -> &'static str {
            match self {
                Error::Success => "",
                Error::InvalidArgument => "",
                Error::Forbidden => "",
            }
        }
    }
    impl core::fmt::Debug for Error {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Error")
                .field("code", &(*self as i32))
                .field("name", &self.name())
                .field("message", &self.message())
                .finish()
        }
    }
    impl core::fmt::Display for Error {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} (error {})", self.name(), *self as i32)
        }
    }

    impl std::error::Error for Error {}
    #[allow(clippy::all)]
    /// Return the secret value associated with the handle
    pub fn get_secret_value(id: &str) -> Result<Secret, Error> {
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(4))]
            struct RetArea([u8; 24]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let vec0 = id;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            let ptr1 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "secrets")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "get-secret-value")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "secrets_get-secret-value")]
                fn wit_import(_: i32, _: i32, _: i32);
            }
            wit_import(ptr0, len0, ptr1);
            match i32::from(*((ptr1 + 0) as *const u8)) {
                0 => Ok({
                    let len2 = *((ptr1 + 8) as *const i32) as usize;

                    Secret {
                        id: String::from_utf8(Vec::from_raw_parts(
                            *((ptr1 + 4) as *const i32) as *mut _,
                            len2,
                            len2,
                        ))
                        .unwrap(),
                        value: match i32::from(*((ptr1 + 12) as *const u8)) {
                            0 => None,
                            1 => Some({
                                let len3 = *((ptr1 + 20) as *const i32) as usize;

                                Vec::from_raw_parts(
                                    *((ptr1 + 16) as *const i32) as *mut _,
                                    len3,
                                    len3,
                                )
                            }),
                            _ => panic!("invalid enum discriminant"),
                        },
                    }
                }),
                1 => Err(match i32::from(*((ptr1 + 4) as *const u8)) {
                    0 => Error::Success,
                    1 => Error::InvalidArgument,
                    2 => Error::Forbidden,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    /// Set the secret value associated with the handle
    pub fn set_secret_value(id: &str, value: &[u8], key: Key<'_>) -> Result<Secret, Error> {
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(4))]
            struct RetArea([u8; 24]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let vec0 = id;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            let vec1 = value;
            let ptr1 = vec1.as_ptr() as i32;
            let len1 = vec1.len() as i32;
            let vec2 = key;
            let ptr2 = vec2.as_ptr() as i32;
            let len2 = vec2.len() as i32;
            let ptr3 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "secrets")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "set-secret-value")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "secrets_set-secret-value")]
                fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32);
            }
            wit_import(ptr0, len0, ptr1, len1, ptr2, len2, ptr3);
            match i32::from(*((ptr3 + 0) as *const u8)) {
                0 => Ok({
                    let len4 = *((ptr3 + 8) as *const i32) as usize;

                    Secret {
                        id: String::from_utf8(Vec::from_raw_parts(
                            *((ptr3 + 4) as *const i32) as *mut _,
                            len4,
                            len4,
                        ))
                        .unwrap(),
                        value: match i32::from(*((ptr3 + 12) as *const u8)) {
                            0 => None,
                            1 => Some({
                                let len5 = *((ptr3 + 20) as *const i32) as usize;

                                Vec::from_raw_parts(
                                    *((ptr3 + 16) as *const i32) as *mut _,
                                    len5,
                                    len5,
                                )
                            }),
                            _ => panic!("invalid enum discriminant"),
                        },
                    }
                }),
                1 => Err(match i32::from(*((ptr3 + 4) as *const u8)) {
                    0 => Error::Success,
                    1 => Error::InvalidArgument,
                    2 => Error::Forbidden,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wasi-secrets"]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 600] = [
    2, 0, 3, 119, 105, 116, 12, 119, 97, 115, 105, 45, 115, 101, 99, 114, 101, 116, 115, 12, 119,
    97, 115, 105, 45, 115, 101, 99, 114, 101, 116, 115, 0, 97, 115, 109, 12, 0, 1, 0, 7, 217, 3, 1,
    65, 4, 1, 66, 13, 1, 112, 125, 1, 107, 0, 1, 114, 2, 2, 105, 100, 115, 5, 118, 97, 108, 117,
    101, 1, 4, 6, 115, 101, 99, 114, 101, 116, 0, 3, 0, 2, 1, 115, 4, 3, 107, 101, 121, 0, 3, 0, 4,
    1, 109, 3, 7, 115, 117, 99, 99, 101, 115, 115, 16, 105, 110, 118, 97, 108, 105, 100, 45, 97,
    114, 103, 117, 109, 101, 110, 116, 9, 102, 111, 114, 98, 105, 100, 100, 101, 110, 4, 5, 101,
    114, 114, 111, 114, 0, 3, 0, 6, 1, 106, 1, 3, 1, 7, 1, 64, 1, 2, 105, 100, 115, 0, 8, 4, 16,
    103, 101, 116, 45, 115, 101, 99, 114, 101, 116, 45, 118, 97, 108, 117, 101, 0, 1, 9, 1, 64, 3,
    2, 105, 100, 115, 5, 118, 97, 108, 117, 101, 0, 3, 107, 101, 121, 5, 0, 8, 4, 16, 115, 101,
    116, 45, 115, 101, 99, 114, 101, 116, 45, 118, 97, 108, 117, 101, 0, 1, 10, 4, 7, 115, 101, 99,
    114, 101, 116, 115, 25, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45, 115, 101, 99, 114, 101,
    116, 115, 47, 115, 101, 99, 114, 101, 116, 115, 5, 0, 1, 65, 2, 1, 66, 13, 1, 112, 125, 1, 107,
    0, 1, 114, 2, 2, 105, 100, 115, 5, 118, 97, 108, 117, 101, 1, 4, 6, 115, 101, 99, 114, 101,
    116, 0, 3, 0, 2, 1, 115, 4, 3, 107, 101, 121, 0, 3, 0, 4, 1, 109, 3, 7, 115, 117, 99, 99, 101,
    115, 115, 16, 105, 110, 118, 97, 108, 105, 100, 45, 97, 114, 103, 117, 109, 101, 110, 116, 9,
    102, 111, 114, 98, 105, 100, 100, 101, 110, 4, 5, 101, 114, 114, 111, 114, 0, 3, 0, 6, 1, 106,
    1, 3, 1, 7, 1, 64, 1, 2, 105, 100, 115, 0, 8, 4, 16, 103, 101, 116, 45, 115, 101, 99, 114, 101,
    116, 45, 118, 97, 108, 117, 101, 0, 1, 9, 1, 64, 3, 2, 105, 100, 115, 5, 118, 97, 108, 117,
    101, 0, 3, 107, 101, 121, 5, 0, 8, 4, 16, 115, 101, 116, 45, 115, 101, 99, 114, 101, 116, 45,
    118, 97, 108, 117, 101, 0, 1, 10, 3, 7, 115, 101, 99, 114, 101, 116, 115, 25, 112, 107, 103,
    58, 47, 119, 97, 115, 105, 45, 115, 101, 99, 114, 101, 116, 115, 47, 115, 101, 99, 114, 101,
    116, 115, 5, 0, 4, 12, 119, 97, 115, 105, 45, 115, 101, 99, 114, 101, 116, 115, 30, 112, 107,
    103, 58, 47, 119, 97, 115, 105, 45, 115, 101, 99, 114, 101, 116, 115, 47, 119, 97, 115, 105,
    45, 115, 101, 99, 114, 101, 116, 115, 4, 1, 0, 45, 9, 112, 114, 111, 100, 117, 99, 101, 114,
    115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 1, 13, 119, 105, 116, 45,
    99, 111, 109, 112, 111, 110, 101, 110, 116, 5, 48, 46, 54, 46, 48, 11, 35, 1, 12, 119, 97, 115,
    105, 45, 115, 101, 99, 114, 101, 116, 115, 17, 112, 107, 103, 58, 47, 119, 97, 115, 105, 45,
    115, 101, 99, 114, 101, 116, 115, 3, 0, 0,
];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
