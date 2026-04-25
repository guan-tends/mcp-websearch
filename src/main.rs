//! MCP WebSearch Server
//! 
//! A Rust MCP server for web search using DuckDuckGo

use clap::Parser;
use tracing::info;

mod config;
mod ddg;
mod error;
mod tools;
mod transport;

use config::{Config, CliArgs, TransportMode};
use tools::WebSearchTool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse arguments first (needed to check transport mode)
    let args = CliArgs::parse();
    
    // Only initialize logging for non-stdio transports
    // Stdio transport needs clean stdout for MCP JSON-RPC
    if args.transport != TransportMode::Stdio {
        tracing_subscriber::fmt::init();
    }
    
    info!("Starting MCP WebSearch server...");
    
    // Load configuration
    let config = Config::load(&args)?;
    
    // Create web search tool handler
    let handler = WebSearchTool::new(config.ddg.clone())?;
    
    // Start server based on transport
    match config.transport.mode.as_str() {
        "stdio" => {
            // No logging in stdio mode - stdout is for MCP protocol only
            transport::stdio::serve(handler, config).await?;
        }
        "http" => {
            info!("Using HTTP transport");
            transport::http::serve(handler, config).await?;
        }
        _ => {
            anyhow::bail!("Unknown transport mode: {}", config.transport.mode);
        }
    }
    
    Ok(())
}
