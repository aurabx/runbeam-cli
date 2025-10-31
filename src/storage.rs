use anyhow::{Context, Result};
use directories::BaseDirs;
use runbeam_sdk::UserInfo;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tracing::{debug, warn};
use keyring::Entry;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HarmonyInstance {
    /// Stable short identifier
    #[serde(default)]
    pub id: String,
    pub ip: String,
    pub port: u16,
    pub label: String,
    #[serde(default = "default_path_prefix")]
    pub path_prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliAuth {
    /// JWT token for API authentication
    pub token: String,
    /// Token expiration timestamp (seconds since epoch)
    #[serde(default)]
    pub expires_at: Option<i64>,
    /// User information from JWT claims
    #[serde(default)]
    pub user: Option<UserInfo>,
}

fn default_path_prefix() -> String {
    "admin".to_string()
}

fn derive_id(ip: &str, port: u16, label: &str) -> String {
    use sha2::{Digest, Sha256};
    let input = format!("{}:{}:{}", ip, port, label);
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let hash = hasher.finalize();
    // first 8 hex chars
    format!(
        "{:02x}{:02x}{:02x}{:02x}",
        hash[0], hash[1], hash[2], hash[3]
    )
}

fn base_dir() -> Result<PathBuf> {
    let bd = BaseDirs::new().context("could not determine base directories")?;
    #[cfg(windows)]
    {
        // Equivalent on Windows: use roaming data dir
        Ok(PathBuf::from(bd.data_dir()).join("runbeam"))
    }
    #[cfg(not(windows))]
    {
        Ok(PathBuf::from(bd.home_dir()).join(".runbeam"))
    }
}

pub fn data_dir() -> Result<PathBuf> {
    let dir = base_dir()?;
    if !dir.exists() {
        fs::create_dir_all(&dir)
            .with_context(|| format!("creating data dir: {}", dir.display()))?;
    }
    Ok(dir)
}

fn harmony_file_path() -> Result<PathBuf> {
    Ok(data_dir()?.join("harmony.json"))
}

fn auth_file_path() -> Result<PathBuf> {
    Ok(data_dir()?.join("auth.json"))
}

pub fn load_harmony_instances() -> Result<Vec<HarmonyInstance>> {
    let path = harmony_file_path()?;
    if !path.exists() {
        return Ok(Vec::new());
    }
    let data = fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    let mut list: Vec<HarmonyInstance> =
        serde_json::from_str(&data).with_context(|| format!("parsing {}", path.display()))?;

    // Backfill missing IDs for older entries and persist once
    let mut changed = false;
    for inst in &mut list {
        if inst.id.is_empty() {
            inst.id = derive_id(&inst.ip, inst.port, &inst.label);
            changed = true;
        }
    }
    if changed {
        save_harmony_instances(&list)?;
    }

    Ok(list)
}

pub fn save_harmony_instances(list: &[HarmonyInstance]) -> Result<()> {
    let path = harmony_file_path()?;
    let tmp_path = tmp_path_for(&path);
    let json = serde_json::to_string_pretty(list)?;
    // Write atomically: write temp, then rename
    {
        let mut f = fs::File::create(&tmp_path)
            .with_context(|| format!("creating {}", tmp_path.display()))?;
        f.write_all(json.as_bytes())?;
        f.sync_all().ok();
    }
    fs::rename(&tmp_path, &path)
        .with_context(|| format!("rename {} -> {}", tmp_path.display(), path.display()))?;
    Ok(())
}

fn tmp_path_for(path: &Path) -> PathBuf {
    let mut p = path.to_path_buf();
    let fname = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    p.set_file_name(format!("{}.tmp", fname));
    p
}

pub fn add_harmony_instance(new_inst: HarmonyInstance) -> Result<()> {
    let mut list = load_harmony_instances()?;

    // De-duplicate by label first, else by ip:port
    if let Some(existing) = list.iter_mut().find(|i| i.label == new_inst.label) {
        // Update fields but preserve ID
        existing.ip = new_inst.ip;
        existing.port = new_inst.port;
        existing.label = new_inst.label;
        existing.path_prefix = new_inst.path_prefix;
    } else if let Some(existing) = list
        .iter_mut()
        .find(|i| i.ip == new_inst.ip && i.port == new_inst.port)
    {
        existing.label = new_inst.label;
        existing.path_prefix = new_inst.path_prefix;
    } else {
        let mut to_add = new_inst;
        if to_add.id.is_empty() {
            to_add.id = derive_id(&to_add.ip, to_add.port, &to_add.label);
        }
        list.push(to_add);
    }

    save_harmony_instances(&list)
}

pub fn remove_harmony_instance_by_label(label: &str) -> Result<bool> {
    let mut list = load_harmony_instances()?;
    let before = list.len();
    list.retain(|i| i.label != label);
    let changed = list.len() != before;
    if changed {
        save_harmony_instances(&list)?;
    }
    Ok(changed)
}

pub fn remove_harmony_instance_by_addr(ip: &str, port: u16) -> Result<bool> {
    let mut list = load_harmony_instances()?;
    let before = list.len();
    list.retain(|i| !(i.ip == ip && i.port == port));
    let changed = list.len() != before;
    if changed {
        save_harmony_instances(&list)?;
    }
    Ok(changed)
}

pub fn remove_harmony_instance_by_id(id: &str) -> Result<bool> {
    let mut list = load_harmony_instances()?;
    let before = list.len();
    list.retain(|i| i.id != id);
    let changed = list.len() != before;
    if changed {
        save_harmony_instances(&list)?;
    }
    Ok(changed)
}

// ============================================================================
// CLI Authentication Storage
// ============================================================================

pub fn load_auth() -> Result<Option<CliAuth>> {
    let path = auth_file_path()?;
    if !path.exists() {
        return Ok(None);
    }
    let data = fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    let auth: CliAuth =
        serde_json::from_str(&data).with_context(|| format!("parsing {}", path.display()))?;
    Ok(Some(auth))
}

pub fn save_auth(auth: &CliAuth) -> Result<()> {
    let path = auth_file_path()?;
    let tmp_path = tmp_path_for(&path);
    let json = serde_json::to_string_pretty(auth)?;
    // Write atomically: write temp, then rename
    {
        let mut f = fs::File::create(&tmp_path)
            .with_context(|| format!("creating {}", tmp_path.display()))?;
        f.write_all(json.as_bytes())?;
        f.sync_all().ok();
    }
    fs::rename(&tmp_path, &path)
        .with_context(|| format!("rename {} -> {}", tmp_path.display(), path.display()))?;
    Ok(())
}

pub fn clear_auth() -> Result<bool> {
    let path = auth_file_path()?;
    if path.exists() {
        fs::remove_file(&path).with_context(|| format!("removing {}", path.display()))?;
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Load authentication and verify the JWT token
///
/// This function loads the stored auth and validates the JWT token using RS256.
/// If verification fails, it logs a warning but still returns the auth (graceful degradation).
///
/// # Returns
///
/// Returns `Ok(Some(auth))` if authentication exists (regardless of verification status),
/// or `Ok(None)` if no authentication is stored.
#[allow(dead_code)]
pub fn load_and_verify_auth() -> Result<Option<CliAuth>> {
    let auth = load_auth()?;

    if let Some(ref auth) = auth {
        debug!("Verifying stored JWT token...");

        // Get API URL for verification - no longer needed with SDK's auto-detection
        // The SDK extracts the base URL from the JWT issuer claim

        // Attempt to verify the token using SDK (async call via runtime)
        let validation_result = tokio::runtime::Runtime::new()
            .expect("Failed to create Tokio runtime")
            .block_on(runbeam_sdk::validate_jwt_token(&auth.token, 24));

        match validation_result {
            Ok(claims) => {
                debug!(
                    "Token verification successful: sub={}, exp={}",
                    claims.sub, claims.exp
                );
            }
            Err(e) => {
                warn!("Token verification failed: {}", e);
                warn!("Token may be expired or invalid. Consider running `runbeam login` again.");
                // Note: We still return the auth to allow commands to proceed.
                // Individual commands can choose to enforce verification if needed.
            }
        }
    }

    Ok(auth)
}

// ============================================================================
// Harmony Encryption Key Storage (Keyring)
// ============================================================================

const KEYRING_SERVICE: &str = "runbeam-harmony";

/// Get the keyring entry for a Harmony instance's encryption key
fn get_encryption_key_entry(instance_id: &str) -> Result<Entry> {
    let account = format!("{}-encryption-key", instance_id);
    Entry::new(KEYRING_SERVICE, &account)
        .with_context(|| format!("Failed to create keyring entry for instance: {}", instance_id))
}

/// Save an encryption key for a Harmony instance to the OS keyring
///
/// The key is stored securely in the OS keyring (macOS Keychain, Linux Secret Service,
/// Windows Credential Manager) and can be used to encrypt tokens for the Harmony instance.
///
/// # Arguments
///
/// * `instance_id` - The Harmony instance ID
/// * `encryption_key` - The base64-encoded encryption key to store
///
/// # Returns
///
/// Returns `Ok(())` if the key was saved successfully
pub fn save_encryption_key(instance_id: &str, encryption_key: &str) -> Result<()> {
    debug!("Saving encryption key for Harmony instance: {}", instance_id);
    
    let entry = get_encryption_key_entry(instance_id)?;
    entry
        .set_password(encryption_key)
        .with_context(|| format!("Failed to save encryption key for instance: {}", instance_id))?;
    
    debug!("Encryption key saved successfully");
    Ok(())
}

/// Load an encryption key for a Harmony instance from the OS keyring
///
/// # Arguments
///
/// * `instance_id` - The Harmony instance ID
///
/// # Returns
///
/// Returns `Ok(Some(key))` if the key exists, `Ok(None)` if not found
pub fn load_encryption_key(instance_id: &str) -> Result<Option<String>> {
    debug!("Loading encryption key for Harmony instance: {}", instance_id);
    
    let entry = get_encryption_key_entry(instance_id)?;
    
    match entry.get_password() {
        Ok(key) => {
            debug!("Encryption key loaded successfully");
            Ok(Some(key))
        }
        Err(keyring::Error::NoEntry) => {
            debug!("No encryption key found for instance");
            Ok(None)
        }
        Err(e) => {
            Err(anyhow::anyhow!("Failed to load encryption key: {}", e))
        }
    }
}

/// Delete an encryption key for a Harmony instance from the OS keyring
///
/// # Arguments
///
/// * `instance_id` - The Harmony instance ID
///
/// # Returns
///
/// Returns `Ok(true)` if the key was deleted, `Ok(false)` if it didn't exist
pub fn delete_encryption_key(instance_id: &str) -> Result<bool> {
    debug!("Deleting encryption key for Harmony instance: {}", instance_id);
    
    let entry = get_encryption_key_entry(instance_id)?;
    
    match entry.delete_credential() {
        Ok(_) => {
            debug!("Encryption key deleted successfully");
            Ok(true)
        }
        Err(keyring::Error::NoEntry) => {
            debug!("No encryption key found to delete");
            Ok(false)
        }
        Err(e) => {
            Err(anyhow::anyhow!("Failed to delete encryption key: {}", e))
        }
    }
}

/// Check if an encryption key exists for a Harmony instance
///
/// # Arguments
///
/// * `instance_id` - The Harmony instance ID
///
/// # Returns
///
/// Returns `true` if an encryption key exists, `false` otherwise
pub fn has_encryption_key(instance_id: &str) -> bool {
    match load_encryption_key(instance_id) {
        Ok(Some(_)) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_id_consistency() {
        let id1 = derive_id("192.168.1.1", 8081, "test-label");
        let id2 = derive_id("192.168.1.1", 8081, "test-label");
        assert_eq!(id1, id2, "derive_id should be deterministic");
    }

    #[test]
    fn test_derive_id_different_inputs() {
        let id1 = derive_id("192.168.1.1", 8081, "label1");
        let id2 = derive_id("192.168.1.1", 8081, "label2");
        let id3 = derive_id("192.168.1.2", 8081, "label1");
        let id4 = derive_id("192.168.1.1", 8082, "label1");

        assert_ne!(id1, id2, "Different labels should produce different IDs");
        assert_ne!(id1, id3, "Different IPs should produce different IDs");
        assert_ne!(id1, id4, "Different ports should produce different IDs");
    }

    #[test]
    fn test_default_path_prefix() {
        assert_eq!(default_path_prefix(), "admin");
    }

    #[test]
    fn test_harmony_instance_serialization() {
        let instance = HarmonyInstance {
            id: "abc123".to_string(),
            ip: "127.0.0.1".to_string(),
            port: 8081,
            label: "test".to_string(),
            path_prefix: "admin".to_string(),
        };

        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: HarmonyInstance =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(instance, deserialized);
    }

    #[test]
    fn test_harmony_instance_default_path_prefix() {
        let json = r#"{
            "id": "abc123",
            "ip": "127.0.0.1",
            "port": 8081,
            "label": "test"
        }"#;

        let instance: HarmonyInstance =
            serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(instance.path_prefix, "admin");
    }

    #[test]
    fn test_cli_auth_serialization() {
        let auth = CliAuth {
            token: "test-token".to_string(),
            expires_at: Some(1234567890),
            user: None,
        };

        let json = serde_json::to_string(&auth).expect("Failed to serialize");
        let deserialized: CliAuth = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(auth.token, deserialized.token);
        assert_eq!(auth.expires_at, deserialized.expires_at);
    }

    #[test]
    fn test_tmp_path_for() {
        use std::path::PathBuf;

        let path = PathBuf::from("/tmp/test.json");
        let tmp = tmp_path_for(&path);
        assert_eq!(tmp, PathBuf::from("/tmp/test.json.tmp"));
    }
}
