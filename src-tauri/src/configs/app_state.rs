use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use crate::utils::watcher_manager::ProcessWatcher;


pub struct AppState {
    pub watcher: Arc<Mutex<Option<ProcessWatcher>>>,
    pub monitors: Arc<Mutex<HashMap<String, String>>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            watcher: Arc::new(Mutex::new(None)),
            monitors: Arc::new(Mutex::new(HashMap::new()))
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
