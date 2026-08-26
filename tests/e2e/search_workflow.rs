//! End-to-End tests for complete search workflows.
//!
//! All tests in this file spawn the MCP server as a subprocess and hit the
//! real DuckDuckGo API. They are marked `#[ignore]` and excluded from CI.
//!
//! Run manually with:
//!
//! ```sh
//! cargo test --test e2e -- --ignored
//! ```

use crate::support::mcp_client::McpTestClient;

// ==================== HAPPY PATH TESTS ====================

/// E2E network test — run with: cargo test -- --ignored
#[tokio::test]
#[ignore = "hits real DuckDuckGo via subprocess"]
async fn test_complete_search_workflow() {
    let mut client = McpTestClient::new()
        .await
        .expect("Failed to create MCP client");

    client.initialize().await.expect("Initialize failed");

    let tools_response = client
        .request("tools/list", serde_json::json!({}))
        .await
        .expect("Tools list failed");
    assert!(tools_response.result.is_some());

    let params = serde_json::json!({
        "name": "web_search",
        "arguments": { "query": "Rust programming language" }
    });

    let search_response = client
        .request("tools/call", params)
        .await
        .expect("Search failed");
    assert!(search_response.result.is_some());

    let result = search_response.result.unwrap();
    let content = result.get("content").unwrap().as_array().unwrap();
    let text = content
        .first()
        .unwrap()
        .get("text")
        .unwrap()
        .as_str()
        .unwrap();
    let response_json: serde_json::Value = serde_json::from_str(text).unwrap();

    assert!(response_json.get("success").unwrap().as_bool().unwrap());

    let results = response_json.get("results").unwrap().as_array().unwrap();
    assert!(!results.is_empty(), "Should have results");

    for result in results {
        assert!(result.get("title").is_some());
        assert!(result.get("url").is_some());
        assert!(result.get("snippet").is_some());
        let url = result.get("url").unwrap().as_str().unwrap();
        assert!(url.starts_with("http"), "URL should be valid: {}", url);
    }
}

/// E2E network test — run with: cargo test -- --ignored
#[tokio::test]
#[ignore = "hits real DuckDuckGo via subprocess"]
async fn test_search_results_have_required_fields() {
    let mut client = McpTestClient::new()
        .await
        .expect("Failed to create MCP client");
    client.initialize().await.expect("Initialize failed");

    let params = serde_json::json!({
        "name": "web_search",
        "arguments": { "query": "test" }
    });

    let response = client
        .request("tools/call", params)
        .await
        .expect("Search failed");

    let result = response.result.unwrap();
    let content = result.get("content").unwrap().as_array().unwrap();
    let text = content
        .first()
        .unwrap()
        .get("text")
        .unwrap()
        .as_str()
        .unwrap();
    let response_json: serde_json::Value = serde_json::from_str(text).unwrap();

    let results = response_json.get("results").unwrap().as_array().unwrap();
    for result in results {
        assert!(!result.get("title").unwrap().as_str().unwrap().is_empty());
        assert!(!result.get("url").unwrap().as_str().unwrap().is_empty());
        assert!(
            result
                .get("url")
                .unwrap()
                .as_str()
                .unwrap()
                .starts_with("http"),
            "URL should be absolute"
        );
    }
}

// ==================== QUERY VARIETY TESTS ====================

/// E2E network test — run with: cargo test -- --ignored
#[tokio::test]
#[ignore = "hits real DuckDuckGo via subprocess"]
async fn test_simple_query() {
    let mut client = McpTestClient::new()
        .await
        .expect("Failed to create MCP client");
    client.initialize().await.expect("Initialize failed");

    let params = serde_json::json!({
        "name": "web_search",
        "arguments": { "query": "hello" }
    });

    let response = client.request("tools/call", params).await;
    assert!(response.is_ok(), "Simple query should succeed");
}

/// E2E network test — run with: cargo test -- --ignored
#[tokio::test]
#[ignore = "hits real DuckDuckGo via subprocess"]
async fn test_complex_query() {
    let mut client = McpTestClient::new()
        .await
        .expect("Failed to create MCP client");
    client.initialize().await.expect("Initialize failed");

    let params = serde_json::json!({
        "name": "web_search",
        "arguments": { "query": "rust async runtime tokio" }
    });

    let response = client.request("tools/call", params).await;
    assert!(response.is_ok(), "Complex query should succeed");
}

/// E2E network test — run with: cargo test -- --ignored
#[tokio::test]
#[ignore = "hits real DuckDuckGo via subprocess"]
async fn test_query_with_special_chars() {
    let mut client = McpTestClient::new()
        .await
        .expect("Failed to create MCP client");
    client.initialize().await.expect("Initialize failed");

    let params = serde_json::json!({
        "name": "web_search",
        "arguments": { "query": "C++ programming" }
    });

    let response = client.request("tools/call", params).await;
    assert!(response.is_ok(), "Query with ++ should succeed");
}

/// E2E network test — run with: cargo test -- --ignored
#[tokio::test]
#[ignore = "hits real DuckDuckGo via subprocess"]
async fn test_query_with_unicode() {
    let mut client = McpTestClient::new()
        .await
        .expect("Failed to create MCP client");
    client.initialize().await.expect("Initialize failed");

    let params = serde_json::json!({
        "name": "web_search",
        "arguments": { "query": "café" }
    });

    let response = client.request("tools/call", params).await;
    assert!(response.is_ok(), "Query with é should succeed");
}

/// E2E network test — run with: cargo test -- --ignored
#[tokio::test]
#[ignore = "hits real DuckDuckGo via subprocess"]
async fn test_query_with_chinese() {
    let mut client = McpTestClient::new()
        .await
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

/// E2E network test — run with: cargo test -- --ignored
#[tokio::test]
#[ignore = "hits real DuckDuckGo via subprocess"]
async fn test_results_limited_to_max() {
    let mut client = McpTestClient::new()
        .await
        .expect("Failed to create MCP client");
    client.initialize().await.expect("Initialize failed");

    let params = serde_json::json!({
        "name": "web_search",
        "arguments": { "query": "popular search term" }
    });

    let response = client
        .request("tools/call", params)
        .await
        .expect("Search failed");

    let result = response.result.unwrap();
    let content = result.get("content").unwrap().as_array().unwrap();
    let text = content
        .first()
        .unwrap()
        .get("text")
        .unwrap()
        .as_str()
        .unwrap();
    let response_json: serde_json::Value = serde_json::from_str(text).unwrap();

    let results = response_json.get("results").unwrap().as_array().unwrap();
    assert!(results.len() <= 5, "Should not exceed MAX_RESULTS");
}

// ==================== ERROR SCENARIO TESTS ====================

/// E2E network test — run with: cargo test -- --ignored
#[tokio::test]
#[ignore = "hits real DuckDuckGo via subprocess"]
async fn test_empty_query_error() {
    let mut client = McpTestClient::new()
        .await
        .expect("Failed to create MCP client");
    client.initialize().await.expect("Initialize failed");

    let params = serde_json::json!({
        "name": "web_search",
        "arguments": { "query": "" }
    });

    let response = client
        .request("tools/call", params)
        .await
        .expect("Call failed");

    let result = response.result.unwrap();
    let content = result.get("content").unwrap().as_array().unwrap();
    let text = content
        .first()
        .unwrap()
        .get("text")
        .unwrap()
        .as_str()
        .unwrap();
    let response_json: serde_json::Value = serde_json::from_str(text).unwrap();

    assert!(!response_json.get("success").unwrap().as_bool().unwrap());
    assert!(response_json.get("error").is_some());
}

/// E2E network test — run with: cargo test -- --ignored
#[tokio::test]
#[ignore = "hits real DuckDuckGo via subprocess"]
async fn test_whitespace_only_query_error() {
    let mut client = McpTestClient::new()
        .await
        .expect("Failed to create MCP client");
    client.initialize().await.expect("Initialize failed");

    let params = serde_json::json!({
        "name": "web_search",
        "arguments": { "query": "   " }
    });

    let response = client
        .request("tools/call", params)
        .await
        .expect("Call failed");

    let result = response.result.unwrap();
    let content = result.get("content").unwrap().as_array().unwrap();
    let text = content
        .first()
        .unwrap()
        .get("text")
        .unwrap()
        .as_str()
        .unwrap();
    let response_json: serde_json::Value = serde_json::from_str(text).unwrap();

    assert!(!response_json.get("success").unwrap().as_bool().unwrap());
}

/// E2E network test — run with: cargo test -- --ignored
#[tokio::test]
#[ignore = "hits real DuckDuckGo via subprocess"]
async fn test_multiple_searches_in_sequence() {
    let mut client = McpTestClient::new()
        .await
        .expect("Failed to create MCP client");
    client.initialize().await.expect("Initialize failed");

    let queries = vec!["Rust programming", "Python language", "JavaScript"];

    for query in queries {
        let params = serde_json::json!({
            "name": "web_search",
            "arguments": { "query": query }
        });

        let response = client.request("tools/call", params).await;
        assert!(response.is_ok(), "Search for '{}' should succeed", query);

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}
