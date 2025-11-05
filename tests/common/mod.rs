/// Common test utilities and helpers for runbeam-cli tests
///
/// This module provides shared functionality for test setup, mocking,
/// and assertions to reduce code duplication across test files.
use serde_json::Value;
use std::env;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Test environment handle that provides isolated filesystem and environment
///
/// Automatically cleans up when dropped.
pub struct TestEnv {
    /// Temporary directory for test data
    pub temp_dir: TempDir,
    /// Path to the test data directory (inside temp_dir)
    pub data_dir: PathBuf,
    /// Original environment variables to restore on cleanup
    original_env: Vec<(String, Option<String>)>,
}

impl TestEnv {
    /// Create a new test environment with isolated filesystem
    ///
    /// Sets up:
    /// - Temporary directory for test data
    /// - Environment variable overrides
    /// - Clean state for each test
    pub fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let data_dir = temp_dir.path().join(".runbeam");

        fs::create_dir_all(&data_dir).expect("Failed to create data directory");

        // Store original environment variables
        let original_env = vec![
            ("HOME".to_string(), env::var("HOME").ok()),
            (
                "RUNBEAM_API_URL".to_string(),
                env::var("RUNBEAM_API_URL").ok(),
            ),
            ("RUST_LOG".to_string(), env::var("RUST_LOG").ok()),
        ];

        // Set HOME to temp directory for isolation
        unsafe {
            env::set_var("HOME", temp_dir.path());
        }

        TestEnv {
            temp_dir,
            data_dir,
            original_env,
        }
    }

    /// Get the path to the data directory (~/.runbeam equivalent)
    pub fn data_path(&self) -> &PathBuf {
        &self.data_dir
    }

    /// Get the path to a file within the data directory
    pub fn data_file(&self, filename: &str) -> PathBuf {
        self.data_dir.join(filename)
    }

    /// Set an environment variable for this test
    #[allow(dead_code)]
    pub fn set_env(&mut self, key: &str, value: &str) {
        // Store original value if not already stored
        if !self.original_env.iter().any(|(k, _)| k == key) {
            self.original_env
                .push((key.to_string(), env::var(key).ok()));
        }
        unsafe {
            env::set_var(key, value);
        }
    }

    /// Unset an environment variable for this test
    #[allow(dead_code)]
    pub fn unset_env(&mut self, key: &str) {
        // Store original value if not already stored
        if !self.original_env.iter().any(|(k, _)| k == key) {
            self.original_env
                .push((key.to_string(), env::var(key).ok()));
        }
        unsafe {
            env::remove_var(key);
        }
    }

    /// Write a JSON file to the data directory
    pub fn write_json_file(&self, filename: &str, data: &Value) -> PathBuf {
        let path = self.data_file(filename);
        let json = serde_json::to_string_pretty(data).expect("Failed to serialize JSON");
        fs::write(&path, json).expect("Failed to write JSON file");
        path
    }

    /// Read a JSON file from the data directory
    pub fn read_json_file(&self, filename: &str) -> Value {
        let path = self.data_file(filename);
        let content = fs::read_to_string(&path).expect("Failed to read file");
        serde_json::from_str(&content).expect("Failed to parse JSON")
    }
}

impl Drop for TestEnv {
    fn drop(&mut self) {
        // Restore original environment variables
        for (key, original_value) in &self.original_env {
            unsafe {
                match original_value {
                    Some(val) => env::set_var(key, val),
                    None => env::remove_var(key),
                }
            }
        }
    }
}

/// Create a mock Harmony instance for testing
///
/// # Arguments
///
/// * `id` - Instance ID (generated if empty)
/// * `ip` - IP address
/// * `port` - Port number
/// * `label` - Instance label
/// * `path_prefix` - Management API path prefix
pub fn create_mock_harmony_instance(
    id: &str,
    ip: &str,
    port: u16,
    label: &str,
    path_prefix: &str,
) -> Value {
    serde_json::json!({
        "id": if id.is_empty() { format!("{:08x}", port) } else { id.to_string() },
        "ip": ip,
        "port": port,
        "label": label,
        "path_prefix": path_prefix,
    })
}

/// Create a mock CLI config for testing
///
/// # Arguments
///
/// * `api_url` - Optional API URL (None for default)
pub fn create_mock_config(api_url: Option<&str>) -> Value {
    let mut config = serde_json::json!({});
    if let Some(url) = api_url {
        config["api_url"] = serde_json::json!(url);
    }
    config
}

/// Create a mock auth token for testing
///
/// # Arguments
///
/// * `token` - JWT token string
/// * `expires_at` - Optional expiration timestamp
#[allow(dead_code)]
pub fn create_mock_auth(token: &str, expires_at: Option<i64>) -> Value {
    serde_json::json!({
        "token": token,
        "expires_at": expires_at,
        "user": {
            "id": "test-user-id",
            "name": "Test User",
            "email": "test@example.com"
        }
    })
}

/// Assert that a string contains a table with specific headers
///
/// # Arguments
///
/// * `output` - The output string to check
/// * `headers` - Expected header names
///
/// # Panics
///
/// Panics if the headers are not found in the output
pub fn assert_table_output(output: &str, headers: &[&str]) {
    for header in headers {
        assert!(
            output.to_uppercase().contains(&header.to_uppercase()),
            "Output should contain header '{}'\n\nOutput:\n{}",
            header,
            output
        );
    }

    // Check for table separator (dashes)
    assert!(
        output.contains("---") || output.contains("-+-"),
        "Output should contain table separator\n\nOutput:\n{}",
        output
    );
}

/// Assert that a JSON value has the expected structure
///
/// # Arguments
///
/// * `json` - The JSON value to check
/// * `expected_fields` - List of field names that should be present
///
/// # Panics
///
/// Panics if any expected fields are missing
pub fn assert_json_structure(json: &Value, expected_fields: &[&str]) {
    let obj = json.as_object().expect("JSON should be an object");

    for field in expected_fields {
        assert!(
            obj.contains_key(*field),
            "JSON should contain field '{}'\n\nJSON:\n{}",
            field,
            serde_json::to_string_pretty(json).unwrap()
        );
    }
}

/// Assert that a JSON array has objects with expected fields
///
/// # Arguments
///
/// * `json_array` - The JSON array to check
/// * `expected_fields` - List of field names that should be present in each object
///
/// # Panics
///
/// Panics if the array is empty or objects are missing expected fields
pub fn assert_json_array_structure(json_array: &Value, expected_fields: &[&str]) {
    let array = json_array.as_array().expect("JSON should be an array");
    assert!(!array.is_empty(), "JSON array should not be empty");

    for (i, item) in array.iter().enumerate() {
        let obj = item
            .as_object()
            .expect(&format!("Array item {} should be an object", i));

        for field in expected_fields {
            assert!(
                obj.contains_key(*field),
                "Array item {} should contain field '{}'\n\nItem:\n{}",
                i,
                field,
                serde_json::to_string_pretty(item).unwrap()
            );
        }
    }
}

/// Create a minimal valid routes JSON response for testing
pub fn create_mock_routes_response() -> Value {
    serde_json::json!({
        "routes": [
            {
                "path": "/api/test",
                "methods": ["GET", "POST"],
                "description": "Test endpoint",
                "endpoint_name": "test",
                "service_type": "http",
                "pipeline": "default"
            }
        ]
    })
}

/// Create a minimal valid pipelines JSON response for testing
pub fn create_mock_pipelines_response() -> Value {
    serde_json::json!({
        "pipelines": [
            {
                "name": "default",
                "stages": ["validate", "transform", "route"],
                "description": "Default pipeline"
            }
        ]
    })
}

/// Create a minimal valid info JSON response for testing
pub fn create_mock_info_response() -> Value {
    serde_json::json!({
        "version": "1.0.0",
        "status": "healthy",
        "uptime": 12345,
        "routes_count": 10
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_test_env_creates_temp_dir() {
        let env = TestEnv::new();
        assert!(env.temp_dir.path().exists());
        assert!(env.data_dir.exists());
    }

    #[test]
    #[serial]
    fn test_test_env_sets_home() {
        let env = TestEnv::new();
        let current_home = std::env::var("HOME").unwrap();
        assert_eq!(current_home, env.temp_dir.path().to_str().unwrap());
    }

    #[test]
    #[serial]
    fn test_test_env_restores_env_on_drop() {
        // Test with a custom environment variable to avoid interference
        let test_var = "TEST_VAR_FOR_RESTORATION";
        let original_value = "original_test_value";

        unsafe {
            env::set_var(test_var, original_value);
        }

        let captured_before = env::var(test_var).ok();

        {
            let mut test_env = TestEnv::new();
            // Modify the test variable
            test_env.set_env(test_var, "modified_value");
            assert_eq!(env::var(test_var).ok(), Some("modified_value".to_string()));
        }

        // Variable should be restored after TestEnv is dropped
        assert_eq!(
            env::var(test_var).ok(),
            captured_before,
            "Environment variable should be restored to original value after drop"
        );

        // Cleanup
        unsafe {
            env::remove_var(test_var);
        }
    }

    #[test]
    #[serial]
    fn test_write_and_read_json_file() {
        let env = TestEnv::new();
        let data = serde_json::json!({"test": "value"});

        env.write_json_file("test.json", &data);
        let read_data = env.read_json_file("test.json");

        assert_eq!(data, read_data);
    }

    #[test]
    fn test_create_mock_harmony_instance() {
        let instance = create_mock_harmony_instance("abc123", "127.0.0.1", 8081, "test", "admin");

        assert_eq!(instance["id"], "abc123");
        assert_eq!(instance["ip"], "127.0.0.1");
        assert_eq!(instance["port"], 8081);
        assert_eq!(instance["label"], "test");
        assert_eq!(instance["path_prefix"], "admin");
    }

    #[test]
    fn test_create_mock_harmony_instance_generates_id() {
        let instance = create_mock_harmony_instance("", "127.0.0.1", 8081, "test", "admin");

        // Should generate an ID based on port
        assert!(!instance["id"].as_str().unwrap().is_empty());
    }

    #[test]
    fn test_create_mock_config_with_url() {
        let config = create_mock_config(Some("https://api.example.com"));
        assert_eq!(config["api_url"], "https://api.example.com");
    }

    #[test]
    fn test_create_mock_config_without_url() {
        let config = create_mock_config(None);
        assert!(config.as_object().unwrap().is_empty());
    }

    #[test]
    fn test_assert_table_output_success() {
        let output = "ID    | LABEL | IP\n------+-------+----\nabc123| test  | 1.2.3.4";
        assert_table_output(output, &["ID", "LABEL", "IP"]);
    }

    #[test]
    #[should_panic(expected = "should contain header")]
    fn test_assert_table_output_missing_header() {
        let output = "ID    | LABEL\n------+-------\nabc123| test";
        assert_table_output(output, &["ID", "LABEL", "MISSING"]);
    }

    #[test]
    fn test_assert_json_structure_success() {
        let json = serde_json::json!({"id": "123", "name": "test", "value": 42});
        assert_json_structure(&json, &["id", "name", "value"]);
    }

    #[test]
    #[should_panic(expected = "should contain field")]
    fn test_assert_json_structure_missing_field() {
        let json = serde_json::json!({"id": "123", "name": "test"});
        assert_json_structure(&json, &["id", "name", "missing"]);
    }

    #[test]
    fn test_assert_json_array_structure_success() {
        let json = serde_json::json!([
            {"id": "1", "name": "test1"},
            {"id": "2", "name": "test2"}
        ]);
        assert_json_array_structure(&json, &["id", "name"]);
    }

    #[test]
    fn test_mock_responses_have_valid_structure() {
        let routes = create_mock_routes_response();
        assert!(routes["routes"].is_array());

        let pipelines = create_mock_pipelines_response();
        assert!(pipelines["pipelines"].is_array());

        let info = create_mock_info_response();
        assert!(info["version"].is_string());
    }
}
