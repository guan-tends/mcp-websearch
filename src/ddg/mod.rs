//! DuckDuckGo search module

pub mod client;
pub mod parser;

pub use client::DdgClient;
pub use parser::{DdgRegex, MAX_RESULTS, SearchResult};
