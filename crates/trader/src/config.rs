use anyhow::{Context, Result};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub mode: String,                // "paper" | "live"
    pub database_url: String,        // e.g., sqlite://ai_trader.db
    pub exchange_api_key: Option<String>,
    pub exchange_api_secret: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            mode: "paper".to_string(),
            database_url: "sqlite://ai_trader.db".to_string(),
            exchange_api_key: None,
            exchange_api_secret: None,
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let mut builder = config::Config::builder()
            .set_default("mode", "paper")?
            .set_default("database_url", "sqlite://ai_trader.db")?;

        // Files: config/default.toml (required), config/local.toml (optional)
        builder = builder
            .add_source(config::File::with_name("config/default").required(false))
            .add_source(config::File::with_name("config/local").required(false))
            .add_source(config::Environment::with_prefix("TRADER").separator("__"));

        let cfg = builder
            .build()
            .context("failed to build configuration")?;

        let app: AppConfig = cfg
            .try_deserialize()
            .context("failed to deserialize configuration")?;
        Ok(app)
    }
}


