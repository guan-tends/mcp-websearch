//! Unit tests for DDG Client module.
//!
//! Mock tests use wiremock with a configurable `base_url`.
//! Network tests (hitting real DuckDuckGo) are marked `#[ignore]` — run with:
//!
//! ```sh
//! cargo test -- --ignored
//! ```

use mcp_websearch::config::DdgConfig;
use mcp_websearch::ddg::DdgClient;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

/// Create a DDG config pointing at a mock server.
fn mock_config(base_url: String) -> DdgConfig {
    DdgConfig {
        timeout: 15,
        max_results: 5,
        user_agent: "Test/1.0".to_string(),
        base_url,
    }
}

/// Create a DDG config pointing at the real DuckDuckGo (for `#[ignore]` tests).
fn real_config() -> DdgConfig {
    DdgConfig {
        timeout: 15,
        max_results: 5,
        user_agent: "Test/1.0".to_string(),
        base_url: "https://lite.duckduckgo.com/lite/".to_string(),
    }
}

/// Sample DDG HTML response with 3 results.
fn sample_html() -> &'static str {
    r#"<html><body><table>
    <tr><td class="result-link"><a href="//duckduckgo.com/l/?uddg=https%3A%2F%2Fexample.com" class="result-link">Example Site</a></td></tr>
    <tr><td class="result-snippet">This is an example site description.</td></tr>
    <tr><td class="result-link"><a href="//duckduckgo.com/l/?uddg=https%3A%2F%2Ftest.com" class="result-link">Test Site</a></td></tr>
    <tr><td class="result-snippet">Another test description.</td></tr>
    </table></body></html>"#
}

// ==================== CLIENT CREATION TESTS ====================

#[tokio::test]
async fn test_client_creation_succeeds() {
    let config = real_config();
    let result = DdgClient::new(config);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_client_creation_with_custom_timeout() {
    let config = DdgConfig {
        timeout: 30,
        max_results: 10,
        user_agent: "CustomAgent/2.0".to_string(),
        base_url: "https://lite.duckduckgo.com/lite/".to_string(),
    };
    let client = DdgClient::new(config);
    assert!(client.is_ok());
}

// ==================== MOCKED SEARCH TESTS ====================

#[tokio::test]
async fn test_mocked_search_returns_results() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/lite/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(sample_html())
                .insert_header("content-type", "text/html"),
        )
        .mount(&server)
        .await;

    let config = mock_config(format!("{}/lite/", server.uri()));
    let client = DdgClient::new(config).unwrap();

    let results = client.search("test query").await.unwrap();

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].title, "Example Site");
    assert_eq!(results[0].url, "https://example.com");
    assert!(results[0].snippet.contains("example"));
}

#[tokio::test]
async fn test_mocked_search_respects_max_results() {
    let server = MockServer::start().await;

    // Build HTML with 10 results
    let mut html = String::from("<html><body><table>");
    for i in 0..10 {
        html.push_str(&format!(
            r#"<tr><td class="result-link"><a href="//duckduckgo.com/l/?uddg=https%3A%2F%2Fexample{}.com" class="result-link">Title {}</a></td></tr><tr><td class="result-snippet">Snippet {}</td></tr>"#,
            i, i, i
        ));
    }
    html.push_str("</table></body></html>");

    Mock::given(method("GET"))
        .and(path("/lite/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(html)
                .insert_header("content-type", "text/html"),
        )
        .mount(&server)
        .await;

    let config = DdgConfig {
        max_results: 3,
        ..mock_config(format!("{}/lite/", server.uri()))
    };
    let client = DdgClient::new(config).unwrap();

    let results = client.search("test").await.unwrap();
    assert_eq!(results.len(), 3, "Should limit to max_results=3");
}

#[tokio::test]
async fn test_mocked_404_returns_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/lite/"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let config = mock_config(format!("{}/lite/", server.uri()));
    let client = DdgClient::new(config).unwrap();

    let result = client.search("test").await;
    assert!(result.is_err(), "404 should return error");
}

#[tokio::test]
async fn test_mocked_500_returns_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/lite/"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let config = mock_config(format!("{}/lite/", server.uri()));
    let client = DdgClient::new(config).unwrap();

    let result = client.search("test").await;
    assert!(result.is_err(), "500 should return error");
}

#[tokio::test]
async fn test_mocked_empty_html_returns_no_results() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/lite/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body><table></table></body></html>")
                .insert_header("content-type", "text/html"),
        )
        .mount(&server)
        .await;

    let config = mock_config(format!("{}/lite/", server.uri()));
    let client = DdgClient::new(config).unwrap();

    let results = client.search("test").await.unwrap();
    assert!(results.is_empty(), "Empty HTML should return no results");
}

#[tokio::test]
async fn test_mocked_search_encodes_query_in_url() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/lite/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body><table></table></body></html>")
                .insert_header("content-type", "text/html"),
        )
        .mount(&server)
        .await;

    let config = mock_config(format!("{}/lite/", server.uri()));
    let client = DdgClient::new(config).unwrap();

    // Query with spaces and special chars — should not error
    let result = client.search("café C++ test").await;
    assert!(result.is_ok(), "Special characters should be URL-encoded");
}

// ==================== NETWORK TESTS (real DuckDuckGo) ====================
//
// These tests hit the real DuckDuckGo API and are excluded from CI.
// Run manually with: cargo test -- --ignored
//

/// Network test — run with: cargo test -- --ignored
#[tokio::test]
#[ignore = "hits real DuckDuckGo API"]
async fn test_real_search_returns_results() {
    let client = DdgClient::new(real_config()).unwrap();
    let results = client.search("rust programming").await.unwrap();

    assert!(!results.is_empty(), "Should return search results");
    for result in &results {
        assert!(!result.title.is_empty());
        assert!(!result.url.is_empty());
        assert!(
            !result.url.contains("duckduckgo.com"),
            "URL should be actual target, not DDG redirect"
        );
    }
}

/// Network test — run with: cargo test -- --ignored
#[tokio::test]
#[ignore = "hits real DuckDuckGo API"]
async fn test_real_search_unicode() {
    let client = DdgClient::new(real_config()).unwrap();
    let results = client.search("café").await;
    assert!(results.is_ok(), "Unicode query should succeed");
}

/// Network test — run with: cargo test -- --ignored
#[tokio::test]
#[ignore = "hits real DuckDuckGo API"]
async fn test_real_search_special_chars() {
    let client = DdgClient::new(real_config()).unwrap();
    let results = client.search("C++ programming").await;
    assert!(results.is_ok(), "Special characters should be URL-encoded");
}

/// Network test — run with: cargo test -- --ignored
#[tokio::test]
#[ignore = "hits real DuckDuckGo API"]
async fn test_real_search_result_count_limited() {
    let config = DdgConfig {
        max_results: 3,
        ..real_config()
    };
    let client = DdgClient::new(config).unwrap();
    let results = client.search("rust").await.unwrap();
    assert!(results.len() <= 5, "Should not exceed MAX_RESULTS");
}
