//! Unit tests for Error module
//!
//! Error types, conversions, display formatting, propagation

use mcp_websearch::error::{WebSearchError, ParseError};

// ==================== WEBSEARCH ERROR TESTS ====================

#[test]
fn test_websearch_error_http_display() {
    // Create a reqwest error for testing
    let inner = reqwest::Error::from(std::io::Error::new(
        std::io::ErrorKind::Other,
        "connection refused"
    ));
    let err = WebSearchError::Http(inner);
    
    let msg = format!("{}", err);
    assert!(msg.contains("HTTP request failed"));
}

#[test]
fn test_websearch_error_parse_display() {
    let inner = ParseError::InvalidHtml("malformed tag".to_string());
    let err = WebSearchError::Parse(inner);
    
    let msg = format!("{}", err);
    assert!(msg.contains("Parse error"));
}

#[test]
fn test_websearch_error_invalid_response_display() {
    let err = WebSearchError::InvalidResponse("server returned garbage".to_string());
    
    let msg = format!("{}", err);
    assert!(msg.contains("Invalid response"));
    assert!(msg.contains("garbage"));
}

#[test]
fn test_websearch_error_rate_limited_display() {
    let err = WebSearchError::RateLimited;
    
    let msg = format!("{}", err);
    assert_eq!(msg, "Rate limited");
}

// ==================== PARSE ERROR TESTS ====================

#[test]
fn test_parse_error_regex_display() {
    let err = ParseError::Regex("pattern failed".to_string());
    
    let msg = format!("{}", err);
    assert!(msg.contains("Regex match failed"));
    assert!(msg.contains("pattern failed"));
}

#[test]
fn test_parse_error_invalid_html_display() {
    let err = ParseError::InvalidHtml("unclosed tag".to_string());
    
    let msg = format!("{}", err);
    assert!(msg.contains("Invalid HTML"));
    assert!(msg.contains("unclosed tag"));
}

#[test]
fn test_parse_error_incomplete_percent_display() {
    let err = ParseError::IncompletePercent;
    
    let msg = format!("{}", err);
    assert_eq!(msg, "Incomplete percent encoding");
}

#[test]
fn test_parse_error_invalid_utf8_display() {
    let bytes = vec![0x80, 0x81, 0x82];
    let utf8_err = String::from_utf8(bytes).unwrap_err();
    let err = ParseError::InvalidUtf8(utf8_err);
    
    let msg = format!("{}", err);
    assert!(msg.contains("Invalid UTF-8 sequence"));
}

#[test]
fn test_parse_error_invalid_hex_display() {
    let err = ParseError::InvalidHex("GG".to_string());
    
    let msg = format!("{}", err);
    assert!(msg.contains("Invalid hex character"));
    assert!(msg.contains("GG"));
}

// ==================== ERROR CONVERSION TESTS ====================

#[test]
fn test_parse_error_from_utf8_error() {
    let bytes = vec![0xC0, 0x80]; // Invalid UTF-8 sequence
    let utf8_result = String::from_utf8(bytes);
    assert!(utf8_result.is_err());
    
    // Can convert to ParseError
    let parse_result: Result<String, ParseError> = utf8_result.map_err(ParseError::from);
    assert!(parse_result.is_err());
}

// ==================== ERROR PROPAGATION TESTS ====================

#[test]
fn test_question_mark_operator_with_parse_error() {
    fn may_fail() -> Result<String, ParseError> {
        let bytes = vec![0xFF]; // Invalid UTF-8
        let s = String::from_utf8(bytes)?;
        Ok(s)
    }
    
    let result = may_fail();
    assert!(result.is_err());
    
    match result {
        Err(ParseError::InvalidUtf8(_)) => (), // Expected
        _ => panic!("Should be InvalidUtf8 error"),
    }
}

#[test]
fn test_error_chaining() {
    // Test that errors can be chained through From impls
    fn inner() -> Result<String, ParseError> {
        Err(ParseError::InvalidHtml("bad".to_string()))
    }
    
    fn outer() -> Result<String, WebSearchError> {
        let s = inner()?;
        Ok(s)
    }
    
    let result = outer();
    assert!(result.is_err());
    
    match result {
        Err(WebSearchError::Parse(ParseError::InvalidHtml(msg))) => {
            assert_eq!(msg, "bad");
        }
        _ => panic!("Should be Parse->InvalidHtml error"),
    }
}

// ==================== DEBUG FORMAT TESTS ====================

#[test]
fn test_websearch_error_debug_format() {
    let err = WebSearchError::RateLimited;
    let debug = format!("{:?}", err);
    assert!(debug.contains("RateLimited"));
}

#[test]
fn test_parse_error_debug_format() {
    let err = ParseError::IncompletePercent;
    let debug = format!("{:?}", err);
    assert!(debug.contains("IncompletePercent"));
}
