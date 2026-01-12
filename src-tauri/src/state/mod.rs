use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

pub struct AppState {
    pub watch_task: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            watch_task: Arc::new(Mutex::new(None)),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}