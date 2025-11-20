# Commands

This document contains the complete command reference for the Runbeam CLI.

For data directory information and general usage, see the main [README.md](../README.md).

## Basic Commands

### list

Show all available commands.

Usage:
```sh
runbeam list
```

### test-browser (development only)

Test that the CLI can open a browser window on your system.

Usage:
```sh
runbeam test-browser
```

This command is intended for development and troubleshooting only and is not required for normal usage.

## Authentication Commands

Authentication uses secure token storage managed by the Runbeam SDK. Tokens are stored in the OS keyring when available, or in encrypted filesystem storage as a fallback.

Any legacy plaintext token file (`~/.runbeam/auth.json`) is automatically migrated to secure storage on first use.

### login

Log in to Runbeam via browser authentication. Opens a browser window for OAuth authentication and saves the token to secure storage.

The login process:
1. Requests a device token from the API
2. Opens your browser to the authentication page
3. Polls the server every 5 seconds until authentication completes
4. Saves the token to secure storage and verifies it using RS256 with JWKS

If a valid token already exists, the CLI will report that you are already logged in and will not start a new login flow.

Usage:
```sh
runbeam login
```

### logout

Log out and clear stored authentication. Removes the authentication token from secure storage (and any legacy plaintext file, if present).

Usage:
```sh
runbeam logout
```

### verify

Verify the stored authentication token and display decoded information.

This command:
- Loads the current token from secure storage
- Verifies the token using the Runbeam SDK (RS256 + JWKS)
- Prints issuer, subject, audience (if present)
- Prints user and team information (if present)
- Shows when the token expires and how long until expiry

If the token is invalid or expired, a detailed error is shown with guidance to run `runbeam login` again.

Usage:
```sh
runbeam verify
```

## Configuration Commands

The CLI stores configuration in `~/.runbeam/config.json`. Configuration values have the following precedence (highest to lowest):
1. Config file (`~/.runbeam/config.json`)
2. Environment variable (e.g., `RUNBEAM_API_URL`)
3. Default value

### config:set

Set a configuration value.

Arguments:
- `<KEY>`: Configuration key (e.g., "api-url")
- `<VALUE>`: Configuration value

Supported keys:
- `api-url`: The Runbeam API URL (must start with http:// or https://)

Examples:
```sh
runbeam config:set api-url https://api.runbeam.com
runbeam config:set api-url http://localhost:8000
```

### config:get

Get a configuration value or show all configuration.

Arguments:
- `[KEY]`: Optional configuration key (shows all config if not provided)

Examples:
```sh
# Show all configuration
runbeam config:get

# Show specific configuration value
runbeam config:get api-url
```

### config:unset

Unset a configuration value (revert to environment variable or default).

Arguments:
- `<KEY>`: Configuration key to unset

Examples:
```sh
runbeam config:unset api-url
```

## Harmony Commands

These commands are used to manage Harmony instances via the management API and to integrate them with Runbeam Cloud.

### harmony:install

Download and install the Harmony binary.

This command:
1. Detects your operating system and architecture
2. Downloads the appropriate release from GitHub Releases
3. Extracts the binary to a standard location (or custom directory)
4. Sets executable permissions (Unix/macOS)

**Default Install Locations:**
- **macOS**: `~/.local/bin`
- **Linux**: `~/.local/bin` (or standard XDG executable dir)
- **Windows**: `%APPDATA%\runbeam\bin`

Options:
- `--version <VERSION>`: Install a specific version (e.g., "v0.7.0"). Defaults to latest.
- `-d, --dir <DIR>`: Install to a custom directory.

Examples:
```sh
# Install latest version to default location
runbeam harmony:install

# Install specific version
runbeam harmony:install --version v0.6.0

# Install to current directory
runbeam harmony:install --dir .
```

### harmony:add

Register a new Harmony instance.

Options:
- `-i, --ip <IP>`: IP address of the instance [default: 127.0.0.1]
- `-p, --port <PORT>`: Port of the instance [default: 8081]
- `-l, --label <LABEL>`: Internal label; defaults to "ip:port" if not provided
- `-x, --path-prefix <PATH_PREFIX>`: Path prefix for the management API [default: admin]
- `-k, --key <KEY>`: **Deprecated** base64-encoded encryption key. Encryption keys are now managed automatically by the SDK and this option is ignored.

Examples:
```sh
runbeam harmony:add -i 127.0.0.1 -p 8081 -x admin -l my-label
runbeam harmony:add -i 192.168.1.100 -p 8082 -l production
```

### harmony:list

List all registered Harmony instances from the local data directory.

Output is a table with headers: `ID`, `GATEWAY_ID`, `LABEL`, `IP`, `PORT`, `PREFIX`.

Usage:
```sh
runbeam harmony:list
```

### harmony:remove

Remove a registered Harmony instance by ID, label, or by IP:port.

Options:
- `--id <ID>`: Remove by ID (conflicts with --label/--ip/--port)
- `-l, --label <LABEL>`: Remove by label (conflicts with --id/--ip/--port)
- `-i, --ip <IP>`: Remove by IP (requires --port)
- `-p, --port <PORT>`: Remove by port (requires --ip)

Examples:
```sh
# Remove by ID
runbeam harmony:remove --id 1a2b3c4d

# Remove by label
runbeam harmony:remove -l my-label

# Remove by address
runbeam harmony:remove -i 127.0.0.1 -p 8081
```

### harmony:info

Call the management API `GET /{prefix}/info` on a specific instance.

Options:
- `--id <ID>`: Select instance by short ID (conflicts with --label)
- `-l, --label <LABEL>`: Select instance by label (conflicts with --id)

Examples:
```sh
runbeam harmony:info --id 1a2b3c4d
runbeam harmony:info -l my-label
```

### harmony:pipelines

Call the management API `GET /{prefix}/pipelines` on a specific instance.

Options:
- `--id <ID>`: Select instance by short ID (conflicts with --label)
- `-l, --label <LABEL>`: Select instance by label (conflicts with --id)

Examples:
```sh
runbeam harmony:pipelines --id 1a2b3c4d
runbeam harmony:pipelines -l my-label
```

### harmony:routes

Call the management API `GET /{prefix}/routes` on a specific instance.

By default, displays routes as a table. Use `--json` for machine-readable output.

Options:
- `--id <ID>`: Select instance by short ID (conflicts with --label)
- `-l, --label <LABEL>`: Select instance by label (conflicts with --id)
- `--json`: Output raw JSON instead of table

Examples:
```sh
# Display routes as a table (default)
runbeam harmony:routes --id 1a2b3c4d
runbeam harmony:routes -l my-label

# Output raw JSON for machine processing
runbeam harmony:routes --id 1a2b3c4d --json
```

### harmony:reload

Trigger a reload of the Harmony instance configuration by calling `POST /api/reload`.

This command directly calls the reload endpoint on the Harmony instance (bypasses the management API path prefix).

Options:
- `--id <ID>`: Select instance by short ID (conflicts with --label)
- `-l, --label <LABEL>`: Select instance by label (conflicts with --id)

Examples:
```sh
# Reload configuration by instance ID
runbeam harmony:reload --id 1a2b3c4d

# Reload configuration by label
runbeam harmony:reload -l my-label
```

### harmony:authorize

Authorize a Harmony instance to communicate with Runbeam Cloud. This exchanges your user token for a machine-scoped token that the Harmony instance can use and delivers that token to the Harmony proxy.

**Prerequisites**: 
- You must be logged in (`runbeam login`) before authorizing a Harmony instance
- The Harmony instance must have `runbeam.enabled = true` in its configuration file (`config.toml`)

Authorization flow:
1. Uses your user authentication token from `runbeam login`
2. Calls the Runbeam Cloud API to authorize the gateway and issue a machine-scoped token
3. Stores the gateway ID against the Harmony instance locally
4. Sends the machine token to the Harmony proxy at `http://<ip>:<port>/<prefix>/token`

If the Harmony proxy returns `403 Forbidden` and indicates Runbeam is disabled, ensure your Harmony configuration contains:
```toml
[runbeam]
enabled = true
```

Options:
- `--id <ID>`: Select instance by short ID (conflicts with --label)
- `-l, --label <LABEL>`: Select instance by label (conflicts with --id)
- `--update`: After successful authorization, automatically upload configuration using `harmony:update`
- `-y, --yes`: Skip interactive prompts (for CI/automation). When used without `--update`, configuration upload is skipped.

Examples:
```sh
# Authorize by instance ID
runbeam harmony:authorize --id 1a2b3c4d

# Authorize by label
runbeam harmony:authorize -l my-label

# Authorize and automatically upload configuration
runbeam harmony:authorize --id 1a2b3c4d --update

# Non-interactive authorization suitable for CI (no config upload)
runbeam harmony:authorize --id 1a2b3c4d -y
```

### harmony:update

Trigger Harmony to upload its local configuration to Runbeam Cloud by calling the management API `POST /{prefix}/update`.

On success, this command reports the size of the uploaded configuration.

Options:
- `--id <ID>`: Select instance by short ID (conflicts with --label)
- `-l, --label <LABEL>`: Select instance by label (conflicts with --id)

Examples:
```sh
# Upload configuration by ID
runbeam harmony:update --id 1a2b3c4d

# Upload configuration by label
runbeam harmony:update -l my-label
```

### harmony:set-key (deprecated)

Set or update the encryption key for a Harmony instance.

This command is **deprecated**. Encryption keys are now managed automatically by the Runbeam SDK's secure storage backend. The command prints guidance and does not modify any keys.

Usage:
```sh
runbeam harmony:set-key --id <ID> --key <BASE64_KEY>
```

### harmony:show-key (deprecated)

Show the encryption key for a Harmony instance.

This command is **deprecated**. Encryption keys are managed automatically and are not directly accessible. The command prints guidance and does not display any keys.

Usage:
```sh
runbeam harmony:show-key --id <ID>
```

### harmony:delete-key (deprecated)

Delete the encryption key for a Harmony instance.

This command is **deprecated**. Encryption keys are managed automatically by the Runbeam SDK, including their lifecycle. The command prints guidance and does not delete any keys.

Usage:
```sh
runbeam harmony:delete-key --id <ID>
```

## Global Options

The following options are available for all commands:

- `-v, --verbose`: Increase output verbosity (can be repeated: -v, -vv, -vvv)
- `-q, --quiet`: Reduce output (quiet mode)
- `-h, --help`: Print help information
- `-V, --version`: Print version information

Examples:
```sh
runbeam -v harmony:list
runbeam -vv harmony:info -l my-label
runbeam -q harmony:add -i 127.0.0.1 -p 8081
RUST_LOG=debug runbeam harmony:list
```
