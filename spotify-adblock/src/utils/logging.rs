use crate::config::DEBUG_MODE;
use std::sync::atomic::{AtomicUsize, Ordering};

// Log counters for monitoring
static DEBUG_COUNT: AtomicUsize = AtomicUsize::new(0);
static ALLOWED_COUNT: AtomicUsize = AtomicUsize::new(0);
static BLOCKED_COUNT: AtomicUsize = AtomicUsize::new(0);

// Maximum log line length for memory safety
const MAX_LOG_LENGTH: usize = 512;

/// Truncate message to prevent buffer overflows
fn truncate_message(message: &str) -> &str {
    if message.len() > MAX_LOG_LENGTH {
        &message[0..MAX_LOG_LENGTH]
    } else {
        message
    }
}

pub fn log_debug(message: &str) {
    if *DEBUG_MODE {
        DEBUG_COUNT.fetch_add(1, Ordering::Relaxed);
        println!("[DEBUG] {}", truncate_message(message));
    }
}

pub fn log_allowed(context: &str, method: &str, url: &str) {
    ALLOWED_COUNT.fetch_add(1, Ordering::Relaxed);
    println!("[+] {}: {} {}",
             truncate_message(context),
             truncate_message(method),
             truncate_message(url)
    );
}

pub fn log_blocked(context: &str, method: &str, url: &str) {
    BLOCKED_COUNT.fetch_add(1, Ordering::Relaxed);
    println!("[-] {}: {} {}",
             truncate_message(context),
             truncate_message(method),
             truncate_message(url)
    );
}

pub fn log_info(message: &str) {
    println!("[*] {}", truncate_message(message));
}

pub fn log_error(message: &str) {
    println!("[!] Error: {}", truncate_message(message));
}

/// Get statistics about logging activity
pub fn get_log_stats() -> (usize, usize, usize) {
    (
        DEBUG_COUNT.load(Ordering::Relaxed),
        ALLOWED_COUNT.load(Ordering::Relaxed),
        BLOCKED_COUNT.load(Ordering::Relaxed)
    )
}
