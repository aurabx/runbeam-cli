use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{debug, info, warn};

use crate::storage::{self, CliAuth, UserInfo};

#[derive(Debug, Serialize, Deserialize)]
struct StartLoginResponse {
    device_token: String,
    verification_url: String,
    expires_in_seconds: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct CheckLoginResponse {
    status: String,
    #[serde(default)]
    token: Option<String>,
    #[serde(default)]
    expires_in: Option<i64>,
    #[serde(default)]
    user: Option<UserData>,
    #[serde(default)]
    message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserData {
    id: String,
    email: String,
    name: String,
}

/// Get the API base URL from environment or use default
fn api_base_url() -> String {
    std::env::var("RUNBEAM_API_URL").unwrap_or_else(|_| "http://runbeam.lndo.site".to_string())
}

/// Perform the login flow: start login, open browser, poll for completion
pub fn login() -> Result<()> {
    info!("Starting CLI login process...");

    // Check if already logged in
    if let Some(_existing_auth) = storage::load_auth()? {
        println!("✓ Already logged in.");
        println!("  Run `runbeam logout` first if you want to login with a different account.");
        debug!("Existing token found, skipping login");
        return Ok(());
    }

    // Step 1: Start the login process
    let base_url = api_base_url();
    let start_url = format!("{}/api/cli/start-login", base_url);
    
    debug!("Requesting device token from {}", start_url);
    
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(&start_url)
        .send()
        .with_context(|| format!("failed to connect to {}", start_url))?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Failed to start login: HTTP {} - {}",
            response.status(),
            response.text().unwrap_or_default()
        );
    }

    let start_data: StartLoginResponse = response
        .json()
        .context("failed to parse start login response")?;

    debug!(
        "Received device token: {} (expires in {}s)",
        start_data.device_token, start_data.expires_in_seconds
    );

    // Step 2: Open the browser (detached to avoid capturing browser output)
    println!("\n🔐 Opening browser for authentication...");
    
    match open::that_detached(&start_data.verification_url) {
        Ok(_) => {
            println!("   Browser opened successfully.");
            println!("   If the browser didn't open, visit: {}", start_data.verification_url);
            println!();
        }
        Err(e) => {
            warn!("Could not open browser automatically: {}", e);
            println!("\n⚠  Could not open browser automatically.");
            println!("   Please open this URL manually in your browser:");
            println!("   {}\n", start_data.verification_url);
        }
    }

    // Step 3: Poll for authentication
    println!("⏳ Waiting for authentication in browser...");
    
    if start_data.expires_in_seconds <= 0.0 {
        anyhow::bail!("Device token has already expired. Please try again.");
    }
    
    println!("   (This will timeout in {} seconds)", start_data.expires_in_seconds.round());
    println!();

    let check_url = format!("{}/api/cli/check-login/{}", base_url, start_data.device_token);
    let poll_interval = Duration::from_secs(5);
    let max_attempts = ((start_data.expires_in_seconds as u64) / poll_interval.as_secs()) + 2;

    for attempt in 1..=max_attempts {
        debug!("Polling attempt {} of {}", attempt, max_attempts);

        thread::sleep(poll_interval);

        let response = client
            .get(&check_url)
            .send()
            .with_context(|| format!("failed to check login status"))?;

        let status_code = response.status();
        let check_data: CheckLoginResponse = response
            .json()
            .context("failed to parse check login response")?;

        debug!("Poll response: status={}, data={:?}", status_code, check_data);

        match check_data.status.as_str() {
            "authenticated" => {
                // Success! Extract JWT and metadata
                let token = check_data.token.context("no token in authenticated response")?;
                
                // Calculate expiration timestamp
                let expires_at = check_data.expires_in.map(|seconds| {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as i64;
                    now + seconds
                });
                
                // Convert UserData to UserInfo
                let user = check_data.user.map(|u| UserInfo {
                    id: u.id,
                    email: u.email,
                    name: u.name,
                });
                
                let auth = CliAuth { 
                    token,
                    expires_at,
                    user: user.clone(),
                };
                storage::save_auth(&auth)?;

                println!("✅ Authentication successful!");
                if let Some(user_info) = user {
                    println!("   Logged in as: {} ({})", user_info.name, user_info.email);
                }
                println!("   JWT saved to ~/.runbeam/auth.json");
                if let Some(exp) = expires_at {
                    println!("   Token expires in {} hours", (exp - SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64) / 3600);
                }
                info!("User successfully authenticated");
                return Ok(());
            }
            "pending" => {
                // Still waiting
                print!(".");
                std::io::Write::flush(&mut std::io::stdout()).ok();
                continue;
            }
            "expired" => {
                anyhow::bail!("Authentication request expired. Please run `runbeam login` again.");
            }
            "invalid" => {
                anyhow::bail!("Invalid authentication request. Please run `runbeam login` again.");
            }
            _ => {
                warn!("Unexpected status: {}", check_data.status);
                if let Some(msg) = check_data.message {
                    anyhow::bail!("Authentication error: {}", msg);
                } else {
                    anyhow::bail!("Unexpected authentication status: {}", check_data.status);
                }
            }
        }
    }

    println!();
    anyhow::bail!("Authentication timed out. Please run `runbeam login` again.");
}

/// Log out by removing the stored token
pub fn logout() -> Result<()> {
    info!("Logging out...");

    let cleared = storage::clear_auth()?;

    if cleared {
        println!("✅ Logged out successfully.");
        println!("   Authentication token removed.");
        info!("User logged out");
    } else {
        println!("ℹ  Not currently logged in.");
        debug!("No auth file found");
    }

    Ok(())
}

/// Authorize a Harmony instance to communicate with Runbeam Cloud
pub fn authorize_harmony(instance_id: Option<&str>, instance_label: Option<&str>) -> Result<()> {
    info!("Starting Harmony instance authorization...");

    // Load user authentication token
    let auth = storage::load_auth()?
        .context("Not logged in. Please run `runbeam login` first.")?;

    // Load the Harmony instance from storage
    let instances = storage::load_harmony_instances()?;
    
    let instance = if let Some(id) = instance_id {
        instances.iter().find(|i| i.id == id)
    } else if let Some(label) = instance_label {
        instances.iter().find(|i| i.label == label)
    } else {
        anyhow::bail!("Please specify a Harmony instance using --id or --label");
    };

    let instance = instance.context("Harmony instance not found. Use `runbeam harmony:list` to see available instances.")?;

    println!("\n🔐 Authorizing Gateway (Harmony instance): {}", instance.label);
    println!("   Instance ID: {}", instance.id);
    println!("   Address: {}:{}", instance.ip, instance.port);
    println!();

    // Call Harmony management API - Harmony will then call Runbeam to get machine token
    let harmony_auth_url = format!(
        "http://{}:{}/{}/authorize",
        instance.ip, instance.port, instance.path_prefix
    );

    debug!("Calling Harmony management API: {}", harmony_auth_url);

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(&harmony_auth_url)
        .header("Authorization", format!("Bearer {}", auth.token))
        .json(&serde_json::json!({
            "gateway_code": instance.id,
        }))
        .send()
        .with_context(|| format!("Failed to connect to Harmony at {}", harmony_auth_url))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        anyhow::bail!(
            "Harmony authorization failed: HTTP {} - {}",
            status,
            body
        );
    }

    let result: serde_json::Value = response
        .json()
        .context("Failed to parse Harmony response")?;

    println!("✅ Harmony instance authorized successfully!");
    println!();
    println!("   The Harmony instance can now communicate with Runbeam Cloud.");
    if let Some(expires_at) = result["expires_at"].as_str() {
        println!("   Machine token expires at: {}", expires_at);
    }
    if let Some(expires_in) = result["expires_in"].as_i64() {
        println!("   Machine token expires in {} days", expires_in / 86400);
    }

    info!("Harmony instance authorized: {}", instance.id);
    Ok(())
}
