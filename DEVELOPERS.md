# Developers Guide

This document is for contributors and maintainers of the Runbeam CLI.

## Prerequisites

- Rust toolchain (stable) via [rustup](https://rustup.rs)
- Make for packaging tasks
- Optional cross-compilation tools for Linux musl or cargo-zigbuild

Install Rust:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
```

## Build and Run

```sh
cargo build                    # Debug build
cargo build --release         # Release build (optimized)
cargo run -- --help           # Show help
```

Example commands:
```sh
cargo run -- list
cargo run -- harmony:add -i 127.0.0.1 -p 8081 -x admin -l dev
cargo run -- harmony:list
cargo run -- harmony:info -l dev
```

### Install locally
```sh
cargo install --path .
runbeam --help
```

## Lint and Format

```sh
cargo fmt                      # Format code
cargo clippy -- -D warnings   # Lint with warnings as errors
```

## Tests

- No tests exist yet
- The `/samples` directory is intended for future test files when a test suite is added

## Packaging

Makefile targets:
```sh
make build                     # Debug build
make release                   # Release build
make package-macos             # Package for macOS (current arch) → ./tmp/runbeam-macos-<arch>-v<version>.tar.gz
make package-linux             # Package for Linux x86_64 musl → ./tmp/runbeam-linux-x86_64-v<version>.tar.gz
make package-windows           # Package for Windows x86_64 → ./tmp/runbeam-windows-x86_64-v<version>.zip
make clean-artifacts           # Remove ./tmp directory
```

Outputs:
- All artifacts and archives are written to `./tmp/`
- SHA-256 checksums are generated alongside archives as `.sha256` files

Cross-compilation:
- Linux packaging uses static musl builds (`x86_64-unknown-linux-musl`) or cargo-zigbuild
- Ensure the appropriate toolchain is installed for musl or configure cargo-zigbuild
- On macOS, you can optionally install `zig` and `cargo-zigbuild` for easier cross-compiles

## CI and Release Process

- Git tags matching `v*` pattern trigger the release workflow (e.g., `v0.1.0`)
- GitHub Actions builds for Linux (musl), macOS (aarch64), Windows (msvc)
- Built artifacts are uploaded to the GitHub Release along with checksums

Tag and push to publish release artifacts:
```sh
git tag v0.1.0
git push origin v0.1.0
```

## Logging and Verbosity

- Structured logging via `tracing` and `tracing-subscriber`
- Global flags control verbosity: `-v`/`-vv`/`-vvv` and `-q`
- You can override with `RUST_LOG` environment variable

Examples:
```sh
RUST_LOG=debug cargo run -- list
cargo run -- -v harmony:add -i 127.0.0.1 -p 8081
cargo run -- -q harmony:list
```

## Project Conventions and References

- Binary name is `runbeam` as configured in Cargo.toml
- Use `./tmp/` for temporary files and packaging artifacts (not system `/tmp`)
- Schema directory locations are configurable and can point to custom paths
- Encryption (when implemented): AES-256-GCM with ephemeral public key, IV and auth tag base64-encoded
- The `/samples` directory exists for future tests
- CLI stores data in user-specific directory: `~/.runbeam/` on Linux/macOS, equivalent on Windows

For quick developer commands, architecture notes, and detailed build instructions, see [WARP.md](WARP.md) in this repository.

## Architecture Overview

### Entry Point and Flow
- `main.rs`: Initializes structured logging, parses CLI via clap, dispatches to command handlers
- Global flags: `-v`/`--verbose` (repeatable) and `-q`/`--quiet` control logging levels

### Command Organization
- Commands are organized under `src/commands/` as modules
- Current structure:
  - `src/commands/basic.rs`: Contains `list` command
  - `src/commands/harmony/`: Contains harmony-related commands
- Command dispatch happens in `main.rs` matching clap subcommands to handler functions

### Adding New Commands
1. Create module in `src/commands/` and declare in `src/commands/mod.rs`
2. Add clap `Subcommand` variant in `src/cli.rs`
3. Add dispatch logic in `main.rs` match statement

## Development Workflow

1. Make changes to code
2. Test locally: `cargo run -- --help` and test commands
3. Format and lint: `cargo fmt && cargo clippy -- -D warnings`
4. Test packaging if needed: `make clean-artifacts && make package-macos`
5. Commit and push changes
6. For releases, tag and push to trigger CI build