use std::process::Command;

use encoding::{DecoderTrap, Encoding, all::GBK};

#[derive(Debug)]
pub struct Monitor {
    pub instance_id: String,
    pub status: String,
}

pub fn scan_monitors() -> Option<Vec<Monitor>> {
    let output = Command::new("cmd")
        .args(&[
            "/c",
            "chcp 437 > nul && pnputil /enum-devices /class Monitor",
        ])
        .output()
        .unwrap();
    let decoded = GBK.decode(&output.stdout, DecoderTrap::Replace).unwrap();
    let mut monitors = Vec::<Monitor>::new();
    let mut instance_id = String::new();
    let mut status = String::new();
    let mut skip_counter = 0;
    let mut submitted = false;
    for line in decoded.lines() {
        if skip_counter > 0 {
            skip_counter -= 1;
            continue;
        }

        if line.trim().is_empty() {
            continue;
        }

        if line.trim().starts_with("Instance ID:") {
            instance_id = line.replace("Instance ID:", "").trim().to_string();
            skip_counter = 4;
        }
        if line.trim().starts_with("Status:") {
            status = line.replace("Status:", "").trim().to_string();
            skip_counter = 1;
            submitted = true;
        }
        if submitted {
            monitors.push(Monitor {
                instance_id: instance_id.clone(),
                status: status.clone(),
            });
            submitted = false;
        }
    }
    return Some(monitors);
}

pub fn disable_monitor(instance_id: &str) {
    println!("Disabling monitor with Instance ID: {}", instance_id);
    let output = Command::new("pnputil")
        .args(&["/disable-device", instance_id, "/force"])
        .output()
        .unwrap();
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

pub fn enable_monitor(instance_id: &str) {
    println!("Enabling monitor with Instance ID: {}", instance_id);
    let output = Command::new("pnputil")
        .args(&["/enable-device", instance_id, "/force"])
        .output()
        .unwrap();
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
