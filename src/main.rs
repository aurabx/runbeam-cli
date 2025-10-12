mod cli;
mod commands;

use anyhow::Result;
use clap::Parser;
use tracing::{debug, warn};
use tracing_subscriber::{fmt, EnvFilter};
use commands::basic;

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
        Some(cli::Command::Echo { text }) => {
            basic::echo(&text)?;
        }
        Some(cli::Command::Ping) => {
            basic::ping()?;
        }
        None => {
            // No subcommand: show help-like hint
            warn!("no command provided");
            println!("Runbeam CLI: use --help to see available commands.");
        }
    }

    Ok(())
}