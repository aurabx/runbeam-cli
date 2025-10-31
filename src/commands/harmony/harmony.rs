use tracing::info;

/// Add a new Harmony instance via the management API
///
/// Persists to the runbeam data directory. When the live management API exists,
/// this can be extended to perform remote registration as well.
///
/// # Arguments
///
/// * `encryption_key` - Optional base64-encoded encryption key to use for token storage
pub fn harmony_add(
    ip: &str,
    port: u16,
    label: Option<&str>,
    path_prefix: &str,
    encryption_key: Option<&str>,
) -> anyhow::Result<()> {
    let final_label = label
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("{}:{}", ip, port));

    info!(ip = %ip, port = %port, label = %final_label, path_prefix = %path_prefix, "harmony:add");

    // Persist the instance
    let instance = crate::storage::HarmonyInstance {
        id: String::new(),
        ip: ip.to_string(),
        port,
        label: final_label.clone(),
        path_prefix: path_prefix.to_string(),
    };
    crate::storage::add_harmony_instance(instance.clone())?;

    // Get the instance ID (it was generated during add)
    let instances = crate::storage::load_harmony_instances()?;
    let saved_instance = instances
        .iter()
        .find(|i| i.ip == ip && i.port == port && i.label == final_label)
        .ok_or_else(|| anyhow::anyhow!("Failed to retrieve saved instance"))?;

    // Save encryption key if provided
    if let Some(key) = encryption_key {
        crate::storage::save_encryption_key(&saved_instance.id, key)
            .map_err(|e| {
                // If keyring fails, warn but don't fail the entire operation
                eprintln!("⚠️  Warning: Could not save encryption key to keyring: {}", e);
                eprintln!("   You can set it later with: runbeam harmony:set-key --id {}", saved_instance.id);
            })
            .ok();
        
        if crate::storage::has_encryption_key(&saved_instance.id) {
            println!(
                "Added Harmony instance {}:{} (ID: {}) label={} prefix={}",
                ip, port, saved_instance.id, final_label, path_prefix
            );
            println!("✓ Encryption key saved to OS keyring");
        } else {
            println!(
                "Added Harmony instance {}:{} (ID: {}) label={} prefix={}",
                ip, port, saved_instance.id, final_label, path_prefix
            );
        }
    } else {
        println!(
            "Added Harmony instance {}:{} (ID: {}) label={} prefix={}",
            ip, port, saved_instance.id, final_label, path_prefix
        );
        println!("ℹ️  No encryption key provided. Harmony will generate one automatically.");
        println!("   To set a specific key: runbeam harmony:set-key --id {}", saved_instance.id);
    }
    
    Ok(())
}

pub fn harmony_list() -> anyhow::Result<()> {
    let list = crate::storage::load_harmony_instances()?;
    if list.is_empty() {
        println!("No Harmony instances registered.");
        return Ok(());
    }

    // Compute column widths
    let mut w_id = "ID".len();
    let mut w_label = "LABEL".len();
    let mut w_ip = "IP".len();
    let mut w_port = "PORT".len();
    let mut w_prefix = "PREFIX".len();
    for inst in &list {
        if inst.id.len() > w_id {
            w_id = inst.id.len();
        }
        if inst.label.len() > w_label {
            w_label = inst.label.len();
        }
        if inst.ip.len() > w_ip {
            w_ip = inst.ip.len();
        }
        let port_len = inst.port.to_string().len();
        if port_len > w_port {
            w_port = port_len;
        }
        if inst.path_prefix.len() > w_prefix {
            w_prefix = inst.path_prefix.len();
        }
    }

    // Header
    println!(
        "{id:<id_w$} | {label:<label_w$} | {ip:<ip_w$} | {port:<port_w$} | {prefix:<prefix_w$}",
        id = "ID",
        label = "LABEL",
        ip = "IP",
        port = "PORT",
        prefix = "PREFIX",
        id_w = w_id,
        label_w = w_label,
        ip_w = w_ip,
        port_w = w_port,
        prefix_w = w_prefix,
    );
    // Separator
    println!(
        "{id:-<id_w$}-+-{label:-<label_w$}-+-{ip:-<ip_w$}-+-{port:-<port_w$}-+-{prefix:-<prefix_w$}",
        id = "",
        label = "",
        ip = "",
        port = "",
        prefix = "",
        id_w = w_id,
        label_w = w_label,
        ip_w = w_ip,
        port_w = w_port,
        prefix_w = w_prefix,
    );
    // Rows
    for inst in list {
        println!(
            "{id:<id_w$} | {label:<label_w$} | {ip:<ip_w$} | {port:<port_w$} | {prefix:<prefix_w$}",
            id = inst.id,
            label = inst.label,
            ip = inst.ip,
            port = inst.port,
            prefix = inst.path_prefix,
            id_w = w_id,
            label_w = w_label,
            ip_w = w_ip,
            port_w = w_port,
            prefix_w = w_prefix,
        );
    }

    Ok(())
}

pub fn harmony_remove(
    id: Option<&str>,
    label: Option<&str>,
    ip: Option<&str>,
    port: Option<u16>,
) -> anyhow::Result<()> {
    if let Some(id) = id {
        let removed = crate::storage::remove_harmony_instance_by_id(id)?;
        if removed {
            println!("Removed Harmony instance with id '{}'.", id);
        } else {
            println!("No Harmony instance found with id '{}'.", id);
        }
        return Ok(());
    }

    if let Some(label) = label {
        let removed = crate::storage::remove_harmony_instance_by_label(label)?;
        if removed {
            println!("Removed Harmony instance with label '{}'.", label);
        } else {
            println!("No Harmony instance found with label '{}'.", label);
        }
        return Ok(());
    }

    match (ip, port) {
        (Some(ip), Some(port)) => {
            let removed = crate::storage::remove_harmony_instance_by_addr(ip, port)?;
            if removed {
                println!("Removed Harmony instance {}:{}.", ip, port);
            } else {
                println!("No Harmony instance found at {}:{}.", ip, port);
            }
            Ok(())
        }
        _ => {
            anyhow::bail!("provide --id, --label, or both --ip and --port")
        }
    }
}

/// Set or update the encryption key for a Harmony instance
///
/// Stores the key securely in the OS keyring (macOS Keychain, Linux Secret Service,
/// Windows Credential Manager). This key will be used to encrypt tokens when
/// authorizing the Harmony instance.
pub fn harmony_set_key(instance_id: &str, encryption_key: &str) -> anyhow::Result<()> {
    info!(instance_id = %instance_id, "harmony:set-key");

    // Verify the instance exists
    let instances = crate::storage::load_harmony_instances()?;
    let instance = instances
        .iter()
        .find(|i| i.id == instance_id)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Harmony instance '{}' not found. Use 'runbeam harmony:list' to see available instances.",
                instance_id
            )
        })?;

    // Save the encryption key
    crate::storage::save_encryption_key(instance_id, encryption_key)?;

    println!("✓ Encryption key saved for Harmony instance: {} ({})", instance.label, instance_id);
    println!("  This key will be used when authorizing this instance.");
    println!();
    println!("⚠️  Important: Keep this key secure and backed up!");
    println!("  You'll need it to decrypt tokens if you move to a different machine.");

    Ok(())
}

/// Show the encryption key for a Harmony instance
///
/// Retrieves the key from the OS keyring. This is sensitive information and should
/// be handled carefully.
pub fn harmony_show_key(instance_id: &str) -> anyhow::Result<()> {
    info!(instance_id = %instance_id, "harmony:show-key");

    // Verify the instance exists
    let instances = crate::storage::load_harmony_instances()?;
    let instance = instances
        .iter()
        .find(|i| i.id == instance_id)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Harmony instance '{}' not found. Use 'runbeam harmony:list' to see available instances.",
                instance_id
            )
        })?;

    // Load the encryption key
    let key = crate::storage::load_encryption_key(instance_id)?
        .ok_or_else(|| {
            anyhow::anyhow!(
                "No encryption key found for Harmony instance '{}' ({})",
                instance.label,
                instance_id
            )
        })?;

    println!();
    println!("🔐 Encryption Key for Harmony Instance: {} ({})", instance.label, instance_id);
    println!();
    println!("⚠️  WARNING: This is sensitive information! Keep it secure.");
    println!();
    println!("Key: {}", key);
    println!();
    println!("This key is used to encrypt machine tokens for this Harmony instance.");
    println!("Store it in a secure location (password manager, secrets vault, etc.)");
    println!();

    Ok(())
}

/// Delete the encryption key for a Harmony instance
///
/// Removes the key from the OS keyring. This does not affect the Harmony instance
/// configuration or any already-encrypted tokens.
pub fn harmony_delete_key(instance_id: &str) -> anyhow::Result<()> {
    info!(instance_id = %instance_id, "harmony:delete-key");

    // Verify the instance exists
    let instances = crate::storage::load_harmony_instances()?;
    let instance = instances
        .iter()
        .find(|i| i.id == instance_id)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Harmony instance '{}' not found. Use 'runbeam harmony:list' to see available instances.",
                instance_id
            )
        })?;

    // Delete the encryption key
    let deleted = crate::storage::delete_encryption_key(instance_id)?;

    if deleted {
        println!("✓ Encryption key deleted for Harmony instance: {} ({})", instance.label, instance_id);
        println!();
        println!("ℹ️  The Harmony instance will generate a new key automatically when next authorized.");
        println!("  Existing tokens encrypted with the old key will need to be re-authorized.");
    } else {
        println!("ℹ️  No encryption key found for Harmony instance: {} ({})", instance.label, instance_id);
    }

    Ok(())
}
