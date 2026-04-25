//! StdIO Transport
//!
//! Provides stdio-based MCP transport for local tool integration

use crate::config::Config;
use crate::tools::WebSearchTool;
use rmcp::{serve_server, transport::io::stdio};

/// Serve the web search tool over stdio
pub async fn serve(handler: WebSearchTool, _config: Config) -> anyhow::Result<()> {
    let (stdin, stdout) = stdio();
    let running = serve_server(handler, (stdin, stdout)).await?;

    // Wait for the service to complete (blocks until EOF or cancellation)
    running.waiting().await?;

    Ok(())
}
