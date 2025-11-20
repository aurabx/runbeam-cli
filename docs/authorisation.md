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

## Encryption Key Management

**Note:** As of CLI v0.7.0, encryption keys are managed automatically by the SDK using filesystem storage.

The Runbeam SDK automatically manages encryption keys for secure token storage:
- Keys are generated automatically on first use
- Keys are stored at `~/.runbeam/<instance_id>/encryption.key` or provided via `RUNBEAM_ENCRYPTION_KEY` environment variable
- Keys are used transparently for encrypted filesystem token storage
- No manual key management is required