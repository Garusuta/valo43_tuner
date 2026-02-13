pub mod init {
    use tauri::State;

    use crate::{
        commands::config::save_config,
        configs::{app_config::AppConfig, app_state::AppState},
        utils::{
            command_manager::{get_running_process_path, run_command},
            display_manager::DisplayMode,
            watcher_manager::ProcessWatcher,
        },
    };

    #[tauri::command]
    pub async fn scan_game_path() -> Result<(), String> {
        let mut app_config = AppConfig::load_app_config().map_err(|e| e.to_string())?;
        app_config.valorant.launcher_path = get_running_process_path("无畏契约登录器.exe")
            .map(|p| p.to_string_lossy().into_owned());
        app_config.valorant.game_path = get_running_process_path("VALORANT.exe")
            .map(|p| p.parent().unwrap().to_string_lossy().into_owned());
        save_config(app_config)?;
        Ok(())
    }

    #[tauri::command]
    pub async fn create_preset_watcher(state: State<'_, AppState>) -> Result<(), String> {
        let mut app_config = AppConfig::load_app_config().map_err(|e| e.to_string())?;
        let watcher_config = &mut app_config.watcher;
        let valorant_config = &app_config.valorant;

        let mut watcher_guard = state.watcher.lock().await;
        *watcher_guard = Some(ProcessWatcher::new(
            valorant_config.launcher_path.clone().unwrap(),
            DisplayMode {
                width: watcher_config.width,
                height: watcher_config.height,
                refresh_rate: watcher_config.fps,
            },
        ));
        watcher_config.game_path = valorant_config.launcher_path.clone();
        save_config(app_config).map_err(|e| e.to_string())?;
        Ok(())
    }

    #[tauri::command]
    pub async fn start_game() -> Result<(), String> {
        let valorant_config = AppConfig::load_valrant_config().map_err(|e| e.to_string())?;
        let launcher_path = valorant_config.launcher_path.unwrap();
        run_command(&["start", "", launcher_path.as_str()]).map_err(|e| e.to_string())?;
        Ok(())
    }
    
    #[tauri::command]
    pub fn hide_windows_taskbar() -> Result<(), String> {
        Ok(())
    }
}

pub mod cfg {
    use std::path::Path;

    use tracing::info;

    use crate::{
        configs::app_config::AppConfig,
        games::valorant::{get_last_login_user_folder, modify_game_resolution_config},
        utils::command_manager::run_command,
    };

    #[tauri::command]
    pub fn modify_cfg_file() -> Result<(), String> {
        let config = AppConfig::load_valrant_config().map_err(|e| e.to_string())?;
        let user_name = get_last_login_user_folder().map_err(|e| e.to_string())?;
        let user_settings_path = Path::new(&config.game_path.clone().unwrap()).join(format!(
            "ShooterGame\\Saved\\Config\\{}\\WindowsClient\\GameUserSettings.ini",
            user_name
        ));
        let public_settings_path = Path::new(&config.game_path.clone().unwrap())
            .join("ShooterGame\\Saved\\Config\\WindowsClient\\GameUserSettings.ini");
        info!("Modifying settings file: {:?}", user_settings_path);
        modify_game_resolution_config(user_settings_path).map_err(|e| e.to_string())?;
        info!("Modifying settings file: {:?}", public_settings_path);
        modify_game_resolution_config(public_settings_path).map_err(|e| e.to_string())?;
        Ok(())
    }

    #[tauri::command]
    pub fn restore_file_pemission() -> Result<(), String> {
        let config = AppConfig::load_valrant_config().map_err(|e| e.to_string())?;
        let user_name = get_last_login_user_folder().map_err(|e| e.to_string())?;
        let user_settings_path = Path::new(&config.game_path.clone().unwrap()).join(format!(
            "ShooterGame\\Saved\\Config\\{}\\WindowsClient\\GameUserSettings.ini",
            user_name
        ));
        let public_settings_path = Path::new(&config.game_path.clone().unwrap())
            .join("ShooterGame\\Saved\\Config\\WindowsClient\\GameUserSettings.ini");
        run_command(&["attrib", "-R", user_settings_path.to_str().unwrap()])
            .map_err(|e| e.to_string())?;
        run_command(&["attrib", "-R", public_settings_path.to_str().unwrap()])
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
