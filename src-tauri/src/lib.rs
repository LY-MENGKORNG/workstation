use std::env;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn list_pub_key_files() -> Vec<String> {
    let home_dir = env::home_dir().unwrap();
    let ssh_dir = home_dir.join(".ssh");
    let mut files = Vec::new();
    if ssh_dir.is_dir() {
        for entry in walkdir::WalkDir::new(ssh_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_type().is_file() && e.file_name().to_string_lossy().ends_with(".pub")
            })
        {
            files.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    files
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, list_pub_key_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
