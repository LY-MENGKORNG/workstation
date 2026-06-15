use std::env;
use walkdir;

#[tauri::command]
pub fn list_pub_key_files() -> Vec<String> {
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
            println!("{:?}", entry);
            files.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    files
}
