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

## Authentication Commands

### login

Log in to Runbeam via browser authentication. Opens a browser window for OAuth authentication and saves the JWT token to `~/.runbeam/auth.json`.

The login process:
1. Requests a device token from the API
2. Opens your browser to the authentication page
3. Polls the server every 5 seconds until authentication completes
4. Saves the JWT token locally

Usage:
```sh
runbeam login
```

### logout

Log out and clear stored authentication. Removes the JWT token from `~/.runbeam/auth.json`.

Usage:
```sh
runbeam logout
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

These commands are used to manage Harmony instances via the management API.

### harmony:add

Register a new Harmony instance.

Options:
- `-i, --ip <IP>`: IP address of the instance [default: 127.0.0.1]
- `-p, --port <PORT>`: Port of the instance [default: 8081]
- `-l, --label <LABEL>`: Internal label; defaults to "ip:port" if not provided
- `-x, --path-prefix <PATH_PREFIX>`: Path prefix for the management API [default: admin]

Examples:
```sh
runbeam harmony:add -i 127.0.0.1 -p 8081 -x admin -l my-label
runbeam harmony:add -i 192.168.1.100 -p 8082 -l production
```

### harmony:list

List all registered Harmony instances from the local data directory.

Output is a table with headers: ID, LABEL, IP, PORT, PREFIX.

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

Authorize a Harmony instance to communicate with Runbeam Cloud. This exchanges your user token for a machine-scoped token that the Harmony instance can use.

**Prerequisites**: You must be logged in (`runbeam login`) before authorizing a Harmony instance.

Authorization flow:
1. Uses your user authentication token from `runbeam login`
2. Calls the Harmony management API with your token
3. Harmony exchanges your token for a machine-scoped token (30-day expiry)
4. Harmony stores the machine token for future API calls

Options:
- `--id <ID>`: Select instance by short ID (conflicts with --label)
- `-l, --label <LABEL>`: Select instance by label (conflicts with --id)

Examples:
```sh
# Authorize by instance ID
runbeam harmony:authorize --id 1a2b3c4d

# Authorize by label
runbeam harmony:authorize -l my-label
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
