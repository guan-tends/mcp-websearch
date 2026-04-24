//! DuckDuckGo search module

pub mod client;
pub mod parser;

pub use client::DdgClient;
pub use parser::{SearchResult, DdgRegex, MAX_RESULTS};
