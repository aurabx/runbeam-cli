# Runbeam CLI

CLI for managing Runbeam and Harmony.

Homepage: https://runbeam.io <br />
Repository: https://github.com/aurabx/runbeam-cli

## Installation

### Quick Install (Recommended)

Use the automated installation script:

```bash
# macOS and Linux
curl -fsSL https://raw.githubusercontent.com/aurabx/runbeam-cli/main/install.sh | bash
```

Or with wget:
```bash
wget -qO- https://raw.githubusercontent.com/aurabx/runbeam-cli/main/install.sh | bash
```

The script will:
- Detect your platform and architecture automatically
- Download the latest release
- Verify checksums
- Install to an appropriate location (`~/.local/bin`, `/usr/local/bin`, or `~/bin`)
- Provide PATH configuration guidance if needed

### Manual Installation

Download a prebuilt binary from [GitHub Releases](https://github.com/aurabx/runbeam-cli/releases):

**macOS/Linux:**
```sh
# Download for your platform (example: macOS Apple Silicon)
curl -LO https://github.com/aurabx/runbeam-cli/releases/latest/download/runbeam-aarch64-apple-darwin.tar.gz

# Extract and install
tar -xzf runbeam-aarch64-apple-darwin.tar.gz
chmod +x runbeam
mv runbeam ~/.local/bin/  # or /usr/local/bin/
```

**Windows:**
```powershell
# Download and extract the ZIP from GitHub Releases
# Move runbeam.exe to a folder on your PATH
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

# Verify your authentication token (optional)
runbeam verify

# Add a Harmony instance
runbeam harmony:add -i 127.0.0.1 -p 8081 -x admin -l my-label

# Add with a custom encryption key (optional)
runbeam harmony:add -i 127.0.0.1 -p 8081 -l production --key "AGE-SECRET-KEY-1ABC..."

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

# Verify stored authentication token
runbeam verify

# Log out (clears stored token)
runbeam logout
```

**Authentication Flow:**
1. Run `runbeam login`
2. Your browser opens to the Runbeam authentication page
3. Log in with your Runbeam account
4. Authorize the CLI access
5. Return to your terminal - you're now authenticated!

**Secure Token Storage:**

Authentication tokens are stored using encrypted filesystem storage:
- **macOS and Linux**: `~/.runbeam/<instance_id>/auth.json` (encrypted with age encryption)
- **Windows**: `%APPDATA%\runbeam\<instance_id>\auth.json` (encrypted with age encryption)

Encryption keys are sourced from:
1. `RUNBEAM_ENCRYPTION_KEY` environment variable, or
2. Auto-generated at `~/.runbeam/<instance_id>/encryption.key`

**Token Verification:**

The CLI automatically verifies tokens during login using RS256 asymmetric cryptography:
- Tokens are validated using public keys from the JWKS endpoint
- Supports key rotation via Key ID (`kid`)
- JWKS keys are cached for 1 hour (configurable via `RUNBEAM_JWKS_TTL` environment variable)

You can manually verify your token at any time:
```sh
runbeam verify
```

**Environment Variables:**
- `RUNBEAM_API_URL`: Override the API base URL (default: `http://runbeam.lndo.site`)
- `RUNBEAM_JWKS_TTL`: JWKS cache duration in seconds (default: `3600` = 1 hour)

## When to Use Custom Keys

**Use custom encryption keys when:**
- Migrating Harmony instances between machines
- Implementing key rotation policies
- Meeting compliance requirements for key management
- Running multiple Harmony instances with consistent encryption

**Use auto-generated keys (default) when:**
- Running a single local Harmony instance
- No specific compliance requirements
- Simplicity is preferred

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

Homepage: https://runbeam.io  
Repository: https://github.com/aurabx/runbeam-cli
