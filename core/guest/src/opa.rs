#[allow(clippy::all)]
pub mod opa {
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum PolicyError {
        InvalidWasm,
        NoEntrypoint,
    }
    impl PolicyError {
        pub fn name(&self) -> &'static str {
            match self {
                PolicyError::InvalidWasm => "invalid-wasm",
                PolicyError::NoEntrypoint => "no-entrypoint",
            }
        }
        pub fn message(&self) -> &'static str {
            match self {
                PolicyError::InvalidWasm => "",
                PolicyError::NoEntrypoint => "",
            }
        }
    }
    impl core::fmt::Debug for PolicyError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("PolicyError")
                .field("code", &(*self as i32))
                .field("name", &self.name())
                .field("message", &self.message())
                .finish()
        }
    }
    impl core::fmt::Display for PolicyError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} (error {})", self.name(), *self as i32)
        }
    }

    impl std::error::Error for PolicyError {}
    #[derive(Clone)]
    pub struct PolicyParam<'a> {
        pub id: &'a str,
        pub entrypoints: &'a [&'a str],
    }
    impl<'a> core::fmt::Debug for PolicyParam<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("PolicyParam")
                .field("id", &self.id)
                .field("entrypoints", &self.entrypoints)
                .finish()
        }
    }
    #[derive(Clone)]
    pub struct PolicyResult {
        pub id: wit_bindgen::rt::string::String,
        pub entrypoints: wit_bindgen::rt::vec::Vec<wit_bindgen::rt::string::String>,
    }
    impl core::fmt::Debug for PolicyResult {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("PolicyResult")
                .field("id", &self.id)
                .field("entrypoints", &self.entrypoints)
                .finish()
        }
    }
    #[allow(clippy::all)]
    pub fn new_policy(bytes: &[u8]) -> Result<PolicyResult, PolicyError> {
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(4))]
            struct RetArea([u8; 20]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let vec0 = bytes;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            let ptr1 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "opa")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "new-policy")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "opa_new-policy")]
                fn wit_import(_: i32, _: i32, _: i32);
            }
            wit_import(ptr0, len0, ptr1);
            match i32::from(*((ptr1 + 0) as *const u8)) {
                0 => Ok({
                    let len2 = *((ptr1 + 8) as *const i32) as usize;
                    let base4 = *((ptr1 + 12) as *const i32);
                    let len4 = *((ptr1 + 16) as *const i32);
                    let mut result4 = Vec::with_capacity(len4 as usize);
                    for i in 0..len4 {
                        let base = base4 + i * 8;
                        result4.push({
                            let len3 = *((base + 4) as *const i32) as usize;

                            String::from_utf8(Vec::from_raw_parts(
                                *((base + 0) as *const i32) as *mut _,
                                len3,
                                len3,
                            ))
                            .unwrap()
                        });
                    }
                    wit_bindgen::rt::dealloc(base4, (len4 as usize) * 8, 4);

                    PolicyResult {
                        id: String::from_utf8(Vec::from_raw_parts(
                            *((ptr1 + 4) as *const i32) as *mut _,
                            len2,
                            len2,
                        ))
                        .unwrap(),
                        entrypoints: result4,
                    }
                }),
                1 => Err(match i32::from(*((ptr1 + 4) as *const u8)) {
                    0 => PolicyError::InvalidWasm,
                    1 => PolicyError::NoEntrypoint,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    pub fn eval(policy: PolicyParam<'_>, data: &str, input: &str) -> bool {
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, string::String, vec::Vec};
        unsafe {
            let PolicyParam {
                id: id0,
                entrypoints: entrypoints0,
            } = policy;
            let vec1 = id0;
            let ptr1 = vec1.as_ptr() as i32;
            let len1 = vec1.len() as i32;
            let vec3 = entrypoints0;
            let len3 = vec3.len() as i32;
            let layout3 = alloc::Layout::from_size_align_unchecked(vec3.len() * 8, 4);
            let result3 = if layout3.size() != 0 {
                let ptr = alloc::alloc(layout3);
                if ptr.is_null() {
                    alloc::handle_alloc_error(layout3);
                }
                ptr
            } else {
                core::ptr::null_mut()
            };
            for (i, e) in vec3.into_iter().enumerate() {
                let base = result3 as i32 + (i as i32) * 8;
                {
                    let vec2 = e;
                    let ptr2 = vec2.as_ptr() as i32;
                    let len2 = vec2.len() as i32;
                    *((base + 4) as *mut i32) = len2;
                    *((base + 0) as *mut i32) = ptr2;
                }
            }
            let vec4 = data;
            let ptr4 = vec4.as_ptr() as i32;
            let len4 = vec4.len() as i32;
            let vec5 = input;
            let ptr5 = vec5.as_ptr() as i32;
            let len5 = vec5.len() as i32;

            #[link(wasm_import_module = "opa")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "eval")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "opa_eval")]
                fn wit_import(
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                ) -> i32;
            }
            let ret = wit_import(ptr1, len1, result3 as i32, len3, ptr4, len4, ptr5, len5);
            if layout3.size() != 0 {
                alloc::dealloc(result3, layout3);
            }
            match ret {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:opa"]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 492] = [
    2, 0, 3, 111, 112, 97, 3, 111, 112, 97, 3, 111, 112, 97, 0, 97, 115, 109, 12, 0, 1, 0, 7, 145,
    3, 1, 65, 4, 1, 66, 11, 1, 109, 2, 12, 105, 110, 118, 97, 108, 105, 100, 45, 119, 97, 115, 109,
    13, 110, 111, 45, 101, 110, 116, 114, 121, 112, 111, 105, 110, 116, 4, 12, 112, 111, 108, 105,
    99, 121, 45, 101, 114, 114, 111, 114, 0, 3, 0, 0, 1, 112, 115, 1, 114, 2, 2, 105, 100, 115, 11,
    101, 110, 116, 114, 121, 112, 111, 105, 110, 116, 115, 2, 4, 6, 112, 111, 108, 105, 99, 121, 0,
    3, 0, 3, 1, 112, 125, 1, 106, 1, 4, 1, 1, 1, 64, 1, 5, 98, 121, 116, 101, 115, 5, 0, 6, 4, 10,
    110, 101, 119, 45, 112, 111, 108, 105, 99, 121, 0, 1, 7, 1, 64, 3, 6, 112, 111, 108, 105, 99,
    121, 4, 4, 100, 97, 116, 97, 115, 5, 105, 110, 112, 117, 116, 115, 0, 127, 4, 4, 101, 118, 97,
    108, 0, 1, 8, 4, 10, 111, 112, 97, 45, 109, 111, 100, 117, 108, 101, 19, 112, 107, 103, 58, 47,
    111, 112, 97, 47, 111, 112, 97, 45, 109, 111, 100, 117, 108, 101, 5, 0, 1, 65, 2, 1, 66, 11, 1,
    109, 2, 12, 105, 110, 118, 97, 108, 105, 100, 45, 119, 97, 115, 109, 13, 110, 111, 45, 101,
    110, 116, 114, 121, 112, 111, 105, 110, 116, 4, 12, 112, 111, 108, 105, 99, 121, 45, 101, 114,
    114, 111, 114, 0, 3, 0, 0, 1, 112, 115, 1, 114, 2, 2, 105, 100, 115, 11, 101, 110, 116, 114,
    121, 112, 111, 105, 110, 116, 115, 2, 4, 6, 112, 111, 108, 105, 99, 121, 0, 3, 0, 3, 1, 112,
    125, 1, 106, 1, 4, 1, 1, 1, 64, 1, 5, 98, 121, 116, 101, 115, 5, 0, 6, 4, 10, 110, 101, 119,
    45, 112, 111, 108, 105, 99, 121, 0, 1, 7, 1, 64, 3, 6, 112, 111, 108, 105, 99, 121, 4, 4, 100,
    97, 116, 97, 115, 5, 105, 110, 112, 117, 116, 115, 0, 127, 4, 4, 101, 118, 97, 108, 0, 1, 8, 3,
    3, 111, 112, 97, 19, 112, 107, 103, 58, 47, 111, 112, 97, 47, 111, 112, 97, 45, 109, 111, 100,
    117, 108, 101, 5, 0, 4, 3, 111, 112, 97, 12, 112, 107, 103, 58, 47, 111, 112, 97, 47, 111, 112,
    97, 4, 1, 0, 45, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101,
    115, 115, 101, 100, 45, 98, 121, 1, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101,
    110, 116, 5, 48, 46, 54, 46, 48, 11, 17, 1, 3, 111, 112, 97, 8, 112, 107, 103, 58, 47, 111,
    112, 97, 3, 0, 0,
];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
