//! Stdio transport for MCP WebSearch server
//! 
//! Uses stdin/stdout for MCP protocol messages
//! Logs go to stderr

use crate::config::Config;
use rmcp::{
    ServiceExt,
    transport::stdio::stdio,
};
use tokio::io::{stdin, stdout};
use tracing::info;

/// Run the MCP server with stdio transport
pub async fn run_stdio_server<H>(handler: H, config: &Config) -> anyhow::Result<()>
where
    H: rmcp::ServerHandler + Send + Sync + 'static,
{
    info!("Starting MCP WebSearch server on stdio transport");
    info!("DDG timeout: {}s, max_results: {}", 
        config.ddg.timeout, config.ddg.max_results);

    // Create stdio transport (uses stdin/stdout)
    let transport = (stdin(), stdout());
    
    // Serve the handler
    let server = handler.serve(transport).await?;
    
    info!("MCP server initialized, waiting for requests...");
    
    // Wait for server to complete
    let _quit_reason = server.waiting().await?;
    
    info!("MCP server shutting down");
    Ok(())
}
