//! Error types for MCP WebSearch

use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebSearchError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("Parse error: {0}")]
    Parse(#[from] ParseError),
    
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    
    #[error("Rate limited")]
    RateLimited,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Regex match failed: {0}")]
    Regex(String),
    
    #[error("Invalid HTML: {0}")]
    InvalidHtml(String),
    
    #[error("Incomplete percent encoding")]
    IncompletePercent,
    
    #[error("Invalid UTF-8 sequence")]
    InvalidUtf8(std::string::FromUtf8Error),
    
    #[error("Invalid hex character: {0}")]
    InvalidHex(String),
}
