// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils {
	pub mod host_helper;
	pub mod sys_helper;
}

fn main() {
  app_lib::run();
}
