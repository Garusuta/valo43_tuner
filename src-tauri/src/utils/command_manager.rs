use std::{error::Error, path::PathBuf, process::Command};

use sysinfo::System;
use tracing::debug;

pub fn run_command(command: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = Command::new("cmd").arg("/C").args(command).output()?;
    debug!("Running command: {:?}", command);
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout)?;
        debug!("Command output: {}", stdout);
        Ok(stdout)
    } else {
        let stderr = String::from_utf8(output.stderr)?;
        debug!("Command error output: {}", stderr);
        Err(format!("Command failed: {}", stderr).into())
    }
}

pub fn get_running_process_path(process_name: &str) -> Option<PathBuf> {
    let system = System::new_all();

    let matches = system
        .processes()
        .iter()
        .filter(|(_, process)| process.name().eq_ignore_ascii_case(process_name))
        .map(|(_, process)| process)
        .collect::<Vec<_>>();

    match matches.len() {
        0 => {
            debug!("No running process found with name '{}'", process_name);
            None
        }
        1 => {
            if let Some(p) = matches[0].exe() {
                debug!("Found process '{}' at path: {:?}", process_name, p);
                Some(p.to_path_buf())
            } else {
                debug!("Process '{}' found but path is unavailable.", process_name);
                None
            }
        }
        _ => {
            let mut path = PathBuf::new();
            for process in matches {
                if let Some(p) = process.exe() {
                    if path.as_os_str().is_empty() {
                        path = p.to_path_buf();
                    } else {
                        if path != p.to_path_buf() {
                            debug!("Multiple instances of process '{}' found with different paths. Aborting.", process_name);
                            return None;
                        }
                    }
                }
            }
            debug!(
                "Duplicate process '{}' found with path: {:?}",
                process_name, path
            );
            Some(path)
        }
    }
}
