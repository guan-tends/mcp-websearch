//! Mock DuckDuckGo server for testing.
//!
//! Uses wiremock to simulate DDG Lite responses.
//! Currently used by the mock tests in `ddg_client.rs`.

use wiremock::matchers::{method, path_regex};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Mock DDG Lite server for testing.
pub struct MockDDGServer {
    pub server: MockServer,
}

impl MockDDGServer {
    /// Start a new mock DDG server.
    pub async fn new() -> Self {
        Self {
            server: MockServer::start().await,
        }
    }

    /// Get the base URL of the mock server.
    pub fn base_url(&self) -> String {
        self.server.uri()
    }

    /// Mock a successful search response.
    pub async fn mock_search_success(&self, html_response: &str) {
        Mock::given(method("GET"))
            .and(path_regex("/lite/.*"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(html_response)
                    .insert_header("content-type", "text/html"),
            )
            .mount(&self.server)
            .await;
    }

    /// Mock a 404 not found response.
    pub async fn mock_not_found(&self) {
        Mock::given(method("GET"))
            .and(path_regex("/lite/.*"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&self.server)
            .await;
    }

    /// Mock a 500 server error.
    pub async fn mock_server_error(&self) {
        Mock::given(method("GET"))
            .and(path_regex("/lite/.*"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&self.server)
            .await;
    }

    /// Mock an empty result set.
    pub async fn mock_empty_results(&self) {
        let empty_html = r#"<html><body><table></table></body></html>"#;
        Mock::given(method("GET"))
            .and(path_regex("/lite/.*"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(empty_html)
                    .insert_header("content-type", "text/html"),
            )
            .mount(&self.server)
            .await;
    }
}
