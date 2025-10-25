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

# Authenticate with Runbeam (opens browser)
runbeam login

# Add a Harmony instance
runbeam harmony:add -i 127.0.0.1 -p 8081 -x admin -l my-label

# Authorize the Harmony instance to communicate with Runbeam Cloud
runbeam harmony:authorize -l my-label

# List registered instances
runbeam harmony:list

# Query instance info
runbeam harmony:info -l my-label
runbeam harmony:pipelines -l my-label
runbeam harmony:routes -l my-label

# Logout when done
runbeam logout
```

## Authentication

The CLI uses browser-based OAuth authentication:

```sh
# Log in (opens browser for authentication)
runbeam login

# Log out (clears stored token)
runbeam logout
```

**Authentication Flow:**
1. Run `runbeam login`
2. Your browser opens to the Runbeam authentication page
3. Log in with your Runbeam account (via OIDC/SSO)
4. Authorize the CLI access
5. Return to your terminal - you're now authenticated!

The authentication token is stored securely at:
- **macOS and Linux**: `~/.runbeam/auth.json`
- **Windows**: `%APPDATA%\runbeam\auth.json`

**Environment Variables:**
- `RUNBEAM_API_URL`: Override the API base URL (default: `http://runbeam.lndo.site`)

## Harmony Authorization

After adding a Harmony instance, you need to authorize it to communicate with the Runbeam Cloud API:

```sh
# Authorize a Harmony instance by label
runbeam harmony:authorize -l my-label

# Or by instance ID
runbeam harmony:authorize --id 1a2b3c4d
```

**Authorization Flow:**
1. CLI loads your user authentication token
2. CLI calls the Harmony management API with your token
3. Harmony validates the token and contacts Runbeam Cloud
4. Runbeam Cloud issues a machine-scoped token (30-day expiry)
5. Harmony stores the machine token locally
6. Harmony can now make authenticated API calls to Runbeam Cloud

**Security Model:**
- User tokens are short-lived (used only for authorization)
- Machine tokens are scoped to specific Harmony instances
- You can revoke a Harmony instance's access independently
- Tokens can be renewed before expiry

## Data Directory

The CLI stores configuration data in user-specific files:
- **macOS and Linux**: `~/.runbeam/harmony.json` (Harmony instances), `~/.runbeam/auth.json` (authentication token)
- **Windows**: `%APPDATA%\runbeam\harmony.json`, `%APPDATA%\runbeam\auth.json`

You can remove entries using the CLI:
```sh
# Remove by ID
runbeam harmony:remove --id 1a2b3c4d

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


See [docs/commands.md](docs/commands.md) for full details of all options and examples.

## License

Apache-2.0

Homepage: https://harmonyproxy.com  
Repository: https://github.com/aurabx/runbeam-cli
