mod cli;
mod commands;
mod storage;

use anyhow::Result;
use clap::Parser;
use commands::{basic, harmony};
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
        Some(cli::Command::HarmonyAdd {
            ip,
            port,
            label,
            path_prefix,
        }) => {
            harmony::harmony::harmony_add(&ip, port, label.as_deref(), &path_prefix)?;
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
        Some(cli::Command::HarmonyRoutes { id, label }) => {
            harmony::management::routes(id.as_deref(), label.as_deref())?;
        }
        None => {
            // No subcommand: show help-like hint
            warn!("no command provided");
            println!("Runbeam CLI: use --help to see available commands.");
        }
    }

    Ok(())
}
