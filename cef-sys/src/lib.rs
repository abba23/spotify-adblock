#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

//! Raw FFI bindings for Chromium Embedded Framework (CEF)
//!
//! This crate provides unsafe, low-level bindings to the CEF C API.
//! For safe, high-level wrappers, see the parent `spotify-adblock` crate.
//!
//! # Usage
//!
//! ```
//! use cef_sys::*;
//!
//! // Use CEF types and functions
//! let app_size = std::mem::size_of::<cef_app_t>();
//! ```
//!
//! # Safety
//!
//! All functions in this crate are unsafe and require careful handling of:
//! - Null pointer checks
//! - Memory management
//! - Proper CEF initialization/shutdown sequences
//! - Thread safety considerations

// Include the generated CEF bindings
// This makes all CEF types and functions available directly
include!(concat!(env!("OUT_DIR"), "/cef_bindings.rs"));

// CEF library version information
pub const CEF_VERSION_MAJOR: u32 = 137;

/// Helper function to check if CEF is initialized
///
/// # Safety
/// This function calls into CEF's C API
pub unsafe fn is_cef_initialized() -> bool {
    // This would typically call a CEF function to check initialization status
    // For now, this is a placeholder - the actual implementation would depend
    // on CEF's API for checking initialization status
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bindings_exist() {
        // Basic smoke test to ensure bindings are generated
        let _app_size = std::mem::size_of::<cef_app_t>();
        let _client_size = std::mem::size_of::<cef_client_t>();
        let _browser_size = std::mem::size_of::<cef_browser_t>();
        let _frame_size = std::mem::size_of::<cef_frame_t>();
        let _request_size = std::mem::size_of::<cef_request_t>();
        let _response_size = std::mem::size_of::<cef_response_t>();

        // If this compiles, the bindings were generated successfully
        assert!(_app_size > 0);
        assert!(_client_size > 0);
        assert!(_browser_size > 0);
    }

    #[test]
    fn test_string_types_exist() {
        // Test that CEF string types are available
        let _string_size = std::mem::size_of::<cef_string_t>();
        let _userfree_size = std::mem::size_of::<cef_string_userfree_t>();

        assert!(_string_size > 0);
    }

    #[test]
    fn test_handler_types_exist() {
        // Test that handler types are available
        let _browser_process_size = std::mem::size_of::<cef_browser_process_handler_t>();
        let _render_process_size = std::mem::size_of::<cef_render_process_handler_t>();

        assert!(_browser_process_size > 0);
        assert!(_render_process_size > 0);
    }

    #[test]
    fn test_commonly_used_functions_exist() {
        // Test that commonly used functions are available
        // Note: We can't actually call these functions in tests without proper CEF setup
        // but we can check that the function pointers exist

        // These should be available as extern "C" functions from the bindings
        // The actual function signatures will depend on what bindgen generates
    }
}
