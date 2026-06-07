mod cef;
mod config;
mod hook;
mod log;

use crate::cef::{_cef_request_t, _cef_string_utf16_t};
use crate::config::CONFIG;

use libc::{EAI_FAIL, addrinfo, c_char};
use std::{
    ffi::{CStr, c_void},
    ptr, slice,
};

hook! {
    getaddrinfo(node: *const c_char, service: *const c_char, hints: *const addrinfo, res: *const *const addrinfo) -> i32 => real_getaddrinfo {
        let domain = unsafe { CStr::from_ptr(node) }.to_str().unwrap();
        if CONFIG.allowlist.is_match(domain) {
            log_green!("[+] getaddrinfo: {domain}");
            real_getaddrinfo(node, service, hints, res)
        } else {
            log_red!("[-] getaddrinfo: {domain}");
            EAI_FAIL
        }
    }
}

hook! {
    cef_urlrequest_create(request: *mut _cef_request_t, client: *const c_void, request_context: *const c_void) -> *const c_void => real_cef_urlrequest_create {
        let url = extract_url(request);
        if CONFIG.denylist.is_match(&url) {
            log_red!("[-] cef_urlrequest_create: {url}");
            ptr::null()
        } else {
            log_green!("[+] cef_urlrequest_create: {url}");
            real_cef_urlrequest_create(request, client, request_context)
        }
    }
}

fn extract_url(request: *mut _cef_request_t) -> String {
    unsafe {
        let url_cef_string = ((*request).get_url)(request);
        let url = String::from_utf16(slice::from_raw_parts((*url_cef_string).str, (*url_cef_string).length)).unwrap();
        cef_string_userfree_utf16_free(url_cef_string);
        url
    }
}

hook! {
    cef_string_userfree_utf16_free(str: *mut _cef_string_utf16_t) -> () => real_cef_string_userfree_utf16_free {
        real_cef_string_userfree_utf16_free(str);
    }
}
