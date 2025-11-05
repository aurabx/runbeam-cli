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

    println!(
        "Added Harmony instance {}:{} (ID: {}) label={} prefix={}",
        ip, port, saved_instance.id, final_label, path_prefix
    );

    // Note: encryption_key parameter is ignored - SDK now manages encryption automatically
    if encryption_key.is_some() {
        println!("ℹ️  Note: Encryption keys are now managed automatically by the SDK.");
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

/// Set or update the encryption key for a Harmony instance (DEPRECATED)
///
/// This command is deprecated. Encryption keys are now managed automatically
/// by the SDK's secure storage backend.
pub fn harmony_set_key(_instance_id: &str, _encryption_key: &str) -> anyhow::Result<()> {
    println!();
    println!("⚠️  This command is deprecated.");
    println!();
    println!("Encryption keys are now managed automatically by the Runbeam SDK.");
    println!(
        "The SDK uses secure OS-native storage (Keychain, Secret Service, Credential Manager)"
    );
    println!("with automatic fallback to encrypted filesystem storage.");
    println!();
    println!("No manual key management is required.");
    println!();

    Ok(())
}

/// Show the encryption key for a Harmony instance (DEPRECATED)
///
/// This command is deprecated. Encryption keys are now managed automatically
/// by the SDK's secure storage backend.
pub fn harmony_show_key(_instance_id: &str) -> anyhow::Result<()> {
    println!();
    println!("⚠️  This command is deprecated.");
    println!();
    println!("Encryption keys are now managed automatically by the Runbeam SDK.");
    println!(
        "The SDK uses secure OS-native storage (Keychain, Secret Service, Credential Manager)"
    );
    println!("with automatic fallback to encrypted filesystem storage.");
    println!();
    println!("Keys are stored securely and are not directly accessible.");
    println!();

    Ok(())
}

/// Delete the encryption key for a Harmony instance (DEPRECATED)
///
/// This command is deprecated. Encryption keys are now managed automatically
/// by the SDK's secure storage backend.
pub fn harmony_delete_key(_instance_id: &str) -> anyhow::Result<()> {
    println!();
    println!("⚠️  This command is deprecated.");
    println!();
    println!("Encryption keys are now managed automatically by the Runbeam SDK.");
    println!("The SDK handles key lifecycle automatically when storing and retrieving tokens.");
    println!();
    println!("No manual key management is required.");
    println!();

    Ok(())
}
