use crate::tmux::Window;
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
#[serde(untagged)]
pub enum WindowConfig {
    Simple(String),
    Detailed {
        name: Option<String>,
        working_dir: Option<String>,
        cmd: String,
    },
}

impl Into<Window> for WindowConfig {
    fn into(self) -> Window {
        match self {
            WindowConfig::Simple(cmd) => Window {
                name: None,
                working_dir: None,
                cmd,
            },
            WindowConfig::Detailed {
                name,
                working_dir,
                cmd,
            } => Window {
                name,
                working_dir,
                cmd,
            },
        }
    }
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
