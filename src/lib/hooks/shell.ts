import { invoke } from "@tauri-apps/api/core";

export function listSSHKeyFiles() {
    return invoke("list_pub_key_files") as Promise<string[]>;
}

export function readConfigFile() {
    return invoke("read_config_file") as Promise<string>;
}

export function parseSSHConfig(
    config: string,
): Promise<Record<string, string>> {
    return invoke("parse_ssh_config", { config });
}
