# Testing

## Quick Start

```sh
cargo test
```

All non-network tests run by default. 106 tests pass with 0 failures.

## Test Structure

```
tests/
├── unit/           # Unit tests (81 tests)
│   ├── config.rs       # Config loading, CLI overrides, env vars
│   ├── ddg_client.rs   # DDG client with wiremock mocks (6) + network tests (4, ignored)
│   ├── ddg_parser.rs    # URL encoding/decoding, HTML parsing, result extraction
│   ├── error.rs         # Error types, Display impls, From conversions
│   └── web_search.rs    # Tool result construction, serialization
├── integration/    # Integration tests (15 tests, 3 ignored)
│   ├── mcp_protocol.rs   # MCP JSON-RPC protocol compliance
│   └── stdio_transport.rs # Stdio transport lifecycle
├── e2e/            # End-to-end tests (11 tests, all ignored)
│   └── search_workflow.rs # Full search workflows via subprocess
├── support/        # Shared test utilities
│   ├── mock_ddg.rs       # Wiremock-based mock DDG server
│   └── mcp_client.rs     # MCP test client (spawns binary)
└── fixtures/       # Test fixtures
    └── ddg_response.html # Sample DDG Lite HTML response
```

## Network Tests

Tests that hit the real DuckDuckGo API are marked `#[ignore]` and excluded from CI:

```sh
# Run only network tests
cargo test -- --ignored

# Run all tests (including network)
cargo test -- --include-ignored
```

## Mock Tests

Mock tests use [wiremock](https://crates.io/crates/wiremock) with a configurable `base_url`:

- `test_mocked_search_returns_results` — parses DDG HTML from mock
- `test_mocked_search_respects_max_results` — verifies result limiting
- `test_mocked_404_returns_error` — HTTP error handling
- `test_mocked_500_returns_error` — server error handling
- `test_mocked_empty_html_returns_no_results` — empty response handling
- `test_mocked_search_encodes_query_in_url` — URL encoding verification

## Coverage

```sh
cargo install cargo-tarpaulin
cargo tarpaulin --all-features --out Html
```

## CI

GitHub Actions runs on every push/PR:
- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test` (non-network tests only)
- Matrix: Ubuntu + macOS, stable + nightly Rust
