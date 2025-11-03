# Test Suite Documentation

This directory contains the test suite for runbeam-cli. Tests are organized by functionality and use shared utilities from the `common` module.

## Test Organization

### Unit Tests
Located in `src/` with `#[cfg(test)]` modules:
- `src/storage_tests.rs` - Storage module unit tests (serialization, ID generation)

### Integration Tests
Located in `tests/`:
- `cli_parsing_test.rs` - CLI argument parsing tests (19 tests)
- `integration_test.rs` - End-to-end CLI execution tests (13 tests)
- `routes_table_test.rs` - JSON structure validation tests (1 test)
- `common/mod.rs` - Shared test utilities and helpers

## Running Tests

```bash
# Run all tests
cargo test

# Run specific test file
cargo test --test cli_parsing_test

# Run tests matching a pattern
cargo test storage

# Run tests with output
cargo test -- --nocapture

# Run tests serially (for tests that modify global state)
cargo test -- --test-threads=1
```

## Test Utilities (common module)

The `tests/common/mod.rs` module provides shared utilities to reduce test code duplication.

### TestEnv - Isolated Test Environment

Creates an isolated filesystem and environment for tests:

```rust
use common::TestEnv;

#[test]
fn test_something() {
    let env = TestEnv::new();
    
    // Use isolated data directory
    let path = env.data_file("harmony.json");
    
    // Write JSON data
    env.write_json_file("config.json", &serde_json::json!({"api_url": "http://test"}));
    
    // Read JSON data
    let data = env.read_json_file("config.json");
    
    // Environment is automatically cleaned up when `env` is dropped
}
```

**Features:**
- Temporary directory with automatic cleanup
- Isolated `~/.runbeam` equivalent
- Environment variable management with automatic restoration
- JSON file helpers

### Mock Data Generators

Create realistic test data:

```rust
use common::{create_mock_harmony_instance, create_mock_config, create_mock_auth};

let instance = create_mock_harmony_instance("abc123", "127.0.0.1", 8081, "test", "admin");
let config = create_mock_config(Some("https://api.example.com"));
let auth = create_mock_auth("test-token", Some(1234567890));
```

### Mock API Responses

Generate valid API response structures:

```rust
use common::{create_mock_routes_response, create_mock_pipelines_response, create_mock_info_response};

let routes = create_mock_routes_response();
let pipelines = create_mock_pipelines_response();
let info = create_mock_info_response();
```

### Assertion Helpers

Validate output formats:

```rust
use common::{assert_table_output, assert_json_structure, assert_json_array_structure};

// Verify table formatting
assert_table_output(&output, &["ID", "LABEL", "IP"]);

// Verify JSON structure
let json = serde_json::json!({"id": "123", "name": "test"});
assert_json_structure(&json, &["id", "name"]);

// Verify array of objects
let array = serde_json::json!([{"id": "1"}, {"id": "2"}]);
assert_json_array_structure(&array, &["id"]);
```

## Test Naming Conventions

Follow this pattern for test names:

```
test_<function>_<scenario>_<expected_result>
```

Examples:
- `test_load_harmony_instances_missing_file_returns_empty()`
- `test_add_harmony_instance_duplicate_label_updates_existing()`
- `test_config_set_invalid_url_returns_error()`

## Test Categories

### CLI Parsing Tests
Test that command-line arguments are parsed correctly by clap. These tests use `clap::Parser::parse_from()` to verify argument handling without executing commands.

### Integration Tests
Test the CLI binary end-to-end using `assert_cmd`. These tests invoke the actual binary and verify stdout, stderr, and exit codes.

### Unit Tests
Test individual functions in isolation. Use `TestEnv` for tests that need filesystem operations.

## Writing New Tests

### 1. Import Common Utilities

```rust
mod common;

use common::TestEnv;
```

### 2. Use TestEnv for Isolation

```rust
#[test]
fn test_my_feature() {
    let env = TestEnv::new();
    // Test code using env
}
```

### 3. Test Both Success and Error Paths

```rust
#[test]
fn test_feature_success() {
    // Happy path
}

#[test]
fn test_feature_invalid_input() {
    // Error handling
}
```

### 4. Use Descriptive Assertions

```rust
assert_eq!(
    result, expected,
    "Config should load default when file is missing"
);
```

## Test Dependencies

The following crates are available in dev-dependencies:

- **tempfile** - Temporary directories for isolated tests
- **assert_cmd** - CLI testing utilities
- **predicates** - Assertion predicates for assert_cmd
- **mockito** - HTTP mocking for API tests
- **serial_test** - Force tests to run serially when needed

## Adding New Test Dependencies

If you need additional testing utilities, add them to `Cargo.toml`:

```toml
[dev-dependencies]
your-test-crate = "x.y"
```

## Troubleshooting

### Tests Fail Due to Shared State

Some tests may fail when run in parallel if they modify shared global state (like environment variables). Use `serial_test`:

```rust
use serial_test::serial;

#[test]
#[serial]
fn test_modifies_global_state() {
    // Test code
}
```

### Tests Fail Due to Previous Test Data

Tests should use `TestEnv` to ensure isolation. If tests are failing due to leftover data, verify:
1. Tests use `TestEnv::new()` for isolation
2. Tests don't write to real `~/.runbeam` directory
3. Environment variables are properly restored

### Cannot Find Test Module

Make sure to add the module declaration:

```rust
mod common;  // At the top of your test file
```

## Coverage Goals

- **Unit tests**: Test all pure functions and business logic
- **Integration tests**: Test CLI commands end-to-end
- **Error handling**: Test failure scenarios and error messages
- **Edge cases**: Test boundary conditions and unusual inputs

Target: 80%+ code coverage across all modules.
