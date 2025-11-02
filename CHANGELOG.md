# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - 2024-11-01

### Added

- **Secure User Token Storage**
  - User authentication tokens now stored securely via `runbeam-sdk` v0.3.2
  - Primary: OS keyring (Keychain/Secret Service/Credential Manager)
  - Fallback: Encrypted filesystem storage with ChaCha20-Poly1305 AEAD
  - Automatic encryption key generation and secure storage
  - Token isolation from machine tokens

- **Automatic Migration**
  - Automatic migration from legacy plaintext `~/.runbeam/auth.json` to secure storage
  - Migration happens transparently on first run after upgrade
  - Legacy plaintext file removed after successful migration
  - No user action required

- **Enhanced Security**
  - All user tokens encrypted at rest (except in OS keyring)
  - Encryption keys stored securely in OS keyring, never on disk in plaintext
  - Transparent encryption/decryption with no user configuration needed
  - Per-token-type storage isolation (user tokens separate from machine tokens)

### Changed

- **Storage Module Refactoring**
  - Refactored `src/storage.rs` to use SDK generic secure storage
  - `save_user_token()`, `load_user_token()`, `clear_user_token()` use SDK functions
  - Removed duplicate encryption key management code
  - Storage now handled entirely by SDK

- **Encryption Key Management (Deprecated)**
  - Harmony encryption key commands now show deprecation messages
  - `harmony:set-key`, `harmony:show-key`, `harmony:delete-key` deprecated
  - SDK handles all encryption keys automatically
  - User-facing encryption key management no longer needed

- **Documentation**
  - Updated README.md with secure storage information and migration notes
  - Updated WARP.md with secure storage architecture
  - Added migration guide for upgrading from v0.4.x
  - Documented automatic token migration process
  - Marked encryption key commands as deprecated

- **Dependencies**
  - Updated to `runbeam-sdk = "0.3.2"` with generic secure storage support
  - Removed unused encryption-related dependencies from CLI

### Security

- **Improved Token Protection**
  - User tokens no longer stored in plaintext JSON files
  - Automatic encryption for filesystem fallback
  - OS-native credential storage used when available
  - Encryption keys managed securely by SDK

### Backwards Compatibility

- Automatic migration from legacy plaintext storage
- No breaking changes to CLI commands
- Existing workflows continue to work without modification
- Legacy token files automatically cleaned up after migration

## [0.4.0] - 2025-10-28

### Changed
- **SDK Integration**: Migrated JWT verification and authentication logic to `runbeam-sdk` v0.2.0
  - Removed 441 lines of duplicate JWT verification code from CLI
  - Authentication now uses SDK's `AuthManager` for token management
  - JWT verification now handled by SDK's `verify_token()` function
  - Simplified codebase maintenance by centralizing auth logic in SDK
- **Dependencies**: Updated to use `runbeam-sdk = "0.2.0"` with async runtime support
  - Added `tokio` runtime for SDK integration
  - Removed direct dependencies on `jsonwebtoken`, `base64`, and `url` (now via SDK)

### Removed
- Local JWT verification module (`src/jwt.rs`) - functionality moved to SDK
- Direct JWKS fetching and caching logic - now handled by SDK

## [0.3.0] - 2025-10-26

### Added
- **JWT RS256 Verification**: Local JWT token verification using RS256 asymmetric cryptography
  - Validates tokens using public keys from JWKS endpoint
  - Supports key rotation via Key ID (`kid`) in JWT header or payload
  - Automatic JWKS caching with 1-hour TTL (configurable via `RUNBEAM_JWKS_TTL` env var)
  - Graceful fallback to first RS256 key when `kid` is missing
- **New Command**: `runbeam verify` - Verify stored authentication token
  - Displays comprehensive token information (issuer, subject, expiration)
  - Shows user and team details from JWT claims
  - Calculates and displays time remaining until token expiry
  - Clear error messages with troubleshooting guidance
- **Token Verification on Login**: Automatically verifies tokens after successful authentication
  - Provides immediate feedback on token validity
  - Validates signature using RS256 with JWKS public key

### Changed
- **JWT Validation**: Migrated from expecting HS256 (symmetric) to RS256 (asymmetric) tokens
- **Issuer Validation**: URL normalization for flexible issuer claim matching
- **Error Handling**: Enhanced error messages for JWT verification failures
  - Network errors (timeout, connection refused, DNS failures)
  - Invalid token signatures
  - Expired tokens
  - Missing required claims
- **Dependencies**: Added `jsonwebtoken`, `base64`, and `url` crates for JWT support

### Fixed
- Improved error reporting for authentication polling
- Better handling of non-success HTTP responses during login flow
- Added debug logging for JWT verification troubleshooting

### Security
- **Asymmetric Token Verification**: Tokens are now verified using public keys instead of shared secrets
- **Key Rotation Support**: Automatic key selection via `kid` enables seamless key rotation
- **JWKS Caching**: Reduces network calls while maintaining security with TTL-based refresh

### Documentation
- Updated README with token verification examples
- Added comprehensive inline documentation for JWT module
- Updated WARP.md with verify command usage

## [0.2.0] - 2024-XX-XX

### Added
- Initial CLI implementation
- Browser-based OAuth authentication
- Harmony instance management
- Configuration management (`config:set`, `config:get`, `config:unset`)
- Harmony authorization flow

### Changed
- Improved logging and verbosity controls

## [0.1.0] - 2024-XX-XX

### Added
- Initial release
- Basic CLI structure
- Authentication commands (`login`, `logout`)
- Harmony commands (`harmony:add`, `harmony:list`, `harmony:remove`)
- Harmony management API integration

[0.4.0]: https://github.com/aurabx/runbeam-cli/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/aurabx/runbeam-cli/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/aurabx/runbeam-cli/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/aurabx/runbeam-cli/releases/tag/v0.1.0
