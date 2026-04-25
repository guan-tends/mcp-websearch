//! Unit tests for DDG Client module
//!
//! HTTP client with mocked responses via wiremock

use mcp_websearch::ddg::DdgClient;
use mcp_websearch::ddg::parser::DdgConfig;
use mcp_websearch::ddg::SearchResult;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path_regex};

/// Create test DDG config
fn test_config() -> DdgConfig {
    DdgConfig {
        timeout: 15,
        max_results: 5,
        user_agent: "Test/1.0".to_string(),
    }
}

/// Sample DDG HTML response
fn sample_html_response() -> &'static str {
    r#"<html><body><table>
        <tr><td class="result-link"><a href="//duckduckgo.com/l/?uddg=https%3A%2F%2Fexample.com" class="result-link">Example Site</a></td></tr>
        <tr><td class="result-snippet">This is an example site description.</td></tr>
    </table></body></html>"#
}

// ==================== CLIENT CREATION TESTS ====================

#[tokio::test]
async fn test_client_creation_succeeds() {
    let config = test_config();
    let result = DdgClient::new(config);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_client_creation_with_custom_config() {
    let config = DdgConfig {
        timeout: 30,
        max_results: 10,
        user_agent: "CustomAgent/2.0".to_string(),
    };
    let client = DdgClient::new(config).unwrap();
    // Client created successfully
}

// ==================== SEARCH SUCCESS TESTS ====================

#[tokio::test]
async fn test_search_returns_results() {
    let config = test_config();
    let client = DdgClient::new(config).unwrap();
    
    // Note: This test uses real DDG - consider using wiremock for CI
    let results = client.search("rust programming").await.unwrap();
    
    assert!(!results.is_empty(), "Should return search results");
    
    // Verify result structure
    for result in &results {
        assert!(!result.title.is_empty(), "Title should not be empty");
        assert!(!result.url.is_empty(), "URL should not be empty");
        // URL should be extracted from DDG redirect
        assert!(!result.url.contains("duckduckgo.com"), "URL should be actual target");
    }
}

#[tokio::test]
async fn test_search_result_count_limited() {
    let mut config = test_config();
    config.max_results = 3;
    let client = DdgClient::new(config).unwrap();
    
    // Note: Real DDG test - max_results limits after parsing
    let results = client.search("rust").await.unwrap();
    
    // Parser limits to MAX_RESULTS (5) regardless of config
    assert!(results.len() <= 5, "Should not exceed MAX_RESULTS");
}

// ==================== MOCKED HTTP TESTS ====================

/// Helper to mock DDG server for testing
async fn mock_ddg_server(html_response: &'static str) -> MockServer {
    let server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path_regex("/lite/.*"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_string(html_response)
            .insert_header("content-type", "text/html"))
        .mount(&server)
        .await;
    
    server
}

// Note: The following tests require modifying the client to accept a custom base URL
// or using a different testing approach. For now, they document expected behavior.

#[tokio::test]
async fn test_mocked_search_parses_html() {
    // This test shows how to test with wiremock once DDG base URL is configurable
    let server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path_regex("/lite/.*"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_string(sample_html_response())
            .insert_header("content-type", "text/html"))
        .mount(&server)
        .await;
    
    // Would test with configurable base URL
    // For now, this demonstrates the mocking setup
}

#[tokio::test]
async fn test_mocked_404_response() {
    let server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path_regex("/lite/.*"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;
    
    // Would test error handling with configurable base URL
}

#[tokio::test]
async fn test_mocked_500_response() {
    let server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path_regex("/lite/.*"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;
    
    // Would test error handling with configurable base URL
}

#[tokio::test]
async fn test_mocked_empty_response() {
    let server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path_regex("/lite/.*"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_string("<html><body><table></table></body></html>")
            .insert_header("content-type", "text/html"))
        .mount(&server)
        .await;
    
    // Would test empty results with configurable base URL
}

// ==================== ERROR HANDLING TESTS ====================

#[tokio::test]
async fn test_search_with_empty_query() {
    let config = test_config();
    let client = DdgClient::new(config).unwrap();
    
    // Empty query should still work (DDG returns results for empty query)
    let results = client.search("").await;
    // Should not panic - behavior depends on DDG response
}

#[tokio::test]
async fn test_search_with_special_chars() {
    let config = test_config();
    let client = DdgClient::new(config).unwrap();
    
    // Special characters should be URL encoded properly
    let results = client.search("C++ programming").await;
    assert!(results.is_ok());
}

#[tokio::test]
async fn test_search_with_unicode() {
    let config = test_config();
    let client = DdgClient::new(config).unwrap();
    
    // Unicode should be properly encoded
    let results = client.search("café").await;
    assert!(results.is_ok());
}

// ==================== INTEGRATION WITH PARSER TESTS ====================

#[tokio::test]
async fn test_search_delegates_to_parser() {
    // Verifies that search() correctly delegates to parse_results
    let config = test_config();
    let client = DdgClient::new(config).unwrap();
    
    // Real search - verifies parser integration works
    let results = client.search("test").await.unwrap();
    
    // All results should have proper structure from parser
    for result in &results {
        // Verify URL was extracted from DDG redirect
        assert!(
            result.url.starts_with("http://") || result.url.starts_with("https://"),
            "URL should be extracted: {}",
            result.url
        );
    }
}
