use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub sessions: Vec<SessionConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionConfig {
    pub name: String,
    pub windows: Vec<WindowConfig>,
}

// TODO: build from String, see https://serde.rs/string-or-struct.html
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WindowConfig {
    pub name: Option<String>,
    pub working_dir: Option<String>,
    pub cmd: String,
}

pub fn get_config(config_path: &str, name: &str) -> Result<SessionConfig> {
    let config_path: &str = &shellexpand::tilde(config_path);
    let config = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config)?;

    config
        .sessions
        .iter()
        .filter(|s| s.name == name)
        .nth(0)
        .map(|c| c.clone())
        .ok_or(anyhow!("No configuration provided for session '{}'", name))
}
