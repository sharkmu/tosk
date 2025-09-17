use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
struct Config {
    archive_enabled: bool,
}

pub fn load() -> bool {
    let path: PathBuf = config_dir()
        .unwrap()
        .join("tosk/Config.toml");

    match fs::read_to_string(&path) {
        Ok(content) => set_config(&content),
        Err(_) => create_config(&path),
    }
}

fn set_config(content: &str) -> bool {
    let config: Config = toml::from_str(content)
        .expect("Invalid TOML format");
    config.archive_enabled
}

fn create_config(path: &PathBuf) -> bool {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).ok();
    }

    let default = Config {
        archive_enabled: true,
    };

    let toml_str = toml::to_string_pretty(&default).unwrap();
    fs::write(path, toml_str).expect("Failed to write default config");

    default.archive_enabled
}
