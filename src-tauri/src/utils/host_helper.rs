use std::fs;
use std::io::{self, BufRead};

#[tauri::command]
pub fn get_hosts() -> Vec<(String, Vec<String>)>{
	let path = "/etc/hosts";
	let file = fs::File::open(path).unwrap();
	let reader = io::BufReader::new(file);

	let mut entries = Vec::new();

	for line in reader.lines() {
		let line = line.unwrap();
		let line = line.trim();

		if line.is_empty() || line.starts_with('#') {
			continue;
		}
		
		let parts: Vec<&str> = line.split_whitespace().collect();
		if !parts.is_empty() {
			let ip = parts[0].to_string();
			let aliases = parts[1..].iter().map(|s| s.to_string()).collect();
			entries.push((ip, aliases));

		}
	}

	entries
}