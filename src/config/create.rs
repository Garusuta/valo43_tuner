use crate::{config::Configs, constant::CONFIG_PATH};


pub fn create_config() {
    if !CONFIG_PATH.exists() {
        Configs::get("config.toml")
            .map(|file| {
                std::fs::write(&*CONFIG_PATH, file.data).expect("Failed to write config file");
                println!("Config file created at: {}", CONFIG_PATH.display());
            })
            .unwrap_or_else(|| {
                eprintln!("Failed to find the embedded config file.");
            });
    }
}
