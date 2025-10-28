use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_no_command_shows_hint() {
    let mut cmd = Command::cargo_bin("runbeam").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("use --help to see available commands"));
}

#[test]
fn test_list_command_succeeds() {
    let mut cmd = Command::cargo_bin("runbeam").unwrap();
    cmd.arg("list").assert().success();
}

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("runbeam").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Runbeam command-line interface"));
}

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("runbeam").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("runbeam"));
}

#[test]
fn test_harmony_list_succeeds() {
    let mut cmd = Command::cargo_bin("runbeam").unwrap();
    cmd.arg("harmony:list").assert().success();
}

#[test]
fn test_config_get_succeeds() {
    let mut cmd = Command::cargo_bin("runbeam").unwrap();
    cmd.arg("config:get").assert().success();
}

#[test]
fn test_config_get_specific_key() {
    let mut cmd = Command::cargo_bin("runbeam").unwrap();
    cmd.arg("config:get")
        .arg("api-url")
        .assert()
        .success();
}

#[test]
fn test_verbose_flag() {
    let mut cmd = Command::cargo_bin("runbeam").unwrap();
    cmd.arg("-v").arg("list").assert().success();
}

#[test]
fn test_quiet_flag() {
    let mut cmd = Command::cargo_bin("runbeam").unwrap();
    cmd.arg("-q").arg("list").assert().success();
}

#[test]
fn test_invalid_command() {
    let mut cmd = Command::cargo_bin("runbeam").unwrap();
    cmd.arg("invalid-command")
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:"));
}

#[test]
fn test_harmony_add_requires_valid_args() {
    let mut cmd = Command::cargo_bin("runbeam").unwrap();
    // harmony:add with defaults should succeed (no API call verification here)
    cmd.arg("harmony:add").assert().success();
}

#[test]
fn test_config_set_requires_both_args() {
    let mut cmd = Command::cargo_bin("runbeam").unwrap();
    // Missing value argument should fail
    cmd.arg("config:set")
        .arg("api-url")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_config_unset_requires_key() {
    let mut cmd = Command::cargo_bin("runbeam").unwrap();
    // Missing key argument should fail
    cmd.arg("config:unset")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

// Note: Login, logout, and verify commands require actual API interaction
// and are better suited for mock-based testing or manual testing
