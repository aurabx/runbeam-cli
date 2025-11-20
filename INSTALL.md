# Installation Guide

## Quick Install (macOS / Linux)

Use the automated installation script:

```bash
curl -fsSL https://raw.githubusercontent.com/aurabx/runbeam-cli/main/install.sh | bash
```

## cargo-binstall

If you have `cargo-binstall` installed:

```bash
cargo binstall runbeam-cli
```

## Manual Installation

Download the latest release from [GitHub Releases](https://github.com/aurabx/runbeam-cli/releases).

### macOS / Linux

```bash
# Download
curl -LO https://github.com/aurabx/runbeam-cli/releases/latest/download/runbeam-x86_64-unknown-linux-musl.tar.gz
# Extract
tar xzf runbeam-x86_64-unknown-linux-musl.tar.gz
# Install
sudo mv runbeam /usr/local/bin/
```

### Windows

Download the `.zip` file, extract it, and add the binary to your PATH.

## Building from Source

```bash
cargo install runbeam-cli
```
