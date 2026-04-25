//! End-to-End tests for complete search workflows
//!
//! Full user journeys: init → tools/list → web_search → results

use crate::support::mcp_client::McpTestClient;

// ==================== HAPPY PATH TESTS ====================

#[tokio::test]
async fn test_complete_search_workflow() {
    // Create client
    let mut client = McpTestClient::new().await
        .expect("Failed to create MCP client");
    
    // Initialize
    client.initialize().await
        .expect("Initialize failed");
    
    // Get tools list
    let tools_response = client.request("tools/list", serde_json::json!({})).await
        .expect("Tools list failed");
    
    assert!(tools_response.result.is_some());
    
    // Call web_search
    let params = serde_json::json!({
        "name": "web_search",
        "arguments": {
            "query": "Rust programming language"
        }
    });
    
    let search_response = client.request("tools/call", params).await
        .expect("Search failed");
    
    // Verify complete response chain
    assert!(search_response.result.is_some());
    
    let result = search_response.result.unwrap();
    let content = result.get("content").unwrap().as_array().unwrap();
    let text = content.first().unwrap().get("text").unwrap().as_str().unwrap();
    let response_json: serde_json::Value = serde_json::from_str(text).unwrap();
    
    // Verify success
    assert!(response_json.get("success").unwrap().as_bool().unwrap());
    
    // Verify results structure
    let results = response_json.get("results").unwrap().as_array().unwrap();
    assert!(!results.is_empty(), "Should have results");
    
    for result in results {
        assert!(result.get("title").is_some());
        assert!(result.get("url").is_some());
        assert!(result.get("snippet").is_some());
        
        // URL should be valid
        let url = result.get("url").unwrap().as_str().unwrap();
        assert!(url.starts_with("http"), "URL should be valid: {}", url);
    }
}

#[tokio::test]
async fn test_search_results_have_required_fields() {
    let mut client = McpTestClient::new().await
        .expect("Failed to create MCP client");
    
    client.initialize().await.expect("Initialize failed");
    
    let params = serde_json::json!({
        "name": "web_search",
        "arguments": {
            "query": "test"
        }
    });
    
    let response = client.request("tools/call", params).await.expect("Search failed");
    
    let result = response.result.unwrap();
    let content = result.get("content").unwrap().as_array().unwrap();
    let text = content.first().unwrap().get("text").unwrap().as_str().unwrap();
    let response_json: serde_json::Value = serde_json::from_str(text).unwrap();
    
    let results = response_json.get("results").unwrap().as_array().unwrap();
    
    for result in results {
        let title = result.get("title").unwrap().as_str().unwrap();
        let url = result.get("url").unwrap().as_str().unwrap();
        let snippet = result.get("snippet").unwrap().as_str().unwrap();
        
        // Verify types
        assert!(!title.is_empty());
        assert!(!url.is_empty());
        // Snippet may be empty
        
        // Verify URL format
        assert!(
            url.starts_with("http://") || url.starts_with("https://"),
            "URL should be absolute: {}",
            url
        );
    }
}

// ==================== QUERY VARIETY TESTS ====================

#[tokio::test]
async fn test_simple_query() {
    let mut client = McpTestClient::new().await
        .expect("Failed to create MCP client");
    
    client.initialize().await.expect("Initialize failed");
    
    let params = serde_json::json!({
        "name": "web_search",
        "arguments": { "query": "hello" }
    });
    
    let response = client.request("tools/call", params).await;
    assert!(response.is_ok(), "Simple query should succeed");
}

#[tokio::test]
async fn test_complex_query() {
    let mut client = McpTestClient::new().await
        .expect("Failed to create MCP client");
    
    client.initialize().await.expect("Initialize failed");
    
    let params = serde_json::json!({
        "name": "web_search",
        "arguments": { "query": "rust async runtime tokio" }
    });
    
    let response = client.request("tools/call", params).await;
    assert!(response.is_ok(), "Complex query should succeed");
}

#[tokio::test]
async fn test_query_with_special_chars() {
    let mut client = McpTestClient::new().await
        .expect("Failed to create MCP client");
    
    client.initialize().await.expect("Initialize failed");
    
    // C++
    let params = serde_json::json!({
        "name": "web_search",
        "arguments": { "query": "C++ programming" }
    });
    
    let response = client.request("tools/call", params).await;
    assert!(response.is_ok(), "Query with ++ should succeed");
    
    // C#
    let params2 = serde_json::json!({
        "name": "web_search",
        "arguments": { "query": "C# programming" }
    });
    
    let response2 = client.request("tools/call", params2).await;
    assert!(response2.is_ok(), "Query with # should succeed");
}

#[tokio::test]
async fn test_query_with_unicode() {
    let mut client = McpTestClient::new().await
        .expect("Failed to create MCP client");
    
    client.initialize().await.expect("Initialize failed");
    
    // café
    let params = serde_json::json!({
        "name": "web_search",
        "arguments": { "query": "café" }
    });
    
    let response = client.request("tools/call", params).await;
    assert!(response.is_ok(), "Query with é should succeed");
}

#[tokio::test]
async fn test_query_with_chinese() {
    let mut client = McpTestClient::new().await
        .expect("Failed to create MCP client");
    
    client.initialize().await.expect("Initialize failed");
    
    let params = serde_json::json!({
        "name": "web_search",
        "arguments": { "query": "中文" }
    });
    
    let response = client.request("tools/call", params).await;
    assert!(response.is_ok(), "Chinese query should succeed");
}

// ==================== RESULT LIMITS TESTS ====================

#[tokio::test]
async fn test_results_limited_to_max() {
    let mut client = McpTestClient::new().await
        .expect("Failed to create MCP client");
    
    client.initialize().await.expect("Initialize failed");
    
    let params = serde_json::json!({
        "name": "web_search",
        "arguments": { "query": "popular search term" }
    });
    
    let response = client.request("tools/call", params).await.expect("Search failed");
    
    let result = response.result.unwrap();
    let content = result.get("content").unwrap().as_array().unwrap();
    let text = content.first().unwrap().get("text").unwrap().as_str().unwrap();
    let response_json: serde_json::Value = serde_json::from_str(text).unwrap();
    
    let results = response_json.get("results").unwrap().as_array().unwrap();
    
    // Should be limited to MAX_RESULTS (5)
    assert!(results.len() <= 5, "Should not exceed MAX_RESULTS");
}

// ==================== ERROR SCENARIO TESTS ====================

#[tokio::test]
async fn test_empty_query_error() {
    let mut client = McpTestClient::new().await
        .expect("Failed to create MCP client");
    
    client.initialize().await.expect("Initialize failed");
    
    let params = serde_json::json!({
        "name": "web_search",
        "arguments": { "query": "" }
    });
    
    let response = client.request("tools/call", params).await.expect("Call failed");
    
    let result = response.result.unwrap();
    let content = result.get("content").unwrap().as_array().unwrap();
    let text = content.first().unwrap().get("text").unwrap().as_str().unwrap();
    let response_json: serde_json::Value = serde_json::from_str(text).unwrap();
    
    // Empty query should return error response
    assert!(!response_json.get("success").unwrap().as_bool().unwrap());
    assert!(response_json.get("error").is_some());
}

#[tokio::test]
async fn test_whitespace_only_query_error() {
    let mut client = McpTestClient::new().await
        .expect("Failed to create MCP client");
    
    client.initialize().await.expect("Initialize failed");
    
    let params = serde_json::json!({
        "name": "web_search",
        "arguments": { "query": "   " }
    });
    
    let response = client.request("tools/call", params).await.expect("Call failed");
    
    let result = response.result.unwrap();
    let content = result.get("content").unwrap().as_array().unwrap();
    let text = content.first().unwrap().get("text").unwrap().as_str().unwrap();
    let response_json: serde_json::Value = serde_json::from_str(text).unwrap();
    
    // Whitespace-only query should return error
    assert!(!response_json.get("success").unwrap().as_bool().unwrap());
}

#[tokio::test]
async fn test_multiple_searches_in_sequence() {
    let mut client = McpTestClient::new().await
        .expect("Failed to create MCP client");
    
    client.initialize().await.expect("Initialize failed");
    
    // Perform multiple searches
    let queries = vec![
        "Rust programming",
        "Python language",
        "JavaScript",
    ];
    
    for query in queries {
        let params = serde_json::json!({
            "name": "web_search",
            "arguments": { "query": query }
        });
        
        let response = client.request("tools/call", params).await;
        assert!(response.is_ok(), "Search for '{}' should succeed", query);
        
        // Brief delay between searches
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}
