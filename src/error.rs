//! Error types for MCP WebSearch server

use thiserror::Error;

/// Main error type for WebSearch operations
#[derive(Error, Debug)]
pub enum WebSearchError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("URL parsing failed: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("DuckDuckGo parsing failed: {0}")]
    Parse(#[from] ParseError),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Parsing-specific errors
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Missing href attribute in link element")]
    MissingHref,

    #[error("Invalid URL redirect format")]
    InvalidRedirect,

    #[error("Invalid hex in percent-encoding: {0}")]
    InvalidHex(String),

    #[error("Incomplete percent-encoding sequence")]
    IncompletePercent,

    #[error("Invalid UTF-8 sequence: {0}")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),

    #[error("No results found in HTML")]
    NoResults,
}

/// Configuration errors
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to load configuration: {0}")]
    Load(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid value for {field}: {value}")]
    InvalidValue { field: String, value: String },
}
