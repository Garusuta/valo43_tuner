use core::time;

use sysinfo::System;
use tokio::time::interval;

pub async fn watch_valorant<S, F>(on_start: S, on_stop: F)
where
    S: Fn() + Send + 'static,
    F: Fn() + Send + 'static,
{
    let mut sys = System::new_all();
    let mut interval = interval(time::Duration::from_secs(3));
    let mut valorant_running = false;

    loop {
        interval.tick().await;
        sys.refresh_all();

        let is_running = sys
            .processes()
            .iter()
            .any(|(_, process)| process.name().eq_ignore_ascii_case("无畏契约登录器.exe"));
        if is_running != valorant_running {
            valorant_running = is_running;

            if valorant_running {
                on_start();
            } else {
                on_stop();
            }
        }
    }
}
