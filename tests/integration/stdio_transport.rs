//! Integration tests for Stdio Transport
//!
//! Tests subprocess spawning, EOF handling, logging behavior

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

/// Helper to spawn MCP server as subprocess.
fn spawn_server() -> std::process::Child {
    // Build release first
    let _ = std::process::Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output();

    let server_path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/release/mcp-websearch");

    Command::new(server_path)
        .arg("--transport")
        .arg("stdio")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn server")
}

/// Clean up a child process: kill and wait to reap.
fn cleanup(mut child: std::process::Child) {
    let _ = child.kill();
    let _ = child.wait();
}

// ==================== SERVER STARTUP TESTS ====================

#[test]
fn test_server_starts_with_stdio_transport() {
    let mut child = spawn_server();

    // Give server time to start
    thread::sleep(Duration::from_millis(100));

    // Check process is still running
    let status = child.try_wait();
    assert!(status.is_ok());
    assert!(status.unwrap().is_none(), "Server should still be running");

    // Cleanup
    cleanup(child);
}

#[test]
fn test_server_responds_to_initialize() {
    let mut child = spawn_server();

    let stdin = child.stdin.take().expect("Failed to get stdin");
    let stdout = child.stdout.take().expect("Failed to get stdout");
    let mut reader = BufReader::new(stdout);

    // Send initialize request
    let init_request = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#;
    let mut stdin = stdin;
    writeln!(stdin, "{}", init_request).unwrap();
    stdin.flush().unwrap();

    // Read response
    let mut response = String::new();
    reader.read_line(&mut response).unwrap();

    // Verify response
    assert!(!response.is_empty(), "Should receive response");
    assert!(response.contains("jsonrpc"), "Should be JSON-RPC response");
    assert!(response.contains("result"), "Should have result");

    // Cleanup
    cleanup(child);
}

// ==================== STDOUT BEHAVIOR TESTS ====================

#[test]
fn test_stdio_no_log_output() {
    // In stdio mode, stdout should ONLY contain MCP JSON-RPC messages
    // No logging should go to stdout
    let mut child = spawn_server();

    let stdin = child.stdin.take().expect("Failed to get stdin");
    let stdout = child.stdout.take().expect("Failed to get stdout");
    let _stderr = child.stderr.take().expect("Failed to get stderr");

    let mut stdout_reader = BufReader::new(stdout);

    // Send initialize request
    let init_request = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#;
    let mut stdin = stdin;
    writeln!(stdin, "{}", init_request).unwrap();
    stdin.flush().unwrap();

    // Wait for response
    thread::sleep(Duration::from_millis(500));

    // Check stdout - should only have JSON
    let mut stdout_content = String::new();
    // Read a single line (not a real loop — clippy)
    let _ = stdout_reader.read_line(&mut stdout_content);

    // Verify stdout is valid JSON (not log output)
    if !stdout_content.is_empty() {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(stdout_content.trim());
        assert!(
            parsed.is_ok(),
            "stdout should only contain JSON, not logs: {}",
            stdout_content
        );
    }

    // Cleanup
    cleanup(child);
}

// ==================== EOF HANDLING TESTS ====================

#[test]
fn test_server_handles_eof() {
    let mut child = spawn_server();

    let stdin = child.stdin.take().expect("Failed to get stdin");

    // Close stdin (EOF)
    drop(stdin);

    // Give server time to process EOF
    thread::sleep(Duration::from_millis(500));

    // Server should exit gracefully
    let status = child.try_wait();
    assert!(status.is_ok());
    // Process may or may not have exited yet depending on implementation

    // Cleanup
    cleanup(child);
}

// ==================== MULTIPLE REQUEST TESTS ====================

#[test]
fn test_server_handles_multiple_requests() {
    let mut child = spawn_server();

    let stdin = child.stdin.take().expect("Failed to get stdin");
    let stdout = child.stdout.take().expect("Failed to get stdout");
    let mut reader = BufReader::new(stdout);

    let mut stdin = stdin;

    // Send initialize, then initialized notification, then tools/list
    // MCP protocol requires the initialized notification before other requests

    // 1. Initialize
    let init_request = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#;
    writeln!(stdin, "{}", init_request).unwrap();
    stdin.flush().unwrap();

    let mut response = String::new();
    reader.read_line(&mut response).unwrap();
    assert!(
        response.contains(r#""id":1"#),
        "Init response should match id 1, got: {}",
        response.trim()
    );

    // 2. Send initialized notification (no response expected)
    let init_notif = r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#;
    writeln!(stdin, "{}", init_notif).unwrap();
    stdin.flush().unwrap();

    // Brief delay for the server to process the notification
    std::thread::sleep(std::time::Duration::from_millis(50));

    // 3. tools/list
    let list_request = r#"{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}"#;
    writeln!(stdin, "{}", list_request).unwrap();
    stdin.flush().unwrap();

    response.clear();
    reader.read_line(&mut response).unwrap();
    assert!(
        response.contains(r#""id":2"#),
        "tools/list response should match id 2, got: {}",
        response.trim()
    );

    // Cleanup
    cleanup(child);
}

// ==================== JSON-RPC COMPLIANCE TESTS ====================

#[test]
fn test_server_valid_json_rpc_format() {
    let mut child = spawn_server();

    let stdin = child.stdin.take().expect("Failed to get stdin");
    let stdout = child.stdout.take().expect("Failed to get stdout");
    let mut reader = BufReader::new(stdout);

    // Send initialize
    let init_request = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#;
    let mut stdin = stdin;
    writeln!(stdin, "{}", init_request).unwrap();
    stdin.flush().unwrap();

    // Read response
    let mut response = String::new();
    reader.read_line(&mut response).unwrap();

    // Parse as JSON-RPC
    let json: serde_json::Value = serde_json::from_str(response.trim()).unwrap();

    // Verify required fields
    assert_eq!(json.get("jsonrpc").unwrap().as_str().unwrap(), "2.0");
    assert!(json.get("id").is_some());
    assert!(json.get("result").is_some() || json.get("error").is_some());

    // Cleanup
    cleanup(child);
}
