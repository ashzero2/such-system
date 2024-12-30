use std::{collections::HashMap, os::unix::process, thread::sleep, time::Duration};

#[allow(dead_code)]
use sysinfo::System;
use std::process::Command;

static DUR: Duration = Duration::from_secs(1);

#[tauri::command]
pub fn get_cpu_usage() -> f32 {
    let mut sys = System::new_all();

    sys.refresh_cpu_all();
    sleep(DUR);
    sys.refresh_cpu_all();

    let total_cores = sys.cpus().len();
    let total_cpu_usage: f32 = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum();

    let average_usage = total_cpu_usage / total_cores as f32;
    average_usage
}

#[tauri::command]
pub fn get_memory_usage() -> HashMap<String, f64> {
	let mut sys = System::new_all();

	sys.refresh_memory();
	sleep(DUR);
	sys.refresh_memory();

	let used_memory = sys.used_memory();
	let used_gb = (used_memory as f64) / 1_073_741_824.0;

	let total_memory = sys.total_memory();
	let total_gb = (total_memory as f64) / 1_073_741_824.0;

	let mut map = HashMap::new();
	map.insert("total".to_string(), total_gb);
	map.insert("used".to_string(), used_gb);

	map
}

pub fn get_system_details() -> HashMap<String, String> {
	let mut map = HashMap::new();

	match Command::new("hostname").output() {
		Ok(output) => {
			if output.status.success() {
				let hostname = String::from_utf8(output.stdout).unwrap_or_default();
				map.insert("hostname".to_string(), hostname);
			}
		}
		Err(e) => eprintln!("Error getting hostname {}", e)
	}

	map
}
