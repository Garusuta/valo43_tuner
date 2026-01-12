use crate::{config::AppConfig, constant::CONFIG_PATH};
use std::{error::Error, fs};

/// 保存配置到文件
pub fn write_config(config: &AppConfig) -> Result<(), Box<dyn Error>> {
    let content = format!(
        r#"[Desktop]
ResolutionSizeX = {}
ResolutionSizeY = {}
RefreshRate = {}

[Game]
ResolutionSizeX = {}
ResolutionSizeY = {}
RefreshRate = {}
"#,
        config.desktop.width,
        config.desktop.height,
        config.desktop.refresh_rate,
        config.game.width,
        config.game.height,
        config.game.refresh_rate,
    );

    fs::write(&*CONFIG_PATH, content)?;
    println!("Config saved to: {}", CONFIG_PATH.display());
    Ok(())
}
