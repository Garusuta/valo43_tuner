use std::collections::HashMap;

use tauri::State;
use tracing::{debug, info};

use crate::{
    configs::{app_config::AppConfig, app_state::AppState},
    utils::{
        display_manager::{enumerate_monitors, DisplayMode},
        watcher_manager::ProcessWatcher,
    },
};

#[tauri::command]
pub async fn scan_monitors(state: State<'_, AppState>) -> Result<(), String> {
    let mut monitors_guard = state.monitors.lock().await;
    *monitors_guard = enumerate_monitors();
    Ok(())
}

#[tauri::command]
pub async fn get_monitors_map(
    state: State<'_, AppState>,
) -> Result<HashMap<String, String>, String> {
    let monitors_guard = state.monitors.lock().await;
    let monitors_map = monitors_guard.clone();
    Ok(monitors_map)
}

#[tauri::command]
pub async fn select_monitor(
    state: State<'_, AppState>,
    monitor_name: String,
) -> Result<(), String> {
    info!("You have selected {}.", monitor_name);
    let mut watcher_guard = state.watcher.lock().await;
    if let Some(watcher_instance) = watcher_guard.as_mut() {
        debug!("Apply the {} display name to AppState", monitor_name);
        watcher_instance.display_mode.monitor_name = monitor_name;
    } else {
        let watcher_config = AppConfig::load_watcher_config().map_err(|e| e.to_string())?;
        *watcher_guard = Some(ProcessWatcher::new(
            watcher_config.game_path.unwrap(),
            DisplayMode {
                width: watcher_config.width,
                height: watcher_config.height,
                refresh_rate: watcher_config.fps,
                monitor_name: monitor_name,
                ..Default::default()
            },
        ));
    }
    Ok(())
}
