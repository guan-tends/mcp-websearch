//! DuckDuckGo HTTP client

use crate::{
    config::DdgConfig,
    ddg::parser::{DdgRegex, SearchResult, encode_url_query_component, parse_results},
    error::WebSearchError,
};
use reqwest::Client;
use std::time::Duration;

/// DuckDuckGo Lite client
#[derive(Debug, Clone)]
pub struct DdgClient {
    http_client: Client,
    config: DdgConfig,
    regex: DdgRegex,
}

impl DdgClient {
    /// Create a new DDG client with the given configuration
    pub fn new(config: DdgConfig) -> Result<Self, WebSearchError> {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(config.timeout))
            .user_agent(&config.user_agent)
            .build()?;

        Ok(Self {
            http_client,
            config,
            regex: DdgRegex::default(),
        })
    }

    /// Search DuckDuckGo Lite for the given query
    ///
    /// Returns up to MAX_RESULTS (5) search results
    pub async fn search(&self, query: &str) -> Result<Vec<SearchResult>, WebSearchError> {
        let encoded = encode_url_query_component(query);
        let url = format!("{}?q={}", self.config.base_url, encoded);

        let response = self.http_client.get(&url).send().await?;

        // Check for non-200 status
        if !response.status().is_success() {
            return Err(WebSearchError::Http(
                response.error_for_status().unwrap_err(),
            ));
        }

        let html = response.text().await?;
        let results = parse_results(&html, &self.regex, self.config.max_results)?;

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ddg_client_creation() {
        let config = DdgConfig {
            timeout: 15,
            max_results: 5,
            user_agent: "Test".to_string(),
            base_url: "https://lite.duckduckgo.com/lite/".to_string(),
        };

        let client = DdgClient::new(config);
        assert!(client.is_ok());
    }
}
