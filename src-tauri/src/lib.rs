use std::env;
mod utils;
use utils::{
    file::list_pub_key_files,
    shell::{parse_ssh_config, read_config_file, run_command},
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_pub_key_files,
            run_command,
            parse_ssh_config,
            read_config_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
