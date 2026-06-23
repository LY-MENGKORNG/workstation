use std::env;
mod utils;
use utils::{
    file::list_pub_key_files,
    git::{parse_git_hosts, read_git_config, test_git_conn},
    shell::{
        get_conf_by_filename, parse_ssh_config, read_config_file, test_conn, toggle_ssh_entry,
        write_ssh_config,
    },
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_pub_key_files,
            parse_ssh_config,
            get_conf_by_filename,
            read_config_file,
            test_conn,
            write_ssh_config,
            toggle_ssh_entry,
            read_git_config,
            parse_git_hosts,
            test_git_conn,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
