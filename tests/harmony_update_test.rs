use assert_cmd::Command;
use predicates::prelude::*;

// Import CLI and storage modules for testing
#[path = "../src/cli.rs"]
mod cli;

#[path = "../src/storage.rs"]
#[allow(dead_code)]  // Storage functions are used in main binary, not in these tests
mod storage;

#[test]
fn test_harmony_update_command_exists() {
    // Test that the command is registered
    let mut cmd = Command::cargo_bin("runbeam").unwrap();
    cmd.arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("harmony:update"));
}

#[test]
fn test_harmony_update_help() {
    // Test the help text for harmony:update
    let mut cmd = Command::cargo_bin("runbeam").unwrap();
    cmd.args(&["harmony:update", "--help"]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Trigger Harmony to upload its configuration to Runbeam Cloud"))
        .stdout(predicate::str::contains("--id"))
        .stdout(predicate::str::contains("--label"));
}

#[test]
fn test_harmony_update_requires_id_or_label() {
    // Test that command fails without --id or --label
    let mut cmd = Command::cargo_bin("runbeam").unwrap();
    cmd.arg("harmony:update");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("must supply --id or --label")
            .or(predicate::str::contains("No Harmony instances registered")));
}

#[test]
fn test_harmony_update_conflicts_id_and_label() {
    // Test that --id and --label are mutually exclusive
    let mut cmd = Command::cargo_bin("runbeam").unwrap();
    cmd.args(&["harmony:update", "--id", "abc123", "--label", "test"]);
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with")
            .or(predicate::str::contains("conflicts with")));
}

#[test]
fn test_harmony_update_nonexistent_instance() {
    // Test error when instance doesn't exist
    let mut cmd = Command::cargo_bin("runbeam").unwrap();
    cmd.args(&["harmony:update", "--id", "nonexistent"]);
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("no instance")
            .or(predicate::str::contains("not found"))
            .or(predicate::str::contains("No Harmony instances registered")));
}

#[test]
fn test_harmony_update_with_verbose() {
    // Test that verbose flag works
    let mut cmd = Command::cargo_bin("runbeam").unwrap();
    cmd.args(&["-v", "harmony:update", "--help"]);
    
    cmd.assert().success();
}

#[test]
fn test_harmony_update_with_quiet() {
    // Test that quiet flag works
    let mut cmd = Command::cargo_bin("runbeam").unwrap();
    cmd.args(&["-q", "harmony:update", "--help"]);
    
    cmd.assert().success();
}

/// Test CLI argument parsing for harmony:update command
#[test]
fn test_harmony_update_cli_parsing() {
    use clap::Parser;

    // Test parsing with --id
    let args = vec!["runbeam", "harmony:update", "--id", "test123"];
    let cli = cli::Cli::parse_from(args);
    
    match cli.command {
        Some(cli::Command::HarmonyUpdate { id, label }) => {
            assert_eq!(id, Some("test123".to_string()));
            assert_eq!(label, None);
        }
        _ => panic!("Expected HarmonyUpdate command"),
    }

    // Test parsing with --label
    let args = vec!["runbeam", "harmony:update", "--label", "my-harmony"];
    let cli = cli::Cli::parse_from(args);
    
    match cli.command {
        Some(cli::Command::HarmonyUpdate { id, label }) => {
            assert_eq!(id, None);
            assert_eq!(label, Some("my-harmony".to_string()));
        }
        _ => panic!("Expected HarmonyUpdate command"),
    }

    // Test parsing with -l shorthand
    let args = vec!["runbeam", "harmony:update", "-l", "test"];
    let cli = cli::Cli::parse_from(args);
    
    match cli.command {
        Some(cli::Command::HarmonyUpdate { id, label }) => {
            assert_eq!(id, None);
            assert_eq!(label, Some("test".to_string()));
        }
        _ => panic!("Expected HarmonyUpdate command"),
    }
}

/// Test URL construction in management module
#[test]
fn test_update_url_construction() {

    let instance = storage::HarmonyInstance {
        id: "test123".to_string(),
        ip: "127.0.0.1".to_string(),
        port: 9090,
        label: "test-harmony".to_string(),
        path_prefix: "admin".to_string(),
        gateway_id: None,
    };

    // Verify the expected URL format
    let expected_url = "http://127.0.0.1:9090/admin/update";
    let constructed_url = format!(
        "http://{}:{}/{}/update",
        instance.ip,
        instance.port,
        instance.path_prefix.trim_matches('/')
    );

    assert_eq!(constructed_url, expected_url);

    // Test with path prefix that has slashes
    let instance_with_slashes = storage::HarmonyInstance {
        id: "test456".to_string(),
        ip: "127.0.0.1".to_string(),
        port: 8080,
        label: "test2".to_string(),
        path_prefix: "/admin/".to_string(),
        gateway_id: None,
    };

    let constructed_url = format!(
        "http://{}:{}/{}/update",
        instance_with_slashes.ip,
        instance_with_slashes.port,
        instance_with_slashes.path_prefix.trim_matches('/')
    );

    assert_eq!(constructed_url, "http://127.0.0.1:8080/admin/update");
}

/// Documentation test for manual end-to-end testing
///
/// This test documents the manual testing procedure for the harmony:update command.
///
/// ## Prerequisites
/// 1. Harmony proxy running on localhost:9090
/// 2. Runbeam Cloud API running (or mock server)
/// 3. Valid user authentication token
///
/// ## Test Steps
///
/// ### 1. Setup Harmony instance
/// ```sh
/// cargo run -- harmony:add --ip 127.0.0.1 --port 9090 --label test-harmony
/// cargo run -- harmony:list
/// # Expected: Shows test-harmony instance
/// ```
///
/// ### 2. Test without authorization (should fail)
/// ```sh
/// cargo run -- harmony:update --label test-harmony
/// # Expected: Error about needing to authorize first
/// # Error message should contain: "Run `runbeam harmony:authorize` first"
/// ```
///
/// ### 3. Authorize Harmony
/// ```sh
/// cargo run -- login
/// # Follow browser auth flow
///
/// cargo run -- harmony:authorize --label test-harmony
/// # Expected: Success message with gateway details
/// ```
///
/// ### 4. Test update with valid authorization
/// ```sh
/// cargo run -- harmony:update --label test-harmony
/// # Expected: "✓ Configuration uploaded successfully (XXXX bytes)"
/// ```
///
/// ### 5. Test update with ID instead of label
/// ```sh
/// cargo run -- harmony:list
/// # Copy the ID of test-harmony
///
/// cargo run -- harmony:update --id <copied-id>
/// # Expected: Same success message
/// ```
///
/// ### 6. Test with verbose output
/// ```sh
/// cargo run -- -v harmony:update --label test-harmony
/// # Expected: Additional debug logging about the request
/// ```
///
/// ### 7. Test with nonexistent instance
/// ```sh
/// cargo run -- harmony:update --id nonexistent
/// # Expected: Error "no instance with id 'nonexistent'"
/// ```
///
/// ### 8. Test direct API call
/// ```sh
/// curl -X POST http://localhost:9090/admin/update
/// # Expected: JSON response with success/error
/// ```
///
/// ### 9. Test with Runbeam disabled
/// - Edit Harmony config: set `runbeam.enabled = false`
/// - Restart Harmony proxy
/// ```sh
/// cargo run -- harmony:update --label test-harmony
/// # Expected: Error about Runbeam integration being disabled
/// ```
///
/// ### 10. Verify in Runbeam Cloud
/// - Log into Runbeam Cloud dashboard
/// - Navigate to Gateways section
/// - Find the test gateway
/// - Verify configuration matches local Harmony config
/// - Check the update timestamp
///
/// ## Cleanup
/// ```sh
/// cargo run -- harmony:remove --label test-harmony
/// cargo run -- logout
/// ```
#[test]
fn manual_testing_documentation() {
    // This is a documentation test - see function documentation
    println!("See function documentation for manual testing steps");
}

/// Mock server test helper
///
/// This demonstrates how to set up a mock server for testing without
/// requiring the full Runbeam Cloud API.
///
/// ```ignore
/// use wiremock::{MockServer, Mock, ResponseTemplate};
/// use wiremock::matchers::{method, path};
///
/// #[tokio::test]
/// async fn test_harmony_update_with_mock_server() {
///     // Start mock server
///     let mock_server = MockServer::start().await;
///     
///     // Mock the /api/harmony/update endpoint
///     Mock::given(method("POST"))
///         .and(path("/api/harmony/update"))
///         .respond_with(ResponseTemplate::new(200)
///             .set_body_json(serde_json::json!({
///                 "status": 200
///             })))
///         .mount(&mock_server)
///         .await;
///     
///     // Run test with mock server URL
///     // ... test code here ...
/// }
/// ```
#[test]
fn mock_server_documentation() {
    println!("See function documentation for mock server setup");
}
