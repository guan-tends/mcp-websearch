# Testing Guide for mcp-websearch

This document describes the comprehensive test suite for mcp-websearch.

## Test Suite Overview

- **Total Tests**: 111+
- **Coverage**: Unit, Integration, E2E
- **Framework**: Built-in Rust testing + tokio-test + wiremock

## Test Structure

```
tests/
├── support/           # Shared test utilities
│   ├── mod.rs        # Fixture loading helpers
│   ├── mock_ddg.rs   # Wiremock DDG server
│   └── mcp_client.rs # MCP protocol test client
├── fixtures/         # Test data
│   ├── ddg_response.html
│   ├── mcp_init_request.json
│   ├── mcp_init_response.json
│   ├── mcp_tools_list_response.json
│   └── invalid_config.toml
├── unit/            # Unit tests
│   ├── config.rs    # 10 tests
│   ├── ddg_parser.rs # 28 tests
│   ├── ddg_client.rs # 15 tests
│   ├── error.rs     # 12 tests
│   └── web_search.rs # 12 tests
├── integration/     # Integration tests
│   ├── mcp_protocol.rs # 13 tests
│   └── stdio_transport.rs # 7 tests
└── e2e/
    └── search_workflow.rs # 14 tests
```

## Running Tests

### Run All Tests
```bash
cargo test
```

### Run Unit Tests Only
```bash
cargo test --lib
cargo test --test unit
```

### Run Integration Tests
```bash
cargo test --test integration
```

### Run E2E Tests
```bash
cargo test --test e2e
```

### Run Specific Test
```bash
cargo test test_parse_results_from_fixture
```

### Run with Output
```bash
cargo test -- --nocapture
```

### Run with Backtrace
```bash
RUST_BACKTRACE=1 cargo test
```

## Test Categories

### Unit Tests

**Config Module** (`tests/unit/config.rs`)
- File loading from TOML
- Environment variable integration
- CLI argument precedence
- Default values
- Error handling

**DDG Parser** (`tests/unit/ddg_parser.rs`)
- URL encoding/decoding (ASCII, unicode, emoji)
- HTML tag stripping
- Result parsing from DDG HTML
- Regex compilation
- UTF-8 multi-byte handling

**DDG Client** (`tests/unit/ddg_client.rs`)
- Client creation
- HTTP request construction
- Search success paths
- Error handling (404, 500, timeout)
- Wiremock integration

**Error Types** (`tests/unit/error.rs`)
- Error display formatting
- From conversions
- ? operator propagation
- Error chaining

**WebSearch Tool** (`tests/unit/web_search.rs`)
- Tool creation
- Response serialization
- CallToolResult construction
- Schema validation

### Integration Tests

**MCP Protocol** (`tests/integration/mcp_protocol.rs`)
- Initialize handshake
- Server info/capabilities
- tools/list endpoint
- tools/call endpoint
- Error responses
- Multiple sequential requests

**Stdio Transport** (`tests/integration/stdio_transport.rs`)
- Server startup
- JSON-RPC compliance
- No logging to stdout
- EOF handling
- Multiple requests

### E2E Tests

**Search Workflow** (`tests/e2e/search_workflow.rs`)
- Complete user journeys
- Query variety (simple, complex, unicode)
- Special characters (C++, C#)
- Result limits
- Error scenarios

## Test Fixtures

### DDG HTML (`tests/fixtures/ddg_response.html`)
Sample DDG Lite search result HTML for parser testing.

### MCP Protocol JSON
- `mcp_init_request.json` - Initialize request
- `mcp_init_response.json` - Initialize response
- `mcp_tools_list_response.json` - Tools list response

### Config Samples
- `invalid_config.toml` - Malformed config for error testing

## Mock Infrastructure

### MockDDGServer
```rust
use crate::support::mock_ddg::MockDDGServer;

let server = MockDDGServer::new().await;
server.mock_search_success(html_response).await;
server.mock_not_found().await;
server.mock_server_error().await;
```

### McpTestClient
```rust
use crate::support::mcp_client::McpTestClient;

let mut client = McpTestClient::new().await?;
client.initialize().await?;
let response = client.request("tools/list", json!({})).await?;
```

## Coverage

### Generate Coverage Report
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --all-features --out Html
```

### View Coverage Report
Open `tarpaulin-report.html` in browser.

**Current Coverage Target**: 80%+

## CI/CD

Tests run automatically on:
- Push to `main` or `develop`
- Pull requests to `main` or `develop`

### CI Jobs
1. **Format** - `cargo fmt --check`
2. **Lint** - `cargo clippy -- -D warnings`
3. **Test** - Ubuntu + macOS matrix
4. **Build** - Release builds
5. **Coverage** - Tarpaulin + Codecov

## Writing New Tests

### Unit Test Template
```rust
#[test]
fn test_description() {
    // Arrange
    let input = ...;
    
    // Act
    let result = function(input);
    
    // Assert
    assert_eq!(result, expected);
}
```

### Async Test Template
```rust
#[tokio::test]
async fn test_async_description() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

### Integration Test Template
```rust
#[tokio::test]
async fn test_mcp_flow() {
    let mut client = McpTestClient::new().await.unwrap();
    client.initialize().await.unwrap();
    
    let response = client.request("method", params).await.unwrap();
    assert!(response.result.is_some());
}
```

## Debugging Failed Tests

### Get More Details
```bash
cargo test -- --nocapture
```

### Run Single Test with Backtrace
```bash
RUST_BACKTRACE=full cargo test test_name
```

### Check Fixture Files
```bash
ls -la tests/fixtures/
cat tests/fixtures/ddg_response.html
```

## Test Dependencies

From `Cargo.toml`:
```toml
[dev-dependencies]
tokio-test = "0.4"
wiremock = "0.6"
pretty_assertions = "1"
```

## Best Practices

1. **Isolated Tests** - Each test should be independent
2. **Descriptive Names** - `test_what_scenario`
3. **Arrange-Act-Assert** - Clear structure
4. **Use Fixtures** - Real data in files
5. **Mock External** - Wiremock for HTTP
6. **Test Errors** - Don't just test happy path
7. **Documentation** - Comment complex scenarios

## Contributing Tests

When adding features:
1. Add unit tests for new functions
2. Add integration tests for API changes
3. Add E2E tests for user-facing changes
4. Update this document
5. Ensure CI passes

---

For questions or issues, see the main repository or open an issue.
