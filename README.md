# Runbeam CLI

CLI for managing Runbeam and Harmony.

Homepage: https://harmonyproxy.com  
Repository: https://github.com/aurabx/runbeam-cli

## Installation

Download a prebuilt binary from [GitHub Releases](https://github.com/aurabx/runbeam-cli/releases):
- Visit the Releases page for this project
- Choose the archive for your OS and architecture
- Verify checksum and place the binary in your PATH

### macOS
```sh
# Verify checksum
shasum -a 256 runbeam-macos-aarch64-v0.2.0.tar.gz
cat runbeam-macos-aarch64-v0.2.0.tar.gz.sha256

# Extract and install
tar -xzf runbeam-macos-aarch64-v0.2.0.tar.gz
chmod +x runbeam
mv runbeam /usr/local/bin
# Or: mv runbeam ~/.local/bin and ensure ~/.local/bin is on PATH
```

### Linux
```sh
# Verify checksum
sha256sum runbeam-linux-x86_64-v0.2.0.tar.gz
cat runbeam-linux-x86_64-v0.2.0.tar.gz.sha256

# Extract and install
tar -xzf runbeam-linux-x86_64-v0.2.0.tar.gz
chmod +x runbeam
sudo mv runbeam /usr/local/bin
# Or: mv runbeam ~/.local/bin and ensure it is on PATH
```

### Windows
```powershell
# Verify checksum
certutil -hashfile runbeam-windows-x86_64-v0.2.0.zip SHA256

# Extract the ZIP using Explorer or PowerShell
Expand-Archive .\runbeam-windows-x86_64-v0.2.0.zip -DestinationPath .
# Move runbeam.exe to a folder on your PATH or add the folder to PATH
```

### Install from Crates.io
```sh
cargo install runbeam-cli
```

### Install from Source
```sh
# Using a local checkout
cargo install --path .

# Or install directly from Git
cargo install --git https://github.com/aurabx/runbeam-cli
```

## Quickstart

```sh
# List available commands
runbeam list

# Add a Harmony instance
runbeam harmony:add -i 127.0.0.1 -p 8081 -x admin -l my-label

# List registered instances
runbeam harmony:list

# Query instance info
runbeam harmony:info -l my-label
runbeam harmony:pipelines -l my-label
runbeam harmony:routes -l my-label
```

## Data Directory

The CLI stores its data in a user-specific file:
- **macOS and Linux**: `~/.runbeam/harmony.json`
- **Windows**: `%APPDATA%\runbeam\harmony.json`

You can remove entries using the CLI:
```sh
# Remove by label
runbeam harmony:remove -l my-label

# Remove by address
runbeam harmony:remove -i 127.0.0.1 -p 8081
```

You may also edit the JSON file directly if needed. Ensure the file remains valid JSON.

## Logging and Verbosity

- Increase verbosity with `-v`, `-vv`, or `-vvv`
- Quiet mode with `-q`
- Alternatively set `RUST_LOG` environment variable

Examples:
```sh
runbeam -v list
runbeam -q list
RUST_LOG=debug runbeam list
```

## Command Reference

Short overview of available commands:
- **list**: Show all available commands
- **harmony:add**: Register a Harmony instance using IP, port, optional path prefix, and label
- **harmony:list**: List all registered Harmony instances
- **harmony:remove**: Remove a registered instance by label or by IP and port
- **harmony:info**: Show info for a registered instance selected by label or ID
- **harmony:pipelines**: List pipelines for an instance selected by label or ID
- **harmony:routes**: List routes for an instance selected by label or ID

See [docs/commands.md](docs/commands.md) for full details of all options and examples.

## License

Apache-2.0

Homepage: https://harmonyproxy.com  
Repository: https://github.com/aurabx/runbeam-cli
