//! Test support utilities
//!
//! Shared helpers for testing mcp-websearch

pub mod mock_ddg;
pub mod mcp_client;

use std::path::PathBuf;

/// Get the path to a test fixture file
pub fn fixture_path(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/fixtures");
    path.push(name);
    path
}

/// Load a fixture file as a string
pub fn load_fixture(name: &str) -> String {
    std::fs::read_to_string(fixture_path(name))
        .unwrap_or_else(|e| panic!("Failed to load fixture {}: {}", name, e))
}
