use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};


#[derive(RustEmbed)]
#[folder = "configs/"]
struct Configs;

#[derive(Debug, Deserialize, Serialize)]
pub struct ResolutionConfig {
    #[serde(rename = "ResolutionSizeX")]
    pub width: u32,
    #[serde(rename = "ResolutionSizeY")]
    pub height: u32,
    #[serde(rename = "RefreshRate")]
    pub refresh_rate: u32,
}

pub mod create;
pub mod parser;