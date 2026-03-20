use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StoredConfig {
    pub base_url: Option<String>,
    pub account_id: Option<String>,
    pub api_key: Option<String>,
}

pub fn resolve_config_path(explicit_path: Option<PathBuf>) -> Result<PathBuf, String> {
    if let Some(path) = explicit_path {
        return Ok(path);
    }

    let home = env::var_os("HOME").ok_or_else(|| "HOME is not set".to_string())?;
    let mut path = PathBuf::from(home);
    path.push(".config");
    path.push("actobase");
    path.push("client.json");
    Ok(path)
}

pub fn read_config(path: &Path) -> Result<StoredConfig, String> {
    if !path.exists() {
        return Ok(StoredConfig::default());
    }

    let raw = fs::read_to_string(path)
        .map_err(|error| format!("read config '{}': {}", path.display(), error))?;
    serde_json::from_str(&raw)
        .map_err(|error| format!("parse config '{}': {}", path.display(), error))
}

pub fn write_config(path: &Path, config: &StoredConfig) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("create config dir '{}': {}", parent.display(), error))?;
    }

    let payload = serde_json::to_string_pretty(config)
        .map_err(|error| format!("serialize config '{}': {}", path.display(), error))?;
    fs::write(path, payload)
        .map_err(|error| format!("write config '{}': {}", path.display(), error))
}

pub fn clear_config(path: &Path) -> Result<(), String> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!("remove config '{}': {}", path.display(), error)),
    }
}

#[cfg(test)]
mod tests {
    use super::{StoredConfig, clear_config, read_config, write_config};
    use std::path::PathBuf;

    #[test]
    fn config_round_trip() {
        let mut path = std::env::temp_dir();
        path.push(format!("actobase-cli-config-{}.json", std::process::id()));
        let config = StoredConfig {
            base_url: Some("https://actobase.test".to_string()),
            account_id: Some("aboacct_test".to_string()),
            api_key: Some("aboak_test".to_string()),
        };

        write_config(&path, &config).expect("write config");
        let loaded = read_config(&path).expect("read config");
        assert_eq!(loaded.base_url.as_deref(), Some("https://actobase.test"));
        assert_eq!(loaded.account_id.as_deref(), Some("aboacct_test"));
        assert_eq!(loaded.api_key.as_deref(), Some("aboak_test"));
        clear_config(&path).expect("clear config");
        let empty = read_config(&PathBuf::from(&path)).expect("read cleared config");
        assert!(empty.api_key.is_none());
    }
}
