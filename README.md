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
shasum -a 256 runbeam-macos-aarch64-v0.3.0.tar.gz
cat runbeam-macos-aarch64-v0.3.0.tar.gz.sha256

# Extract and install
tar -xzf runbeam-macos-aarch64-v0.3.0.tar.gz
chmod +x runbeam
mv runbeam /usr/local/bin
# Or: mv runbeam ~/.local/bin and ensure ~/.local/bin is on PATH
```

### Linux
```sh
# Verify checksum
sha256sum runbeam-linux-x86_64-v0.3.0.tar.gz
cat runbeam-linux-x86_64-v0.3.0.tar.gz.sha256

# Extract and install
tar -xzf runbeam-linux-x86_64-v0.3.0.tar.gz
chmod +x runbeam
sudo mv runbeam /usr/local/bin
# Or: mv runbeam ~/.local/bin and ensure it is on PATH
```

### Windows
```powershell
# Verify checksum
certutil -hashfile runbeam-windows-x86_64-v0.3.0.zip SHA256

# Extract the ZIP using Explorer or PowerShell
Expand-Archive .\runbeam-windows-x86_64-v0.3.0.zip -DestinationPath .
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
3. Log in with your Runbeam account (via OIDC/SSO)
4. Authorize the CLI access
5. Return to your terminal - you're now authenticated!

The authentication token is stored securely at:
- **macOS and Linux**: `~/.runbeam/auth.json`
- **Windows**: `%APPDATA%\runbeam\auth.json`

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
2. CLI retrieves the encryption key from secure OS keyring (if configured)
3. CLI calls the Runbeam Cloud API to authorize the gateway
4. Runbeam Cloud issues a machine-scoped token (30-day expiry)
5. CLI sends the token and encryption key to Harmony
6. Harmony stores the machine token encrypted with the provided key
7. Harmony can now make authenticated API calls to Runbeam Cloud

**Security Model:**
- User tokens are short-lived (used only for authorization)
- Machine tokens are encrypted at rest using age X25519 encryption
- Each Harmony instance can have its own encryption key
- Encryption keys are stored securely in OS keyring (macOS Keychain, Linux Secret Service, Windows Credential Manager)
- You can revoke a Harmony instance's access independently
- Tokens can be renewed before expiry

## Encryption Key Management

The CLI can manage encryption keys for Harmony instances, providing control over how machine tokens are encrypted:

### Setting Encryption Keys

```sh
# Set key during instance creation
runbeam harmony:add -i 192.168.1.100 -p 9090 -l production --key "AGE-SECRET-KEY-1ABC..."

# Set or update key for existing instance
runbeam harmony:set-key --id abc123de --key "AGE-SECRET-KEY-1ABC..."
```

### Viewing Encryption Keys

```sh
# Display the encryption key for backup/migration
runbeam harmony:show-key --id abc123de
```

⚠️ **Warning**: The encryption key is sensitive information. Store it securely (password manager, secrets vault).

### Deleting Encryption Keys

```sh
# Remove encryption key from keyring
runbeam harmony:delete-key --id abc123de
```

After deleting a key, Harmony will automatically generate a new one on next authorization.

### Key Storage

Encryption keys are stored in your OS keyring:
- **macOS**: Keychain
- **Linux**: Secret Service API (freedesktop.org)
- **Windows**: Credential Manager

Keys are associated with Harmony instance IDs and automatically used during authorization.

### When to Use Custom Keys

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

Homepage: https://harmonyproxy.com  
Repository: https://github.com/aurabx/runbeam-cli
