use std::sync::Arc;
use tauri::State;

use crate::{
    config::{AppConfig, create::create_config, parser::{parser_app_config, parser_config}, writer::write_config},
    display::display::change_display_mode,
    monitor::monitor::{disable_monitor, enable_monitor, scan_monitors},
    state::AppState,
    valorant::watch_valorant,
};

// ==================== 监听相关命令 ====================

#[tauri::command]
pub async fn start_watching(state: State<'_, AppState>) -> Result<(), String> {
    // 检查是否已经在监听
    let mut task_guard = state.watch_task.lock().await;
    
    if task_guard.is_some() {
        return Err("Already watching".to_string());
    }

    create_config();
    
    let monitors_arc =Arc::new(scan_monitors().expect("Wtf? No monitors found?"));
    let monitors_for_start = Arc::clone(&monitors_arc);
    let monitors_for_stop = Arc::clone(&monitors_arc);

    let resolution_config_arc = Arc::new(
        parser_config().map_err(|e| format!("Failed to parse config: {:?}", e))?
    );
    
    let resolution_config_for_start = Arc::clone(&resolution_config_arc);
    let resolution_config_for_stop = Arc::clone(&resolution_config_arc);

    let on_start = move || {
        println!("检测到 Valorant 启动，禁用显示器...");
        for monitor in monitors_for_start.iter() {
            if monitor.status == "Started" {
                disable_monitor(&monitor.instance_id);
            }
        }
        
        let game_config = resolution_config_for_start
            .get("Game")
            .expect("No game config found");
        let result = change_display_mode(
            game_config.width,
            game_config.height,
            game_config.refresh_rate,
            true,
        );
        println!("更改分辨率结果: {:?}", result);
    };

    let on_stop = move || {
        println!("检测到 Valorant 关闭，启用显示器...");
        for monitor in monitors_for_stop.iter() {
            if monitor.status == "Started" {
                enable_monitor(&monitor.instance_id);
            }
        }
        
        let desktop_config = resolution_config_for_stop
            .get("Desktop")
            .expect("No desktop config found");
        let result = change_display_mode(
            desktop_config.width,
            desktop_config.height,
            desktop_config.refresh_rate,
            true,
        );
        println!("更改分辨率结果: {:?}", result);
    };

    let task = tokio::spawn(async move {
        watch_valorant(on_start, on_stop).await;
    });
    
    *task_guard = Some(task);
    
    println!("监听已启动");
    Ok(())
}

#[tauri::command]
pub async fn stop_watching(state: State<'_, AppState>) -> Result<(), String> {
    let mut task_guard = state.watch_task.lock().await;
    
    if let Some(task) = task_guard.take() {
        task.abort();
        println!("监听已停止");
        Ok(())
    } else {
        Err("Not watching".to_string())
    }
}

#[tauri::command]
pub async fn get_watching_status(state: State<'_, AppState>) -> Result<bool, String> {
    let task_guard = state.watch_task.lock().await;
    Ok(task_guard.is_some())
}

#[tauri::command]
pub async fn toggle_watching(state: State<'_, AppState>) -> Result<bool, String> {
    let is_watching = {
        let task_guard = state.watch_task.lock().await;
        task_guard.is_some()
    };
    
    if is_watching {
        stop_watching(state).await?;
        Ok(false)
    } else {
        start_watching(state).await?;
        Ok(true)
    }
}

// ==================== 配置相关命令 ====================

#[tauri::command]
pub async fn get_config() -> Result<AppConfig, String> {
    create_config();
    parser_app_config().map_err(|e| format!("Failed to get config: {}", e))
}

#[tauri::command]
pub async fn save_config(config: AppConfig) -> Result<(), String> {
    write_config(&config).map_err(|e| format!("Failed to save config: {}", e))
}

/// 检测当前进程是否以管理员权限运行

#[tauri::command]
pub fn is_elevated() -> bool {
    #[cfg(target_os = "windows")]
    {
        use std::mem;
        use windows::Win32::Foundation::{HANDLE, CloseHandle};
        use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
        use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

        unsafe {
            let mut token_handle: HANDLE = HANDLE::default();
            
            if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle).is_err() {
                return false;
            }

            let mut elevation: TOKEN_ELEVATION = mem::zeroed();
            let mut size: u32 = mem::size_of::<TOKEN_ELEVATION>() as u32;

            let result = GetTokenInformation(
                token_handle,
                TokenElevation,
                Some(&mut elevation as *mut _ as *mut _),
                size,
                &mut size,
            );

            let _ = CloseHandle(token_handle);

            result.is_ok() && elevation.TokenIsElevated != 0
        }
    }
}