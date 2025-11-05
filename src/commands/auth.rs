use anyhow::{Context, Result};
use runbeam_sdk::{RunbeamClient, UserInfo, validate_jwt_token as sdk_validate_jwt};
use serde::{Deserialize, Serialize};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{debug, info, warn};

use crate::commands::config;
use crate::storage::{self, CliAuth};

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
    user: Option<UserInfo>,
    #[serde(default)]
    message: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct HarmonyErrorResponse {
    error: String,
    message: String,
}

/// Get the API base URL from config, environment, or use default
fn api_base_url() -> Result<String> {
    config::get_api_url()
}

/// Perform the login flow: start login, open browser, poll for completion
pub fn login() -> Result<()> {
    info!("Starting CLI login process...");

    // Check if already logged in with a valid token
    if let Some(existing_auth) = storage::load_auth()? {
        // Verify the token is still valid
        let validation_result = tokio::runtime::Runtime::new()
            .expect("Failed to create Tokio runtime")
            .block_on(sdk_validate_jwt(&existing_auth.token, 24));

        if validation_result.is_ok() {
            println!("✓ Already logged in with a valid token.");
            println!("  Run `runbeam logout` first if you want to login with a different account.");
            debug!("Valid token found, skipping login");
            return Ok(());
        } else {
            println!("ℹ️  Existing token is invalid or expired. Logging in again...");
            debug!("Invalid/expired token found, proceeding with login");
        }
    }

    // Step 1: Start the login process
    let base_url = api_base_url()?;
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
            println!(
                "   If the browser didn't open, visit: {}",
                start_data.verification_url
            );
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

    println!(
        "   (This will timeout in {} seconds)",
        start_data.expires_in_seconds.round()
    );
    println!();

    let check_url = format!(
        "{}/api/cli/check-login/{}",
        base_url, start_data.device_token
    );
    let poll_interval = Duration::from_secs(5);
    let max_attempts = ((start_data.expires_in_seconds as u64) / poll_interval.as_secs()) + 2;

    for attempt in 1..=max_attempts {
        debug!("Polling attempt {} of {}", attempt, max_attempts);

        thread::sleep(poll_interval);

        let response = client
            .get(&check_url)
            .send()
            .with_context(|| "failed to check login status".to_string())?;

        let status_code = response.status();

        // Check for non-success status codes
        if !status_code.is_success() {
            let error_body = response
                .text()
                .unwrap_or_else(|_| "<unable to read response>".to_string());
            debug!("Non-success response: {} - {}", status_code, error_body);
            anyhow::bail!("Login check failed: HTTP {} - {}", status_code, error_body);
        }

        let check_data: CheckLoginResponse = response
            .json()
            .with_context(|| "failed to parse check login response")?;

        debug!(
            "Poll response: status={}, data={:?}",
            status_code, check_data
        );

        match check_data.status.as_str() {
            "authenticated" => {
                // Success! Extract JWT and metadata
                let token = check_data
                    .token
                    .context("no token in authenticated response")?;
                let token_clone = token.clone();

                // Calculate expiration timestamp
                let expires_at = check_data.expires_in.map(|seconds| {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as i64;
                    now + seconds
                });

                // User info is already in the correct format (UserInfo from SDK)
                let user = check_data.user.clone();

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
                    println!(
                        "   Token expires in {} hours",
                        (exp - SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs() as i64)
                            / 3600
                    );
                }

                // Verify the token using SDK (RS256 with JWKS)
                let validation_result = tokio::runtime::Runtime::new()
                    .expect("Failed to create Tokio runtime")
                    .block_on(sdk_validate_jwt(&token_clone, 24));

                match validation_result {
                    Ok(jwt_claims) => {
                        debug!("JWT verification successful: iss={}", jwt_claims.iss);
                        println!("   Token verified using RS256 ✓");
                    }
                    Err(e) => {
                        warn!("JWT verification failed: {}", e);
                        println!("   ⚠  Token verification failed: {}", e);
                    }
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
    let auth = storage::load_auth()?.context("Not logged in. Please run `runbeam login` first.")?;

    // Load the Harmony instance from storage
    let instances = storage::load_harmony_instances()?;

    let instance = if let Some(id) = instance_id {
        instances.iter().find(|i| i.id == id)
    } else if let Some(label) = instance_label {
        instances.iter().find(|i| i.label == label)
    } else {
        anyhow::bail!("Please specify a Harmony instance using --id or --label");
    };

    let instance = instance.context(
        "Harmony instance not found. Use `runbeam harmony:list` to see available instances.",
    )?;

    println!(
        "\n🔐 Authorizing Gateway (Harmony instance): {}",
        instance.label
    );
    println!("   Instance ID: {}", instance.id);
    println!("   Address: {}:{}", instance.ip, instance.port);
    println!();

    // Use SDK's RunbeamClient to authorize the gateway
    // Get API base URL from config
    let api_url = api_base_url()?;
    debug!("Using API URL: {}", api_url);

    // Create SDK client and authorize gateway
    let client = RunbeamClient::new(api_url);

    // Create Tokio runtime for async operations
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    let auth_response = runtime
        .block_on(client.authorize_gateway(
            &auth.token,
            &instance.id,
            None, // machine_public_key
            None, // metadata
        ))
        .context("Failed to authorize gateway with Runbeam Cloud")?;

    println!("✅ Gateway authorized with Runbeam Cloud!");
    println!();
    println!(
        "   Gateway: {} ({})",
        auth_response.gateway.name, auth_response.gateway.code
    );
    println!("   Gateway ID: {}", auth_response.gateway.id);
    println!("   Machine token expires at: {}", auth_response.expires_at);

    // Calculate and display expiry in days
    let expires_in_days = (auth_response.expires_in / 86400.0).round() as i64;
    println!("   Machine token expires in {} days", expires_in_days);

    if !auth_response.abilities.is_empty() {
        println!("   Token abilities: {}", auth_response.abilities.join(", "));
    }

    if let Some(authorized_by) = &auth_response.gateway.authorized_by {
        println!(
            "   Authorized by: {} ({})",
            authorized_by.name, authorized_by.email
        );
    }
    println!();

    info!("Gateway authorized: {}", auth_response.gateway.id);

    // Send machine token to Harmony proxy instance
    println!(
        "\n📡 Sending token to Harmony proxy at {}:{}...",
        instance.ip, instance.port
    );

    let harmony_url = format!(
        "http://{}:{}/{}/token",
        instance.ip, instance.port, instance.path_prefix
    );
    debug!("Posting token to: {}", harmony_url);

    let token_payload = serde_json::json!({
        "machine_token": auth_response.machine_token,
        "expires_at": auth_response.expires_at,
        "gateway_id": auth_response.gateway.id,
        "gateway_code": auth_response.gateway.code,
        "abilities": auth_response.abilities,
    });

    let http_client = reqwest::Client::new();
    let post_result: Result<(reqwest::StatusCode, Option<String>), reqwest::Error> = runtime
        .block_on(async {
            let response = http_client
                .post(&harmony_url)
                .json(&token_payload)
                .timeout(std::time::Duration::from_secs(5))
                .send()
                .await?;

            let status = response.status();
            if status.is_success() {
                Ok((status, None))
            } else {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                Ok((status, Some(error_text)))
            }
        });

    match post_result {
        Ok((status, error_text)) => {
            if status.is_success() {
                println!("✅ Token saved to Harmony proxy successfully!");
                println!();
                println!("🎉 Authorization complete! Harmony is ready to use.");
            } else if status == reqwest::StatusCode::FORBIDDEN {
                // Handle 403 Forbidden - check if it's the runbeam.enabled issue
                let is_runbeam_disabled = if let Some(ref text) = error_text {
                    // Try to parse as JSON error response
                    if let Ok(error_response) = serde_json::from_str::<HarmonyErrorResponse>(text) {
                        error_response
                            .message
                            .contains("Runbeam Cloud integration is disabled")
                            || error_response.message.contains("runbeam.enabled")
                    } else {
                        // Fallback: check raw text
                        text.contains("Runbeam Cloud integration is disabled")
                            || text.contains("runbeam.enabled")
                    }
                } else {
                    false
                };

                if is_runbeam_disabled {
                    println!("⚠️  Harmony proxy rejected the authorization (HTTP 403):");
                    println!("   Runbeam Cloud integration is disabled on the Harmony instance.");
                    println!();
                    println!("   To fix this:");
                    println!("   1. Edit your Harmony configuration file (config.toml)");
                    println!("   2. Set: [runbeam]\n      enabled = true");
                    println!("   3. Restart Harmony and try again:");
                    println!("      runbeam harmony:authorize --id {}", instance.id);
                    println!();
                    println!(
                        "The gateway is authorized with Runbeam Cloud, but the token could not"
                    );
                    println!("be delivered to the Harmony instance.");
                } else {
                    // Generic 403 error
                    println!("⚠️  Failed to save token to Harmony proxy (HTTP 403 Forbidden):");
                    if let Some(text) = error_text {
                        println!("   {}", text);
                    }
                    println!();
                    println!("The gateway is authorized with Runbeam Cloud, but you'll need to");
                    println!(
                        "manually configure the token in Harmony or restart the authorization."
                    );
                }
            } else {
                // Other non-success status codes
                println!(
                    "⚠️  Failed to save token to Harmony proxy (HTTP {}):",
                    status
                );
                if let Some(text) = error_text {
                    println!("   {}", text);
                }
                println!();
                println!("The gateway is authorized with Runbeam Cloud, but you'll need to");
                println!("manually configure the token in Harmony or restart the authorization.");
            }
        }
        Err(e) => {
            println!("⚠️  Could not connect to Harmony proxy:");
            println!("   {}", e);
            println!();
            println!("The gateway is authorized with Runbeam Cloud, but the token could not");
            println!("be delivered to the Harmony instance. Please ensure Harmony is running at");
            println!(
                "{}:{} and try again, or manually configure the token.",
                instance.ip, instance.port
            );
        }
    }
    println!();

    Ok(())
}

/// Verify the stored authentication token
pub fn verify_token() -> Result<()> {
    info!("Verifying stored authentication token...");

    // Load authentication from storage
    let auth = storage::load_auth()?
        .context("No authentication token found. Please run `runbeam login` first.")?;

    println!("\n🔐 Verifying JWT token...");
    println!();

    // Validate the token using SDK (async)
    let validation_result = tokio::runtime::Runtime::new()
        .expect("Failed to create Tokio runtime")
        .block_on(sdk_validate_jwt(&auth.token, 24));

    match validation_result {
        Ok(claims) => {
            println!("✅ Token is valid!");
            println!();
            println!("Token Information:");
            println!("  Issuer:       {}", claims.iss);
            println!("  Subject:      {}", claims.sub);
            if let Some(aud) = &claims.aud {
                println!("  Audience:     {}", aud);
            }
            println!();

            // Display user information if available
            if let Some(user) = &claims.user {
                println!("User Information:");
                println!("  Name:         {}", user.name);
                println!("  Email:        {}", user.email);
                println!("  User ID:      {}", user.id);
                println!();
            }

            // Display team information if available
            if let Some(team) = &claims.team {
                println!("Team Information:");
                println!("  Name:         {}", team.name);
                println!("  Team ID:      {}", team.id);
                println!();
            }

            // Display expiration information
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;
            let time_remaining = claims.exp - now;

            if time_remaining > 0 {
                let hours = time_remaining / 3600;
                let minutes = (time_remaining % 3600) / 60;
                println!("Expiration:");
                println!("  Expires at:   {} (Unix timestamp)", claims.exp);
                if hours > 24 {
                    println!("  Time left:    {} days, {} hours", hours / 24, hours % 24);
                } else if hours > 0 {
                    println!("  Time left:    {} hours, {} minutes", hours, minutes);
                } else {
                    println!("  Time left:    {} minutes", minutes);
                }
            } else {
                println!("⚠️  Warning: Token has expired!");
                println!("  Expired at:   {} (Unix timestamp)", claims.exp);
                println!();
                println!("Please run `runbeam login` to get a new token.");
            }

            info!("Token verification successful");
            Ok(())
        }
        Err(e) => {
            println!("❌ Token verification failed!");
            println!();
            println!("Error: {}", e);
            println!();
            println!("This usually means:");
            println!("  • The token has expired");
            println!("  • The token signature is invalid");
            println!("  • The API URL has changed");
            println!();
            println!("Please run `runbeam login` to get a new token.");

            warn!("Token verification failed: {}", e);
            anyhow::bail!("Token verification failed")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harmony_error_response_parsing() {
        // Test parsing valid JSON error response
        let json = r#"{"error":"Forbidden","message":"Runbeam Cloud integration is disabled. Set runbeam.enabled=true in configuration to use this endpoint."}"#;
        let result: Result<HarmonyErrorResponse, _> = serde_json::from_str(json);
        assert!(result.is_ok());

        let error_response = result.unwrap();
        assert_eq!(error_response.error, "Forbidden");
        assert!(
            error_response
                .message
                .contains("Runbeam Cloud integration is disabled")
        );
        assert!(error_response.message.contains("runbeam.enabled"));
    }

    #[test]
    fn test_harmony_error_detection() {
        // Test that we correctly detect the runbeam.enabled error
        let json = r#"{"error":"Forbidden","message":"Runbeam Cloud integration is disabled. Set runbeam.enabled=true in configuration to use this endpoint."}"#;

        if let Ok(error_response) = serde_json::from_str::<HarmonyErrorResponse>(json) {
            let is_runbeam_disabled = error_response
                .message
                .contains("Runbeam Cloud integration is disabled")
                || error_response.message.contains("runbeam.enabled");
            assert!(is_runbeam_disabled);
        } else {
            panic!("Failed to parse error response");
        }
    }

    #[test]
    fn test_generic_403_error() {
        // Test that generic 403 errors don't trigger the specific message
        let json = r#"{"error":"Forbidden","message":"Access denied"}"#;

        if let Ok(error_response) = serde_json::from_str::<HarmonyErrorResponse>(json) {
            let is_runbeam_disabled = error_response
                .message
                .contains("Runbeam Cloud integration is disabled")
                || error_response.message.contains("runbeam.enabled");
            assert!(!is_runbeam_disabled);
        }
    }

    #[test]
    fn test_malformed_json_fallback() {
        // Test that malformed JSON doesn't crash
        let malformed = "This is not JSON";
        let result: Result<HarmonyErrorResponse, _> = serde_json::from_str(malformed);
        assert!(result.is_err());

        // Test fallback to raw text detection
        let is_runbeam_disabled = malformed.contains("Runbeam Cloud integration is disabled")
            || malformed.contains("runbeam.enabled");
        assert!(!is_runbeam_disabled);
    }

    #[test]
    fn test_raw_text_detection() {
        // Test that we can detect the error in raw text (non-JSON)
        let raw_text =
            "Runbeam Cloud integration is disabled. Set runbeam.enabled=true in configuration.";
        let is_runbeam_disabled = raw_text.contains("Runbeam Cloud integration is disabled")
            || raw_text.contains("runbeam.enabled");
        assert!(is_runbeam_disabled);
    }
}
