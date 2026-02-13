use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs};
use toml;
use tracing::info;

use crate::utils::constant_manager::CONFIG_FILE;

#[derive(RustEmbed)]
#[folder = "configs/"]
pub struct EmbedConfigs;

impl EmbedConfigs {
    pub fn init() -> Result<(), Box<dyn Error>> {
        if !CONFIG_FILE.exists() {
            let content = EmbedConfigs::get("config.toml").unwrap();
            fs::write(CONFIG_FILE.as_path(), content.data.as_ref())?;
        }
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppConfig {
    #[serde(rename = "Watcher")]
    pub watcher: WatcherConfig,
    #[serde(rename = "Valorant")]
    pub valorant: ValorantConfig,
    #[serde(rename = "Development")]
    pub development: DevelopmentConfig,
}

impl AppConfig {
    pub fn load_app_config() -> Result<AppConfig, Box<dyn Error>> {
        info!("Loading app configuration.");
        let config_content = fs::read_to_string(CONFIG_FILE.as_path())?;
        let app_config = toml::from_str::<AppConfig>(config_content.as_str())?;
        Ok(app_config)
    }

    pub fn load_valrant_config() -> Result<ValorantConfig, Box<dyn Error>> {
        info!("Loading valorant configuration.");
        let config_content = fs::read_to_string(CONFIG_FILE.as_path())?;
        let app_config = toml::from_str::<AppConfig>(config_content.as_str())?;
        let valorant_config = app_config.valorant;
        Ok(valorant_config)
    }

    pub fn load_watcher_config() -> Result<WatcherConfig, Box<dyn Error>> {
        info!("Loading watcher configuration.");
        let config_content = fs::read_to_string(CONFIG_FILE.as_path())?;
        let app_config = toml::from_str::<AppConfig>(config_content.as_str())?;
        let watcher_config = app_config.watcher;
        Ok(watcher_config)
    }

    pub fn save_to_local(&self) -> Result<(), Box<dyn Error>> {
        info!("Saving to local storage");
        let updated = toml::to_string_pretty(self)?;
        fs::write(CONFIG_FILE.as_path(), updated)?;
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WatcherConfig {
    #[serde(rename = "GamePath")]
    pub game_path: Option<String>,
    #[serde(rename = "Width")]
    pub width: u32,
    #[serde(rename = "Height")]
    pub height: u32,
    #[serde(rename = "Fps")]
    pub fps: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ValorantConfig {
    #[serde(rename = "LauncherPath")]
    pub launcher_path: Option<String>,
    #[serde(rename = "GamePath")]
    pub game_path: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DevelopmentConfig {
    #[serde(rename = "Debug")]
    debug: bool,
}
