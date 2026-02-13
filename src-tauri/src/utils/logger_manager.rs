use std::{fs, io::stdout, time::{Duration, SystemTime}};

use time::UtcOffset;
use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, fmt::{time::OffsetTime, writer::MakeWriterExt}};

use crate::utils::constant_manager::WORK_DIR;


pub fn init_logger() -> tracing_appender::non_blocking::WorkerGuard {
    cleanup_old_logs(WORK_DIR.to_str().unwrap(), 7);
    // 获取本地时区偏移量
    let local_offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC); // 如果获取失败则回退到 UTC
    let format =
        time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
    // 构建 Local Timer
    let timer = OffsetTime::new(local_offset, format);

    let file_appender = rolling::daily(WORK_DIR.as_path(), "app.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let multi_writer = non_blocking.and(stdout);

    // 构建 EnvFilter (从环境变量 RUST_LOG 读取，默认 INFO)
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_timer(timer)
        .with_writer(multi_writer)
        .with_ansi(false)
        .init();

    guard
}

fn cleanup_old_logs(dir: &str, days: u64) {
    let limit = Duration::from_secs(days * 24 * 60 * 60);
    let now = SystemTime::now();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        if now.duration_since(modified).unwrap_or(Duration::ZERO) > limit {
                            let _ = fs::remove_file(path); // 忽略删除失败的情况
                        }
                    }
                }
            }
        }
    }
}
