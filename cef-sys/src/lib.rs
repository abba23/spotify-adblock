#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

//! Raw FFI bindings for Chromium Embedded Framework (CEF)
//!
//! This crate provides unsafe, low-level bindings to the CEF C API.
//! For safe, high-level wrappers, see the parent `spotify-adblock` crate.

// Include the generated CEF bindings
include!(concat!(env!("OUT_DIR"), "/cef_bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bindings_exist() {
        // Basic smoke test to ensure bindings are generated
        let _app_size = std::mem::size_of::<cef_app_t>();
        let _client_size = std::mem::size_of::<cef_client_t>();
        // If this compiles, the bindings were generated successfully
    }
}
