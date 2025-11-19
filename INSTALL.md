# Installing runbeam-cli

Choose your preferred installation method below:

## 🍺 Homebrew (macOS & Linux)

```bash
brew install aurabx/tap/runbeam
```

This is the easiest method for macOS and Linux users. The formula supports both Intel and Apple Silicon Macs.

**Verify installation:**
```bash
runbeam --version
```

**Update to latest:**
```bash
brew upgrade runbeam
```

## 📦 cargo-binstall (All Platforms)

For fast binary installation without compiling:

```bash
cargo install cargo-binstall
cargo binstall runbeam-cli
```

This downloads precompiled binaries for your platform and installs them to `~/.cargo/bin`.

## 🔧 Cargo (All Platforms)

Build and install from source:

```bash
cargo install runbeam-cli
```

This requires Rust to be installed. Visit [rust-lang.org](https://www.rust-lang.org/tools/install) to install Rust if needed.

## 📥 Direct Download (All Platforms)

Download precompiled binaries from [GitHub Releases](https://github.com/aurabx/runbeam-cli/releases):

### macOS
- `runbeam-x86_64-apple-darwin.tar.gz` — Intel Mac
- `runbeam-aarch64-apple-darwin.tar.gz` — Apple Silicon (M1/M2/M3)

### Linux
- `runbeam-x86_64-unknown-linux-musl.tar.gz` — x86_64
- `runbeam-aarch64-unknown-linux-musl.tar.gz` — ARM64

### Windows
- `runbeam-x86_64-pc-windows-msvc.zip` — x86_64

**Extract and use:**

macOS/Linux:
```bash
tar xzf runbeam-<target>.tar.gz
./runbeam --version
# Optional: move to PATH
sudo mv runbeam /usr/local/bin/
```

Windows:
```powershell
Expand-Archive runbeam-x86_64-pc-windows-msvc.zip
.\runbeam\runbeam.exe --version
# Optional: add to PATH
```

## ✅ Verify Installation

After installation, verify it works:

```bash
runbeam --version
runbeam --help
```

## 🔐 Verify Binary Integrity

Each release includes SHA256 checksums. Download the corresponding `.sha256` file and verify:

macOS/Linux:
```bash
shasum -a 256 -c runbeam-<target>.sha256
```

Windows:
```powershell
Get-FileHash runbeam-x86_64-pc-windows-msvc.zip -Algorithm SHA256
# Compare output with contents of runbeam-x86_64-pc-windows-msvc.sha256
```

## 🆘 Troubleshooting

### "command not found: runbeam"

The binary is not in your PATH. Either:
1. Use the full path: `/path/to/runbeam`
2. Add the installation directory to your PATH:
   - **Homebrew**: automatically in PATH
   - **cargo install**: `~/.cargo/bin` should be in PATH (add to shell profile if not)
   - **Manual**: move binary to a directory in PATH (e.g., `/usr/local/bin`)

### Binary not executable (Unix)

Make the binary executable:
```bash
chmod +x runbeam
```

### macOS "cannot be opened because the developer cannot be verified"

macOS may block unsigned binaries. Allow it:
```bash
xattr -d com.apple.quarantine runbeam
chmod +x runbeam
```

### Windows security warning

Windows SmartScreen may warn about the unsigned binary. This is normal for open-source binaries. Click "More info" → "Run anyway" to proceed.

## 📍 Configuration Directory

After first run, configuration is stored at:
- **macOS/Linux**: `~/.runbeam/`
- **Windows**: `%APPDATA%\runbeam\`

Tokens are stored securely using your OS keyring or encrypted filesystem storage (see [Secure Storage Architecture](WARP.md#secure-storage-architecture) in WARP.md).

## 🔄 Updating

- **Homebrew**: `brew upgrade runbeam`
- **cargo-binstall**: `cargo binstall runbeam-cli --force`
- **Cargo**: `cargo install --force runbeam-cli`
- **Manual**: Download new binary from [releases](https://github.com/aurabx/runbeam-cli/releases)

## Getting Started

Once installed, authenticate and manage Harmony instances:

```bash
# Login to Runbeam
runbeam login

# View configuration
runbeam config:get

# List registered Harmony instances
runbeam harmony:list

# Add a new Harmony instance
runbeam harmony:add --ip 127.0.0.1 --port 9090 --label my-harmony

# Get help
runbeam --help
runbeam harmony:add --help
```

For full documentation, see the [main README](README.md) and [WARP.md](WARP.md).
