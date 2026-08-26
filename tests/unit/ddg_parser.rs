//! Unit tests for DDG Parser module
//!
//! URL encoding/decoding, HTML parsing, edge cases

use mcp_websearch::ddg::SearchResult;
use mcp_websearch::ddg::parser::*;

// ==================== URL ENCODING TESTS ====================

#[test]
fn test_encode_url_simple_space() {
    assert_eq!(encode_url_query_component("hello world"), "hello+world");
}

#[test]
fn test_encode_url_no_special_chars() {
    assert_eq!(encode_url_query_component("hello"), "hello");
    assert_eq!(encode_url_query_component("HelloWorld"), "HelloWorld");
}

#[test]
fn test_encode_url_special_chars() {
    assert_eq!(encode_url_query_component("a+b"), "a%2Bb");
    assert_eq!(encode_url_query_component("100%"), "100%25");
}

#[test]
fn test_encode_url_unicode() {
    // café = caf + UTF-8 é
    assert_eq!(encode_url_query_component("café"), "caf%C3%A9");
}

#[test]
fn test_encode_url_chinese() {
    // 中文 = %E4%B8%AD%E6%96%87
    assert_eq!(encode_url_query_component("中文"), "%E4%B8%AD%E6%96%87");
}

#[test]
fn test_encode_url_emoji() {
    // 🦀 = %F0%9F%A6%80 (crab emoji)
    assert_eq!(encode_url_query_component("🦀"), "%F0%9F%A6%80");
}

#[test]
fn test_encode_url_rfc_unreserved() {
    // RFC 3986 unreserved: A-Za-z0-9 -_.~
    assert_eq!(encode_url_query_component("hello-world"), "hello-world");
    assert_eq!(encode_url_query_component("hello_world"), "hello_world");
    assert_eq!(encode_url_query_component("hello.world"), "hello.world");
    assert_eq!(encode_url_query_component("hello~world"), "hello~world");
}

// ==================== URL DECODING TESTS ====================

#[test]
fn test_decode_url_space_plus() {
    assert_eq!(decode_url_component("hello+world").unwrap(), "hello world");
}

#[test]
fn test_decode_url_percent_encoded() {
    assert_eq!(
        decode_url_component("https%3A%2F%2Fexample.com").unwrap(),
        "https://example.com"
    );
}

#[test]
fn test_decode_url_utf8_multibyte() {
    // café in UTF-8: 0xC3 0xA9
    assert_eq!(decode_url_component("caf%C3%A9").unwrap(), "café");
}

#[test]
fn test_decode_url_chinese() {
    assert_eq!(decode_url_component("%E4%B8%AD%E6%96%87").unwrap(), "中文");
}

#[test]
fn test_decode_url_incomplete_percent_error() {
    let result = decode_url_component("hello%2");
    assert!(result.is_err());
}

#[test]
fn test_decode_url_invalid_hex_error() {
    let result = decode_url_component("hello%GG");
    assert!(result.is_err());
}

#[test]
fn test_decode_url_mixed_content() {
    // Mixed: plain + encoded + UTF-8
    assert_eq!(
        decode_url_component("hello%20world+%C3%A9").unwrap(),
        "hello world é"
    );
}

// ==================== HTML STRIPPING TESTS ====================

#[test]
fn test_strip_html_simple_tags() {
    let html = "<p>Hello World</p>";
    assert_eq!(html.strip_html(), "Hello World");
}

#[test]
fn test_strip_html_with_entities() {
    let html = "Hello &amp; World &lt;script&gt;";
    assert_eq!(html.strip_html(), "Hello & World <script>");
}

#[test]
fn test_strip_html_all_entities() {
    let html = "&amp;&lt;&gt;&quot;&#x27;&#39;&nbsp;";
    // &amp;→& &lt;→< &gt;→> &quot;→" &#x27;→' &#39;→' &nbsp;→(space)
    assert_eq!(html.strip_html(), "&<>\"'' ");
}

#[test]
fn test_strip_html_nested_tags() {
    let html = "<div><p><b>Bold</b> text</p></div>";
    assert_eq!(html.strip_html(), "Bold text");
}

#[test]
fn test_strip_html_empty() {
    assert_eq!("".strip_html(), "");
}

#[test]
fn test_strip_html_no_tags() {
    let text = "Just plain text";
    assert_eq!(text.strip_html(), "Just plain text");
}

// ==================== EXTRACT URL FROM REDIRECT TESTS ====================

#[test]
fn test_extract_url_ddg_redirect() {
    let href = "//duckduckgo.com/l/?uddg=https%3A%2F%2Fexample.com";
    assert_eq!(
        extract_url_from_redirect(href).unwrap(),
        "https://example.com"
    );
}

#[test]
fn test_extract_url_protocol_relative() {
    let href = "//example.com/path";
    assert_eq!(
        extract_url_from_redirect(href).unwrap(),
        "https://example.com/path"
    );
}

#[test]
fn test_extract_url_direct() {
    let href = "https://example.com";
    assert_eq!(
        extract_url_from_redirect(href).unwrap(),
        "https://example.com"
    );
}

#[test]
fn test_extract_url_with_query_params() {
    let href = "//duckduckgo.com/l/?uddg=https%3A%2F%2Fexample.com%3Ffoo%3Dbar";
    assert_eq!(
        extract_url_from_redirect(href).unwrap(),
        "https://example.com?foo=bar"
    );
}

// ==================== PARSE RESULTS TESTS ====================

#[test]
fn test_parse_results_from_fixture() {
    let html = crate::support::load_fixture("ddg_response.html");
    let regex = DdgRegex::default();
    let results = parse_results(&html, &regex, MAX_RESULTS).unwrap();

    assert!(!results.is_empty(), "Should parse results from fixture");

    // Check first result
    let first = &results[0];
    assert_eq!(first.title, "Rust Programming Language");
    assert_eq!(first.url, "https://www.rust-lang.org");
    assert!(first.snippet.contains("empowering"));
}

#[test]
fn test_parse_results_empty_html() {
    let html = "<html><body></body></html>";
    let regex = DdgRegex::default();
    let results = parse_results(html, &regex, MAX_RESULTS).unwrap();
    assert!(results.is_empty());
}

#[test]
fn test_parse_results_max_limit() {
    // HTML with more than MAX_RESULTS (5) results
    let mut html = String::from("<html><body><table>");
    for i in 0..10 {
        html.push_str(&format!(
            r#"<tr><td class="result-link"><a href="//duckduckgo.com/l/?uddg=https%3A%2F%2Fexample{}.com" class="result-link">Title {}</a></td></tr><tr><td class="result-snippet">Snippet {}</td></tr>"#,
            i, i, i
        ));
    }
    html.push_str("</table></body></html>");

    let regex = DdgRegex::default();
    let results = parse_results(&html, &regex, MAX_RESULTS).unwrap();

    assert_eq!(results.len(), MAX_RESULTS, "Should limit to MAX_RESULTS");
}

#[test]
fn test_parse_results_preserves_order() {
    let html = r#"<html><body><table>
        <tr><td class="result-link"><a href="//duckduckgo.com/l/?uddg=https%3A%2F%2Ffirst.com" class="result-link">First</a></td></tr>
        <tr><td class="result-snippet">First snippet</td></tr>
        <tr><td class="result-link"><a href="//duckduckgo.com/l/?uddg=https%3A%2F%2Fsecond.com" class="result-link">Second</a></td></tr>
        <tr><td class="result-snippet">Second snippet</td></tr>
    </table></body></html>"#;

    let regex = DdgRegex::default();
    let results = parse_results(html, &regex, MAX_RESULTS).unwrap();

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].title, "First");
    assert_eq!(results[0].url, "https://first.com");
    assert_eq!(results[1].title, "Second");
    assert_eq!(results[1].url, "https://second.com");
}

// ==================== REGEX COMPILATION TESTS ====================

#[test]
fn test_ddg_regex_compiles() {
    // Should not panic - validates all regex patterns are valid
    let _regex = DdgRegex::default();
}

#[test]
fn test_link_regex_matches() {
    let regex = DdgRegex::default();
    let html = r#"<a class="result-link" href="test">Title</a>"#;
    assert!(regex.link_regex.is_match(html));
}

#[test]
fn test_snippet_regex_matches() {
    let regex = DdgRegex::default();
    let html = r#"<td class="result-snippet">Some snippet text</td>"#;
    assert!(regex.snippet_regex.is_match(html));
}

// ==================== SEARCH RESULT STRUCT TESTS ====================

#[test]
fn test_search_result_equality() {
    let a = SearchResult {
        title: "Title".to_string(),
        url: "https://example.com".to_string(),
        snippet: "Snippet".to_string(),
    };
    let b = SearchResult {
        title: "Title".to_string(),
        url: "https://example.com".to_string(),
        snippet: "Snippet".to_string(),
    };
    assert_eq!(a, b);
}

#[test]
fn test_search_result_clone() {
    let original = SearchResult {
        title: "Title".to_string(),
        url: "https://example.com".to_string(),
        snippet: "Snippet".to_string(),
    };
    let cloned = original.clone();
    assert_eq!(original, cloned);
}
