//! MCP WebSearch Server
//! 
//! A port of the Kotlin WebSearchTool to Rust as an MCP server
//! Supports stdio and HTTP transports

mod config;
mod ddg;
mod error;
mod tools;
mod transport;

use crate::{
    config::{CliArgs, Config, TransportMode},
    tools::WebSearchTool,
};
use clap::Parser;
use std::io;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse CLI arguments
    let args = CliArgs::parse();
    
    // Load configuration (hierarchy: CLI > Env > File > Defaults)
    let config = Config::load(&args)?;
    
    // Initialize logging
    init_logging(&config.logging.level)?;
    
    info!("MCP WebSearch Server v{} starting", env!("CARGO_PKG_VERSION"));
    
    // Create the WebSearch tool
    let tool = WebSearchTool::new(config.ddg.clone())
        .map_err(|e| anyhow::anyhow!("Failed to create WebSearch tool: {}", e))?;
    
    // Dispatch to appropriate transport
    match args.transport {
        TransportMode::Stdio => {
            info!("Using stdio transport");
            transport::run_stdio_server(tool, &config).await?;
        }
        TransportMode::Http => {
            info!("Using HTTP transport");
            transport::run_http_server(tool, &config).await?;
        }
    }
    
    Ok(())
}

/// Initialize tracing with appropriate settings
/// 
/// For stdio mode: logs MUST go to stderr only (stdout reserved for MCP protocol)
fn init_logging(level: &str) -> anyhow::Result<()> {
    use tracing_subscriber::{
        fmt::format::FmtSpan,
        layer::SubscriberExt,
        util::SubscriberInitExt,
        EnvFilter,
    };
    
    // Create JSON formatter (structured logging)
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_writer(io::stderr)  // Critical: log to stderr
        .with_span_events(FmtSpan::CLOSE)
        .json()
        .flatten_event(true);
    
    // Set up filter from RUST_LOG or default
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(level));
    
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .init();
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse() {
        // Test that CLI parses with defaults
        let args = CliArgs::parse_from(&["mcp-websearch"]);
        assert!(matches!(args.transport, TransportMode::Stdio));
    }
}
