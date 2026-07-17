use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    #[serde(default = "default_base_url")]
    pub base_url: String,
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
}

fn default_base_url() -> String {
    "https://api.deepseek.com".into()
}
fn default_model() -> String {
    "deepseek-chat".into()
}
fn default_max_tokens() -> u32 {
    8192
}
fn default_temperature() -> f32 {
    0.7
}

impl Config {
    pub fn load() -> Result<Self> {
        if let Ok(key) = std::env::var("DEEPSEEK_API_KEY") {
            return Ok(Config {
                api_key: key,
                base_url: std::env::var("DEEPSEEK_BASE_URL").unwrap_or_else(|_| default_base_url()),
                model: std::env::var("DEEPSEEK_MODEL").unwrap_or_else(|_| default_model()),
                max_tokens: default_max_tokens(),
                temperature: default_temperature(),
            });
        }

        let config_path = config_path().context("could not determine config path")?;
        if config_path.exists() {
            let content =
                std::fs::read_to_string(&config_path).context("failed to read config file")?;
            let config: Config =
                toml::from_str(&content).context("failed to parse config file")?;
            return Ok(config);
        }

        anyhow::bail!(
            "No API key found. Set DEEPSEEK_API_KEY env var or create {} with:\n[api_key]\napi_key = \"sk-...\"",
            config_path.display()
        );
    }

    pub fn config_dir() -> Result<PathBuf> {
        let dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("deepseek-cli");
        std::fs::create_dir_all(&dir).ok();
        Ok(dir)
    }
}

fn config_path() -> Result<PathBuf> {
    Ok(Config::config_dir()?.join("config.toml"))
}
