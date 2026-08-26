//! DuckDuckGo search module

pub mod client;
pub mod parser;

pub use client::DdgClient;
pub use parser::SearchResult;

// Re-exported for test access (used via glob imports in test files).
#[cfg(test)]
#[allow(unused_imports)]
pub use parser::{DdgRegex, MAX_RESULTS};
