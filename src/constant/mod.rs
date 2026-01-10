use std::{env, path::PathBuf, sync::LazyLock};

pub static CURRENT_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(&env::current_exe().unwrap())
        .parent()
        .unwrap()
        .to_path_buf()
});
pub static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| CURRENT_DIR.join("config.toml"));