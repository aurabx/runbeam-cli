use anyhow::{Context, Result};
use directories::BaseDirs;
use runbeam_sdk::UserInfo;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tracing::{debug, warn};

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
