// src-tauri/src/config/mod.rs
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};

#[derive(RustEmbed)]
#[folder = "configs/"]
pub struct Configs;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResolutionConfig {
    #[serde(rename = "ResolutionSizeX")]
    pub width: u32,
    #[serde(rename = "ResolutionSizeY")]
    pub height: u32,
    #[serde(rename = "RefreshRate")]
    pub refresh_rate: u32,
}

/// 前端传输用的配置结构
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    #[serde(rename = "Desktop")]
    pub desktop: ResolutionConfig,
    #[serde(rename = "Game")]
    pub game: ResolutionConfig,
}

pub mod create;
pub mod parser;
pub mod writer;