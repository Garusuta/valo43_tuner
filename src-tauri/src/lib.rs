mod commands;
mod config;
mod constant;
mod display;
mod monitor;
mod state;
mod valorant;

use commands::{
    get_config,
    get_watching_status,
    save_config,
    start_watching,
    stop_watching,
    toggle_watching,
    is_elevated
};
use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::new())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // 配置命令
            get_config,
            save_config,
            // 监听命令
            start_watching,
            stop_watching,
            get_watching_status,
            toggle_watching,
            // 管理员权限检测
            is_elevated
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}