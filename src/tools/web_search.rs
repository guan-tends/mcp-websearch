//! Web Search MCP Tool
//!
//! Implements the web_search tool using rmcp macros
//! Ported from Kotlin WebSearchTool with exact behavior

use crate::{
    config::DdgConfig,
    ddg::{DdgClient, SearchResult},
};
use rmcp::{
    ErrorData as McpError, ServerHandler,
    handler::server::router::tool::ToolRouter,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
    schemars::{self, JsonSchema},
    tool, tool_handler, tool_router,
};
use serde::Serialize;

/// WebSearch tool handler
#[derive(Clone)]
pub struct WebSearchTool {
    client: DdgClient,
    tool_router: ToolRouter<Self>,
}

impl WebSearchTool {
    /// Create a new WebSearch tool with the given DDG configuration
    pub fn new(config: DdgConfig) -> anyhow::Result<Self> {
        let client = DdgClient::new(config)?;
        let tool_router = Self::tool_router();
        Ok(Self {
            client,
            tool_router,
        })
    }
}

/// Parameters for web_search tool
#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct WebSearchParams {
    /// The search query
    query: String,
}

/// Response structure for web_search tool
#[derive(Debug, Serialize)]
struct WebSearchResponse {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    results: Option<Vec<SearchResult>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[tool_router]
impl WebSearchTool {
    /// Search the web for current information
    ///
    /// Returns titles, URLs, and snippets from DuckDuckGo
    #[tool(
        name = "web_search",
        description = "Search the web for current information. Returns titles, URLs, and snippets. Before answering questions about recent events, news, current prices, weather, or anything time-sensitive, search first. Also use this when you're unsure about facts or the user asks you to look something up."
    )]
    async fn search(
        &self,
        Parameters(params): Parameters<WebSearchParams>,
    ) -> Result<CallToolResult, McpError> {
        // Validate query
        if params.query.trim().is_empty() {
            return Err(McpError::invalid_request("Search query is required", None));
        }

        // Execute search
        match self.client.search(&params.query).await {
            Ok(results) => {
                let response = WebSearchResponse {
                    success: true,
                    results: Some(results.clone()),
                    message: Some(format!("Found {} results", results.len())),
                    error: None,
                };

                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string(&response).map_err(|e| {
                        McpError::internal_error(
                            format!("Failed to serialize response: {}", e),
                            None,
                        )
                    })?,
                )]))
            }
            Err(e) => {
                let response = WebSearchResponse {
                    success: false,
                    results: None,
                    message: None,
                    error: Some(format!("Search failed: {}", e)),
                };

                Ok(CallToolResult::error(vec![Content::text(
                    serde_json::to_string(&response).map_err(|e| {
                        McpError::internal_error(
                            format!("Failed to serialize error response: {}", e),
                            None,
                        )
                    })?,
                )]))
            }
        }
    }
}

#[tool_handler]
impl ServerHandler for WebSearchTool {}
