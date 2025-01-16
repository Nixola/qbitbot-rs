use anyhow::{Context, Result};
use serde::Deserialize;

use crate::Args;

#[derive(Deserialize)]
pub struct Config {
    /// Telegram bot token
    pub token: String,

    /// Telegram owner user ID
    pub user_id: u64,

    /// Telegram chat id
    pub chat_id: i64,

    /// Qbittorrent username
    pub username: String,

    /// Qbittorrent password
    pub password: String,

    /// Qbittorrent url
    pub host: String,
}

pub fn load_config(args: Args) -> Result<Config> {
    let mut config = config::Config::builder();

    // /etc/qbitbot/config.toml
    config = config.add_source(config::File::with_name("/etc/qbitbot/config.toml").required(false));

    if let Some(home_dir) = dirs::home_dir() {
        let unixy_config_path = home_dir.join(".config").join("qbitbot").join("config.toml");

        // ~/.config/qbitbot/config.toml
        config = config.add_source(
            config::File::with_name(unixy_config_path.to_str().unwrap()).required(false),
        );

        if let Some(config_dir) = dirs::config_dir() {
            let config_path = config_dir.join("qbitbot").join("config.toml");

            if config_path != unixy_config_path {
                // ~/Library/Application Support/qbitbot/config.toml
                config = config.add_source(
                    config::File::with_name(config_path.to_str().unwrap()).required(false),
                );
            }
        }
    }

    if let Some(config_file) = &args.config {
        // --config foo.toml
        config = config.add_source(config::File::with_name(config_file));
    }

    let config = config
        .add_source(config::Environment::with_prefix("QBITBOT"))
        .build()?
        .try_deserialize::<Config>()
        .context("Invalid config")?;

    Ok(config)
}
