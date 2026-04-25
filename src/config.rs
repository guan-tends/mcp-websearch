//! Configuration management using figment

use clap::Parser;
use figment::{
    Figment,
    providers::{Env, Format, Toml},
};
use serde::Deserialize;
use std::path::PathBuf;

/// CLI arguments
#[derive(Parser, Debug, Clone)]
#[command(name = "mcp-websearch")]
#[command(about = "MCP WebSearch server - DuckDuckGo Lite search tool")]
#[command(version)]
pub struct CliArgs {
    /// Transport mode
    #[arg(short, long, value_enum, default_value = "stdio")]
    pub transport: TransportMode,

    /// Config file path
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// HTTP host (for HTTP transport)
    #[arg(long, env = "MCP_HOST")]
    pub host: Option<String>,

    /// HTTP port (for HTTP transport)
    #[arg(long, env = "MCP_PORT")]
    pub port: Option<u16>,

    /// Log level
    #[arg(short, long, env = "RUST_LOG")]
    pub log_level: Option<String>,
}

#[derive(Debug, Clone, Copy, Default, clap::ValueEnum, PartialEq)]
pub enum TransportMode {
    #[default]
    Stdio,
    Http,
}

/// Main configuration structure
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub transport: TransportConfig,
    pub ddg: DdgConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TransportConfig {
    pub mode: String,
    pub http: Option<HttpConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HttpConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DdgConfig {
    pub timeout: u64,
    pub max_results: usize,
    pub user_agent: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

impl Config {
    /// Load configuration from hierarchy:
    /// 1. CLI args (highest priority)
    /// 2. Environment variables (prefixed with MCP_)
    /// 3. Config file (if specified)
    /// 4. defaults.toml (lowest priority)
    pub fn load(args: &CliArgs) -> anyhow::Result<Self> {
        let mut figment = Figment::new()
            .merge(Toml::file(
                "/home/guan/src/mcp-websearch/config/default.toml",
            ))
            .merge(Env::prefixed("MCP_"));

        // Merge user-specified config file if provided
        if let Some(config_path) = &args.config {
            figment = figment.merge(Toml::file(config_path));
        }

        // Apply CLI overrides
        if let Some(host) = &args.host {
            figment = figment.merge(("transport.http.host", host.clone()));
        }
        if let Some(port) = args.port {
            figment = figment.merge(("transport.http.port", port));
        }

        // Set transport mode from CLI
        let mode = match args.transport {
            TransportMode::Stdio => "stdio",
            TransportMode::Http => "http",
        };
        figment = figment.merge(("transport.mode", mode));

        Ok(figment.extract()?)
    }

    /// Get HTTP address as a string
    /// Get HTTP address as a string
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
                user_agent: "Mozilla/5.0 (compatible; Kai-MCP/1.0)".to_string(),
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "json".to_string(),
            },
        }
    }
}
