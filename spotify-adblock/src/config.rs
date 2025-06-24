use lazy_static::lazy_static;
use regex::RegexSet;
use serde::Deserialize;
use std::{env, fs::read_to_string, path::PathBuf};

// Constants for fault containment
const MAX_CONFIG_SIZE: usize = 1024 * 1024; // 1MB limit for config

/// Debug mode enabled via environment variable
lazy_static! {
    pub static ref DEBUG_MODE: bool = env::var("SPOTIFY_ADBLOCK_DEBUG").is_ok();
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(with = "serde_regex")]
    pub allowlist: RegexSet,
    #[serde(with = "serde_regex")]
    pub denylist: RegexSet,
}

lazy_static! {
    pub static ref CONFIG: Config = load_config();
}

/// Load configuration from multiple potential locations with fault tolerance
fn load_config() -> Config {
    let config_paths = vec![
        PathBuf::from("config.toml"),
        match env::var("XDG_CONFIG_HOME") {
            Ok(xdg_config_home) => PathBuf::from(xdg_config_home),
            #[allow(deprecated)] // std::env::home_dir() is only broken on Windows
            Err(_) => env::home_dir().unwrap_or_default().join(".config")
        }.join("spotify-adblock/config.toml"),
        PathBuf::from("/etc/spotify-adblock/config.toml"),
    ];

    if let Some(path) = config_paths.into_iter().find(|path| path.exists()) {
        println!("[*] Config file: {}", path.to_str().unwrap_or("(invalid path)"));
        match read_to_string(&path) {
            Ok(config_string) if config_string.len() <= MAX_CONFIG_SIZE => {
                match toml::from_str(&config_string) {
                    Ok(config) => {
                        return config;
                    }
                    Err(error) => {
                        println!("[*] Error: Parse config file ({error})");
                    }
                }
            },
            Ok(_) => println!("[*] Error: Config file too large (exceeds {MAX_CONFIG_SIZE} bytes)"),
            Err(error) => {
                println!("[*] Error: Read config file ({error})");
            }
        }
    } else {
        println!("[*] Error: No config file found");
    }

    // Default empty configuration - safe fallback
    Config {
        allowlist: RegexSet::empty(),
        denylist: RegexSet::empty(),
    }
}
