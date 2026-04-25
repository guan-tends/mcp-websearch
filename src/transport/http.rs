//! HTTP Transport
//!
//! Provides HTTP-based MCP transport (simplified for now)

use crate::config::Config;
use crate::tools::WebSearchTool;
use tokio::net::TcpListener;

/// Serve the web search tool over HTTP
pub async fn serve(_handler: WebSearchTool, config: Config) -> anyhow::Result<()> {
    let addr = config.http_addr()
        .ok_or_else(|| anyhow::anyhow!("HTTP transport requires http config"))?;
    let listener = TcpListener::bind(&addr).await?;
    
    tracing::info!("HTTP server listening on {}", addr);
    tracing::warn!("HTTP transport requires full StreamableHTTP implementation");
    tracing::info!("For now, use --transport stdio for full functionality");
    
    // Accept connections and handle them
    // Note: Full StreamableHTTP implementation requires axum, tower, tokio-util
    // For now, we provide a simplified TCP-based transport
    
    loop {
        let (socket, peer_addr) = listener.accept().await?;
        tracing::info!("Connection from {}", peer_addr);
        
        // TODO: Implement full StreamableHTTP with session management
        // This requires: axum, tower, tokio-util, and proper SessionManager
        
        // For now, just close the connection
        drop(socket);
    }
}
