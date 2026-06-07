use log::info;
use regex::RegexSet;
use serde::Deserialize;
use std::{fs, path::PathBuf, sync::LazyLock};

pub static CONFIG: LazyLock<Config> = LazyLock::new(load_config);

#[derive(Deserialize)]
pub struct Config {
    #[serde(with = "serde_regex")]
    pub allowlist: RegexSet,
    #[serde(with = "serde_regex")]
    pub denylist: RegexSet,
}

const RELATIVE_CONFIG_DIR: &str = "spotify-adblock";
const CONFIG_NAME: &str = "config.toml";
const GLOBAL_CONFIG_DIR: &str = "/etc";

fn load_config() -> Config {
    let config_path = find_config().expect("Failed to find config.");
    info!("Found config: {}.", config_path.display());

    let config_string = fs::read_to_string(&config_path)
        .unwrap_or_else(|error| panic!("Failed to read config {}: {error}.", config_path.display()));
    toml::from_str(&config_string)
        .unwrap_or_else(|error| panic!("Failed to parse config {}: {error}.", config_path.display()))
}

fn find_config() -> Option<PathBuf> {
    let relative_config_path = PathBuf::from(RELATIVE_CONFIG_DIR).join(CONFIG_NAME);
    [dirs::config_dir(), Some(PathBuf::from(GLOBAL_CONFIG_DIR))]
        .into_iter()
        .flatten()
        .map(|config_dir| config_dir.join(&relative_config_path))
        .find(|config_path| config_path.exists())
}
