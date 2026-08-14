mod cef;
mod config;
mod hook;
mod log;

use crate::cef::{_cef_request_t, _cef_string_utf16_t};
use crate::config::CONFIG;

use std::{ffi::c_void, ptr, slice};

hook! {
    cef_urlrequest_create(request: *mut _cef_request_t, client: *const c_void, request_context: *const c_void) -> *const c_void => real_cef_urlrequest_create {
        let url = extract_url(request);
        if CONFIG.denylist.is_match(&url) {
            log_red!("[-] {url}");
            ptr::null()
        } else {
            log_green!("[+] {url}");
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
