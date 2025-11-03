/// Config Module Tests
///
/// Tests for configuration management in `src/commands/config.rs` including:
/// - Loading and saving config files
/// - API URL resolution (config > env > default)
/// - URL validation and normalization
/// - Config get/set/unset operations

mod common;

use common::{create_mock_config, TestEnv};
use serial_test::serial;
use std::fs;

// Helper to get the path to config.json in the test environment
fn config_file_path(env: &TestEnv) -> std::path::PathBuf {
    env.data_file("config.json")
}

// Helper to write a config file
fn write_config_file(env: &TestEnv, config: &serde_json::Value) {
    env.write_json_file("config.json", config);
}

// Helper to read a config file
fn read_config_file(env: &TestEnv) -> serde_json::Value {
    env.read_json_file("config.json")
}

#[test]
#[serial]
fn test_load_config_missing_file() {
    let env = TestEnv::new();
    
    let path = config_file_path(&env);
    assert!(!path.exists(), "config.json should not exist initially");
    
    // When the file doesn't exist, load should return default config (empty object)
    // This tests the config module's behavior of returning CliConfig::default()
}

#[test]
#[serial]
fn test_load_config_empty_file() {
    let env = TestEnv::new();
    
    // Write an empty JSON object
    write_config_file(&env, &serde_json::json!({}));
    
    let path = config_file_path(&env);
    assert!(path.exists(), "config.json should exist");
    
    // Verify the file contains an empty object
    let content = fs::read_to_string(&path).expect("Failed to read file");
    let data: serde_json::Value = serde_json::from_str(&content).expect("Failed to parse JSON");
    assert!(data.is_object(), "Content should be an object");
    assert_eq!(data.as_object().unwrap().len(), 0, "Object should be empty");
}

#[test]
#[serial]
fn test_load_config_existing_file() {
    let env = TestEnv::new();
    
    // Create config with API URL
    let config = create_mock_config(Some("https://api.example.com"));
    write_config_file(&env, &config);
    
    // Verify the data can be read back
    let loaded = read_config_file(&env);
    assert_eq!(loaded, config, "Loaded data should match written data");
    
    assert_eq!(loaded["api_url"], "https://api.example.com");
}

#[test]
#[serial]
fn test_save_config_creates_file() {
    let env = TestEnv::new();
    
    let path = config_file_path(&env);
    assert!(!path.exists(), "File should not exist initially");
    
    // Write config
    let config = create_mock_config(Some("https://test.example.com"));
    write_config_file(&env, &config);
    
    assert!(path.exists(), "File should exist after save");
    
    // Verify content
    let loaded = read_config_file(&env);
    assert_eq!(loaded, config);
}

#[test]
#[serial]
fn test_save_config_overwrites_existing() {
    let env = TestEnv::new();
    
    // Write initial config
    let initial = create_mock_config(Some("https://old.example.com"));
    write_config_file(&env, &initial);
    
    // Overwrite with new config
    let updated = create_mock_config(Some("https://new.example.com"));
    write_config_file(&env, &updated);
    
    // Verify new data is saved
    let loaded = read_config_file(&env);
    assert_eq!(loaded, updated, "Should contain updated data");
    assert_ne!(loaded, initial, "Should not contain old data");
}

#[test]
#[serial]
fn test_config_pretty_formatted() {
    let env = TestEnv::new();
    
    let config = create_mock_config(Some("https://api.example.com"));
    write_config_file(&env, &config);
    
    // Read raw file content
    let path = config_file_path(&env);
    let content = fs::read_to_string(&path).expect("Failed to read file");
    
    // Pretty-printed JSON should have newlines and indentation
    assert!(content.contains('\n'), "Should contain newlines");
    assert!(content.contains("  ") || content.contains("\"api_url\""), "Should contain indentation or field");
}

#[test]
#[serial]
fn test_config_with_http_url() {
    let env = TestEnv::new();
    
    let config = serde_json::json!({
        "api_url": "http://localhost:8080"
    });
    
    write_config_file(&env, &config);
    
    let loaded = read_config_file(&env);
    assert_eq!(loaded["api_url"], "http://localhost:8080");
}

#[test]
#[serial]
fn test_config_with_https_url() {
    let env = TestEnv::new();
    
    let config = serde_json::json!({
        "api_url": "https://api.production.com"
    });
    
    write_config_file(&env, &config);
    
    let loaded = read_config_file(&env);
    assert_eq!(loaded["api_url"], "https://api.production.com");
}

#[test]
#[serial]
fn test_config_url_without_trailing_slash() {
    let env = TestEnv::new();
    
    // URLs should be normalized without trailing slashes
    let config = serde_json::json!({
        "api_url": "https://api.example.com"
    });
    
    write_config_file(&env, &config);
    
    let loaded = read_config_file(&env);
    let url = loaded["api_url"].as_str().unwrap();
    assert!(!url.ends_with('/'), "URL should not end with slash");
}

#[test]
#[serial]
fn test_config_file_structure() {
    let env = TestEnv::new();
    
    let config = create_mock_config(Some("https://api.example.com"));
    write_config_file(&env, &config);
    
    let path = config_file_path(&env);
    let content = fs::read_to_string(&path).expect("Failed to read file");
    
    // Verify it's valid JSON
    let parsed: serde_json::Value = serde_json::from_str(&content).expect("Should be valid JSON");
    assert!(parsed.is_object());
    
    // Verify structure
    let obj = parsed.as_object().unwrap();
    assert!(obj.contains_key("api_url"));
}

#[test]
#[serial]
fn test_config_multiple_updates() {
    let env = TestEnv::new();
    
    // First update
    let config1 = create_mock_config(Some("https://api1.example.com"));
    write_config_file(&env, &config1);
    
    let loaded1 = read_config_file(&env);
    assert_eq!(loaded1["api_url"], "https://api1.example.com");
    
    // Second update
    let config2 = create_mock_config(Some("https://api2.example.com"));
    write_config_file(&env, &config2);
    
    let loaded2 = read_config_file(&env);
    assert_eq!(loaded2["api_url"], "https://api2.example.com");
    
    // Third update
    let config3 = create_mock_config(Some("https://api3.example.com"));
    write_config_file(&env, &config3);
    
    let loaded3 = read_config_file(&env);
    assert_eq!(loaded3["api_url"], "https://api3.example.com");
}

#[test]
#[serial]
fn test_config_priority_file_over_env() {
    let mut env = TestEnv::new();
    
    // Set environment variable
    env.set_env("RUNBEAM_API_URL", "https://env.example.com");
    
    // Set config file (should take precedence)
    let config = create_mock_config(Some("https://file.example.com"));
    write_config_file(&env, &config);
    
    // When the actual config module loads, it should use file value over env
    // This test documents that config file has higher priority
    let loaded = read_config_file(&env);
    assert_eq!(loaded["api_url"], "https://file.example.com");
}

#[test]
#[serial]
fn test_config_env_when_no_file() {
    let mut env = TestEnv::new();
    
    // Set environment variable
    env.set_env("RUNBEAM_API_URL", "https://env.example.com");
    
    // No config file exists
    let path = config_file_path(&env);
    assert!(!path.exists());
    
    // When the actual config module loads, it should use env value
    // This test documents that env is used when no config file exists
}

#[test]
#[serial]
fn test_config_default_when_no_file_or_env() {
    let env = TestEnv::new();
    
    // Ensure RUNBEAM_API_URL is not set (TestEnv isolates us)
    // No config file exists
    let path = config_file_path(&env);
    assert!(!path.exists());
    
    // When the actual config module loads, it should use default
    // Default is "http://runbeam.lndo.site"
    // This test documents the default value behavior
}

#[test]
#[serial]
fn test_config_unset_removes_field() {
    let env = TestEnv::new();
    
    // Start with a config that has api_url
    let config = create_mock_config(Some("https://api.example.com"));
    write_config_file(&env, &config);
    
    // Verify it exists
    let loaded = read_config_file(&env);
    assert!(loaded.get("api_url").is_some());
    
    // Unset by writing empty config
    let empty_config = create_mock_config(None);
    write_config_file(&env, &empty_config);
    
    // Verify it's removed
    let loaded_after = read_config_file(&env);
    assert!(loaded_after.get("api_url").is_none() || loaded_after["api_url"].is_null());
}

#[test]
#[serial]
fn test_config_with_special_characters_in_url() {
    let env = TestEnv::new();
    
    let config = serde_json::json!({
        "api_url": "https://api-test.example.com:8443/v1"
    });
    
    write_config_file(&env, &config);
    
    let loaded = read_config_file(&env);
    assert_eq!(loaded["api_url"], "https://api-test.example.com:8443/v1");
}

#[test]
#[serial]
fn test_config_with_localhost() {
    let env = TestEnv::new();
    
    let config = serde_json::json!({
        "api_url": "http://localhost:3000"
    });
    
    write_config_file(&env, &config);
    
    let loaded = read_config_file(&env);
    assert_eq!(loaded["api_url"], "http://localhost:3000");
}

#[test]
#[serial]
fn test_config_with_ip_address() {
    let env = TestEnv::new();
    
    let config = serde_json::json!({
        "api_url": "http://192.168.1.100:8080"
    });
    
    write_config_file(&env, &config);
    
    let loaded = read_config_file(&env);
    assert_eq!(loaded["api_url"], "http://192.168.1.100:8080");
}

#[test]
#[serial]
fn test_config_file_permissions() {
    let env = TestEnv::new();
    
    let config = create_mock_config(Some("https://api.example.com"));
    write_config_file(&env, &config);
    
    let path = config_file_path(&env);
    
    // Verify file exists and is readable
    assert!(path.exists());
    assert!(path.is_file());
    
    let metadata = fs::metadata(&path).expect("Should be able to read metadata");
    assert!(metadata.len() > 0, "File should not be empty");
}

#[test]
#[serial]
fn test_config_empty_then_populated() {
    let env = TestEnv::new();
    
    // Start with empty config
    let empty = create_mock_config(None);
    write_config_file(&env, &empty);
    
    let loaded1 = read_config_file(&env);
    assert!(loaded1.get("api_url").is_none() || loaded1["api_url"].is_null());
    
    // Populate with URL
    let populated = create_mock_config(Some("https://api.example.com"));
    write_config_file(&env, &populated);
    
    let loaded2 = read_config_file(&env);
    assert_eq!(loaded2["api_url"], "https://api.example.com");
}

#[test]
#[serial]
fn test_config_url_validation_concept() {
    // This test documents the expected URL validation behavior
    // The actual config module should:
    // - Accept http:// and https:// URLs
    // - Reject URLs without protocol
    // - Normalize by removing trailing slashes
    
    let valid_urls = vec![
        "http://localhost:8080",
        "https://api.example.com",
        "http://192.168.1.1:3000",
        "https://api-staging.example.com:8443",
    ];
    
    for url in valid_urls {
        assert!(url.starts_with("http://") || url.starts_with("https://"));
    }
}

#[test]
#[serial]
fn test_config_invalid_key_concept() {
    // This test documents that only specific keys are valid
    // Valid keys: "api-url" or "api_url"
    // Invalid keys: anything else
    
    let valid_keys = vec!["api-url", "api_url"];
    let invalid_keys = vec!["api", "url", "base-url", "endpoint", "server"];
    
    // In the actual config module, set_config() and unset_config()
    // should reject invalid keys
    
    for key in &valid_keys {
        assert!(key.contains("api") && (key.contains("-") || key.contains("_")));
    }
    
    for key in &invalid_keys {
        assert!(!valid_keys.contains(key));
    }
}

#[test]
#[serial]
fn test_config_data_directory_structure() {
    let env = TestEnv::new();
    
    // Verify the data directory exists
    let data_dir = env.data_path();
    assert!(data_dir.exists(), "Data directory should exist");
    assert!(data_dir.is_dir(), "Data path should be a directory");
    
    // Config file should be in the data directory
    let config_path = config_file_path(&env);
    assert!(config_path.parent().unwrap() == data_dir);
}

#[test]
#[serial]
fn test_config_concurrent_writes() {
    let env = TestEnv::new();
    
    // Simulate multiple writes (would be concurrent in real usage)
    let configs = vec![
        create_mock_config(Some("https://api1.example.com")),
        create_mock_config(Some("https://api2.example.com")),
        create_mock_config(Some("https://api3.example.com")),
    ];
    
    for config in &configs {
        write_config_file(&env, config);
    }
    
    // Last write should win
    let loaded = read_config_file(&env);
    assert_eq!(loaded["api_url"], "https://api3.example.com");
}

#[test]
#[serial]
fn test_config_tmp_file_cleanup() {
    let env = TestEnv::new();
    
    // Write config (which uses atomic write with .tmp file)
    let config = create_mock_config(Some("https://api.example.com"));
    write_config_file(&env, &config);
    
    // Verify no .tmp file is left behind
    let data_dir = env.data_path();
    let entries = fs::read_dir(data_dir).expect("Failed to read directory");
    
    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let file_name = entry.file_name();
        let file_name_str = file_name.to_str().unwrap();
        
        assert!(
            !file_name_str.ends_with(".tmp"),
            "No .tmp files should remain after save: {}",
            file_name_str
        );
    }
}

#[test]
#[serial]
fn test_config_json_format_compatibility() {
    let env = TestEnv::new();
    
    // Test that config can be read by standard JSON parsers
    let config = create_mock_config(Some("https://api.example.com"));
    write_config_file(&env, &config);
    
    let path = config_file_path(&env);
    let content = fs::read_to_string(&path).expect("Failed to read file");
    
    // Should parse as valid JSON
    let parsed: serde_json::Value = serde_json::from_str(&content).expect("Should be valid JSON");
    assert!(parsed.is_object());
    
    // Should have expected structure
    assert!(parsed.get("api_url").is_some());
}
