use anyhow::{Context, Result};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, info, warn};

use crate::storage;

/// JWT claims structure matching Runbeam API format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    /// Issuer - the Runbeam Cloud API base URL
    pub iss: String,
    /// Subject - User or Team ID
    pub sub: String,
    /// Audience - 'runbeam-cli' or 'runbeam-api' (optional)
    #[serde(default)]
    pub aud: Option<String>,
    /// Expiration time (Unix timestamp)
    pub exp: i64,
    /// Issued at time (Unix timestamp)
    pub iat: i64,
    /// Key ID used for signing (optional, from header)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    /// User information
    #[serde(default)]
    pub user: Option<UserInfo>,
    /// Team information
    #[serde(default)]
    pub team: Option<TeamInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamInfo {
    pub id: String,
    pub name: String,
}

/// JWKS (JSON Web Key Set) response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jwks {
    pub keys: Vec<JwkKey>,
}

/// Individual JSON Web Key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwkKey {
    /// Key type (should be "RSA" for RS256)
    pub kty: String,
    /// Public key use (should be "sig" for signature verification)
    #[serde(rename = "use")]
    pub use_: String,
    /// Key ID - used to match against JWT header
    pub kid: String,
    /// Algorithm (should be "RS256")
    pub alg: String,
    /// RSA modulus (base64url encoded)
    pub n: String,
    /// RSA exponent (base64url encoded)
    pub e: String,
}

/// JWKS cache structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct JwksCache {
    jwks: Jwks,
    cached_at: u64,
    ttl_seconds: u64,
}

impl JwksCache {
    fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        (now - self.cached_at) > self.ttl_seconds
    }
}

/// Get the path to the JWKS cache file
fn jwks_cache_path() -> Result<std::path::PathBuf> {
    Ok(storage::data_dir()?.join("jwks_cache.json"))
}

/// Get the JWKS cache TTL from environment or use default
fn get_jwks_ttl() -> u64 {
    std::env::var("RUNBEAM_JWKS_TTL")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3600) // Default: 1 hour
}

/// Load cached JWKS from filesystem
fn load_jwks_cache() -> Result<Option<JwksCache>> {
    let path = jwks_cache_path()?;
    if !path.exists() {
        debug!("JWKS cache does not exist at {}", path.display());
        return Ok(None);
    }

    let data = fs::read_to_string(&path)
        .with_context(|| format!("reading JWKS cache from {}", path.display()))?;

    let cache: JwksCache = serde_json::from_str(&data)
        .with_context(|| format!("parsing JWKS cache from {}", path.display()))?;

    if cache.is_expired() {
        debug!("JWKS cache is expired, will refresh");
        Ok(None)
    } else {
        debug!(
            "Loaded valid JWKS cache (age: {}s)",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                - cache.cached_at
        );
        Ok(Some(cache))
    }
}

/// Save JWKS to cache with atomic write
fn save_jwks_cache(jwks: &Jwks) -> Result<()> {
    let path = jwks_cache_path()?;
    let tmp_path = path.with_extension("json.tmp");

    let cache = JwksCache {
        jwks: jwks.clone(),
        cached_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        ttl_seconds: get_jwks_ttl(),
    };

    let json = serde_json::to_string_pretty(&cache)?;

    // Write atomically: write temp, then rename
    {
        let mut f = fs::File::create(&tmp_path)
            .with_context(|| format!("creating {}", tmp_path.display()))?;
        f.write_all(json.as_bytes())?;
        f.sync_all().ok();
    }

    fs::rename(&tmp_path, &path)
        .with_context(|| format!("rename {} -> {}", tmp_path.display(), path.display()))?;

    debug!("Saved JWKS cache to {}", path.display());
    Ok(())
}

/// Fetch JWKS from the API endpoint
fn fetch_jwks(api_url: &str) -> Result<Jwks> {
    // JWKS endpoint is at /api/.well-known/jwks.json
    let jwks_url = format!(
        "{}/api/.well-known/jwks.json",
        api_url.trim_end_matches('/')
    );
    debug!("Fetching JWKS from {}", jwks_url);

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let response = client
        .get(&jwks_url)
        .send()
        .with_context(|| format!("failed to fetch JWKS from {}", jwks_url))?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Failed to fetch JWKS: HTTP {} - {}",
            response.status(),
            response.text().unwrap_or_default()
        );
    }

    let jwks: Jwks = response.json().context("failed to parse JWKS response")?;

    debug!("Fetched JWKS with {} keys", jwks.keys.len());
    Ok(jwks)
}

/// Get JWKS with caching support
fn get_jwks(api_url: &str, force_refresh: bool) -> Result<Jwks> {
    if !force_refresh {
        if let Some(cache) = load_jwks_cache()? {
            debug!("Using cached JWKS");
            return Ok(cache.jwks);
        }
    }

    let jwks = fetch_jwks(api_url)?;
    save_jwks_cache(&jwks)?;
    Ok(jwks)
}

/// Convert JWK to jsonwebtoken DecodingKey
fn jwk_to_decoding_key(jwk: &JwkKey) -> Result<DecodingKey> {
    // Validate key type
    if jwk.kty != "RSA" {
        anyhow::bail!("Unsupported key type: {}, expected RSA", jwk.kty);
    }

    // Create DecodingKey from RSA components (expects base64url-encoded strings)
    // The jsonwebtoken library expects the raw base64url strings, not decoded bytes
    DecodingKey::from_rsa_components(&jwk.n, &jwk.e).with_context(|| {
        format!(
            "failed to create decoding key from RSA components for key {}",
            jwk.kid
        )
    })
}

/// Check if token is expired
#[allow(dead_code)]
pub fn is_token_expired(claims: &JwtClaims) -> bool {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    claims.exp < now
}

/// Extract API base URL from issuer claim
fn normalize_issuer(iss: &str) -> String {
    // Try to parse as URL and extract origin
    if let Ok(url) = url::Url::parse(iss) {
        // Get scheme + host + port
        let scheme = url.scheme();
        let host = url.host_str().unwrap_or("");
        let port = url.port().map(|p| format!(":{}", p)).unwrap_or_default();
        format!("{}://{}{}", scheme, host, port)
    } else {
        // If parsing fails, return as-is
        iss.to_string()
    }
}

/// Validate JWT token using RS256 and JWKS
///
/// This function:
/// 1. Decodes the JWT header to extract the key ID (kid)
/// 2. Fetches JWKS from the API (with caching)
/// 3. Finds the matching public key
/// 4. Verifies the token signature using RS256
/// 5. Validates standard claims (exp, iss, sub)
///
/// # Arguments
///
/// * `token` - The JWT token string to validate
/// * `api_url` - The Runbeam API base URL (used to fetch JWKS and validate issuer)
///
/// # Returns
///
/// Returns `Ok(JwtClaims)` if validation succeeds, or `Err` if validation fails.
pub fn validate_jwt_token(token: &str, api_url: &str) -> Result<JwtClaims> {
    debug!("Validating JWT token (length: {})", token.len());

    // Decode header to extract kid (standard location)
    let header = decode_header(token).context("failed to decode JWT header")?;

    let kid_from_header = header.kid.as_deref();
    debug!(
        "JWT header: alg={:?}, kid={:?}",
        header.alg, kid_from_header
    );

    // Fetch JWKS
    let jwks = get_jwks(api_url, false).context("failed to fetch JWKS for token verification")?;

    if jwks.keys.is_empty() {
        anyhow::bail!("JWKS endpoint returned no keys");
    }

    // First pass decode to get kid from payload if not in header
    // We need to do this before key selection
    let kid_from_payload = if kid_from_header.is_none() {
        // Do an insecure decode just to read the payload claims
        let mut insecure_validation = Validation::new(Algorithm::RS256);
        insecure_validation.insecure_disable_signature_validation();

        if let Ok(token_data) =
            decode::<JwtClaims>(token, &DecodingKey::from_secret(&[]), &insecure_validation)
        {
            token_data.claims.kid.clone()
        } else {
            None
        }
    } else {
        None
    };

    // Determine final kid: prefer header, fallback to payload
    let kid = kid_from_header.map(|s| s.to_string()).or(kid_from_payload);

    if let Some(ref kid_value) = kid {
        debug!(
            "Using kid: {} (from {})",
            kid_value,
            if kid_from_header.is_some() {
                "header"
            } else {
                "payload"
            }
        );
    }

    // Find matching key by kid, or use first RS256 key
    let jwk = if let Some(kid_value) = &kid {
        jwks.keys
            .iter()
            .find(|k| k.kid == *kid_value)
            .with_context(|| format!("no key found in JWKS with kid={}", kid_value))?
    } else {
        warn!("JWT has no kid in header or payload, using first RS256 key from JWKS");
        jwks.keys
            .iter()
            .find(|k| k.alg == "RS256")
            .context("no RS256 key found in JWKS")?
    };

    debug!("Using JWK key: kid={}, alg={}", jwk.kid, jwk.alg);

    // Convert JWK to DecodingKey
    let decoding_key = jwk_to_decoding_key(jwk)?;

    // Configure validation
    let mut validation = Validation::new(Algorithm::RS256);
    validation.validate_exp = true;
    validation.validate_nbf = false;
    // Disable automatic issuer validation - we'll do it manually to allow URL normalization
    validation.required_spec_claims.clear(); // Remove 'iss' from required claims

    // Decode and validate the token
    let token_data = decode::<JwtClaims>(token, &decoding_key, &validation).map_err(|e| {
        warn!("JWT validation failed: {}", e);
        anyhow::anyhow!(
            "Token validation failed: {}. Please run `runbeam login` to get a new token.",
            e
        )
    })?;

    let mut claims = token_data.claims;

    // Determine kid: prefer header (standard), fallback to payload (tymon/jwt-auth)
    let kid = kid_from_header
        .map(|s| s.to_string())
        .or_else(|| claims.kid.clone());

    if let Some(ref kid_value) = kid {
        debug!(
            "Using kid: {} (from {})",
            kid_value,
            if kid_from_header.is_some() {
                "header"
            } else {
                "payload"
            }
        );
    }

    // Store the resolved kid in claims for reference
    claims.kid = kid;

    // Validate issuer matches API URL (with normalization)
    let normalized_iss = normalize_issuer(&claims.iss);
    let normalized_api = normalize_issuer(api_url);

    if normalized_iss != normalized_api {
        anyhow::bail!(
            "Token issuer mismatch: expected '{}', got '{}'. This token is not valid for this API.",
            normalized_api,
            normalized_iss
        );
    }

    // Validate required claims
    if claims.sub.is_empty() {
        anyhow::bail!("Missing or empty subject (sub) claim");
    }

    info!(
        "JWT validation successful: sub={}, kid={:?}, exp={}",
        claims.sub, claims.kid, claims.exp
    );

    Ok(claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_issuer() {
        assert_eq!(normalize_issuer("http://example.com"), "http://example.com");
        assert_eq!(
            normalize_issuer("http://example.com:8000"),
            "http://example.com:8000"
        );
        assert_eq!(
            normalize_issuer("http://example.com/api/some/path"),
            "http://example.com"
        );
    }

    #[test]
    fn test_jwks_cache_expiry() {
        let cache = JwksCache {
            jwks: Jwks { keys: vec![] },
            cached_at: 0, // Unix epoch
            ttl_seconds: 3600,
        };
        assert!(cache.is_expired());

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let fresh_cache = JwksCache {
            jwks: Jwks { keys: vec![] },
            cached_at: now,
            ttl_seconds: 3600,
        };
        assert!(!fresh_cache.is_expired());
    }
}
