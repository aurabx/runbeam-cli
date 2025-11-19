# Runbeam CLI

CLI for managing Runbeam and Harmony.

Homepage: https://harmonyproxy.com  
Repository: https://github.com/aurabx/runbeam-cli

## Quick Install

**macOS/Linux (Homebrew):**
```sh
brew tap aurabx/tap
brew install runbeam
```

**All Platforms (Cargo):**
```sh
cargo install runbeam-cli
```

**All Platforms (Binary):**
```sh
cargo install cargo-binstall
cargo binstall runbeam-cli
```

For detailed installation instructions and other methods, see [INSTALL.md](INSTALL.md).

## Installation

See [INSTALL.md](INSTALL.md) for comprehensive installation instructions including:
- 🍺 Homebrew (macOS & Linux)
- 📦 cargo-binstall (All Platforms)
- 🔧 Cargo (All Platforms)
- 📥 Direct binary downloads
- 🔐 Checksum verification
- 🆘 Troubleshooting

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

**Secure Token Storage:**

As of v0.7.0, authentication tokens are stored using encrypted filesystem storage:
- **macOS and Linux**: `~/.runbeam/<instance_id>/auth.json` (encrypted with age encryption)
- **Windows**: `%APPDATA%\runbeam\<instance_id>\auth.json` (encrypted with age encryption)

Encryption keys are sourced from:
1. `RUNBEAM_ENCRYPTION_KEY` environment variable, or
2. Auto-generated at `~/.runbeam/<instance_id>/encryption.key`

**Breaking Change in v0.7.0:**

OS keyring storage (macOS Keychain, Linux Secret Service, Windows Credential Manager) has been removed to simplify dependencies and improve cross-platform compatibility. Existing tokens stored in OS keyring will NOT be automatically migrated.

**Migration from v0.6.x:**

If you're upgrading from v0.6.x or earlier:
1. Run `runbeam login` to re-authenticate
2. Run `runbeam harmony:authorize --label <name>` for each Harmony instance

**Note**: Machine tokens expire after 30 days anyway, so losing keyring-stored tokens has minimal long-term impact.

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
- Encryption keys are auto-generated or provided via `RUNBEAM_ENCRYPTION_KEY` environment variable
- You can revoke a Harmony instance's access independently
- Tokens can be renewed before expiry

## Encryption Key Management

**Note:** As of CLI v0.7.0, encryption keys are managed automatically by the SDK using filesystem storage.

The Runbeam SDK automatically manages encryption keys for secure token storage:
- Keys are generated automatically on first use
- Keys are stored at `~/.runbeam/<instance_id>/encryption.key` or provided via `RUNBEAM_ENCRYPTION_KEY` environment variable
- Keys are used transparently for encrypted filesystem token storage
- No manual key management is required

### Legacy Commands (Deprecated)

The following commands are no longer needed but are retained for backwards compatibility:

```sh
# These commands now show informational messages
runbeam harmony:set-key --id abc123de --key "AGE-SECRET-KEY-1ABC..."
runbeam harmony:show-key --id abc123de
runbeam harmony:delete-key --id abc123de
```

**Migration Note:**
If you previously used custom encryption keys, they are no longer used for user token storage. The SDK handles all encryption keys automatically and securely.

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
