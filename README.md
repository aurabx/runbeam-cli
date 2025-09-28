# runbeam

A Rust-based command-line interface (CLI) for the runbeam project.

## Features
- Fast native binary with no runtime dependencies
- Clap-based CLI with subcommands
  - `ping` — simple liveness check (prints `pong`)
  - `echo <text>` — prints the provided text
- Verbosity controls: `-v/--verbose` (repeatable) and `-q/--quiet`
- Structured logging via `tracing` (configure with `RUST_LOG` or `-v` flags)

## Getting Started

### Prerequisites
- Rust toolchain (stable). Install via https://rustup.rs

### Build
```sh
cargo build
```

### Run
```sh
# Show help
cargo run -- --help

# Subcommands
cargo run -- ping
cargo run -- echo "Hello from runbeam"

# Verbose / quiet
cargo run -- -v ping
cargo run -- -q ping

# Or with RUST_LOG
RUST_LOG=debug cargo run -- ping
```

### Install locally
```sh
cargo install --path .
runbeam ping
```

## Packaging and Artifacts
We output all temporary artifacts into `./tmp`.

### Release build (optimized)
```sh
cargo build --release
# macOS: optional strip to reduce size
strip -x target/release/runbeam || true
```

### Makefile targets
```sh
make build            # Debug build
make release          # Optimized build
make package-macos    # Package macOS (current arch) → ./tmp/runbeam-macos-<arch>-v<version>.tar.gz
make package-linux    # Package Linux (x86_64 musl) → ./tmp/runbeam-linux-x86_64-v<version>.tar.gz
make package-windows  # Package Windows (x86_64 msvc) → ./tmp/runbeam-windows-x86_64-v<version>.zip
make clean-artifacts  # Remove ./tmp
```

Notes:
- `package-linux` builds a static MUSL binary (`x86_64-unknown-linux-musl`). On macOS, you can optionally install `zig` and `cargo-zigbuild` for easier cross-compiles.
- Checksums (`.sha256`) are generated alongside archives.

## Continuous Integration (GitHub Actions)
- Workflow: `.github/workflows/release.yml`
- Triggers on tags matching `v*` (e.g., `v0.1.0`) and manual dispatch
- Matrix builds and packages:
  - Linux: `x86_64-unknown-linux-musl`
  - macOS: `aarch64-apple-darwin`
  - Windows: `x86_64-pc-windows-msvc`
- Artifacts are uploaded to the GitHub Release for the tag

Tag and push to publish release artifacts:
```sh
git tag v0.1.0
git push origin v0.1.0
```

## Development
- Logging: set `RUST_LOG` (e.g., `RUST_LOG=debug`) or pass `-v`/`-vv` for more verbosity; `-q` for quieter output.
- Tests: none yet. We can add a scaffolding test suite upon request.

## License
TBD. Add the appropriate license to the repository if required (e.g., `LICENSE` file and `license` field in `Cargo.toml`).
