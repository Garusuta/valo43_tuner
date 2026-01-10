use crate::{config::ResolutionConfig, constant::CONFIG_PATH};
use std::{collections::HashMap, error::Error, fs::read_to_string};
use toml::from_str;

pub fn parser_config() -> Result<HashMap<String,ResolutionConfig>, Box<dyn Error>> {
    let config_content = read_to_string(&*CONFIG_PATH).expect("Failed to read config file");
    let resolution_config = from_str::<HashMap<String,ResolutionConfig>>(&config_content)?;
    Ok(resolution_config)
}
