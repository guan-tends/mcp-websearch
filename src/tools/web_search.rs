//! Web Search MCP Tool
//! 
//! Implements the web_search tool using rmcp macros
//! Ported from Kotlin WebSearchTool with exact behavior

use crate::{
    config::DdgConfig,
    ddg::{DdgClient, SearchResult},
};
use rmcp::{
    handler::server::wrapper::Parameters,
    model::CallToolResult,
    schemars::{self, JsonSchema},
    tool, tool_router,
};
use serde::Serialize;

/// WebSearch tool handler
#[derive(Clone)]
pub struct WebSearchTool {
    client: DdgClient,
}

impl WebSearchTool {
    /// Create a new WebSearch tool with the given DDG configuration
    pub fn new(config: DdgConfig) -> anyhow::Result<Self> {
        let client = DdgClient::new(config)?;
        Ok(Self { client })
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

#[tool_router(server_handler)]
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
    ) -> CallToolResult {
        // Validate query
        if params.query.trim().is_empty() {
            let response = WebSearchResponse {
                success: false,
                results: None,
                message: None,
                error: Some("Query is required".to_string()),
            };
            return CallToolResult {
                content: vec![rmcp::model::Content::text(
                    serde_json::to_string(&response).unwrap_or_default()
                )],
                is_error: true,
                ..Default::default()
            };
        }

        // Perform search
        match self.client.search(&params.query).await {
            Ok(results) => {
                if results.is_empty() {
                    let response = WebSearchResponse {
                        success: true,
                        results: Some(vec![]),
                        message: Some("No results found".to_string()),
                        error: None,
                    };
                    CallToolResult {
                        content: vec![rmcp::model::Content::text(
                            serde_json::to_string(&response).unwrap_or_default()
                        )],
                        is_error: false,
                        ..Default::default()
                    }
                } else {
                    let response = WebSearchResponse {
                        success: true,
                        results: Some(results),
                        message: None,
                        error: None,
                    };
                    CallToolResult {
                        content: vec![rmcp::model::Content::text(
                            serde_json::to_string(&response).unwrap_or_default()
                        )],
                        is_error: false,
                        ..Default::default()
                    }
                }
            }
            Err(e) => {
                let response = WebSearchResponse {
                    success: false,
                    results: None,
                    message: None,
                    error: Some(format!("Search failed: {}", e)),
                };
                CallToolResult {
                    content: vec![rmcp::model::Content::text(
                        serde_json::to_string(&response).unwrap_or_default()
                    )],
                    is_error: true,
                    ..Default::default()
                }
            }
        }
    }
}
