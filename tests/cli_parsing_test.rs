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
    assert!(matches!(
        args.command,
        Some(cli::Command::HarmonyList)
    ));
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
