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
    /// List available commands
    List,

    /// Log in to Runbeam via browser authentication
    Login,

    /// Log out and clear stored authentication
    Logout,

    /// Verify the stored authentication token
    Verify,

    /// Add a new Harmony instance via the management API
    #[command(name = "harmony:add")]
    HarmonyAdd {
        /// IP address of the instance
        #[arg(short = 'i', long = "ip", default_value = "127.0.0.1")]
        ip: String,
        /// Port of the instance
        #[arg(short = 'p', long = "port", default_value_t = 8081)]
        port: u16,
        /// Internal label; defaults to "ip:port" if not provided
        #[arg(short = 'l', long = "label")]
        label: Option<String>,
        /// Path prefix for the management API (e.g. "admin")
        #[arg(short = 'x', long = "path-prefix", default_value = "admin")]
        path_prefix: String,
        /// Base64-encoded encryption key for token storage (optional)
        #[arg(short = 'k', long = "key")]
        encryption_key: Option<String>,
    },

    /// List registered Harmony instances
    #[command(name = "harmony:list")]
    HarmonyList,

    /// Remove a registered Harmony instance by ID, label, or ip:port
    #[command(name = "harmony:remove")]
    HarmonyRemove {
        /// Remove by ID (conflicts with --label/--ip/--port)
        #[arg(long = "id", conflicts_with_all = ["label", "ip", "port"])]
        id: Option<String>,
        /// Remove by label (conflicts with --id/--ip/--port)
        #[arg(short = 'l', long = "label", conflicts_with_all = ["id", "ip", "port"])]
        label: Option<String>,
        /// Remove by IP (requires --port)
        #[arg(short = 'i', long = "ip", requires = "port")]
        ip: Option<String>,
        /// Remove by port (requires --ip)
        #[arg(short = 'p', long = "port", requires = "ip")]
        port: Option<u16>,
    },

    /// Call management API: GET /{prefix}/info
    #[command(name = "harmony:info")]
    HarmonyInfo {
        /// Select instance by short ID
        #[arg(long = "id", conflicts_with = "label")]
        id: Option<String>,
        /// Select instance by label
        #[arg(short = 'l', long = "label", conflicts_with = "id")]
        label: Option<String>,
    },

    /// Call management API: GET /{prefix}/pipelines
    #[command(name = "harmony:pipelines")]
    HarmonyPipelines {
        /// Select instance by short ID
        #[arg(long = "id", conflicts_with = "label")]
        id: Option<String>,
        /// Select instance by label
        #[arg(short = 'l', long = "label", conflicts_with = "id")]
        label: Option<String>,
    },

    /// Call management API: GET /{prefix}/routes
    #[command(name = "harmony:routes")]
    HarmonyRoutes {
        /// Select instance by short ID
        #[arg(long = "id", conflicts_with = "label")]
        id: Option<String>,
        /// Select instance by label
        #[arg(short = 'l', long = "label", conflicts_with = "id")]
        label: Option<String>,
        /// Output raw JSON instead of table
        #[arg(long = "json")]
        json: bool,
    },

    /// Reload the Harmony instance configuration
    #[command(name = "harmony:reload")]
    HarmonyReload {
        /// Select instance by short ID
        #[arg(long = "id", conflicts_with = "label")]
        id: Option<String>,
        /// Select instance by label
        #[arg(short = 'l', long = "label", conflicts_with = "id")]
        label: Option<String>,
    },

    /// Authorize a Harmony instance to communicate with Runbeam Cloud
    #[command(name = "harmony:authorize")]
    HarmonyAuthorize {
        /// Select instance by short ID
        #[arg(long = "id", conflicts_with = "label")]
        id: Option<String>,
        /// Select instance by label
        #[arg(short = 'l', long = "label", conflicts_with = "id")]
        label: Option<String>,
    },

    /// Set or update the encryption key for a Harmony instance
    #[command(name = "harmony:set-key")]
    HarmonySetKey {
        /// Harmony instance ID
        #[arg(long = "id")]
        id: String,
        /// Base64-encoded encryption key
        #[arg(short = 'k', long = "key")]
        encryption_key: String,
    },

    /// Show the encryption key for a Harmony instance
    #[command(name = "harmony:show-key")]
    HarmonyShowKey {
        /// Harmony instance ID
        #[arg(long = "id")]
        id: String,
    },

    /// Delete the encryption key for a Harmony instance
    #[command(name = "harmony:delete-key")]
    HarmonyDeleteKey {
        /// Harmony instance ID
        #[arg(long = "id")]
        id: String,
    },

    /// Test browser opening (development only)
    #[command(name = "test-browser")]
    TestBrowser,

    /// Set a configuration value
    #[command(name = "config:set")]
    ConfigSet {
        /// Configuration key (e.g., "api-url")
        key: String,
        /// Configuration value
        value: String,
    },

    /// Get a configuration value or show all config
    #[command(name = "config:get")]
    ConfigGet {
        /// Configuration key (optional, shows all if not provided)
        key: Option<String>,
    },

    /// Unset a configuration value
    #[command(name = "config:unset")]
    ConfigUnset {
        /// Configuration key to unset
        key: String,
    },
}
