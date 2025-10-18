use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    let cef_root = env::var("CEF_ROOT")
        .expect("CEF_ROOT environment variable must be set to the CEF distribution path");

    let cef_path = PathBuf::from(&cef_root);

    // Verify CEF directory structure
    assert!(
        cef_path.join("include").exists(),
        "CEF include directory not found. Check CEF_ROOT path: {cef_root}"
    );

    println!("cargo:rerun-if-changed={cef_root}/include");
    println!("cargo:rerun-if-env-changed=CEF_ROOT");

    // Set up library search paths
    println!("cargo:rustc-link-search=native={cef_root}/Release");
    println!("cargo:rustc-link-search=native={cef_root}/Debug");

    // Link CEF libraries
    println!("cargo:rustc-link-lib=cef");

    // Platform-specific linking for Linux
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=X11");
        println!("cargo:rustc-link-lib=Xcomposite");
        println!("cargo:rustc-link-lib=Xcursor");
        println!("cargo:rustc-link-lib=Xdamage");
        println!("cargo:rustc-link-lib=Xext");
        println!("cargo:rustc-link-lib=Xfixes");
        println!("cargo:rustc-link-lib=Xi");
        println!("cargo:rustc-link-lib=Xrandr");
        println!("cargo:rustc-link-lib=Xrender");
        println!("cargo:rustc-link-lib=Xss");
        println!("cargo:rustc-link-lib=Xtst");
        println!("cargo:rustc-link-lib=xkbcommon");
        println!("cargo:rustc-link-lib=pango-1.0");
        println!("cargo:rustc-link-lib=cairo");
        println!("cargo:rustc-link-lib=gtk-3");
        println!("cargo:rustc-link-lib=gdk-3");
        println!("cargo:rustc-link-lib=glib-2.0");
        println!("cargo:rustc-link-lib=gobject-2.0");
        println!("cargo:rustc-link-lib=atk-1.0");
        println!("cargo:rustc-link-lib=gio-2.0");
        println!("cargo:rustc-link-lib=fontconfig");
        println!("cargo:rustc-link-lib=freetype");
    }

    // Create wrapper header that includes all CEF C API headers
    let wrapper_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("cef_wrapper.h");
    let wrapper_content = create_wrapper_header();
    fs::write(&wrapper_path, wrapper_content).expect("Failed to write wrapper header");

    // Generate bindings with correct include paths
    let bindings = bindgen::Builder::default()
        .header(wrapper_path.to_string_lossy())
        // CRITICAL: Add CEF root to include path so relative includes work
        .clang_arg(format!("-I{cef_root}"))
        .clang_arg(format!("-I{cef_root}/include"))

        // Allow all CEF functions, types, and constants
        .allowlist_function("cef_.*")
        .allowlist_type("cef_.*")
        .allowlist_var("CEF_.*")
        .allowlist_var("cef_.*")

        // Block some problematic items
        .blocklist_type("max_align_t")
        .blocklist_function("bindgen_test_layout_.*")

        // Handle different calling conventions
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })

        // Generate comments for documentation
        .generate_comments(true)

        // REMOVED: .ctypes_prefix() - use bindgen's default ctypes behavior
        // This will use std::os::raw::c_int, etc. instead of std::ffi::c_int

        // Handle size_t and other types properly
        .size_t_is_usize(true)

        // Generate Debug implementations
        .derive_debug(true)
        .derive_default(true)
        .derive_copy(true)

        // Handle function pointers properly
        .trust_clang_mangling(false)

        // Parse callbacks for additional processing
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))

        .generate()
        .expect("Unable to generate CEF bindings");

    // Write bindings to file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("cef_bindings.rs"))
        .expect("Couldn't write CEF bindings!");

    println!("CEF bindings generated successfully!");
}

fn create_wrapper_header() -> String {
    // Use the correct include paths that work with CEF's directory structure
    r#"
// CEF C API Wrapper Header for Rust Bindings

// Include all main CEF C API headers using paths relative to CEF root
#include "include/capi/cef_base_capi.h"
#include "include/capi/cef_app_capi.h"
#include "include/capi/cef_client_capi.h"
#include "include/capi/cef_browser_capi.h"
#include "include/capi/cef_browser_process_handler_capi.h"
#include "include/capi/cef_render_process_handler_capi.h"
#include "include/capi/cef_life_span_handler_capi.h"
#include "include/capi/cef_load_handler_capi.h"
#include "include/capi/cef_display_handler_capi.h"
#include "include/capi/cef_context_menu_handler_capi.h"
#include "include/capi/cef_dialog_handler_capi.h"
#include "include/capi/cef_download_handler_capi.h"
#include "include/capi/cef_drag_handler_capi.h"
#include "include/capi/cef_find_handler_capi.h"
#include "include/capi/cef_focus_handler_capi.h"
#include "include/capi/cef_frame_handler_capi.h"
#include "include/capi/cef_jsdialog_handler_capi.h"
#include "include/capi/cef_keyboard_handler_capi.h"
#include "include/capi/cef_permission_handler_capi.h"
#include "include/capi/cef_print_handler_capi.h"
#include "include/capi/cef_render_handler_capi.h"
#include "include/capi/cef_request_handler_capi.h"
#include "include/capi/cef_resource_request_handler_capi.h"
#include "include/capi/cef_request_capi.h"
#include "include/capi/cef_response_capi.h"
#include "include/capi/cef_urlrequest_capi.h"
#include "include/capi/cef_cookie_capi.h"
#include "include/capi/cef_request_context_capi.h"
#include "include/capi/cef_resource_handler_capi.h"
#include "include/capi/cef_response_filter_capi.h"
#include "include/capi/cef_scheme_capi.h"
#include "include/capi/cef_command_line_capi.h"
#include "include/capi/cef_process_message_capi.h"
#include "include/capi/cef_values_capi.h"
#include "include/capi/cef_frame_capi.h"
#include "include/capi/cef_image_capi.h"
#include "include/capi/cef_menu_model_capi.h"
#include "include/capi/cef_dom_capi.h"
#include "include/capi/cef_v8_capi.h"
#include "include/capi/cef_callback_capi.h"
#include "include/capi/cef_task_capi.h"
#include "include/capi/cef_thread_capi.h"
#include "include/capi/cef_waitable_event_capi.h"
#include "include/capi/cef_trace_capi.h"
#include "include/capi/cef_file_util_capi.h"
#include "include/capi/cef_path_util_capi.h"
#include "include/capi/cef_process_util_capi.h"
#include "include/capi/cef_stream_capi.h"
#include "include/capi/cef_string_visitor_capi.h"
#include "include/capi/cef_zip_reader_capi.h"
#include "include/capi/cef_xml_reader_capi.h"
#include "include/capi/cef_accessibility_handler_capi.h"
#include "include/capi/cef_audio_handler_capi.h"
#include "include/capi/cef_auth_callback_capi.h"
#include "include/capi/cef_command_handler_capi.h"
#include "include/capi/cef_devtools_message_observer_capi.h"
#include "include/capi/cef_download_item_capi.h"
#include "include/capi/cef_drag_data_capi.h"
#include "include/capi/cef_i18n_util_capi.h"
#include "include/capi/cef_media_router_capi.h"
#include "include/capi/cef_menu_model_delegate_capi.h"
#include "include/capi/cef_navigation_entry_capi.h"
#include "include/capi/cef_origin_whitelist_capi.h"
#include "include/capi/cef_parser_capi.h"
#include "include/capi/cef_preference_capi.h"
#include "include/capi/cef_print_settings_capi.h"
#include "include/capi/cef_registration_capi.h"
#include "include/capi/cef_resource_bundle_capi.h"
#include "include/capi/cef_resource_bundle_handler_capi.h"
#include "include/capi/cef_server_capi.h"
#include "include/capi/cef_shared_memory_region_capi.h"
#include "include/capi/cef_shared_process_message_builder_capi.h"
#include "include/capi/cef_ssl_info_capi.h"
#include "include/capi/cef_ssl_status_capi.h"
#include "include/capi/cef_task_manager_capi.h"
#include "include/capi/cef_unresponsive_process_callback_capi.h"
#include "include/capi/cef_x509_certificate_capi.h"
#include "include/capi/cef_crash_util_capi.h"
"#.to_string()
}
