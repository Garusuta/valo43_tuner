use std::sync::Arc;
use valo43_tuner::{
    config::{create::create_config, parser::parser_config},
    display::display::change_display_mode,
    monitor::monitor::{disable_monitor, enable_monitor, scan_monitors},
    valorant::watch_valorant,
};

#[tokio::main]
async fn main() {
    create_config();
    let monitors_arc = Arc::new(scan_monitors().expect("Wtf? No monitors found?"));
    let monitors_for_start = Arc::clone(&monitors_arc);
    let monitors_for_stop = Arc::clone(&monitors_arc);

    // resulotion_config 存储了完整的配置数据
    let resulotion_config_arc = Arc::new(parser_config().expect("Failed to parse config"));
    
    // 为 on_start 闭包克隆一个 Arc
    let resulotion_config_for_start = Arc::clone(&resulotion_config_arc);
    // 为 on_stop 闭包克隆一个 Arc
    let resulotion_config_for_stop = Arc::clone(&resulotion_config_arc);

    let on_start = move || {
        println!("检测到 Valorant 启动，禁用显示器...");
        for monitor in monitors_for_start.iter() {
            if monitor.status == "Started" {
                disable_monitor(&monitor.instance_id);
            }
        }
        // 在闭包内部获取 game_config 的引用
        let game_config = resulotion_config_for_start // resulotion_config_for_start 是 Arc，它被 move 到这里
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
        // 在闭包内部获取 desktop_config 的引用
        let desktop_config = resulotion_config_for_stop // resulotion_config_for_stop 是 Arc，它被 move 到这里
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

    let task = tokio::spawn(watch_valorant(on_start, on_stop));
    println!("监听无畏契约进程中，按 Ctrl+C 退出...");

    tokio::signal::ctrl_c().await.expect("监听信号失败");
    println!("退出监听");

    task.abort();
}