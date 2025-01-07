use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to get config directory")]
    NoConfigDir,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub token: String,
    #[serde(default)]
    pub session_token: String,
    #[serde(default)]
    pub session_expires: Option<String>,
}

impl Config {
    pub fn config_path() -> Result<PathBuf, ConfigError> {
        let proj_dirs = ProjectDirs::from("com", "blinko", "note-cli")
            .ok_or(ConfigError::NoConfigDir)?;
        let config_dir = proj_dirs.config_dir();
        fs::create_dir_all(config_dir)?;
        Ok(config_dir.join("config.json"))
    }

    pub fn load() -> Result<Self, ConfigError> {
        let config_path = Self::config_path()?;
        if !config_path.exists() {
            return Ok(Config {
                token: String::new(),
                session_token: String::new(),
                session_expires: None,
            });
        }
        let config_str = fs::read_to_string(config_path)?;
        Ok(serde_json::from_str(&config_str)?)
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let config_path = Self::config_path()?;
        let config_str = serde_json::to_string_pretty(self)?;
        fs::write(config_path, config_str)?;
        Ok(())
    }

    pub fn is_session_valid(&self) -> bool {
        !self.session_token.is_empty()
        // You could add expiry checking here if needed
    }
} 