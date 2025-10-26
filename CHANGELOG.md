# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[0.3.0]: https://github.com/aurabx/runbeam-cli/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/aurabx/runbeam-cli/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/aurabx/runbeam-cli/releases/tag/v0.1.0
