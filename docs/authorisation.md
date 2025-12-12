# Harmony Authorization

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

## Out-of-Band Token Generation (CI/CD)

For CI/CD pipelines and containerized deployments where browser-based authentication isn't possible or a Harmony instance isn't running, use `token:get` to obtain a machine token directly:

```sh
# Get a machine token for a gateway code (with full output)
runbeam token:get -g my-gateway-code

# For scripting, use --raw to output only the token
runbeam token:get -g my-gateway-code --raw

# Capture as environment variable
export RUNBEAM_MACHINE_TOKEN=$(runbeam token:get -g my-gateway-code --raw)
```

This command:
- Does not require a registered Harmony instance
- Does not attempt to send the token to a Harmony proxy
- Creates the gateway if it doesn't exist
- Outputs the token for capture and injection as an environment variable

### Alternative: API Token Method

You can also create a Sanctum API token in the Runbeam Cloud UI and call the API directly:

```sh
curl -X POST https://your-runbeam-instance/api/harmony/authorize \
  -H "Authorization: Bearer <sanctum_api_token>" \
  -H "Content-Type: application/json" \
  -d '{"gateway_code": "my-gateway-code"}'
```

The response includes the `machine_token` field which can be extracted and used.

## Encryption Key Management

**Note:** As of CLI v0.7.0, encryption keys are managed automatically by the SDK using filesystem storage.

The Runbeam SDK automatically manages encryption keys for secure token storage:
- Keys are generated automatically on first use
- Keys are stored at `~/.runbeam/<instance_id>/encryption.key` or provided via `RUNBEAM_ENCRYPTION_KEY` environment variable
- Keys are used transparently for encrypted filesystem token storage
- No manual key management is required
