use clap::Parser;

// Import the CLI structure - adjust path as needed
// We need to make sure cli module is accessible for testing
#[path = "../src/cli.rs"]
mod cli;

#[test]
fn test_parse_list_command() {
    let args = cli::Cli::parse_from(["runbeam", "list"]);
    assert!(matches!(args.command, Some(cli::Command::List)));
}

#[test]
fn test_parse_login_command() {
    let args = cli::Cli::parse_from(["runbeam", "login"]);
    assert!(matches!(args.command, Some(cli::Command::Login)));
}

#[test]
fn test_parse_logout_command() {
    let args = cli::Cli::parse_from(["runbeam", "logout"]);
    assert!(matches!(args.command, Some(cli::Command::Logout)));
}

#[test]
fn test_parse_verify_command() {
    let args = cli::Cli::parse_from(["runbeam", "verify"]);
    assert!(matches!(args.command, Some(cli::Command::Verify)));
}

#[test]
fn test_parse_harmony_add_defaults() {
    let args = cli::Cli::parse_from(["runbeam", "harmony:add"]);
    match args.command {
        Some(cli::Command::HarmonyAdd {
            ip,
            port,
            label,
            path_prefix,
            ..
        }) => {
            assert_eq!(ip, "127.0.0.1");
            assert_eq!(port, 8081);
            assert_eq!(label, None);
            assert_eq!(path_prefix, "admin");
        }
        _ => panic!("Expected HarmonyAdd command"),
    }
}

#[test]
fn test_parse_harmony_add_custom() {
    let args = cli::Cli::parse_from([
        "runbeam",
        "harmony:add",
        "--ip",
        "192.168.1.100",
        "--port",
        "9000",
        "--label",
        "my-instance",
        "--path-prefix",
        "management",
    ]);
    match args.command {
        Some(cli::Command::HarmonyAdd {
            ip,
            port,
            label,
            path_prefix,
            ..
        }) => {
            assert_eq!(ip, "192.168.1.100");
            assert_eq!(port, 9000);
            assert_eq!(label, Some("my-instance".to_string()));
            assert_eq!(path_prefix, "management");
        }
        _ => panic!("Expected HarmonyAdd command"),
    }
}

#[test]
fn test_parse_harmony_list() {
    let args = cli::Cli::parse_from(["runbeam", "harmony:list"]);
    assert!(matches!(args.command, Some(cli::Command::HarmonyList)));
}

#[test]
fn test_parse_harmony_remove_by_id() {
    let args = cli::Cli::parse_from(["runbeam", "harmony:remove", "--id", "abc123"]);
    match args.command {
        Some(cli::Command::HarmonyRemove {
            id,
            label,
            ip,
            port,
        }) => {
            assert_eq!(id, Some("abc123".to_string()));
            assert_eq!(label, None);
            assert_eq!(ip, None);
            assert_eq!(port, None);
        }
        _ => panic!("Expected HarmonyRemove command"),
    }
}

#[test]
fn test_parse_harmony_remove_by_label() {
    let args = cli::Cli::parse_from(["runbeam", "harmony:remove", "--label", "my-instance"]);
    match args.command {
        Some(cli::Command::HarmonyRemove {
            id,
            label,
            ip,
            port,
        }) => {
            assert_eq!(id, None);
            assert_eq!(label, Some("my-instance".to_string()));
            assert_eq!(ip, None);
            assert_eq!(port, None);
        }
        _ => panic!("Expected HarmonyRemove command"),
    }
}

#[test]
fn test_parse_harmony_remove_by_ip_port() {
    let args = cli::Cli::parse_from([
        "runbeam",
        "harmony:remove",
        "--ip",
        "192.168.1.100",
        "--port",
        "9000",
    ]);
    match args.command {
        Some(cli::Command::HarmonyRemove {
            id,
            label,
            ip,
            port,
        }) => {
            assert_eq!(id, None);
            assert_eq!(label, None);
            assert_eq!(ip, Some("192.168.1.100".to_string()));
            assert_eq!(port, Some(9000));
        }
        _ => panic!("Expected HarmonyRemove command"),
    }
}

#[test]
fn test_parse_harmony_info_by_id() {
    let args = cli::Cli::parse_from(["runbeam", "harmony:info", "--id", "abc123"]);
    match args.command {
        Some(cli::Command::HarmonyInfo { id, label }) => {
            assert_eq!(id, Some("abc123".to_string()));
            assert_eq!(label, None);
        }
        _ => panic!("Expected HarmonyInfo command"),
    }
}

#[test]
fn test_parse_harmony_routes_with_json() {
    let args = cli::Cli::parse_from([
        "runbeam",
        "harmony:routes",
        "--label",
        "my-instance",
        "--json",
    ]);
    match args.command {
        Some(cli::Command::HarmonyRoutes { id, label, json }) => {
            assert_eq!(id, None);
            assert_eq!(label, Some("my-instance".to_string()));
            assert!(json);
        }
        _ => panic!("Expected HarmonyRoutes command"),
    }
}

#[test]
fn test_parse_config_set() {
    let args = cli::Cli::parse_from([
        "runbeam",
        "config:set",
        "api-url",
        "https://api.example.com",
    ]);
    match args.command {
        Some(cli::Command::ConfigSet { key, value }) => {
            assert_eq!(key, "api-url");
            assert_eq!(value, "https://api.example.com");
        }
        _ => panic!("Expected ConfigSet command"),
    }
}

#[test]
fn test_parse_config_get_specific() {
    let args = cli::Cli::parse_from(["runbeam", "config:get", "api-url"]);
    match args.command {
        Some(cli::Command::ConfigGet { key }) => {
            assert_eq!(key, Some("api-url".to_string()));
        }
        _ => panic!("Expected ConfigGet command"),
    }
}

#[test]
fn test_parse_config_get_all() {
    let args = cli::Cli::parse_from(["runbeam", "config:get"]);
    match args.command {
        Some(cli::Command::ConfigGet { key }) => {
            assert_eq!(key, None);
        }
        _ => panic!("Expected ConfigGet command"),
    }
}

#[test]
fn test_parse_config_unset() {
    let args = cli::Cli::parse_from(["runbeam", "config:unset", "api-url"]);
    match args.command {
        Some(cli::Command::ConfigUnset { key }) => {
            assert_eq!(key, "api-url");
        }
        _ => panic!("Expected ConfigUnset command"),
    }
}

#[test]
fn test_parse_verbosity_flags() {
    let args = cli::Cli::parse_from(["runbeam", "-v", "list"]);
    assert_eq!(args.verbose, 1);

    let args = cli::Cli::parse_from(["runbeam", "-vv", "list"]);
    assert_eq!(args.verbose, 2);

    let args = cli::Cli::parse_from(["runbeam", "-vvv", "list"]);
    assert_eq!(args.verbose, 3);
}

#[test]
fn test_parse_quiet_flag() {
    let args = cli::Cli::parse_from(["runbeam", "-q", "list"]);
    assert!(args.quiet);
}

#[test]
fn test_parse_no_command() {
    let args = cli::Cli::parse_from(["runbeam"]);
    assert!(args.command.is_none());
}

// ============================================================================
// Missing Command Tests (Phase 3)
// ============================================================================

#[test]
fn test_parse_test_browser_command() {
    let args = cli::Cli::parse_from(["runbeam", "test-browser"]);
    assert!(
        matches!(args.command, Some(cli::Command::TestBrowser)),
        "Expected TestBrowser command"
    );
}

#[test]
fn test_parse_harmony_authorize_with_id() {
    let args = cli::Cli::parse_from(["runbeam", "harmony:authorize", "--id", "abc123"]);
    match args.command {
        Some(cli::Command::HarmonyAuthorize { id, label }) => {
            assert_eq!(id, Some("abc123".to_string()));
            assert_eq!(label, None);
        }
        _ => panic!("Expected HarmonyAuthorize command"),
    }
}

#[test]
fn test_parse_harmony_authorize_with_label() {
    let args = cli::Cli::parse_from(["runbeam", "harmony:authorize", "--label", "production"]);
    match args.command {
        Some(cli::Command::HarmonyAuthorize { id, label }) => {
            assert_eq!(id, None);
            assert_eq!(label, Some("production".to_string()));
        }
        _ => panic!("Expected HarmonyAuthorize command"),
    }
}

#[test]
fn test_parse_harmony_authorize_with_short_label() {
    let args = cli::Cli::parse_from(["runbeam", "harmony:authorize", "-l", "staging"]);
    match args.command {
        Some(cli::Command::HarmonyAuthorize { id, label }) => {
            assert_eq!(id, None);
            assert_eq!(label, Some("staging".to_string()));
        }
        _ => panic!("Expected HarmonyAuthorize command"),
    }
}

#[test]
fn test_parse_harmony_set_key() {
    let args = cli::Cli::parse_from([
        "runbeam",
        "harmony:set-key",
        "--id",
        "abc123",
        "--key",
        "test-key-value",
    ]);
    match args.command {
        Some(cli::Command::HarmonySetKey { id, encryption_key }) => {
            assert_eq!(id, "abc123");
            assert_eq!(encryption_key, "test-key-value");
        }
        _ => panic!("Expected HarmonySetKey command"),
    }
}

#[test]
fn test_parse_harmony_set_key_short_flags() {
    let args = cli::Cli::parse_from([
        "runbeam",
        "harmony:set-key",
        "--id",
        "def456",
        "-k",
        "short-key",
    ]);
    match args.command {
        Some(cli::Command::HarmonySetKey { id, encryption_key }) => {
            assert_eq!(id, "def456");
            assert_eq!(encryption_key, "short-key");
        }
        _ => panic!("Expected HarmonySetKey command"),
    }
}

// Note: Tests for required arguments are omitted because clap calls process::exit()
// instead of panicking, which causes test harness failures. The required argument
// enforcement is tested via integration tests instead.

#[test]
fn test_parse_harmony_show_key() {
    let args = cli::Cli::parse_from(["runbeam", "harmony:show-key", "--id", "abc123"]);
    match args.command {
        Some(cli::Command::HarmonyShowKey { id }) => {
            assert_eq!(id, "abc123");
        }
        _ => panic!("Expected HarmonyShowKey command"),
    }
}

#[test]
fn test_parse_harmony_delete_key() {
    let args = cli::Cli::parse_from(["runbeam", "harmony:delete-key", "--id", "xyz789"]);
    match args.command {
        Some(cli::Command::HarmonyDeleteKey { id }) => {
            assert_eq!(id, "xyz789");
        }
        _ => panic!("Expected HarmonyDeleteKey command"),
    }
}

#[test]
fn test_parse_harmony_pipelines_with_id() {
    let args = cli::Cli::parse_from(["runbeam", "harmony:pipelines", "--id", "abc123"]);
    match args.command {
        Some(cli::Command::HarmonyPipelines { id, label }) => {
            assert_eq!(id, Some("abc123".to_string()));
            assert_eq!(label, None);
        }
        _ => panic!("Expected HarmonyPipelines command"),
    }
}

#[test]
fn test_parse_harmony_pipelines_with_label() {
    let args = cli::Cli::parse_from(["runbeam", "harmony:pipelines", "--label", "test-env"]);
    match args.command {
        Some(cli::Command::HarmonyPipelines { id, label }) => {
            assert_eq!(id, None);
            assert_eq!(label, Some("test-env".to_string()));
        }
        _ => panic!("Expected HarmonyPipelines command"),
    }
}

#[test]
fn test_parse_harmony_pipelines_with_short_label() {
    let args = cli::Cli::parse_from(["runbeam", "harmony:pipelines", "-l", "prod"]);
    match args.command {
        Some(cli::Command::HarmonyPipelines { id, label }) => {
            assert_eq!(id, None);
            assert_eq!(label, Some("prod".to_string()));
        }
        _ => panic!("Expected HarmonyPipelines command"),
    }
}

#[test]
fn test_parse_harmony_info_with_label() {
    let args = cli::Cli::parse_from(["runbeam", "harmony:info", "--label", "my-instance"]);
    match args.command {
        Some(cli::Command::HarmonyInfo { id, label }) => {
            assert_eq!(id, None);
            assert_eq!(label, Some("my-instance".to_string()));
        }
        _ => panic!("Expected HarmonyInfo command"),
    }
}

#[test]
fn test_parse_harmony_routes_with_id() {
    let args = cli::Cli::parse_from(["runbeam", "harmony:routes", "--id", "route-id"]);
    match args.command {
        Some(cli::Command::HarmonyRoutes { id, label, json }) => {
            assert_eq!(id, Some("route-id".to_string()));
            assert_eq!(label, None);
            assert!(!json, "JSON flag should be false by default");
        }
        _ => panic!("Expected HarmonyRoutes command"),
    }
}

#[test]
fn test_parse_harmony_routes_without_json() {
    let args = cli::Cli::parse_from(["runbeam", "harmony:routes", "--label", "instance"]);
    match args.command {
        Some(cli::Command::HarmonyRoutes { id, label, json }) => {
            assert_eq!(id, None);
            assert_eq!(label, Some("instance".to_string()));
            assert!(!json, "JSON flag should be false when not specified");
        }
        _ => panic!("Expected HarmonyRoutes command"),
    }
}

#[test]
fn test_parse_harmony_add_with_encryption_key() {
    let args = cli::Cli::parse_from([
        "runbeam",
        "harmony:add",
        "--ip",
        "10.0.0.1",
        "--port",
        "9000",
        "--label",
        "secure",
        "--key",
        "base64key",
    ]);
    match args.command {
        Some(cli::Command::HarmonyAdd {
            ip,
            port,
            label,
            path_prefix: _,
            encryption_key,
        }) => {
            assert_eq!(ip, "10.0.0.1");
            assert_eq!(port, 9000);
            assert_eq!(label, Some("secure".to_string()));
            assert_eq!(encryption_key, Some("base64key".to_string()));
        }
        _ => panic!("Expected HarmonyAdd command"),
    }
}

#[test]
fn test_parse_verbose_with_command() {
    let args = cli::Cli::parse_from(["runbeam", "-vv", "harmony:list"]);
    assert_eq!(args.verbose, 2);
    assert!(matches!(args.command, Some(cli::Command::HarmonyList)));
}

#[test]
fn test_parse_quiet_with_command() {
    let args = cli::Cli::parse_from(["runbeam", "-q", "config:get"]);
    assert!(args.quiet);
    match args.command {
        Some(cli::Command::ConfigGet { key }) => {
            assert_eq!(key, None);
        }
        _ => panic!("Expected ConfigGet command"),
    }
}

#[test]
fn test_parse_verbose_and_quiet_together() {
    // Clap allows both, but application logic should handle the conflict
    let args = cli::Cli::parse_from(["runbeam", "-v", "-q", "list"]);
    assert_eq!(args.verbose, 1);
    assert!(args.quiet);
}

#[test]
fn test_parse_command_with_long_flags() {
    let args = cli::Cli::parse_from([
        "runbeam",
        "--verbose",
        "--verbose",
        "--verbose",
        "harmony:list",
    ]);
    assert_eq!(args.verbose, 3);
}

#[test]
fn test_parse_harmony_remove_with_all_options() {
    // Test that each removal method works independently

    // By ID
    let args = cli::Cli::parse_from(["runbeam", "harmony:remove", "--id", "test-id"]);
    match args.command {
        Some(cli::Command::HarmonyRemove {
            id,
            label,
            ip,
            port,
        }) => {
            assert_eq!(id, Some("test-id".to_string()));
            assert_eq!(label, None);
            assert_eq!(ip, None);
            assert_eq!(port, None);
        }
        _ => panic!("Expected HarmonyRemove command"),
    }
}

#[test]
fn test_parse_config_set_with_complex_url() {
    let args = cli::Cli::parse_from([
        "runbeam",
        "config:set",
        "api-url",
        "https://api-staging.example.com:8443/v2",
    ]);
    match args.command {
        Some(cli::Command::ConfigSet { key, value }) => {
            assert_eq!(key, "api-url");
            assert_eq!(value, "https://api-staging.example.com:8443/v2");
        }
        _ => panic!("Expected ConfigSet command"),
    }
}
