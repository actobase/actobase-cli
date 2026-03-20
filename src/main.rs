mod cli;
mod client;
mod config;
mod output;

use clap::Parser;
use cli::{AccountCommand, AuthCommand, Cli, Command, QaCommand, WatchCommand};
use config::{StoredConfig, clear_config, read_config, resolve_config_path, write_config};
use serde_json::{Value, json};
use std::path::Path;

fn main() {
    let cli = Cli::parse();
    if let Err(error) = run(cli) {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    let config_path = resolve_config_path(cli.config)?;
    let mut stored = read_config(&config_path)?;

    match cli.command {
        Command::Auth { command } => run_auth(command, &config_path, &cli.base_url, &mut stored),
        Command::Account { command } => {
            run_account(command, &config_path, &cli.base_url, &mut stored)
        }
        Command::Watch { command } => run_watch(command, &config_path, &cli.base_url, &mut stored),
        Command::Qa { command } => run_qa(command, &config_path, &cli.base_url, &mut stored),
    }
}

fn run_auth(
    command: AuthCommand,
    config_path: &Path,
    base_url: &str,
    stored: &mut StoredConfig,
) -> Result<(), String> {
    match command {
        AuthCommand::ImportKey(args) => {
            stored.base_url = Some(base_url.to_string());
            stored.api_key = Some(args.token);
            if let Some(account_id) = args.account_id {
                stored.account_id = Some(account_id);
            }
            write_config(config_path, stored)?;
            print_json(&json!({
                "ok": true,
                "config_path": config_path.display().to_string(),
                "base_url": stored.base_url,
                "account_id": stored.account_id,
                "authenticated": stored.api_key.is_some(),
            }))
        }
        AuthCommand::Show => {
            println!(
                "{}",
                output::render_stored_config(stored, &config_path.display().to_string())?
            );
            Ok(())
        }
        AuthCommand::Clear => {
            clear_config(config_path)?;
            print_json(&json!({
                "ok": true,
                "config_path": config_path.display().to_string(),
                "cleared": true,
            }))
        }
    }
}

fn run_account(
    command: AccountCommand,
    config_path: &Path,
    base_url: &str,
    stored: &mut StoredConfig,
) -> Result<(), String> {
    match command {
        AccountCommand::Status => {
            let api_key = require_api_key(stored)?;
            let payload = client::get_json_with_api_key(base_url, "/watch/api/account", api_key)?;
            update_stored_account_from_payload(stored, &payload);
            store_if_changed(config_path, base_url, stored)?;
            print_json(&payload)
        }
    }
}

fn run_watch(
    command: WatchCommand,
    config_path: &Path,
    base_url: &str,
    stored: &mut StoredConfig,
) -> Result<(), String> {
    match command {
        WatchCommand::Request(args) => {
            let payload = json!({
                "contact_email": args.email,
                "auth_mode": args.auth_mode.as_api_value(),
                "watch_name": args.name,
                "target_url": args.url,
                "mode": args.mode.as_api_value(),
                "interval_seconds": args.interval_seconds,
                "expected_text": args.expect,
                "webhook_url": args.webhook,
            });
            let response = client::post_json_with_api_key(
                base_url,
                "/watch/api/start",
                &payload,
                stored.api_key.as_deref(),
            )?;
            update_stored_account_from_payload(stored, &response);
            store_if_changed(config_path, base_url, stored)?;
            print_json(&response)
        }
    }
}

fn run_qa(
    command: QaCommand,
    config_path: &Path,
    base_url: &str,
    stored: &mut StoredConfig,
) -> Result<(), String> {
    match command {
        QaCommand::Request(args) => {
            let page_paths = if args.paths.is_empty() {
                Value::String(String::new())
            } else {
                Value::String(args.paths.join("\n"))
            };
            let payload = json!({
                "contact_email": args.email,
                "site_url": args.site,
                "qa_focus": args.focus.as_api_value(),
                "cadence": args.cadence.as_api_value(),
                "page_paths": page_paths,
                "repo_url": args.repo,
                "priority_concern": args.concern,
            });
            let response = client::post_json_with_api_key(
                base_url,
                "/qa/api/start",
                &payload,
                stored.api_key.as_deref(),
            )?;
            update_stored_account_from_payload(stored, &response);
            store_if_changed(config_path, base_url, stored)?;
            print_json(&response)
        }
    }
}

fn update_stored_account_from_payload(stored: &mut StoredConfig, payload: &Value) {
    if let Some(account_id) = payload
        .get("account")
        .and_then(|value| value.get("account_id"))
        .and_then(Value::as_str)
    {
        stored.account_id = Some(account_id.to_string());
    }

    if let Some(api_key) = payload
        .get("account")
        .and_then(|value| value.get("api_key"))
        .and_then(|value| value.get("token"))
        .and_then(Value::as_str)
    {
        stored.api_key = Some(api_key.to_string());
    }
}

fn store_if_changed(
    config_path: &Path,
    base_url: &str,
    stored: &mut StoredConfig,
) -> Result<(), String> {
    stored.base_url = Some(base_url.to_string());
    if stored.api_key.is_some() || stored.account_id.is_some() {
        write_config(config_path, stored)?;
    }
    Ok(())
}

fn require_api_key(stored: &StoredConfig) -> Result<&str, String> {
    stored
        .api_key
        .as_deref()
        .ok_or_else(|| "no stored api key; run `actobase watch request`, `actobase qa request`, or `actobase auth import-key` first".to_string())
}

fn print_json(payload: &Value) -> Result<(), String> {
    println!("{}", output::render_success(payload)?);
    Ok(())
}
