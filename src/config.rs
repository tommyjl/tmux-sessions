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
    pub windows: Vec<String>,
}

pub fn get_config(name: &str) -> Result<SessionConfig> {
    let config = fs::read_to_string("example.toml")?; // TODO: Get from ~/.config/tsesh/tsesh.toml
    let config: Config = toml::from_str(&config)?;

    config
        .sessions
        .iter()
        .filter(|s| s.name == name)
        .nth(0)
        .map(|c| c.clone())
        .ok_or(anyhow!("No configuration provided for session '{}'", name))
}
