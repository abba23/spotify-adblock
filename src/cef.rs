#![allow(non_camel_case_types)]

#[repr(C)]
pub struct _cef_request_t {
    base: [u8; 40],
    is_read_only: [u8; 8],
    pub get_url: unsafe extern "C" fn(self_: *mut _cef_request_t) -> *mut _cef_string_utf16_t,
}

#[repr(C)]
pub struct _cef_string_utf16_t {
    pub str: *mut u16,
    pub length: usize,
}
