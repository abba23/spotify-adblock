mod cef;

use cef::{
    _cef_request_context_t, _cef_request_t, _cef_urlrequest_client_t, cef_string_userfree_utf16_free, cef_urlrequest_t,
};
use lazy_static::lazy_static;
use libc::{addrinfo, c_char, dlsym, EAI_FAIL, RTLD_NEXT};
use regex::Regex;
use serde::Deserialize;
use std::ffi::CStr;
use std::fs::read_to_string;
use std::mem;
use std::path::PathBuf;
use std::ptr::null;
use std::slice::from_raw_parts;
use std::string::String;

macro_rules! hook {
    ($function_name:ident($($parameter_name:ident: $parameter_type:ty),*) -> $return_type:ty => $new_function_name:ident $body:block) => {
        lazy_static! {
            static ref $new_function_name: fn($($parameter_type),*) -> $return_type = unsafe {
                let function_name = CStr::from_bytes_with_nul(concat!(stringify!($function_name), "\0").as_bytes()).unwrap();
                let function_pointer = dlsym(RTLD_NEXT, function_name.as_ptr());
                if function_pointer.is_null() {
                    panic!("[*] Error: Unable to find function \"{}\"", stringify!($function_name));
                }
                mem::transmute(function_pointer)
            };
        }

        #[no_mangle]
        pub unsafe extern "C" fn $function_name($($parameter_name: $parameter_type),*) -> $return_type {
            $body
        }
    }
}

mod cfg {
    pub struct Config {
        pub allowlist: Vec<Regex>,
        pub denylist: Vec<Regex>,
    }

    impl<'de> Deserialize<'de> for Config {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as serde::Deserializer<'de>>::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let wrapped_cfg = WrappedConfig::deserialize(deserializer)?;
            Ok(Config {
                allowlist: wrapped_cfg
                    .allowlist
                    .into_iter()
                    .map(|RegexWrapper(r)| -> Regex { r })
                    .collect(),
                denylist: wrapped_cfg
                    .denylist
                    .into_iter()
                    .map(|RegexWrapper(r)| -> Regex { r })
                    .collect(),
            })
        }
    }

    use crate::Deserialize;
    use crate::Regex;

    struct RegexWrapper(Regex);

    #[derive(Deserialize)]
    struct WrappedConfig {
        allowlist: Vec<RegexWrapper>,
        denylist: Vec<RegexWrapper>,
    }

    impl<'de> Deserialize<'de> for RegexWrapper {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as serde::Deserializer<'de>>::Error>
        where
            D: serde::Deserializer<'de>,
        {
            match Regex::new(String::deserialize(deserializer)?.as_str()) {
                Ok(regex) => Ok(RegexWrapper(regex)),
                Err(e) => todo!(),
            }
        }
    }
} // mod cfg

lazy_static! {
    static ref CONFIG: cfg::Config = {
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
        cfg::Config {
            allowlist: Vec::new(),
            denylist: Vec::new(),
        }
    };
}

hook! {
    getaddrinfo(node: *const c_char, service: *const c_char, hints: *const addrinfo, res: *const *const addrinfo) -> i32 => REAL_GETADDRINFO {
        let domain = CStr::from_ptr(node).to_str().unwrap();

        if listed(domain, &CONFIG.allowlist) {
            println!("[+] getaddrinfo:\t\t {}", domain);
            REAL_GETADDRINFO(node, service, hints, res)
        } else {
            println!("[-] getaddrinfo:\t\t {}", domain);
            EAI_FAIL
        }
    }
}

hook! {
    cef_urlrequest_create(request: *mut _cef_request_t, client: *const _cef_urlrequest_client_t, request_context: *const _cef_request_context_t) -> *const cef_urlrequest_t => REAL_CEF_URLREQUEST_CREATE {
        let url_cef = (*request).get_url.unwrap()(request);
        let url_utf16 = from_raw_parts((*url_cef).str_, (*url_cef).length as usize);
        let url = String::from_utf16(url_utf16).unwrap();
        cef_string_userfree_utf16_free(url_cef);

        if listed(&url, &CONFIG.denylist) {
            println!("[-] cef_urlrequest_create:\t {}", url);
            null()
        } else {
            println!("[+] cef_urlrequest_create:\t {}", url);
            REAL_CEF_URLREQUEST_CREATE(request, client, request_context)
        }
    }
}

fn listed(element: &str, regex_list: &Vec<Regex>) -> bool {
    regex_list.iter().any(|regex| -> bool { regex.is_match(element) })
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn it_works() {
        toml::from_str::<cfg::Config>(read_to_string("config.toml").unwrap().as_str()).unwrap();
    }
}
