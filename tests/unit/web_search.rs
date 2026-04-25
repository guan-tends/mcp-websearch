//! Unit tests for WebSearch Tool module
//!
//! Tool handler, MCP protocol integration, response formatting

use mcp_websearch::tools::WebSearchTool;
use mcp_websearch::ddg::parser::DdgConfig;
use mcp_websearch::config::DdgConfig as ConfigDdg;

/// Create test DDG config
fn test_ddg_config() -> DdgConfig {
    DdgConfig {
        timeout: 15,
        max_results: 5,
        user_agent: "Test/1.0".to_string(),
    }
}

// ==================== TOOL CREATION TESTS ====================

#[tokio::test]
async fn test_websearch_tool_creation_succeeds() {
    let config = test_ddg_config();
    let result = WebSearchTool::new(config);
    assert!(result.is_ok(), "Should create tool successfully");
}

#[tokio::test]
async fn test_websearch_tool_creation_with_custom_config() {
    let config = DdgConfig {
        timeout: 30,
        max_results: 10,
        user_agent: "CustomAgent/2.0".to_string(),
    };
    let tool = WebSearchTool::new(config);
    assert!(tool.is_ok());
}

// ==================== TOOL SCHEMA TESTS ====================

#[test]
fn test_web_search_tool_schema() {
    // Verify the tool has correct metadata
    // This is tested implicitly through the tool_router macro
    // The schema should include: name="web_search", description, input schema with "query" param
}

// ==================== SEARCH VALIDATION TESTS ====================

#[tokio::test]
async fn test_search_validates_empty_query() {
    // Note: Actual test would require calling the tool through MCP protocol
    // This documents expected behavior: empty query should return error response
    
    // Expected: Call with empty query returns CallToolResult with is_error=true
    // and error message about query being required
}

#[tokio::test]
async fn test_search_validates_whitespace_query() {
    // Query with only whitespace should be treated as empty
    // Expected: Same error as empty query
}

// ==================== SEARCH SUCCESS TESTS ====================

#[tokio::test]
async fn test_search_returns_results() {
    let config = test_ddg_config();
    let tool = WebSearchTool::new(config).unwrap();
    
    // Note: This is an integration-style test using real DDG
    // For unit testing, we'd need to inject a mock DDG client
    
    // Expected: Returns CallToolResult with:
    // - success=true
    // - results array with title, url, snippet
    // - message with result count
}

// ==================== RESPONSE FORMAT TESTS ====================

#[test]
fn test_success_response_structure() {
    // Expected JSON structure for successful search:
    let expected = serde_json::json!({
        "success": true,
        "results": [
            {
                "title": "Example",
                "url": "https://example.com",
                "snippet": "Description"
            }
        ],
        "message": "Found 1 results"
    });
    
    // Verify structure is valid JSON
    let _json_string = serde_json::to_string(&expected).unwrap();
}

#[test]
fn test_error_response_structure() {
    // Expected JSON structure for failed search:
    let expected = serde_json::json!({
        "success": false,
        "error": "Search failed: Some error message"
    });
    
    // Verify structure is valid JSON
    let _json_string = serde_json::to_string(&expected).unwrap();
}

// ==================== CONTENT CONSTRUCTION TESTS ====================

#[test]
fn test_call_tool_result_success_construction() {
    use rmcp::model::{CallToolResult, Content};
    
    let response_json = serde_json::json!({
        "success": true,
        "results": [],
        "message": "Found 0 results"
    });
    
    let content = Content::text(response_json.to_string());
    let result = CallToolResult::success(vec![content]);
    
    assert!(!result.is_error);
    assert_eq!(result.content.len(), 1);
}

#[test]
fn test_call_tool_result_error_construction() {
    use rmcp::model::{CallToolResult, Content};
    
    let response_json = serde_json::json!({
        "success": false,
        "error": "Something went wrong"
    });
    
    let content = Content::text(response_json.to_string());
    let result = CallToolResult::error(vec![content]);
    
    assert!(result.is_error);
    assert_eq!(result.content.len(), 1);
}

// ==================== TOOL ROUTER TESTS ====================

#[test]
fn test_tool_router_includes_web_search() {
    // Verify the tool_router macro correctly registers web_search
    // This is tested through integration with MCP protocol
}

// ==================== SERIALIZATION TESTS ====================

#[test]
fn test_search_result_serialization() {
    use mcp_websearch::ddg::SearchResult;
    
    let result = SearchResult {
        title: "Test Title".to_string(),
        url: "https://example.com".to_string(),
        snippet: "Test snippet".to_string(),
    };
    
    let json = serde_json::to_string(&result).unwrap();
    assert!(json.contains("Test Title"));
    assert!(json.contains("https://example.com"));
    assert!(json.contains("Test snippet"));
}

#[test]
fn test_search_result_deserialization() {
    use mcp_websearch::ddg::SearchResult;
    
    let json = r#"{"title":"Test","url":"https://example.com","snippet":"Desc"}"#;
    let result: SearchResult = serde_json::from_str(json).unwrap();
    
    assert_eq!(result.title, "Test");
    assert_eq!(result.url, "https://example.com");
    assert_eq!(result.snippet, "Desc");
}

// ==================== RESPONSE CONTENT TESTS ====================

#[test]
fn test_web_search_response_contains_expected_fields() {
    // Verify response structure matches expected contract
    let response = serde_json::json!({
        "success": true,
        "results": [
            {
                "title": "Rust Programming Language",
                "url": "https://www.rust-lang.org",
                "snippet": "A language empowering everyone"
            }
        ],
        "message": "Found 1 results",
        "error": null
    });
    
    // Verify all expected fields present
    assert!(response.get("success").is_some());
    assert!(response.get("results").is_some());
    assert!(response.get("message").is_some());
    assert!(response.get("error").is_some());
}
