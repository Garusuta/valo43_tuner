use crate::configs::app_state::AppState;

pub mod configs;
pub mod games;
pub mod utils;

mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState::new())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // 配置命令
            commands::config::load_config,
            commands::config::save_config,
            commands::config::reset_config,
            // 监听命令
            commands::watcher::toggle_watching,
            commands::watcher::get_watching_status,
            commands::watcher::get_gaming_status,
            // 无畏契约
            commands::valorant::init::scan_game_path,
            commands::valorant::init::create_preset_watcher,
            commands::valorant::init::start_game,
            commands::valorant::init::hide_windows_taskbar,
            commands::valorant::cfg::modify_cfg_file,
            commands::valorant::cfg::restore_file_pemission,
            // 显示
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
