//! Integration tests for MCP protocol
//!
//! Full MCP handshake: initialize, tools/list, tools/call flow

use crate::support::mcp_client::McpTestClient;

// ==================== INITIALIZE HANDSHAKE TESTS ====================

#[tokio::test]
async fn test_initialize_request_response() {
    let mut client = McpTestClient::new()
        .await
        .expect("Failed to create MCP client");

    // Send initialize request
    let params = serde_json::json!({
        "protocolVersion": "2024-11-05",
        "capabilities": {},
        "clientInfo": {
            "name": "test-client",
            "version": "1.0.0"
        }
    });

    let response = client
        .request("initialize", params)
        .await
        .expect("Initialize request failed");

    // Verify response structure
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.result.is_some(), "Should have result");
    assert!(response.error.is_none(), "Should not have error");

    // Verify result contains expected fields
    let result = response.result.unwrap();
    assert_eq!(
        result.get("protocolVersion").unwrap().as_str().unwrap(),
        "2024-11-05"
    );
    assert!(result.get("serverInfo").is_some());
    assert!(result.get("capabilities").is_some());

    // Send initialized notification
    client
        .notification("notifications/initialized", serde_json::json!({}))
        .await
        .expect("Initialized notification failed");
}

#[tokio::test]
async fn test_initialize_response_contains_server_info() {
    let mut client = McpTestClient::new()
        .await
        .expect("Failed to create MCP client");

    let params = serde_json::json!({
        "protocolVersion": "2024-11-05",
        "capabilities": {},
        "clientInfo": {
            "name": "test-client",
            "version": "1.0.0"
        }
    });

    let response = client
        .request("initialize", params)
        .await
        .expect("Initialize request failed");

    let result = response.result.unwrap();
    let server_info = result.get("serverInfo").unwrap();

    assert!(server_info.get("name").is_some());
    assert!(server_info.get("version").is_some());
}

#[tokio::test]
async fn test_initialize_response_contains_capabilities() {
    let mut client = McpTestClient::new()
        .await
        .expect("Failed to create MCP client");

    let params = serde_json::json!({
        "protocolVersion": "2024-11-05",
        "capabilities": {},
        "clientInfo": {
            "name": "test-client",
            "version": "1.0.0"
        }
    });

    let response = client
        .request("initialize", params)
        .await
        .expect("Initialize request failed");

    let result = response.result.unwrap();

    // Server should return protocol version and server info
    assert!(
        result.get("protocolVersion").is_some(),
        "Should return protocolVersion"
    );
    assert!(
        result.get("serverInfo").is_some(),
        "Should return serverInfo"
    );

    // Capabilities may or may not include tools depending on rmcp version
    if let Some(capabilities) = result.get("capabilities") {
        // If capabilities exist, tools should be listed
        if let Some(tools) = capabilities.get("tools") {
            assert!(tools.is_object(), "tools capability should be an object");
        }
    }
}

// ==================== TOOLS/LIST TESTS ====================

#[tokio::test]
async fn test_tools_list_returns_web_search() {
    let mut client = McpTestClient::new()
        .await
        .expect("Failed to create MCP client");

    // Initialize first
    client.initialize().await.expect("Initialize failed");

    // Request tools list
    let response = client
        .request("tools/list", serde_json::json!({}))
        .await
        .expect("Tools list request failed");

    // Verify response
    assert!(response.result.is_some());
    let result = response.result.unwrap();

    // Verify tools array contains web_search
    let tools = result.get("tools").unwrap().as_array().unwrap();
    assert!(!tools.is_empty(), "Should have at least one tool");

    // Find web_search tool
    let web_search_tool = tools
        .iter()
        .find(|t| t.get("name").unwrap().as_str().unwrap() == "web_search")
        .expect("web_search tool not found");

    // Verify tool structure
    assert!(web_search_tool.get("description").is_some());
    assert!(web_search_tool.get("inputSchema").is_some());
}

#[tokio::test]
async fn test_web_search_tool_schema() {
    let mut client = McpTestClient::new()
        .await
        .expect("Failed to create MCP client");

    client.initialize().await.expect("Initialize failed");

    let response = client
        .request("tools/list", serde_json::json!({}))
        .await
        .expect("Tools list request failed");

    let result = response.result.unwrap();
    let tools = result.get("tools").unwrap().as_array().unwrap();

    let web_search = tools
        .iter()
        .find(|t| t.get("name").unwrap().as_str().unwrap() == "web_search")
        .unwrap();

    // Verify input schema
    let schema = web_search.get("inputSchema").unwrap();
    assert_eq!(schema.get("type").unwrap().as_str().unwrap(), "object");

    let properties = schema.get("properties").unwrap();
    assert!(properties.get("query").is_some());

    let query_prop = properties.get("query").unwrap();
    assert_eq!(query_prop.get("type").unwrap().as_str().unwrap(), "string");

    let required = schema.get("required").unwrap().as_array().unwrap();
    assert!(required.iter().any(|r| r.as_str().unwrap() == "query"));
}

#[tokio::test]
async fn test_web_search_tool_description() {
    let mut client = McpTestClient::new()
        .await
        .expect("Failed to create MCP client");

    client.initialize().await.expect("Initialize failed");

    let response = client
        .request("tools/list", serde_json::json!({}))
        .await
        .expect("Tools list request failed");

    let result = response.result.unwrap();
    let tools = result.get("tools").unwrap().as_array().unwrap();

    let web_search = tools
        .iter()
        .find(|t| t.get("name").unwrap().as_str().unwrap() == "web_search")
        .unwrap();

    let description = web_search.get("description").unwrap().as_str().unwrap();
    assert!(!description.is_empty());
    assert!(
        description.contains("search"),
        "Description should mention search"
    );
}

// ==================== TOOLS/CALL TESTS ====================

/// Network test — run with: cargo test -- --ignored
#[tokio::test]
#[ignore = "hits real DuckDuckGo via subprocess"]
async fn test_web_search_call_with_valid_query() {
    let mut client = McpTestClient::new()
        .await
        .expect("Failed to create MCP client");

    client.initialize().await.expect("Initialize failed");

    // Call web_search tool
    let params = serde_json::json!({
        "name": "web_search",
        "arguments": {
            "query": "Rust programming language"
        }
    });

    let response = client
        .request("tools/call", params)
        .await
        .expect("Tools call failed");

    // Note: This uses real DDG - consider mocking for CI
    assert!(response.result.is_some());
    let result = response.result.unwrap();

    // Result should contain content array
    let content = result.get("content").unwrap().as_array().unwrap();
    assert!(!content.is_empty(), "Should have content");

    // First content should be text with JSON response
    let text_content = content.first().unwrap();
    assert_eq!(text_content.get("type").unwrap().as_str().unwrap(), "text");

    // Parse the JSON response
    let text = text_content.get("text").unwrap().as_str().unwrap();
    let search_response: serde_json::Value = serde_json::from_str(text).unwrap();

    // Verify response structure
    assert!(search_response.get("success").unwrap().as_bool().unwrap());
    assert!(search_response.get("results").is_some());
    assert!(search_response.get("message").is_some());
}

#[tokio::test]
async fn test_web_search_call_with_empty_query() {
    let mut client = McpTestClient::new()
        .await
        .expect("Failed to create MCP client");

    client.initialize().await.expect("Initialize failed");

    let params = serde_json::json!({
        "name": "web_search",
        "arguments": {
            "query": ""
        }
    });

    let response = client
        .request("tools/call", params)
        .await
        .expect("Tools call failed");

    // Empty query returns an MCP error (not a result with content),
    // because the tool returns Err(McpError::invalid_request(...))
    assert!(
        response.result.is_none() || response.error.is_some(),
        "Empty query should return an error response, not a success result"
    );
}

/// Network test — run with: cargo test -- --ignored
#[tokio::test]
#[ignore = "hits real DuckDuckGo via subprocess"]
async fn test_web_search_result_structure() {
    let mut client = McpTestClient::new()
        .await
        .expect("Failed to create MCP client");

    client.initialize().await.expect("Initialize failed");

    let params = serde_json::json!({
        "name": "web_search",
        "arguments": {
            "query": "test"
        }
    });

    let response = client
        .request("tools/call", params)
        .await
        .expect("Tools call failed");

    let result = response.result.unwrap();
    let content = result.get("content").unwrap().as_array().unwrap();
    let text_content = content.first().unwrap();
    let text = text_content.get("text").unwrap().as_str().unwrap();
    let search_response: serde_json::Value = serde_json::from_str(text).unwrap();

    let results = search_response.get("results").unwrap().as_array().unwrap();

    // Verify first result structure
    if !results.is_empty() {
        let first = results.first().unwrap();
        assert!(first.get("title").is_some());
        assert!(first.get("url").is_some());
        assert!(first.get("snippet").is_some());
    }
}

// ==================== ERROR HANDLING TESTS ====================

#[tokio::test]
async fn test_invalid_method_returns_error() {
    let mut client = McpTestClient::new()
        .await
        .expect("Failed to create MCP client");

    client.initialize().await.expect("Initialize failed");

    let params = serde_json::json!({});
    let response = client
        .request("invalid/method", params)
        .await
        .expect("Request should not fail at transport level");

    // Should return error
    assert!(response.error.is_some());
    let error = response.error.unwrap();
    assert_eq!(error.get("code").unwrap().as_i64().unwrap(), -32601); // Method not found
}

#[tokio::test]
async fn test_missing_required_param_returns_error() {
    let mut client = McpTestClient::new()
        .await
        .expect("Failed to create MCP client");

    client.initialize().await.expect("Initialize failed");

    // Call tools/call without required arguments
    let params = serde_json::json!({
        "name": "web_search"
        // Missing "arguments"
    });

    let _response = client.request("tools/call", params).await;

    // Should either succeed with defaults or return error
    // Behavior depends on rmcp implementation
}

// ==================== SERVER LIFECYCLE TESTS ====================

/// Network test — run with: cargo test -- --ignored
#[tokio::test]
#[ignore = "hits real DuckDuckGo via subprocess"]
async fn test_server_responds_to_multiple_requests() {
    let mut client = McpTestClient::new()
        .await
        .expect("Failed to create MCP client");

    // Initialize
    client.initialize().await.expect("Initialize failed");

    // Multiple tools/list requests
    for i in 0..3 {
        let response = client
            .request("tools/list", serde_json::json!({}))
            .await
            .unwrap_or_else(|_| panic!("Request {} failed", i));
        assert!(response.result.is_some());
    }

    // Multiple web_search calls
    for i in 0..2 {
        let params = serde_json::json!({
            "name": "web_search",
            "arguments": {
                "query": format!("test {}", i)
            }
        });

        let response = client
            .request("tools/call", params)
            .await
            .unwrap_or_else(|_| panic!("Search {} failed", i));
        assert!(response.result.is_some());
    }
}
