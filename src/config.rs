//! Configuration management using figment.
//!
//! Configuration is loaded from a hierarchy of sources, with later sources
//! overriding earlier ones:
//!
//! 1. CLI arguments (highest priority)
//! 2. Environment variables (prefixed with `MCP_`)
//! 3. User-specified config file (if `--config` is provided)
//! 4. Built-in defaults (embedded at compile time via `include_str!`)

use clap::Parser;
use figment::{
    Figment,
    providers::{Env, Format, Toml},
};
use serde::Deserialize;
use std::path::PathBuf;

/// CLI arguments.
#[derive(Parser, Debug, Clone)]
#[command(name = "mcp-websearch")]
#[command(about = "MCP WebSearch server — DuckDuckGo Lite search tool")]
#[command(version)]
pub struct CliArgs {
    /// Transport mode (stdio or http).
    #[arg(short, long, value_enum, default_value = "stdio")]
    pub transport: TransportMode,

    /// Path to a TOML config file. Overrides built-in defaults.
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// HTTP bind host (for HTTP transport).
    #[arg(long, env = "MCP_HOST")]
    pub host: Option<String>,

    /// HTTP bind port (for HTTP transport).
    #[arg(long, env = "MCP_PORT")]
    pub port: Option<u16>,

    /// Log level (trace, debug, info, warn, error).
    #[arg(short, long, env = "RUST_LOG")]
    pub log_level: Option<String>,
}

/// Transport mode selection.
#[derive(Debug, Clone, Copy, Default, clap::ValueEnum, PartialEq)]
pub enum TransportMode {
    #[default]
    Stdio,
    Http,
}

/// Root configuration structure.
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct Config {
    pub transport: TransportConfig,
    pub ddg: DdgConfig,
    pub logging: LoggingConfig,
}

/// Transport-layer configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct TransportConfig {
    pub mode: String,
    pub http: Option<HttpConfig>,
}

/// HTTP transport configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct HttpConfig {
    pub host: String,
    pub port: u16,
}

/// DuckDuckGo client configuration.
///
/// `base_url` allows pointing the client at a mock server for testing.
/// Defaults to `https://lite.duckduckgo.com/lite/`.
#[derive(Debug, Clone, Deserialize)]
pub struct DdgConfig {
    pub timeout: u64,
    pub max_results: usize,
    pub user_agent: String,
    #[serde(default = "default_base_url")]
    pub base_url: String,
}

fn default_base_url() -> String {
    "https://lite.duckduckgo.com/lite/".to_string()
}

/// Logging configuration.
///
/// Fields are deserialized from the config file and read at startup.
/// `dead_code` warnings are suppressed because the fields are accessed
/// through the figment deserialization pipeline, not direct code access.
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

impl Config {
    /// Load configuration from the source hierarchy.
    ///
    /// Order of precedence (later overrides earlier):
    /// 1. Built-in defaults (compiled into the binary)
    /// 2. User-specified config file (`--config`)
    /// 3. Environment variables (`MCP_` prefix)
    /// 4. CLI arguments (highest priority)
    pub fn load(args: &CliArgs) -> anyhow::Result<Self> {
        let mut figment = Figment::new()
            .merge(Toml::string(include_str!("../config/default.toml")))
            .merge(Env::prefixed("MCP_"));

        // Merge user-specified config file if provided.
        if let Some(config_path) = &args.config {
            figment = figment.merge(Toml::file(config_path));
        }

        // Apply CLI overrides.
        if let Some(host) = &args.host {
            figment = figment.merge(("transport.http.host", host.clone()));
        }
        if let Some(port) = args.port {
            figment = figment.merge(("transport.http.port", port));
        }

        // Set transport mode from CLI (always wins over file/env).
        let mode = match args.transport {
            TransportMode::Stdio => "stdio",
            TransportMode::Http => "http",
        };
        figment = figment.merge(("transport.mode", mode));

        Ok(figment.extract()?)
    }

    /// Get the HTTP bind address as `host:port`, if HTTP config is present.
    pub fn http_addr(&self) -> Option<String> {
        self.transport
            .http
            .as_ref()
            .map(|h| format!("{}:{}", h.host, h.port))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            transport: TransportConfig {
                mode: "stdio".to_string(),
                http: Some(HttpConfig {
                    host: "127.0.0.1".to_string(),
                    port: 3000,
                }),
            },
            ddg: DdgConfig {
                timeout: 15,
                max_results: 5,
                user_agent: "Mozilla/5.0 (compatible; mcp-websearch/1.0)".to_string(),
                base_url: default_base_url(),
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "json".to_string(),
            },
        }
    }
}
