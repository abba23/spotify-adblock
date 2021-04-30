mod cef_urlrequest_capi;

use cef_urlrequest_capi::{
    _cef_request_context_t, _cef_request_t, _cef_urlrequest_client_t, cef_string_userfree_utf16_free, cef_urlrequest_t,
};
use lazy_static::lazy_static;
use libc::{addrinfo, c_char, EAI_FAIL};
use redhook::{hook, real};
use regex::Regex;
use serde::Deserialize;
use std::ffi::CStr;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::ptr::null;
use std::slice::from_raw_parts;
use std::string::String;

#[derive(Deserialize)]
struct Config {
    allowlist: Vec<String>,
    denylist: Vec<String>,
}

lazy_static! {
    static ref CONFIG: Config = {
        let config_paths = vec![
            PathBuf::from("config.toml"),
            #[allow(deprecated)] // std::env::home_dir() is only broken on Windows
            std::env::home_dir().unwrap().join(".config/spotify-adblock/config.toml"),
            PathBuf::from("/etc/spotify-adblock/config.toml"),
        ];

        if let Some(path) = config_paths.into_iter().find(|path| path.exists()) {
            println!("[*] Config file: {}", path.to_str().unwrap());
            match read_to_string(path) {
                Ok(config_string) => match toml::from_str(&config_string) {
                    Ok(config) => {
                        return config;
                    }
                    Err(error) => {
                        println!("[*] Error: Parse config file ({})", error);
                    }
                },
                Err(error) => {
                    println!("[*] Error: Read config file ({})", error);
                }
            }
        } else {
            println!("[*] Error: No config file");
        };
        Config {
            allowlist: Vec::new(),
            denylist: Vec::new(),
        }
    };
}

fn listed(element: &str, regex_list: &Vec<String>) -> bool {
    for regex_string in regex_list {
        // TODO: only generate each regex once outside of loop
        match Regex::new(&regex_string) {
            Ok(regex) => {
                if regex.is_match(element) {
                    return true;
                }
            }
            Err(error) => {
                println!("[*] Warning: Invalid regex ({})", error);
            }
        }
    }
    false
}

hook! {
    unsafe fn getaddrinfo(node: *const c_char, service: *const c_char, hints: *const addrinfo, res: *const *const addrinfo) -> i32 => _getaddrinfo {
        let domain = CStr::from_ptr(node).to_str().unwrap();

        if listed(domain, &CONFIG.allowlist) {
            println!("[+] getaddrinfo:\t\t {}", domain);
            real!(getaddrinfo)(node, service, hints, res)
        } else {
            println!("[-] getaddrinfo:\t\t {}", domain);
            EAI_FAIL
        }
    }
}

hook! {
    unsafe fn cef_urlrequest_create(request: *mut _cef_request_t, client: *const _cef_urlrequest_client_t, request_context: *const _cef_request_context_t) -> *const cef_urlrequest_t => _cef_urlrequest_create {
        let url_cef = (*request).get_url.unwrap()(request);
        let url_utf16 = from_raw_parts((*url_cef).str_, (*url_cef).length as usize);
        let url = String::from_utf16(url_utf16).unwrap();
        cef_string_userfree_utf16_free(url_cef);

        if listed(&url, &CONFIG.denylist) {
            println!("[-] cef_urlrequest_create:\t {}", url);
            null()
        } else {
            println!("[+] cef_urlrequest_create:\t {}", url);
            real!(cef_urlrequest_create)(request, client, request_context)
        }
    }
}
