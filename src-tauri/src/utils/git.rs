#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitConfigEntry {
    pub key: String,
    pub value: String,
    pub section: String,
}

#[derive(serde::Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GitConfig {
    pub entries: Vec<GitConfigEntry>,
    pub raw: String,
}

#[derive(serde::Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GitHost {
    pub host: String,
    pub user: String,
    pub hostname: String,
    pub identity_file: String,
    pub connected: bool,
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

#[tauri::command]
pub fn read_git_config() -> Result<GitConfig, String> {
    let global = run_command("git config --global --list").ok();
    let raw = global.unwrap_or_default();
    let entries: Vec<GitConfigEntry> = raw
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let (key, value) = line.split_once('=')?;
            let section = key.split('.').next().unwrap_or("").to_string();
            Some(GitConfigEntry {
                key: key.to_string(),
                value: value.to_string(),
                section,
            })
        })
        .collect();

    Ok(GitConfig {
        raw,
        entries,
    })
}

#[tauri::command]
pub fn parse_git_hosts() -> Result<Vec<GitHost>, String> {
    let config = run_command("cat ~/.ssh/config").ok().unwrap_or_default();
    let mut hosts = Vec::new();
    let mut current = GitHost::default();

    for line in config.lines() {
        let l = line.trim();
        if l.starts_with('#') || l.is_empty() {
            if !current.host.is_empty() {
                hosts.push(std::mem::take(&mut current));
            }
            continue;
        }

        if let Some(kv) = l.split_once(' ') {
            let k = kv.0.trim();
            let v = kv.1.trim();
            match k {
                "Host" => current.host = v.to_string(),
                "HostName" => current.hostname = v.to_string(),
                "User" => current.user = v.to_string(),
                "IdentityFile" => current.identity_file = v.to_string(),
                _ => {}
            }
        }
    }

    if !current.host.is_empty() {
        hosts.push(current);
    }

    Ok(hosts)
}

#[tauri::command]
pub fn test_git_conn(host: String) -> Result<bool, String> {
    let result = run_command(&format!("ssh -T git@{}", host));
    Ok(result.is_ok())
}
