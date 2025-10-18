use cef_sys::{_cef_request_context_t, _cef_request_t, _cef_urlrequest_client_t, cef_urlrequest_t};
use crate::config::{CONFIG, DEBUG_MODE};
use lazy_static::lazy_static;
use std::{ptr::null, slice::from_raw_parts, string::String};

use crate::hook;
use crate::hooks::memory::cef_string_userfree_utf16_free;
use crate::utils::logging;

// Constants for fault containment
const MAX_URL_LENGTH: usize = 2048;

/// URL classification with bounded execution and radiation hardening
struct UrlClassification {
    is_discord_rpc: bool,
    is_gabo: bool,
    is_dealer: bool,
    is_ad_related: bool,
    is_product_state: bool,
}

/// Fault-contained URL classifier with bounded execution
fn classify_url(url: &str) -> UrlClassification {
    // Ensure URL is within reasonable bounds (fault containment)
    let url = if url.len() > MAX_URL_LENGTH {
        &url[0..MAX_URL_LENGTH]
    } else {
        url
    };

    UrlClassification {
        is_discord_rpc: url.contains("discord") ||
            url.contains("discordapp") ||
            url.contains("presence") ||
            url.contains("/presence2/") ||
            url.contains("connect-state") ||
            url.contains("rpc"),

        // Gabo service - ONLY allow non-ad events
        is_gabo: url.contains("gabo-receiver-service") &&
                 !url.contains("/advertisement") &&
                 !url.contains("/ad-opportunity") &&
                 !url.contains("/adlogic") &&
                 !url.contains("/ads"),

        is_dealer: url.contains("dealer"),

        // Product state monitoring (for premium checks)
        is_product_state: url.contains("product_state") || url.contains("product-state"),

        // COMPREHENSIVE ad detection criteria
        is_ad_related:
            // Core ad endpoints
            url.contains("/ads/") ||
            url.contains("ad-logic") ||
            url.contains("adlogic") ||

            // CRITICAL: Track classification marker
            url.contains("injected-ad") ||

            // Third-party ad networks
            url.contains("doubleclick") ||
            url.contains("googleads") ||
            url.contains("adswizz") ||

            // Analytics and tracking
            url.contains("analytics") ||
            (url.contains("clientsettings") && url.contains("api")) ||
            (url.contains("track") && url.contains("event")) ||

            // Sponsored/Promoted content endpoints
            url.contains("sponsor") ||
            url.contains("/promotion/") ||
            url.contains("spotify:promotion:") ||
            url.contains("/partner/") ||
            url.contains("spotify:partner:") ||
            url.contains("partnership") ||
            url.contains("promoted") ||

            // Display ads and companion content
            url.contains("companion-ad") ||
            url.contains("companion_content") ||
            url.contains("companion-content") ||
            url.contains("canvas_ad") ||
            url.contains("canvas-ad") ||
            url.contains("/figs/") ||

            // Skip limit enforcement
            url.contains("RemainingSkipsRequest") ||
            url.contains("RemainingSkipsResponse") ||
            url.contains("skip-limit") ||
            url.contains("skip_limit") ||

            // Display segments (sponsored playlist banners)
            url.contains("display-segments") ||
            url.contains("display_segments") ||
            url.contains("DisplaySegments") ||

            // Gabo ad events (but not all gabo)
            (url.contains("gabo-receiver-service") && (
                url.contains("/advertisement") ||
                url.contains("/ad-opportunity") ||
                url.contains("/adlogic") ||
                url.contains("/ads")
            )) ||

            // Misc ad-related
            url.contains("brand") ||
            url.contains("whatsapp") ||
            url.contains("hpto") ||
            url.contains("takeover")
    }
}

hook! {
    cef_urlrequest_create(request: *mut _cef_request_t, client: *const _cef_urlrequest_client_t, request_context: *const _cef_request_context_t) -> *const cef_urlrequest_t => REAL_CEF_URLREQUEST_CREATE {
        // Validate input pointers
        if request.is_null() {
            logging::log_error("Null request pointer in cef_urlrequest_create");
            return null();
        }

        // Extract URL with safety checks
        let url_cef = unsafe {
            if let Some(get_url) = (*request).get_url { get_url(request) } else {
                logging::log_error("Missing get_url function in request");
                return REAL_CEF_URLREQUEST_CREATE(request, client, request_context);
            }
        };

        if url_cef.is_null() {
            return REAL_CEF_URLREQUEST_CREATE(request, client, request_context);
        }

        // Safely extract URL and method strings
        let url_utf16 = unsafe { from_raw_parts((*url_cef).str_, (*url_cef).length as usize) };
        let url = String::from_utf16(url_utf16).unwrap_or_else(|_| String::new());

        let method_cef = unsafe { (*request).get_method.unwrap()(request) };
        let method_utf16 = unsafe { from_raw_parts((*method_cef).str_, (*method_cef).length) };
        let method = String::from_utf16(method_utf16).unwrap_or_else(|_| String::from("UNKNOWN"));
        cef_string_userfree_utf16_free(method_cef);

        // Classify URL using fault-contained function
        let classification = classify_url(&url);

        // Debug mode handling
        if *DEBUG_MODE {
            logging::log_debug(&format!("{method} {url}"));
            let result = REAL_CEF_URLREQUEST_CREATE(request, client, request_context);
            cef_string_userfree_utf16_free(url_cef);
            return result;
        }

        // Decision logic with proper cleanup in all paths

        // Monitor product state checks (informational)
        if classification.is_product_state {
            logging::log_debug(&format!("⚠️  PRODUCT STATE CHECK: {} {}", method, url));
        }

        if classification.is_discord_rpc {
            logging::log_allowed("DISCORD RPC", &method, &url);
            let result = REAL_CEF_URLREQUEST_CREATE(request, client, request_context);
            cef_string_userfree_utf16_free(url_cef);
            return result;
        } else if classification.is_gabo || classification.is_dealer {
            logging::log_allowed("SERVICE", &method, &url);
            let result = REAL_CEF_URLREQUEST_CREATE(request, client, request_context);
            cef_string_userfree_utf16_free(url_cef);
            return result;
        }

        if classification.is_ad_related {
            logging::log_blocked("BLOCKED AD", &method, &url);
            // No response capturing for now to avoid segfaults
            cef_string_userfree_utf16_free(url_cef);
            return null();
        }

        let result = if CONFIG.denylist.is_match(&url) {
            logging::log_blocked("BLOCKED CONFIG", &method, &url);
            null()
        } else {
            logging::log_allowed("ALLOWED", &method, &url);
            REAL_CEF_URLREQUEST_CREATE(request, client, request_context)
        };

        cef_string_userfree_utf16_free(url_cef);
        result
    }
}
