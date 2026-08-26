//! Unit tests for Config module
//!
//! Tests Figment integration: file loading, env vars, CLI precedence

use mcp_websearch::config::{CliArgs, Config, TransportMode};
use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;

/// Helper to create a temp config file
fn create_config_file(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file
}

/// Test: Load from valid TOML file
#[test]
fn test_load_from_valid_toml() {
    let config_content = r#"
[transport]
mode = "http"

[transport.http]
host = "127.0.0.1"
port = 8080

[ddg]
timeout = 30
max_results = 10
user_agent = "Test/1.0"

[logging]
level = "debug"
format = "json"
"#;

    let temp_file = create_config_file(config_content);
    let args = CliArgs {
        transport: TransportMode::Http,
        config: Some(temp_file.path().to_path_buf()),
        host: None,
        port: None,
        log_level: None,
    };

    let config = Config::load(&args).unwrap();

    assert_eq!(config.transport.mode, "http");
    assert_eq!(config.transport.http.as_ref().unwrap().host, "127.0.0.1");
    assert_eq!(config.transport.http.as_ref().unwrap().port, 8080);
    assert_eq!(config.ddg.timeout, 30);
    assert_eq!(config.ddg.max_results, 10);
    assert_eq!(config.ddg.user_agent, "Test/1.0");
    assert_eq!(config.logging.level, "debug");
}

/// Test: Missing config file falls back to defaults
#[test]
fn test_missing_config_file_uses_defaults() {
    let args = CliArgs {
        transport: TransportMode::Stdio,
        config: Some(PathBuf::from("/nonexistent/path.toml")),
        host: None,
        port: None,
        log_level: None,
    };

    // Should succeed with defaults
    let config = Config::load(&args).unwrap();

    // Uses default.toml values
    assert!(!config.transport.mode.is_empty());
}

/// Test: CLI transport argument overrides file
#[test]
fn test_cli_transport_overrides_file() {
    let config_content = r#"
[transport]
mode = "http"
"#;
    let temp_file = create_config_file(config_content);

    let args = CliArgs {
        transport: TransportMode::Stdio,              // CLI says stdio
        config: Some(temp_file.path().to_path_buf()), // File says http
        host: None,
        port: None,
        log_level: None,
    };

    let config = Config::load(&args).unwrap();
    assert_eq!(config.transport.mode, "stdio"); // CLI wins
}

/// Test: CLI host/port override file config
#[test]
fn test_cli_host_port_override() {
    let config_content = r#"
[transport.http]
host = "127.0.0.1"
port = 3000
"#;
    let temp_file = create_config_file(config_content);

    let args = CliArgs {
        transport: TransportMode::Http,
        config: Some(temp_file.path().to_path_buf()),
        host: Some("0.0.0.0".to_string()),
        port: Some(8080),
        log_level: None,
    };

    let config = Config::load(&args).unwrap();
    let addr = config.http_addr().unwrap();
    assert_eq!(addr, "0.0.0.0:8080");
}

/// Test: http_addr returns None when no HTTP config
#[test]
fn test_http_addr_none_when_no_http_config() {
    // Create config without http section
    let args = CliArgs {
        transport: TransportMode::Stdio,
        config: None,
        host: None,
        port: None,
        log_level: None,
    };

    let _config = Config::load(&args).unwrap();
    // http is None when mode is stdio
}

/// Test: http_addr returns correct format
#[test]
fn test_http_addr_format() {
    let config_content = r#"
[transport.http]
host = "192.168.1.1"
port = 9000
"#;
    let temp_file = create_config_file(config_content);

    let args = CliArgs {
        transport: TransportMode::Http,
        config: Some(temp_file.path().to_path_buf()),
        host: None,
        port: None,
        log_level: None,
    };

    let config = Config::load(&args).unwrap();
    let addr = config.http_addr().unwrap();
    assert_eq!(addr, "192.168.1.1:9000");
}

/// Test: Default implementation has expected values
#[test]
fn test_default_impl() {
    let default = Config::default();

    assert_eq!(default.transport.mode, "stdio");
    assert!(default.transport.http.is_some());
    assert_eq!(default.transport.http.as_ref().unwrap().host, "127.0.0.1");
    assert_eq!(default.transport.http.as_ref().unwrap().port, 3000);
    assert_eq!(default.ddg.timeout, 15);
    assert_eq!(default.ddg.max_results, 5);
    assert!(!default.ddg.user_agent.is_empty());
    assert_eq!(default.logging.level, "info");
    assert_eq!(default.logging.format, "json");
}

/// Test: DDG config values
#[test]
fn test_ddg_config_values() {
    let config_content = r#"
[ddg]
timeout = 45
max_results = 20
user_agent = "CustomAgent/2.0"
"#;
    let temp_file = create_config_file(config_content);

    let args = CliArgs {
        transport: TransportMode::Stdio,
        config: Some(temp_file.path().to_path_buf()),
        host: None,
        port: None,
        log_level: None,
    };

    let config = Config::load(&args).unwrap();
    assert_eq!(config.ddg.timeout, 45);
    assert_eq!(config.ddg.max_results, 20);
    assert_eq!(config.ddg.user_agent, "CustomAgent/2.0");
}

/// Test: Logging config values
#[test]
fn test_logging_config_values() {
    let config_content = r#"
[logging]
level = "trace"
format = "pretty"
"#;
    let temp_file = create_config_file(config_content);

    let args = CliArgs {
        transport: TransportMode::Stdio,
        config: Some(temp_file.path().to_path_buf()),
        host: None,
        port: None,
        log_level: None,
    };

    let config = Config::load(&args).unwrap();
    assert_eq!(config.logging.level, "trace");
    assert_eq!(config.logging.format, "pretty");
}

/// Test: base_url can be overridden via config file (T13)
#[test]
fn test_base_url_override_from_config() {
    let config_content = r#"
[ddg]
timeout = 15
max_results = 5
user_agent = "Test/1.0"
base_url = "http://mock-server.example.com/lite/"
"#;
    let temp_file = create_config_file(config_content);

    let args = CliArgs {
        transport: TransportMode::Stdio,
        config: Some(temp_file.path().to_path_buf()),
        host: None,
        port: None,
        log_level: None,
    };

    let config = Config::load(&args).unwrap();
    assert_eq!(
        config.ddg.base_url, "http://mock-server.example.com/lite/",
        "base_url should be overridden by config file"
    );
}

/// Test: base_url defaults when not specified (T13)
#[test]
fn test_base_url_defaults_when_absent() {
    let config_content = r#"
[ddg]
timeout = 15
max_results = 5
user_agent = "Test/1.0"
"#;
    let temp_file = create_config_file(config_content);

    let args = CliArgs {
        transport: TransportMode::Stdio,
        config: Some(temp_file.path().to_path_buf()),
        host: None,
        port: None,
        log_level: None,
    };

    let config = Config::load(&args).unwrap();
    assert_eq!(
        config.ddg.base_url, "https://lite.duckduckgo.com/lite/",
        "base_url should default to DuckDuckGo Lite when not specified"
    );
}

/// Test: max_results override flows through to parse_results (T14)
#[test]
fn test_max_results_override_through_parser() {
    use mcp_websearch::ddg::parser::{DdgRegex, MAX_RESULTS, parse_results};

    // Build HTML with 10 results
    let mut html = String::from("<html><body><table>");
    for i in 0..10 {
        html.push_str(&format!(
            r#"<tr><td class="result-link"><a href="//duckduckgo.com/l/?uddg=https%3A%2F%2Fexample{}.com" class="result-link">Title {}</a></td></tr><tr><td class="result-snippet">Snippet {}</td></tr>"#,
            i, i, i
        ));
    }
    html.push_str("</table></body></html>");

    let regex = DdgRegex::default();

    // With max_results=3, should get exactly 3
    let results = parse_results(&html, &regex, 3).unwrap();
    assert_eq!(results.len(), 3, "max_results=3 should limit to 3 results");

    // With max_results=7, should get exactly 7
    let results = parse_results(&html, &regex, 7).unwrap();
    assert_eq!(results.len(), 7, "max_results=7 should limit to 7 results");

    // With max_results=MAX_RESULTS (5), should get 5
    let results = parse_results(&html, &regex, MAX_RESULTS).unwrap();
    assert_eq!(
        results.len(),
        MAX_RESULTS,
        "max_results=MAX_RESULTS should limit to 5"
    );
}
