//! Mock DuckDuckGo Server
//!
//! Uses wiremock to simulate DDG Lite responses for testing

use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path_regex};

/// Mock DDG Lite server for testing
pub struct MockDDGServer {
    server: MockServer,
}

impl MockDDGServer {
    /// Start a new mock DDG server
    pub async fn new() -> Self {
        let server = MockServer::start().await;
        Self { server }
    }

    /// Get the base URL of the mock server
    pub fn base_url(&self) -> String {
        self.server.uri()
    }

    /// Mock a successful search response
    pub async fn mock_search_success(&self, html_response: &str) {
        Mock::given(method("GET"))
            .and(path_regex("/lite/.*"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_string(html_response)
                .insert_header("content-type", "text/html"))
            .mount(&self.server)
            .await;
    }

    /// Mock a 404 not found response
    pub async fn mock_not_found(&self) {
        Mock::given(method("GET"))
            .and(path_regex("/lite/.*"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&self.server)
            .await;
    }

    /// Mock a 500 server error
    pub async fn mock_server_error(&self) {
        Mock::given(method("GET"))
            .and(path_regex("/lite/.*"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&self.server)
            .await;
    }

    /// Mock an empty result set
    pub async fn mock_empty_results(&self) {
        let empty_html = r#"<html><body><table></table></body></html>"#;
        Mock::given(method("GET"))
            .and(path_regex("/lite/.*"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_string(empty_html)
                .insert_header("content-type", "text/html"))
            .mount(&self.server)
            .await;
    }

    /// Mock a timeout (no response)
    pub async fn mock_timeout(&self) {
        // Don't mount any mock - requests will timeout
    }
}

/// Sample DDG HTML response with 3 results
pub fn sample_ddg_response() -> &'static str {
    r#"<html>
<head><title>Search Results</title></head>
<body>
<table>
    <tr>
        <td class="result-link">
            <a href="//duckduckgo.com/l/?uddg=https%3A%2F%2Fwww.rust-lang.org" class="result-link">Rust Programming Language</a>
        </td>
    </tr>
    <tr>
        <td class="result-snippet">A systems programming language that runs blazingly fast.</td>
    </tr>
    <tr>
        <td class="result-link">
            <a href="//duckduckgo.com/l/?uddg=https%3A%2F%2Fgithub.com%2Frust-lang%2Frust" class="result-link">GitHub - rust-lang/rust</a>
        </td>
    </tr>
    <tr>
        <td class="result-snippet">The Rust programming language repository.</td>
    </tr>
    <tr>
        <td class="result-link">
            <a href="//duckduckgo.com/l/?uddg=https%3A%2F%2Fdoc.rust-lang.org" class="result-link">The Rust Programming Language - Documentation</a>
        </td>
    </tr>
    <tr>
        <td class="result-snippet">Official Rust documentation and learning resources.</td>
    </tr>
</table>
</body>
</html>"#
}
