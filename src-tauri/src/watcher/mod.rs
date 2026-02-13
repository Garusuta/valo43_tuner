use std::{path::Path, sync::{
    Arc, atomic::{AtomicBool, Ordering}
}};

use sysinfo::{ProcessesToUpdate, System};
use tauri::State;
use tokio::{
    sync::Mutex,
    task::JoinHandle,
    time::{interval, Duration},
};
use tracing::{debug, info, warn};

use crate::{
    config::load_watcher_config,
    display::{change_display_mode, restore_default_settings, DisplayMode},
    state::AppState,
};

pub struct ProcessWatcher {
    process_path: String,
    display_mode: DisplayMode,
    is_running: Arc<AtomicBool>,
    task: Mutex<Option<JoinHandle<()>>>,
}

impl ProcessWatcher {
    pub fn new(process_path: String, display_mode: DisplayMode) -> Self {
        Self {
            process_path,
            display_mode,
            is_running: Arc::new(AtomicBool::new(false)),
            task: Mutex::new(None),
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::Relaxed)
    }

    pub async fn start(&self) {
        let mut task_guard = self.task.lock().await;

        if task_guard.is_some() {
            warn!("{} already watching, skipping", self.process_path);
            return;
        }

        let process_path = self.process_path.clone();
        let display_mode = self.display_mode.clone();
        let is_running = self.is_running.clone();

        let task = tokio::spawn(async move {
            info!("Start watching process: {}", process_path);

            let mut ticker = interval(Duration::from_secs(2));
            let mut system = System::new_all();

            let on_start = || {
                let _ = change_display_mode(&display_mode, false);
            };
            let on_stop = || {
                let _ = restore_default_settings();
            };

            loop {
                ticker.tick().await;

                system.refresh_processes(ProcessesToUpdate::All, true);

                let running_now = system
                    .processes()
                    .iter()
                    .any(|(_, p)| p.exe().unwrap_or(Path::new("s")).to_string_lossy().into_owned() == process_path);

                let running_prev = is_running.swap(running_now, Ordering::Relaxed);

                if running_prev != running_now {
                    if running_now {
                        (on_start)();
                    } else {
                        (on_stop)();
                    }
                }
            }
        });

        *task_guard = Some(task);
    }

    pub async fn stop(&self) {
        let mut task_guard = self.task.lock().await;

        if let Some(task) = task_guard.take() {
            task.abort();
            info!("{} stop", self.process_path);
        } else {
            warn!("{} not watching", self.process_path);
        }
    }
}

#[tauri::command]
pub async fn toggle_watching(state: State<'_, AppState>) -> Result<bool, String> {
    let mut watcher_guard = state.watcher.lock().await;

    if let Some(watcher_instance) = watcher_guard.as_mut() {
        if watcher_instance.task.lock().await.is_some() {
            info!("Ready to stop");
            watcher_instance.stop().await;
            Ok(false)
        } else {
            info!("Ready to start");
            watcher_instance.start().await;
            Ok(true)
        }
    } else {
        let watcher_config = load_watcher_config().map_err(|e| e.to_string())?;
        *watcher_guard = Some(ProcessWatcher::new(
            watcher_config.game_path.unwrap(),
            DisplayMode {
                width: watcher_config.width,
                height: watcher_config.height,
                refresh_rate: watcher_config.fps,
            },
        ));
        if let Some(watcher_instance) = watcher_guard.as_mut() {
            info!("Ready to start");
            watcher_instance.start().await;
            Ok(true)
        } else {
            Err("Failed to create watcher instance".to_string())
        }
    }
}

#[tauri::command]
pub async fn get_gaming_status(state: State<'_, AppState>) -> Result<bool, String> {
    let mut watcher_guard = state.watcher.lock().await;

    if let Some(watcher_instance) = watcher_guard.as_mut() {
        Ok(watcher_instance.is_running())
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub async fn get_watching_status(state: State<'_, AppState>) -> Result<bool, String> {
    let mut watcher_guard = state.watcher.lock().await;

    if let Some(watcher_instance) = watcher_guard.as_mut() {
        let task_guard = watcher_instance.task.lock().await;
        if task_guard.is_some() {
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}
