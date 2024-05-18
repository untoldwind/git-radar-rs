use anyhow::Result;
use std::fs;

pub mod types;

pub fn get_app_config() -> Result<types::Config> {
    if let Some(config_dir) = dirs::config_dir() {
        let config_file = config_dir.join("git-radar-rs").join("config.toml");
        if config_file.exists() {
            let content = fs::read_to_string(config_file)?;
            return Ok(toml::from_str(&content)?);
        }
    }
    Ok(Default::default())
}
