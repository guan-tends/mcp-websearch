//! Transport implementations for MCP WebSearch

pub mod http;
pub mod stdio;

pub use http::serve as serve_http;
pub use stdio::serve as serve_stdio;
