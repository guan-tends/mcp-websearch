//! Error types for MCP WebSearch.
//!
//! Error variants marked `#[allow(dead_code)]` are reserved for future
/// functionality (rate-limit handling, response validation, regex diagnostics)
/// and are intentionally kept to maintain a complete error taxonomy.
use thiserror::Error;

/// Top-level errors for the web search system.
///
/// Variants `InvalidResponse` and `RateLimited` are reserved for future
/// use (DDG API hardening, rate-limit detection) and intentionally kept.
#[derive(Error, Debug)]
pub enum WebSearchError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] ParseError),

    /// Reserved for future response-validation logic.
    #[allow(dead_code)]
    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    /// Reserved for future rate-limit detection.
    #[allow(dead_code)]
    #[error("Rate limited")]
    RateLimited,
}

/// Errors during HTML parsing and URL decoding.
///
/// Variants `Regex` and `InvalidHtml` are reserved for future diagnostic
/// enhancements and intentionally kept.
#[derive(Error, Debug)]
pub enum ParseError {
    /// Reserved for future regex diagnostics.
    #[allow(dead_code)]
    #[error("Regex match failed: {0}")]
    Regex(String),

    /// Reserved for future HTML validation.
    #[allow(dead_code)]
    #[error("Invalid HTML: {0}")]
    InvalidHtml(String),

    #[error("Incomplete percent encoding")]
    IncompletePercent,

    #[error("Invalid UTF-8 sequence")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),

    #[error("Invalid hex character: {0}")]
    InvalidHex(String),
}
