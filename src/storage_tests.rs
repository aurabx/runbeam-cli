#[cfg(test)]
mod tests {
    use super::super::*;
    use std::fs;
    use tempfile::TempDir;

    // Helper to create a test environment
    fn setup_test_env() -> TempDir {
        TempDir::new().expect("Failed to create temp dir")
    }

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

    // Integration tests that use actual file I/O would go here,
    // but they require mocking the base_dir() function or using
    // environment variables to override paths. For now, we focus
    // on unit tests of pure functions.
}
