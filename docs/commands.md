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
