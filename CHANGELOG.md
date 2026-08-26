# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-08-25

### Added
- DuckDuckGo Lite web search via MCP `web_search` tool
- Stdio transport for local MCP client integration
- HTTP transport (experimental) for remote connectivity
- Configurable `base_url` for mock testing and alternative search endpoints
- Configurable `max_results` flowing through config → client → parser
- Compile-time embedded default config (no filesystem dependency)
- `OnceLock`-based regex caching for hot-path performance
- 106 tests: 81 unit, 15 integration, 10 in-crate, 18 network (ignored by default)
- Wiremock-based mock tests for DDG client (6 scenarios)
- Full config file support via Figment (TOML + env vars + CLI overrides)
- FUNDING.yml with crypto donation addresses

### Changed
- User agent string: `Kai-MCP/1.0` → `mcp-websearch/1.0`
- `parse_results` now takes `max_results` as a parameter instead of using a global constant
- `DdgConfig` canonicalized in `config.rs` (no re-export from parser)
- Error variants `InvalidResponse`, `RateLimited`, `Regex`, `InvalidHtml` reserved with `#[allow(dead_code)]` for future use
- Library + binary crate structure (tests import via `mcp_websearch::*`)

### Removed
- Custom `Trim` trait (replaced with idiomatic `.trim().to_string()`)
- Unused `html_tag_regex` field from `DdgRegex`
- 5 dead fixture files (mcp_*.json, invalid_config.toml)
- Hardcoded absolute config path
