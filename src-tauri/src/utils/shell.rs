use dirs;

#[derive(serde::Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Conf {
    pub host: String,
    pub hostname: String,
    pub user: String,
    pub identity_file: String,
}

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
pub fn parse_ssh_config(config: &str) -> Result<Vec<Conf>, String> {
    let mut result = Vec::new();
    let mut conf = Conf::default();

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

#[tauri::command]
pub fn get_conf_by_filename(f: String) -> Result<Conf, String> {
    let conf = read_config_file().expect("Unabled to read config file");
    let result = parse_ssh_config(&conf.to_string()).expect("Unabled to parse config");

    let test = result.into_iter().find(|r: &Conf| r.identity_file == f);
    test.ok_or("Conf not found".to_string())
}

#[tauri::command]
pub fn test_conn(host: String) -> Result<bool, String> {
    let result = run_command(&format!("ssh -T git@{}", host));
    Ok(result.is_ok())
}

#[tauri::command]
pub fn write_ssh_config(config: String) -> Result<(), String> {
    let ssh_dir = dirs::home_dir()
        .ok_or("Unable to find home directory")?
        .join(".ssh");
    std::fs::create_dir_all(&ssh_dir).map_err(|e| e.to_string())?;
    let config_path = ssh_dir.join("config");
    std::fs::write(&config_path, &config).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn toggle_ssh_entry(host: String, enabled: bool) -> Result<(), String> {
    let config = read_config_file()?;
    let mut lines: Vec<String> = config.lines().map(|l| l.to_string()).collect();
    let mut in_block = false;
    let mut in_target = false;
    let mut modified = false;

    for i in 0..lines.len() {
        let trimmed = lines[i].trim().to_string();

        if trimmed.starts_with("Host ") || trimmed.starts_with("#Host ") {
            if in_target && !in_block {
                break;
            }
            let clean_host = trimmed
                .trim_start_matches('#')
                .trim()
                .strip_prefix("Host ")
                .unwrap_or("")
                .trim();
            in_target = clean_host == host;
            in_block = true;
            continue;
        }

        if in_target {
            if enabled && lines[i].trim_start().starts_with('#') {
                lines[i] = lines[i].trim_start_matches('#').to_string();
                modified = true;
            } else if !enabled
                && !lines[i].trim_start().starts_with('#')
                && !lines[i].trim().is_empty()
            {
                lines[i] = format!("#{}", lines[i]);
                modified = true;
            }
        }

        if in_block && !in_target {
            in_block = false;
        }
    }

    if modified {
        write_ssh_config(lines.join("\n"))?;
    }
    Ok(())
}
