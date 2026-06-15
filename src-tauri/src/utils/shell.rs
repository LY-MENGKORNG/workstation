#[derive(serde::Serialize, Default, Debug)]
pub struct ConfigStructure {
    pub host: String,
    pub hostname: String,
    pub user: String,
    pub identity_file: String,
}

#[tauri::command]
pub fn run_command(command: &str) -> Result<String, String> {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/**
 * Reads the config file from SSH and returns its contents as a formatted string.
 */
#[tauri::command]
pub fn read_config_file() -> Result<String, String> {
    run_command("cat ~/.ssh/config")
}

#[tauri::command]
pub fn parse_ssh_config(config: &str) -> Result<Vec<ConfigStructure>, String> {
    let mut result = Vec::new();
    let mut conf = ConfigStructure::default();

    for line in config.lines() {
        let l = line.trim();

        if l.starts_with('#') || l.is_empty() {
            if !conf.host.is_empty() {
                result.push(std::mem::take(&mut conf));
            }
            continue;
        }

        if let Some(kv) = l.split_once(' ') {
            let k = kv.0.trim();
            let v = kv.1.trim();

            match k {
                "HostName" => conf.hostname = v.to_string(),
                "Host" => conf.host = v.to_string(),
                "User" => conf.user = v.to_string(),
                "IdentityFile" => conf.identity_file = v.to_string(),
                _ => {}
            }
        }
    }

    if !conf.host.is_empty() {
        result.push(conf);
    }

    Ok(result)
}
