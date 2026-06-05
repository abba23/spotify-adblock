use log::info;
use regex::RegexSet;
use serde::Deserialize;
use std::{path::Path, sync::LazyLock};
use std::path::PathBuf;

pub static CONFIG: LazyLock<Config> = LazyLock::new(read_config);

const RELATIVE_CONFIG_DIR: &str = "spotify-adblock";
const CONFIG_NAME: &str = "config.toml";
const GLOBAL_CONFIG_DIR: &str = "/etc";

fn read_config() -> Config {
    let config_path = determine_config_path();
    info!("Found config: {config_path:?}.");

    let config_string = std::fs::read_to_string(&config_path)
        .unwrap_or_else(|error| panic!("Failed to read config {config_path:?}: {error}."));
    toml::from_str(&config_string)
        .unwrap_or_else(|error| panic!("Failed to parse config {config_path:?}: {error}."))
}

fn determine_config_path() -> PathBuf {
    let relative_config_path = Path::new(RELATIVE_CONFIG_DIR).join(CONFIG_NAME);
    dirs::config_dir()
        .map(|xdg_config_dir| xdg_config_dir.join(&relative_config_path))
        .filter(|path| path.exists())
        .unwrap_or(Path::new(GLOBAL_CONFIG_DIR).join(&relative_config_path))
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(with = "serde_regex")]
    pub allowlist: RegexSet,
    #[serde(with = "serde_regex")]
    pub denylist: RegexSet,
}
