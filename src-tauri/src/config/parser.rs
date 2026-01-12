use crate::{
    config::{AppConfig, ResolutionConfig},
    constant::CONFIG_PATH,
};
use std::{collections::HashMap, error::Error, fs::read_to_string};
use toml::from_str;

/// 解析配置为 HashMap（保持原有功能）
pub fn parser_config() -> Result<HashMap<String, ResolutionConfig>, Box<dyn Error>> {
    let config_content = read_to_string(&*CONFIG_PATH)?;
    let resolution_config = from_str::<HashMap<String, ResolutionConfig>>(&config_content)?;
    Ok(resolution_config)
}

/// 解析配置为 AppConfig（给前端用）
pub fn parser_app_config() -> Result<AppConfig, Box<dyn Error>> {
    let config_content = read_to_string(&*CONFIG_PATH)?;
    let config = from_str::<AppConfig>(&config_content)?;
    Ok(config)
}