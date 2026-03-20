use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "actobase",
    version,
    about = "Thin Rust client for hosted Actobase utilities."
)]
pub struct Cli {
    #[arg(
        long,
        global = true,
        env = "ACTOBASE_BASE_URL",
        default_value = "https://actobase.com"
    )]
    pub base_url: String,
    #[arg(long, global = true, env = "ACTOBASE_CONFIG_PATH")]
    pub config: Option<PathBuf>,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Auth {
        #[command(subcommand)]
        command: AuthCommand,
    },
    Account {
        #[command(subcommand)]
        command: AccountCommand,
    },
    Watch {
        #[command(subcommand)]
        command: WatchCommand,
    },
    Qa {
        #[command(subcommand)]
        command: QaCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum AuthCommand {
    ImportKey(ImportKeyArgs),
    Show,
    Clear,
}

#[derive(Debug, Args)]
pub struct ImportKeyArgs {
    pub token: String,
    #[arg(long)]
    pub account_id: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum AccountCommand {
    Status,
}

#[derive(Debug, Subcommand)]
pub enum WatchCommand {
    Request(WatchRequestArgs),
}

#[derive(Debug, Args)]
pub struct WatchRequestArgs {
    #[arg(long)]
    pub url: String,
    #[arg(long, value_enum, default_value_t = WatchMode::Http)]
    pub mode: WatchMode,
    #[arg(long = "every", default_value_t = 60)]
    pub interval_seconds: u64,
    #[arg(long)]
    pub expect: Option<String>,
    #[arg(long)]
    pub webhook: Option<String>,
    #[arg(long)]
    pub name: Option<String>,
    #[arg(long)]
    pub email: Option<String>,
    #[arg(long, value_enum, default_value_t = WatchAuthMode::ApiKeyOnly)]
    pub auth_mode: WatchAuthMode,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum WatchMode {
    Http,
    Html,
    AiHtml,
    AiImage,
}

impl WatchMode {
    pub fn as_api_value(&self) -> &'static str {
        match self {
            Self::Http => "http",
            Self::Html => "html",
            Self::AiHtml => "ai_html",
            Self::AiImage => "ai_image",
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum WatchAuthMode {
    ApiKeyOnly,
    ApiKeyPlusEmail,
}

impl WatchAuthMode {
    pub fn as_api_value(&self) -> &'static str {
        match self {
            Self::ApiKeyOnly => "api_key_only",
            Self::ApiKeyPlusEmail => "api_key_plus_email",
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum QaCommand {
    Request(QaRequestArgs),
}

#[derive(Debug, Args)]
pub struct QaRequestArgs {
    #[arg(long)]
    pub site: String,
    #[arg(long, value_enum, default_value_t = QaFocus::Visual)]
    pub focus: QaFocus,
    #[arg(long, value_enum, default_value_t = QaCadence::OneTime)]
    pub cadence: QaCadence,
    #[arg(long = "path")]
    pub paths: Vec<String>,
    #[arg(long)]
    pub repo: Option<String>,
    #[arg(long)]
    pub concern: Option<String>,
    #[arg(long)]
    pub email: Option<String>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum QaFocus {
    Visual,
    Copy,
    VisualCopy,
    Recurring,
}

impl QaFocus {
    pub fn as_api_value(&self) -> &'static str {
        match self {
            Self::Visual => "visual",
            Self::Copy => "copy",
            Self::VisualCopy => "visual_copy",
            Self::Recurring => "recurring",
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum QaCadence {
    OneTime,
    OnDeploy,
    Daily,
}

impl QaCadence {
    pub fn as_api_value(&self) -> &'static str {
        match self {
            Self::OneTime => "one_time",
            Self::OnDeploy => "on_deploy",
            Self::Daily => "daily",
        }
    }
}
