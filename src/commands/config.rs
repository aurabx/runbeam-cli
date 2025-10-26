use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use tracing::{debug, info};

use crate::storage;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CliConfig {
    /// Custom API URL (overrides RUNBEAM_API_URL env var)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_url: Option<String>,
}

/// Get the path to the config file
fn config_file_path() -> Result<std::path::PathBuf> {
    Ok(storage::data_dir()?.join("config.json"))
}

/// Load the CLI configuration
pub fn load_config() -> Result<CliConfig> {
    let path = config_file_path()?;
    if !path.exists() {
        return Ok(CliConfig::default());
    }

    let data = fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;

    let config: CliConfig =
        serde_json::from_str(&data).with_context(|| format!("parsing {}", path.display()))?;

    Ok(config)
}

/// Save the CLI configuration
fn save_config(config: &CliConfig) -> Result<()> {
    let path = config_file_path()?;
    let tmp_path = path.with_extension("json.tmp");

    let json = serde_json::to_string_pretty(config)?;

    // Write atomically: write temp, then rename
    {
        let mut f = fs::File::create(&tmp_path)
            .with_context(|| format!("creating {}", tmp_path.display()))?;
        f.write_all(json.as_bytes())?;
        f.sync_all().ok();
    }

    fs::rename(&tmp_path, &path)
        .with_context(|| format!("rename {} -> {}", tmp_path.display(), path.display()))?;

    Ok(())
}

/// Get the effective API URL (config > env > default)
pub fn get_api_url() -> Result<String> {
    // Priority: 1. Config file, 2. Environment variable, 3. Default
    let config = load_config()?;

    if let Some(url) = config.api_url {
        return Ok(url);
    }

    if let Ok(url) = std::env::var("RUNBEAM_API_URL") {
        return Ok(url);
    }

    Ok("http://runbeam.lndo.site".to_string())
}

/// Set a configuration value
pub fn set_config(key: &str, value: &str) -> Result<()> {
    info!("Setting config: {} = {}", key, value);

    let mut config = load_config()?;

    match key {
        "api-url" | "api_url" => {
            // Validate URL format
            if !value.starts_with("http://") && !value.starts_with("https://") {
                anyhow::bail!("API URL must start with http:// or https://");
            }

            // Remove trailing slash
            let normalized_url = value.trim_end_matches('/').to_string();

            config.api_url = Some(normalized_url.clone());
            save_config(&config)?;

            println!("✅ API URL set to: {}", normalized_url);
            println!("   Saved to ~/.runbeam/config.json");
            println!();
            println!("   This will override the RUNBEAM_API_URL environment variable.");

            debug!("Config saved: api_url = {}", normalized_url);
        }
        _ => {
            anyhow::bail!("Unknown config key: {}. Valid keys: api-url", key);
        }
    }

    Ok(())
}

/// Unset a configuration value (revert to environment variable or default)
pub fn unset_config(key: &str) -> Result<()> {
    info!("Unsetting config: {}", key);

    let mut config = load_config()?;

    match key {
        "api-url" | "api_url" => {
            if config.api_url.is_none() {
                println!("ℹ  API URL is not set in config.");
                return Ok(());
            }

            config.api_url = None;
            save_config(&config)?;

            println!("✅ API URL unset.");
            println!("   Config removed from ~/.runbeam/config.json");

            // Show what will be used instead
            let fallback = std::env::var("RUNBEAM_API_URL")
                .unwrap_or_else(|_| "http://runbeam.lndo.site".to_string());
            println!("   Will now use: {}", fallback);

            debug!("Config cleared: api_url");
        }
        _ => {
            anyhow::bail!("Unknown config key: {}. Valid keys: api-url", key);
        }
    }

    Ok(())
}

/// Get a configuration value
pub fn get_config(key: Option<&str>) -> Result<()> {
    let config = load_config()?;

    match key {
        Some("api-url") | Some("api_url") => {
            let effective_url = get_api_url()?;
            let source = if config.api_url.is_some() {
                "config file"
            } else if std::env::var("RUNBEAM_API_URL").is_ok() {
                "environment variable"
            } else {
                "default"
            };

            println!("API URL: {} (from {})", effective_url, source);
        }
        Some(k) => {
            anyhow::bail!("Unknown config key: {}. Valid keys: api-url", k);
        }
        None => {
            // Show all config
            println!("Current configuration:");
            println!();

            let api_url = get_api_url()?;
            let source = if config.api_url.is_some() {
                "config file"
            } else if std::env::var("RUNBEAM_API_URL").is_ok() {
                "environment"
            } else {
                "default"
            };

            println!("  api-url: {} ({})", api_url, source);
            println!();
            println!("Configuration file: ~/.runbeam/config.json");
        }
    }

    Ok(())
}
