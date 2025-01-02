mod utils {
    pub mod host_helper;
    pub mod sys_helper;
}

use utils::sys_helper::{get_cpu_usage, get_memory_usage, get_system_details};
use utils::host_helper::get_hosts;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_cpu_usage,
            get_memory_usage,
            get_system_details,
			get_hosts
        ]) // Combine both handlers here.
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
