use clap::{Parser, Subcommand};

/// runbeam: Rust-based CLI
///
/// Global flags:
/// -v / -vv / -vvv to increase verbosity
/// -q to reduce output
#[derive(Debug, Parser)]
#[command(name = "runbeam", version, about = "Runbeam command-line interface", long_about = None)]
pub struct Cli {
    /// Increase output verbosity (-v, -vv, -vvv)
    #[arg(short = 'v', long = "verbose", action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Reduce output (quiet)
    #[arg(short = 'q', long = "quiet", global = true)]
    pub quiet: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Print the provided text (example command)
    Echo { text: String, },
    /// Simple liveness check (example command)
    Ping,
    HarmonyAdd { text:  String, },
}