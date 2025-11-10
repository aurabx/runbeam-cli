# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Common Commands

### Build and Run
```sh
cargo build                    # Debug build
cargo build --release         # Release build (optimized)
cargo run -- --help           # Show help
cargo run -- ping             # Test with ping command
cargo run -- echo "text"      # Test with echo command  
cargo install --path .        # Install locally
```

### Lint and Format
```sh
cargo fmt                      # Format code
cargo clippy -- -D warnings   # Lint with warnings as errors
```

### Tests
```sh
cargo test                     # Run all tests
cargo test <pattern>           # Run specific tests by name
cargo test --lib               # Run library/unit tests only
cargo test --test '*'          # Run integration tests only
cargo test -- --nocapture      # Show stdout/stderr from tests
```

**Test Organization:**
- **Unit tests**: `src/storage_tests.rs` - Tests for storage module (serialization, ID generation, path handling)
- **CLI parsing tests**: `tests/cli_parsing_test.rs` - Tests for command-line argument parsing with clap
- **Integration tests**: `tests/integration_test.rs` - End-to-end CLI invocation tests using assert_cmd
- **Routes table test**: `tests/routes_table_test.rs` - JSON structure validation for harmony routes

**Test Dependencies:**
- `tempfile` - For creating temporary directories in tests
- `assert_cmd` - For testing CLI binary execution
- `predicates` - For assertion predicates in integration tests

### Packaging
```sh
make build                     # Debug build
make release                   # Release build
make package-macos             # Package for macOS (current arch) → ./tmp/runbeam-macos-<arch>-v<version>.tar.gz
make package-linux             # Package for Linux x86_64 musl → ./tmp/runbeam-linux-x86_64-v<version>.tar.gz
make package-windows           # Package for Windows x86_64 → ./tmp/runbeam-windows-x86_64-v<version>.zip
make clean-artifacts           # Remove ./tmp directory
```

All packaging outputs:
- Binaries and archives in `./tmp/`
- SHA-256 checksums generated alongside archives (`.sha256` files)
- Linux cross-compilation requires musl toolchain or `cargo-zigbuild`

### Logging and Verbosity
```sh
RUST_LOG=debug cargo run -- ping    # Set log level via environment
cargo run -- -v ping                # Increase verbosity (-v, -vv, -vvv)
cargo run -- -q ping                # Quiet mode (warnings only)
```

### Configuration
```sh
runbeam config:get                            # Show all configuration
runbeam config:get api-url                    # Show specific config value
runbeam config:set api-url https://api.runbeam.com  # Set API URL
runbeam config:unset api-url                  # Unset API URL (revert to env or default)
```

### Harmony Management
```sh
runbeam harmony:add --ip 127.0.0.1 --port 9090 --label my-harmony  # Register Harmony instance
runbeam harmony:list                                                # List registered instances
runbeam harmony:authorize --label my-harmony                        # Authorize with Runbeam Cloud
runbeam harmony:update --label my-harmony                           # Upload config to Runbeam Cloud
runbeam harmony:update --id abc123                                  # Update by short ID
runbeam harmony:info --label my-harmony                             # Get system information
runbeam harmony:routes --label my-harmony                           # List routes
runbeam harmony:pipelines --label my-harmony                        # List pipelines
```

Configuration priority (highest to lowest):
1. Config file (`~/.runbeam/config.json`)
2. Environment variable (`RUNBEAM_API_URL`)
3. Default (`http://runbeam.lndo.site`)

## High-Level Architecture

### Entry Point and Flow
- `main.rs`: Initializes structured logging (tracing-subscriber with EnvFilter), parses CLI via clap, dispatches to command handlers
- Global flags: `-v`/`--verbose` (repeatable) and `-q`/`--quiet` control logging levels

### Command Organization
- Commands are organized under `src/commands/` as modules
- Current structure:
  - `src/commands/auth.rs`: Authentication commands (login, logout)
  - `src/commands/basic.rs`: Basic utility commands
  - `src/commands/config.rs`: Configuration management (set, get, unset)
  - `src/commands/harmony/`: Harmony proxy management commands
- Command dispatch happens in `main.rs` matching clap subcommands to handler functions

### Adding New Commands
1. Create module in `src/commands/` and declare in `src/commands/mod.rs`
2. Add clap `Subcommand` variant in `src/cli.rs`
3. Add dispatch logic in `main.rs` match statement

### Conventions
- Binary name: `runbeam` (configured in Cargo.toml `[[bin]]`)
- Temporary files and packaging artifacts: `./tmp/` directory
- Release build optimizations: LTO thin, panic=abort, opt-level="z"
- Dependencies:
  - `clap` (CLI argument parsing)
  - `anyhow` (error handling)
  - `tracing` + `tracing-subscriber` (structured logging)
  - `reqwest` (HTTP client)
  - `open` (browser opening for OAuth)
  - `runbeam-sdk` (API client and secure token storage)
- Configuration: CLI configuration stored at `~/.runbeam/config.json`
  - API URL precedence: config file > `RUNBEAM_API_URL` environment variable > default (`http://runbeam.lndo.site`)
  - Managed via `runbeam config:set`, `config:get`, and `config:unset` commands
  - Example: `runbeam config:set api-url https://api.runbeam.com`
- Authentication: Browser-based OAuth flow with device tokens (similar to Heroku/Fly.io)
  - Uses configured API URL from config file, environment variable, or default
  - Polls server every 5 seconds with 10-minute timeout
  - **Secure Token Storage** (v0.3.0+):
    - User tokens stored securely via `runbeam-sdk` generic storage
    - Primary: OS keyring (Keychain, Secret Service, Credential Manager)
    - Fallback: Encrypted filesystem storage using ChaCha20-Poly1305 AEAD
    - Automatic migration from legacy plaintext `~/.runbeam/auth.json` to secure storage
    - Tokens stored at identifier: `runbeam/user_token.json` (keyring) or `~/.runbeam/user_token.json` (encrypted)
  - Token verification: RS256 JWT validation with JWKS endpoint discovery
- Harmony Authorization: Two-phase authentication model
  - Phase 1: User authenticates via `runbeam login` (short-lived token)
  - Phase 2: User authorizes Harmony instance via `runbeam harmony:authorize`
  - Harmony exchanges user token for machine-scoped token (30-day expiry)
  - Separation of concerns: user identity vs machine identity

## CI/Release Process

- Triggered on git tags matching `v*` pattern (e.g., `v0.1.0`)
- Multi-platform builds: Linux (musl), macOS (aarch64), Windows (msvc)
- Packages binaries into `./tmp/`, generates checksums, uploads to GitHub Release

## Secure Storage Architecture

### Overview

The CLI uses the `runbeam-sdk` for secure token storage with automatic backend selection and encryption:

**Storage Backend Selection:**
1. First attempts OS keyring (Keychain/Secret Service/Credential Manager)
2. Falls back to encrypted filesystem storage if keyring unavailable
3. Encryption is transparent - no user configuration needed

**Token Types:**
- **User tokens**: Authentication tokens from `runbeam login` (stored as `runbeam/user_token.json`)
- **Machine tokens**: Gateway authorization tokens (stored as `runbeam/machine_token.json`)

### Security Features

**OS Keyring Storage (Primary):**
- macOS: Keychain
- Linux: Secret Service API (freedesktop.org)
- Windows: Credential Manager
- No encryption needed (OS handles security)

**Encrypted Filesystem Storage (Fallback):**
- Algorithm: ChaCha20-Poly1305 AEAD
- Key derivation: Argon2id with random salt
- Encryption key stored in OS keyring at `runbeam/encryption_key`
- Token files: `~/.runbeam/user_token.json`, `~/.runbeam/machine_token.json`

### Migration from Legacy Storage

The CLI automatically migrates legacy plaintext tokens on first run:

**Migration Flow:**
1. Detects legacy file: `~/.runbeam/auth.json` (plaintext)
2. Loads token and user info from legacy file
3. Saves token to secure storage using SDK generic functions
4. Removes legacy plaintext file
5. Future operations use secure storage only

**Implementation:**
```rust
// src/storage.rs - automatic migration
if legacy_path.exists() {
    // Load from plaintext file
    let legacy_token = load_legacy_token(&legacy_path)?;
    
    // Save to secure storage
    save_token_generic(&secure_storage, &legacy_token, "user_token").await?;
    
    // Remove plaintext file
    fs::remove_file(legacy_path)?;
    info!("Migrated token to secure storage");
}
```

### Storage Module

The `src/storage.rs` module provides:

**Token Operations:**
- `save_user_token()` - Save user authentication token securely
- `load_user_token()` - Load user token from secure storage
- `clear_user_token()` - Remove user token from storage
- Legacy migration handled automatically in `load_user_token()`

**Backend Selection:**
```rust
fn create_storage() -> Box<dyn StorageBackend> {
    match KeyringStorage::new("runbeam") {
        keyring if keyring.is_available() => Box::new(keyring),
        _ => Box::new(FilesystemStorage::new()),
    }
}
```

## Project Conventions

- CLI built in Rust with binary named `runbeam`
- Use `./tmp/` for temporary files and packaging artifacts (not system `/tmp`)
- Schema validation directories are configurable (can point to `../jmix` or custom paths)
- A `/samples` directory is intended for test files when test suite is added
- **Security**: All tokens stored using SDK secure storage (keyring or encrypted filesystem)
- **Legacy support**: Automatic migration from plaintext `~/.runbeam/auth.json` to secure storage
