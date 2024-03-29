// Generated by `wit-bindgen` 0.15.0. DO NOT EDIT!
pub mod akkoro {
    pub mod secrets {

        #[allow(clippy::all)]
        pub mod secret_storage {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_section;
            /// TODO this should be a capability handle or `resource`
            pub type Key = wit_bindgen::rt::string::String;
            #[derive(Clone)]
            pub struct Secret {
                pub id: wit_bindgen::rt::string::String,
                pub value: Option<wit_bindgen::rt::vec::Vec<u8>>,
            }
            impl ::core::fmt::Debug for Secret {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("Secret")
                        .field("id", &self.id)
                        .field("value", &self.value)
                        .finish()
                }
            }
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, PartialEq)]
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
            impl ::core::fmt::Debug for Error {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("Error")
                        .field("code", &(*self as i32))
                        .field("name", &self.name())
                        .field("message", &self.message())
                        .finish()
                }
            }
            impl ::core::fmt::Display for Error {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    write!(f, "{} (error {})", self.name(), *self as i32)
                }
            }

            impl std::error::Error for Error {}

            impl Error {
                pub(crate) unsafe fn _lift(val: u8) -> Error {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }

                    match val {
                        0 => Error::Success,
                        1 => Error::InvalidArgument,
                        2 => Error::Forbidden,

                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }

            #[allow(unused_unsafe, clippy::all)]
            /// Return the secret value associated with the handle
            pub fn get_secret_value(id: &str) -> Result<Secret, Error> {
                #[allow(unused_imports)]
                use wit_bindgen::rt::{alloc, string::String, vec::Vec};
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([u8; 24]);
                    let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                    let vec0 = id;
                    let ptr0 = vec0.as_ptr() as i32;
                    let len0 = vec0.len() as i32;
                    let ptr1 = ret_area.as_mut_ptr() as i32;
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "akkoro:secrets/secret-storage")]
                    extern "C" {
                        #[link_name = "get-secret-value"]
                        fn wit_import(_: i32, _: i32, _: i32);
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: i32, _: i32) {
                        unreachable!()
                    }
                    wit_import(ptr0, len0, ptr1);
                    let l2 = i32::from(*((ptr1 + 0) as *const u8));
                    match l2 {
                        0 => {
                            let e = {
                                let l3 = *((ptr1 + 4) as *const i32);
                                let l4 = *((ptr1 + 8) as *const i32);
                                let len5 = l4 as usize;
                                let bytes5 = Vec::from_raw_parts(l3 as *mut _, len5, len5);
                                let l6 = i32::from(*((ptr1 + 12) as *const u8));

                                Secret {
                                    id: wit_bindgen::rt::string_lift(bytes5),
                                    value: match l6 {
                                        0 => None,
                                        1 => {
                                            let e = {
                                                let l7 = *((ptr1 + 16) as *const i32);
                                                let l8 = *((ptr1 + 20) as *const i32);
                                                let len9 = l8 as usize;

                                                Vec::from_raw_parts(l7 as *mut _, len9, len9)
                                            };
                                            Some(e)
                                        }
                                        _ => wit_bindgen::rt::invalid_enum_discriminant(),
                                    },
                                }
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l10 = i32::from(*((ptr1 + 4) as *const u8));

                                Error::_lift(l10 as u8)
                            };
                            Err(e)
                        }
                        _ => wit_bindgen::rt::invalid_enum_discriminant(),
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Set the secret value associated with the handle
            pub fn set_secret_value(id: &str, value: &[u8], key: &Key) -> Result<Secret, Error> {
                #[allow(unused_imports)]
                use wit_bindgen::rt::{alloc, string::String, vec::Vec};
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([u8; 24]);
                    let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
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
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "akkoro:secrets/secret-storage")]
                    extern "C" {
                        #[link_name = "set-secret-value"]
                        fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32);
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32) {
                        unreachable!()
                    }
                    wit_import(ptr0, len0, ptr1, len1, ptr2, len2, ptr3);
                    let l4 = i32::from(*((ptr3 + 0) as *const u8));
                    match l4 {
                        0 => {
                            let e = {
                                let l5 = *((ptr3 + 4) as *const i32);
                                let l6 = *((ptr3 + 8) as *const i32);
                                let len7 = l6 as usize;
                                let bytes7 = Vec::from_raw_parts(l5 as *mut _, len7, len7);
                                let l8 = i32::from(*((ptr3 + 12) as *const u8));

                                Secret {
                                    id: wit_bindgen::rt::string_lift(bytes7),
                                    value: match l8 {
                                        0 => None,
                                        1 => {
                                            let e = {
                                                let l9 = *((ptr3 + 16) as *const i32);
                                                let l10 = *((ptr3 + 20) as *const i32);
                                                let len11 = l10 as usize;

                                                Vec::from_raw_parts(l9 as *mut _, len11, len11)
                                            };
                                            Some(e)
                                        }
                                        _ => wit_bindgen::rt::invalid_enum_discriminant(),
                                    },
                                }
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l12 = i32::from(*((ptr3 + 4) as *const u8));

                                Error::_lift(l12 as u8)
                            };
                            Err(e)
                        }
                        _ => wit_bindgen::rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:secrets"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 914] = [
    3, 0, 7, 115, 101, 99, 114, 101, 116, 115, 0, 97, 115, 109, 13, 0, 1, 0, 7, 210, 1, 1, 65, 2,
    1, 66, 13, 1, 115, 4, 0, 3, 107, 101, 121, 3, 0, 0, 1, 112, 125, 1, 107, 2, 1, 114, 2, 2, 105,
    100, 115, 5, 118, 97, 108, 117, 101, 3, 4, 0, 6, 115, 101, 99, 114, 101, 116, 3, 0, 4, 1, 109,
    3, 7, 115, 117, 99, 99, 101, 115, 115, 16, 105, 110, 118, 97, 108, 105, 100, 45, 97, 114, 103,
    117, 109, 101, 110, 116, 9, 102, 111, 114, 98, 105, 100, 100, 101, 110, 4, 0, 5, 101, 114, 114,
    111, 114, 3, 0, 6, 1, 106, 1, 5, 1, 7, 1, 64, 1, 2, 105, 100, 115, 0, 8, 4, 0, 16, 103, 101,
    116, 45, 115, 101, 99, 114, 101, 116, 45, 118, 97, 108, 117, 101, 1, 9, 1, 64, 3, 2, 105, 100,
    115, 5, 118, 97, 108, 117, 101, 2, 3, 107, 101, 121, 1, 0, 8, 4, 0, 16, 115, 101, 116, 45, 115,
    101, 99, 114, 101, 116, 45, 118, 97, 108, 117, 101, 1, 10, 4, 1, 29, 97, 107, 107, 111, 114,
    111, 58, 115, 101, 99, 114, 101, 116, 115, 47, 115, 101, 99, 114, 101, 116, 45, 115, 116, 111,
    114, 97, 103, 101, 5, 0, 11, 20, 1, 0, 14, 115, 101, 99, 114, 101, 116, 45, 115, 116, 111, 114,
    97, 103, 101, 3, 0, 0, 7, 240, 1, 1, 65, 2, 1, 65, 2, 1, 66, 13, 1, 115, 4, 0, 3, 107, 101,
    121, 3, 0, 0, 1, 112, 125, 1, 107, 2, 1, 114, 2, 2, 105, 100, 115, 5, 118, 97, 108, 117, 101,
    3, 4, 0, 6, 115, 101, 99, 114, 101, 116, 3, 0, 4, 1, 109, 3, 7, 115, 117, 99, 99, 101, 115,
    115, 16, 105, 110, 118, 97, 108, 105, 100, 45, 97, 114, 103, 117, 109, 101, 110, 116, 9, 102,
    111, 114, 98, 105, 100, 100, 101, 110, 4, 0, 5, 101, 114, 114, 111, 114, 3, 0, 6, 1, 106, 1, 5,
    1, 7, 1, 64, 1, 2, 105, 100, 115, 0, 8, 4, 0, 16, 103, 101, 116, 45, 115, 101, 99, 114, 101,
    116, 45, 118, 97, 108, 117, 101, 1, 9, 1, 64, 3, 2, 105, 100, 115, 5, 118, 97, 108, 117, 101,
    2, 3, 107, 101, 121, 1, 0, 8, 4, 0, 16, 115, 101, 116, 45, 115, 101, 99, 114, 101, 116, 45,
    118, 97, 108, 117, 101, 1, 10, 3, 1, 29, 97, 107, 107, 111, 114, 111, 58, 115, 101, 99, 114,
    101, 116, 115, 47, 115, 101, 99, 114, 101, 116, 45, 115, 116, 111, 114, 97, 103, 101, 5, 0, 4,
    1, 22, 97, 107, 107, 111, 114, 111, 58, 115, 101, 99, 114, 101, 116, 115, 47, 115, 101, 99,
    114, 101, 116, 115, 4, 0, 11, 13, 1, 0, 7, 115, 101, 99, 114, 101, 116, 115, 3, 2, 0, 0, 200,
    2, 12, 112, 97, 99, 107, 97, 103, 101, 45, 100, 111, 99, 115, 0, 123, 34, 105, 110, 116, 101,
    114, 102, 97, 99, 101, 115, 34, 58, 123, 34, 115, 101, 99, 114, 101, 116, 45, 115, 116, 111,
    114, 97, 103, 101, 34, 58, 123, 34, 100, 111, 99, 115, 34, 58, 34, 80, 114, 111, 118, 105, 100,
    101, 115, 32, 105, 110, 116, 101, 114, 102, 97, 99, 101, 32, 116, 111, 32, 97, 32, 115, 101,
    99, 114, 101, 116, 115, 32, 115, 116, 111, 114, 101, 34, 44, 34, 102, 117, 110, 99, 115, 34,
    58, 123, 34, 103, 101, 116, 45, 115, 101, 99, 114, 101, 116, 45, 118, 97, 108, 117, 101, 34,
    58, 34, 82, 101, 116, 117, 114, 110, 32, 116, 104, 101, 32, 115, 101, 99, 114, 101, 116, 32,
    118, 97, 108, 117, 101, 32, 97, 115, 115, 111, 99, 105, 97, 116, 101, 100, 32, 119, 105, 116,
    104, 32, 116, 104, 101, 32, 104, 97, 110, 100, 108, 101, 34, 44, 34, 115, 101, 116, 45, 115,
    101, 99, 114, 101, 116, 45, 118, 97, 108, 117, 101, 34, 58, 34, 83, 101, 116, 32, 116, 104,
    101, 32, 115, 101, 99, 114, 101, 116, 32, 118, 97, 108, 117, 101, 32, 97, 115, 115, 111, 99,
    105, 97, 116, 101, 100, 32, 119, 105, 116, 104, 32, 116, 104, 101, 32, 104, 97, 110, 100, 108,
    101, 34, 125, 44, 34, 116, 121, 112, 101, 115, 34, 58, 123, 34, 107, 101, 121, 34, 58, 123, 34,
    100, 111, 99, 115, 34, 58, 34, 84, 79, 68, 79, 32, 116, 104, 105, 115, 32, 115, 104, 111, 117,
    108, 100, 32, 98, 101, 32, 97, 32, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 32, 104, 97,
    110, 100, 108, 101, 32, 111, 114, 32, 96, 114, 101, 115, 111, 117, 114, 99, 101, 96, 34, 125,
    125, 125, 125, 125, 0, 70, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111,
    99, 101, 115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111,
    110, 101, 110, 116, 6, 48, 46, 49, 56, 46, 50, 16, 119, 105, 116, 45, 98, 105, 110, 100, 103,
    101, 110, 45, 114, 117, 115, 116, 6, 48, 46, 49, 53, 46, 48,
];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
