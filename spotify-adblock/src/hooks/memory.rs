use cef_sys::cef_string_userfree_utf16_t;  // Changed from crate::cef
use lazy_static::lazy_static;

use crate::hook;

hook! {
    cef_string_userfree_utf16_free(_str: cef_string_userfree_utf16_t) -> () => REAL_CEF_STRING_USERFREE_UTF16_FREE {
        // Null pointer checks for radiation hardened operation
        let is_null = _str.is_null();

        // Fault containment - double check to protect against bit flips
        let is_really_null = _str.is_null();

        // Only proceed if both checks agree
        if !is_null && !is_really_null {
            REAL_CEF_STRING_USERFREE_UTF16_FREE(_str);
        }
    }
}
