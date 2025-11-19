# Self-Installing Script for runbeam-cli

This directory contains an `install.sh` script that automates the installation of `runbeam-cli` from GitHub Releases.

## Features

- **Multi-platform support**: Detects and installs the correct binary for macOS (aarch64, x86_64), Linux (x86_64), and Windows
- **Automatic binary selection**: Fetches the latest release from GitHub and downloads the appropriate pre-built binary
- **Checksum verification**: Verifies binary integrity using SHA-256 checksums
- **Smart installation**: Detects writable directories and installs to the best location:
  - `~/.local/bin` (if it exists)
  - `/usr/local/bin` (with or without sudo)
  - `~/bin` (fallback, creates if needed)
- **PATH management**: Suggests how to update your PATH if needed
- **User-friendly**: Colorized output with clear status messages

## Installation Methods

### Option 1: Using curl (Recommended)

Install the latest version directly:

```bash
curl -fsSL https://raw.githubusercontent.com/aurabx/runbeam-cli/main/install.sh | bash
```

### Option 2: Using wget

```bash
wget -qO- https://raw.githubusercontent.com/aurabx/runbeam-cli/main/install.sh | bash
```

### Option 3: Manual download and run

```bash
wget https://raw.githubusercontent.com/aurabx/runbeam-cli/main/install.sh
chmod +x install.sh
./install.sh
```

## How It Works

1. **Platform Detection**: Identifies your OS (macOS, Linux, Windows) and CPU architecture
2. **Version Lookup**: Queries GitHub API to find the latest release
3. **Download**: Fetches the appropriate pre-built binary archive for your platform
4. **Verification**: Downloads and validates the SHA-256 checksum
5. **Extraction**: Uncompresses the archive
6. **Installation**: Copies the binary to an appropriate directory in your PATH
7. **Verification**: Tests that the binary runs and shows the version
8. **PATH Guidance**: Suggests how to update your shell configuration if needed

## Supported Platforms

| Platform | Architecture | Status |
|----------|-------------|--------|
| macOS | aarch64 (Apple Silicon) | ✓ Supported |
| macOS | x86_64 (Intel) | ✓ Supported |
| Linux | x86_64 | ✓ Supported |
| Linux | aarch64 | ✗ Not available (build from source) |
| Windows | x86_64 | ✓ Supported |

## Troubleshooting

### "curl: command not found"

Install curl or use wget instead:

```bash
wget -qO- https://raw.githubusercontent.com/aurabx/runbeam-cli/main/install.sh | bash
```

### Binary not in PATH after installation

The script may have installed to `~/bin` if `/usr/local/bin` wasn't writable. Add it to your PATH:

**For zsh** (add to `~/.zshrc`):
```bash
export PATH="$HOME/.local/bin:$HOME/bin:$PATH"
```

**For bash** (add to `~/.bashrc`):
```bash
export PATH="$HOME/.local/bin:$HOME/bin:$PATH"
```

Then reload your shell:
```bash
source ~/.zshrc  # or ~/.bashrc
```

### "Checksum verification failed"

This indicates the downloaded file is corrupted or the checksum file is missing. The script will skip verification if the checksum file isn't available, but if you see this error during verification, try again:

```bash
curl -fsSL https://raw.githubusercontent.com/aurabx/runbeam-cli/main/install.sh | bash
```

### macOS Security Warning

On macOS, you may see a Gatekeeper warning. The binary is code-signed by Aurabox. You can proceed or add it to exceptions:

```bash
xattr -d com.apple.quarantine ~/.local/bin/runbeam
# or wherever the binary was installed
```

## Hosting the Script

To make the install script available globally, you need to:

1. **Upload to a web server** that supports raw file serving:
   - GitHub (via raw.githubusercontent.com)
   - CDN like jsDelivr or unpkg
   - Your own web server

2. **For GitHub**:
   - Commit and push `install.sh` to your repository
   - The script will be accessible at: `https://raw.githubusercontent.com/aurabx/runbeam-cli/main/install.sh`

3. **For other hosting**, update the download URL in any documentation

## Security Considerations

- The script uses `set -e` to exit on errors
- All downloads are over HTTPS
- Checksums are verified (unless unavailable)
- The script uses `mktemp` for secure temporary directories
- Binaries are made executable only after extraction
- Installation uses `sudo` only when necessary

## Script Customization

To use this script for a different project, modify these variables at the top of `install.sh`:

```bash
REPO="owner/repo-name"           # Your GitHub repo
BINARY_NAME="myapp"              # Your binary name
```

The rest of the script will adapt automatically based on your Makefile's packaging conventions (assuming it follows the same naming pattern).

## Uninstallation

To remove runbeam-cli:

```bash
rm ~/.local/bin/runbeam  # or /usr/local/bin/runbeam
```

To also remove configuration:

```bash
rm -rf ~/.runbeam
```
