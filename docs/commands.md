# Commands

This file contains a list of the intended commands for this application

## Harmony commands

These commands are used to run harmony instances, via the management API

### harmony:add

Add a new Harmony instance

- `-i, --ip`: The IP address of the instance, defaults to 127.0.0.1
- `-p, --port`: The port the instance is on, defaults to 8081
- `-l, --label`: The internal label, defaults to `ip:port`
- `-x, --path-prefix`: Path prefix for the management API (default `admin`)

Example

```bash
runbeam harmony:add -i 127.0.0.1 -p 8081 -x admin
```

### harmony:list

List all registered Harmony instances (from the runbeam data directory).

Output is a table with headers: ID, LABEL, IP, PORT, PREFIX.

Example

```bash
runbeam harmony:list
```

### harmony:remove

Remove a registered Harmony instance by label or by ip:port.

- Remove by label:
  ```bash
  runbeam harmony:remove -l my-label
  ```
- Remove by address:
  ```bash
  runbeam harmony:remove -i 127.0.0.1 -p 8081
  ```

### harmony:info

Call the management API /{prefix}/info on a specific instance.

Examples

```bash
runbeam harmony:info --id 1a2b3c4d
runbeam harmony:info -l my-label
```

### harmony:pipelines

Call the management API /{prefix}/pipelines on a specific instance.

Examples

```bash
runbeam harmony:pipelines --id 1a2b3c4d
runbeam harmony:pipelines -l my-label
```

### harmony:routes

Call the management API /{prefix}/routes on a specific instance.

Examples

```bash
runbeam harmony:routes --id 1a2b3c4d
runbeam harmony:routes -l my-label
```
