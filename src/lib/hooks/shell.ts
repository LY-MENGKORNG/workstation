import { invoke } from "@tauri-apps/api/core";

export type Conf = {
	host: string;
	hostname: string;
	user: string;
	identityFile: string;
};

export type GitConfigEntry = {
	key: string;
	value: string;
	section: string;
};

export type GitConfig = {
	entries: GitConfigEntry[];
	raw: string;
};

export type GitHost = {
	host: string;
	user: string;
	hostname: string;
	identityFile: string;
	connected: boolean;
};

export function listSSHKeyFiles() {
	return invoke("list_pub_key_files") as Promise<string[]>;
}

export function readConfigFile() {
	return invoke("read_config_file") as Promise<string>;
}

export function parseSSHConfig(config: string): Promise<Conf[]> {
	return invoke("parse_ssh_config", { config });
}

export function getConfByFilename(f: string): Promise<Conf> {
	return invoke("get_conf_by_filename", { f });
}

export function testConn(host: Conf["host"]): Promise<unknown> {
	return invoke("test_conn", { host });
}

export function writeSSHConfig(config: string): Promise<void> {
	return invoke("write_ssh_config", { config });
}

export function toggleSSHEntry(host: string, enabled: boolean): Promise<void> {
	return invoke("toggle_ssh_entry", { host, enabled });
}

export function readGitConfig(): Promise<GitConfig> {
	return invoke("read_git_config");
}

export function parseGitHosts(): Promise<GitHost[]> {
	return invoke("parse_git_hosts");
}

export function testGitConn(host: string): Promise<boolean> {
	return invoke("test_git_conn", { host });
}
