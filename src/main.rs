mod cli;
mod commands;
mod storage;

use anyhow::Result;
use clap::Parser;
use commands::{auth, basic, config, harmony};
use tracing::{debug, warn};
use tracing_subscriber::{EnvFilter, fmt};

fn init_tracing(verbosity: u8, quiet: bool) {
    // Base level: info, increase with -v; quiet forces warn
    let level = if quiet {
        "warn"
    } else {
        match verbosity {
            0 => "info",
            1 => "debug",
            _ => "trace",
        }
    };

    let env_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| level.to_string());

    let _ = fmt()
        .with_env_filter(EnvFilter::new(env_filter))
        .with_target(false)
        .with_level(true)
        .try_init();
}

fn main() -> Result<()> {
    let args = cli::Cli::parse();

    init_tracing(args.verbose, args.quiet);

    debug!(?args.verbose, quiet = args.quiet, "logging initialized");

    match args.command {
        Some(cli::Command::List) => {
            basic::list_commands()?;
        }
        Some(cli::Command::Login) => {
            auth::login()?;
        }
        Some(cli::Command::Logout) => {
            auth::logout()?;
        }
        Some(cli::Command::Verify) => {
            auth::verify_token()?;
        }
        Some(cli::Command::HarmonyAdd {
            ip,
            port,
            label,
            path_prefix,
            encryption_key,
        }) => {
            harmony::harmony::harmony_add(&ip, port, label.as_deref(), &path_prefix, encryption_key.as_deref())?;
        }
        Some(cli::Command::HarmonyList) => {
            harmony::harmony::harmony_list()?;
        }
        Some(cli::Command::HarmonyRemove {
            id,
            label,
            ip,
            port,
        }) => {
            harmony::harmony::harmony_remove(id.as_deref(), label.as_deref(), ip.as_deref(), port)?;
        }
        Some(cli::Command::HarmonyInfo { id, label }) => {
            harmony::management::info(id.as_deref(), label.as_deref())?;
        }
        Some(cli::Command::HarmonyPipelines { id, label }) => {
            harmony::management::pipelines(id.as_deref(), label.as_deref())?;
        }
        Some(cli::Command::HarmonyRoutes { id, label, json }) => {
            harmony::management::routes(id.as_deref(), label.as_deref(), json)?;
        }
        Some(cli::Command::HarmonyReload { id, label }) => {
            harmony::management::reload(id.as_deref(), label.as_deref())?;
        }
        Some(cli::Command::HarmonyAuthorize { id, label }) => {
            auth::authorize_harmony(id.as_deref(), label.as_deref())?;
        }
        Some(cli::Command::HarmonySetKey { id, encryption_key }) => {
            harmony::harmony::harmony_set_key(&id, &encryption_key)?;
        }
        Some(cli::Command::HarmonyShowKey { id }) => {
            harmony::harmony::harmony_show_key(&id)?;
        }
        Some(cli::Command::HarmonyDeleteKey { id }) => {
            harmony::harmony::harmony_delete_key(&id)?;
        }
        Some(cli::Command::TestBrowser) => {
            println!("Testing browser opening...");
            match open::that_detached("https://www.google.com") {
                Ok(_) => println!("✓ Browser opened successfully"),
                Err(e) => println!("✗ Failed to open browser: {}", e),
            }
        }
        Some(cli::Command::ConfigSet { key, value }) => {
            config::set_config(&key, &value)?;
        }
        Some(cli::Command::ConfigGet { key }) => {
            config::get_config(key.as_deref())?;
        }
        Some(cli::Command::ConfigUnset { key }) => {
            config::unset_config(&key)?;
        }
        None => {
            // No subcommand: show help-like hint
            warn!("no command provided");
            println!("Runbeam CLI: use --help to see available commands.");
        }
    }

    Ok(())
}
