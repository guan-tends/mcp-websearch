//! HTTP transport for MCP WebSearch server
//! 
//! Uses rmcp's streamable HTTP server in stateless JSON mode
//! No SSE - simple POST/JSON request-response

use crate::config::Config;
use rmcp::{
    ServiceExt,
    transport::streamable_http_server::{
        StreamableHttpServer, StreamableHttpServerConfig,
    },
};
use std::time::Duration;
use tracing::info;

/// Run the MCP server with HTTP transport
/// 
/// Uses stateless mode with JSON responses (no SSE streaming)
pub async fn run_http_server<H>(handler: H, config: &Config) -> anyhow::Result<()>
where
    H: rmcp::ServerHandler + Send + Sync + 'static,
{
    let addr = config.http_addr();
    
    info!("Starting MCP WebSearch server on HTTP transport");
    info!("Listening on http://{}", addr);
    info!("DDG timeout: {}s, max_results: {}", 
        config.ddg.timeout, config.ddg.max_results);

    // Configure streamable HTTP server in stateless JSON mode
    // stateful_mode=false: No session persistence
    // json_response=true: Returns JSON directly (no SSE)
    let server_config = StreamableHttpServerConfig {
        stateful_mode: false,   // Stateless - no session management
        json_response: true,    // JSON responses, no SSE
        sse_keep_alive: None,   // Not using SSE
        sse_retry: None,        // Not using SSE
        ..Default::default()
    };

    // Create and serve
    let server = StreamableHttpServer::serve(addr.parse()?, server_config).await?;
    let server = server.with_service(handler);
    
    info!("MCP server initialized, ready for requests");
    
    // Wait for shutdown signal
    tokio::signal::ctrl_c().await?;
    
    info!("Shutdown signal received, stopping server...");
    server.cancel();
    
    info!("MCP server shutdown complete");
    Ok(())
}
