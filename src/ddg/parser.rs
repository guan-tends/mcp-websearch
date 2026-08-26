//! DuckDuckGo HTML parser
//!
//! Ported from Kotlin WebSearchTool with exact regex patterns
//! UTF-8 decoding bug FIXED: accumulates bytes before UTF-8 decode

use crate::error::ParseError;
use regex::Regex;

/// Default maximum results to return.
///
/// Used as the default value for `DdgConfig.max_results` and as a
/// reference constant in tests. The actual limit at runtime is
/// determined by the `max_results` parameter passed to `parse_results`.
#[allow(dead_code)]
pub const MAX_RESULTS: usize = 5;

/// Regex patterns (exact from Kotlin WebSearchTool.kt)
#[derive(Debug, Clone)]
pub struct DdgRegex {
    /// `<a[^>]+class=['"]result-link['"][^>]*>([\s\S]*?)</a>`
    pub link_regex: Regex,
    /// `href=['"]([^'"]*?)['"]`
    pub href_regex: Regex,
    /// `<td[^>]+class=['"]result-snippet['"][^>]*>([\s\S]*?)</td>`
    pub snippet_regex: Regex,
    /// `<a\s[^>]*class=['"]result-link['"][^>]*>`
    pub full_link_regex: Regex,
    /// `uddg=([^&]+)`
    pub uddg_regex: Regex,
}

impl Default for DdgRegex {
    fn default() -> Self {
        Self {
            link_regex: Regex::new(r#"<a[^>]+class=['"]result-link['"][^>]*>([\s\S]*?)</a>"#)
                .unwrap(),
            href_regex: Regex::new(r#"href=['"]([^'"]*?)['"]"#).unwrap(),
            snippet_regex: Regex::new(
                r#"<td[^>]+class=['"]result-snippet['"][^>]*>([\s\S]*?)</td>"#,
            )
            .unwrap(),
            full_link_regex: Regex::new(r#"<a\s[^>]*class=['"]result-link['"][^>]*>"#).unwrap(),
            uddg_regex: Regex::new(r#"uddg=([^&]+)"#).unwrap(),
        }
    }
}

/// Single search result
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
}

/// Parse DDG Lite HTML into structured results
///
/// Algorithm ported exactly from Kotlin:
/// 1. Find all link tags, links, and snippets via regex
/// 2. Iterate by index alignment
/// 3. Extract href, title, snippet
/// 4. Strip HTML, decode URLs, filter empty
/// 5. Cap at MAX_RESULTS
pub fn parse_results(
    html: &str,
    regex: &DdgRegex,
    max_results: usize,
) -> Result<Vec<SearchResult>, ParseError> {
    let mut results = Vec::new();

    // Find all matches (like Kotlin's findAll)
    let link_tags: Vec<_> = regex.full_link_regex.find_iter(html).collect();
    let links: Vec<_> = regex.link_regex.captures_iter(html).collect();
    let snippets: Vec<_> = regex.snippet_regex.captures_iter(html).collect();

    // Iterate by index alignment (fragile but matches Kotlin behavior)
    for i in 0..links.len() {
        if results.len() >= max_results {
            break;
        }

        // Get link tag (for href extraction)
        let link_tag = link_tags.get(i).map(|m| m.as_str()).unwrap_or("");
        if link_tag.is_empty() {
            continue;
        }

        // Extract href from link tag
        let href = regex
            .href_regex
            .captures(link_tag)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str())
            .unwrap_or("");
        if href.is_empty() {
            continue;
        }

        // Extract title from links capture group 1
        let title = links
            .get(i)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str())
            .unwrap_or("")
            .strip_html()
            .trim()
            .to_string();
        if title.is_empty() {
            continue;
        }

        // Extract snippet (optional, may be empty)
        let snippet = snippets
            .get(i)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str())
            .unwrap_or("")
            .strip_html()
            .trim()
            .to_string();

        // Extract actual URL from DDG redirect
        let url = extract_url_from_redirect(href)?;

        if !url.is_empty() && !title.is_empty() {
            results.push(SearchResult {
                title,
                url,
                snippet,
            });
        }
    }

    Ok(results)
}

/// Extract actual URL from DDG redirect wrapper
///
/// DDG format: `//duckduckgo.com/l/?uddg=ENCODED_URL`
/// Extract actual URL from DDG redirect wrapper.
///
/// DDG format: `//duckduckgo.com/l/?uddg=ENCODED_URL`.
/// Uses a cached regex to avoid recompilation on every call.
pub fn extract_url_from_redirect(href: &str) -> Result<String, ParseError> {
    static RE: std::sync::OnceLock<DdgRegex> = std::sync::OnceLock::new();
    let regex = RE.get_or_init(DdgRegex::default);

    // Try to extract uddg parameter
    if let Some(cap) = regex.uddg_regex.captures(href) {
        if let Some(encoded) = cap.get(1) {
            return decode_url_component(encoded.as_str());
        }
    }

    // Not a redirect — use as-is.
    // Handle protocol-relative URLs (//example.com).
    if href.starts_with("//") {
        return Ok(format!("https:{}", href));
    }

    Ok(href.to_string())
}

/// **FIXED** URL decoder with proper UTF-8 handling
///
/// Kotlin original had bug: decoded each %XX byte individually as char
/// This version: accumulates bytes, then UTF-8 decodes
pub fn decode_url_component(encoded: &str) -> Result<String, ParseError> {
    let mut bytes = Vec::new();
    let mut chars = encoded.chars();

    while let Some(c) = chars.next() {
        match c {
            '%' => {
                // Read two hex digits
                let hex1 = chars.next().ok_or(ParseError::IncompletePercent)?;
                let hex2 = chars.next().ok_or(ParseError::IncompletePercent)?;

                let hex_str = format!("{}{}", hex1, hex2);
                let byte = u8::from_str_radix(&hex_str, 16)
                    .map_err(|_| ParseError::InvalidHex(hex_str))?;

                bytes.push(byte);
            }
            '+' => {
                bytes.push(b' ');
            }
            c => {
                // For non-ASCII characters, encode as UTF-8 bytes
                let char_bytes = c.encode_utf8(&mut [0; 4]).as_bytes().to_vec();
                bytes.extend(char_bytes);
            }
        }
    }

    // **THE FIX**: Decode entire byte sequence as UTF-8
    String::from_utf8(bytes).map_err(ParseError::InvalidUtf8)
}

/// Encode URL query component (RFC 3986 + space->+)
pub fn encode_url_query_component(query: &str) -> String {
    let mut result = String::with_capacity(query.len() * 3);

    for c in query.chars() {
        match c {
            // RFC 3986 unreserved characters
            c if c.is_ascii_alphanumeric() || "-_.~".contains(c) => {
                result.push(c);
            }
            // Space becomes + (application/x-www-form-urlencoded)
            ' ' => {
                result.push('+');
            }
            // All other characters: UTF-8 encode then percent-encode
            c => {
                let mut buf = [0; 4];
                let encoded = c.encode_utf8(&mut buf);
                for byte in encoded.bytes() {
                    result.push('%');
                    result.push_str(&format!("{:02X}", byte));
                }
            }
        }
    }

    result
}

/// Strip HTML tags and decode common entities
///
/// Ported from Kotlin: 7 specific entity replacements
pub trait StripHtml {
    fn strip_html(&self) -> String;
}

impl StripHtml for str {
    fn strip_html(&self) -> String {
        static RE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
        let tag_regex = RE.get_or_init(|| Regex::new(r"<[^>]*>").unwrap());

        // Remove all HTML tags
        let no_tags = tag_regex.replace_all(self, "");

        // Replace 7 specific entities (in order, matching the original Kotlin implementation)
        no_tags
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&#x27;", "'")
            .replace("&#39;", "'")
            .replace("&nbsp;", " ")
    }
}

impl StripHtml for String {
    fn strip_html(&self) -> String {
        self.as_str().strip_html()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_url_simple() {
        assert_eq!(encode_url_query_component("hello world"), "hello+world");
    }

    #[test]
    fn test_encode_url_special_chars() {
        assert_eq!(encode_url_query_component("a+b"), "a%2Bb");
    }

    #[test]
    fn test_decode_url_simple() {
        assert_eq!(decode_url_component("hello+world").unwrap(), "hello world");
    }

    #[test]
    fn test_decode_url_encoded() {
        assert_eq!(
            decode_url_component("https%3A%2F%2Fexample.com").unwrap(),
            "https://example.com"
        );
    }

    #[test]
    fn test_decode_url_utf8() {
        // This is the UTF-8 fix test!
        // café in UTF-8: 0xC3 0xA9
        assert_eq!(decode_url_component("caf%C3%A9").unwrap(), "café");
    }

    #[test]
    fn test_decode_url_chinese() {
        // Test multi-byte UTF-8
        assert_eq!(decode_url_component("%E4%B8%AD%E6%96%87").unwrap(), "中文");
    }

    #[test]
    fn test_strip_html() {
        assert_eq!("<p>Hello &amp; World</p>".strip_html(), "Hello & World");
    }

    #[test]
    fn test_extract_url_from_redirect() {
        let href = "//duckduckgo.com/l/?uddg=https%3A%2F%2Fexample.com";
        assert_eq!(
            extract_url_from_redirect(href).unwrap(),
            "https://example.com"
        );
    }

    #[test]
    fn test_extract_url_protocol_relative() {
        let href = "//example.com";
        assert_eq!(
            extract_url_from_redirect(href).unwrap(),
            "https://example.com"
        );
    }
}
