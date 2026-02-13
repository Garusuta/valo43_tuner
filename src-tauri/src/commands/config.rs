use std::fs;

use crate::{
    configs::app_config::{AppConfig, EmbedConfigs}, utils::constant_manager::CONFIG_FILE,
};

#[tauri::command]
pub fn load_config() -> Result<AppConfig, String> {
    let app_config = AppConfig::load_app_config().map_err(|e| e.to_string())?;
    Ok(app_config)
}

#[tauri::command]
pub fn save_config(app_config: AppConfig) -> Result<(), String> {
    app_config.save_to_local().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn reset_config() -> Result<(), String> {
    let content = EmbedConfigs::get("config.toml").unwrap();
    fs::write(CONFIG_FILE.as_path(), content.data.as_ref()).map_err(|e| e.to_string())?;
    Ok(())
}
