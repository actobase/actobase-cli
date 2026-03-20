use crate::config::StoredConfig;
use serde_json::{Value, json};

pub fn render_stored_config(config: &StoredConfig, path: &str) -> Result<String, String> {
    let api_key_preview = config.api_key.as_deref().map(mask_api_key);
    serde_json::to_string_pretty(&json!({
        "config_path": path,
        "base_url": config.base_url,
        "account_id": config.account_id,
        "api_key_preview": api_key_preview,
        "api_key_present": config.api_key.is_some(),
        "authenticated": config.api_key.is_some(),
    }))
    .map_err(|error| format!("serialize config output: {}", error))
}

pub fn render_success(payload: &Value) -> Result<String, String> {
    serde_json::to_string_pretty(payload).map_err(|error| format!("serialize output: {}", error))
}

fn mask_api_key(api_key: &str) -> String {
    if api_key.len() <= 12 {
        return api_key.to_string();
    }
    format!("{}...{}", &api_key[..8], &api_key[api_key.len() - 4..])
}
