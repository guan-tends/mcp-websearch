# MCP WebSearch Server

[![CI](https://img.shields.io/github/actions/workflow/status/guan-tends/mcp-websearch/ci.yml?branch=main&label=CI&logo=github)](https://github.com/guan-tends/mcp-websearch/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85+-orange.svg?logo=rust)](https://www.rust-lang.org)

A Rust MCP (Model Context Protocol) server for web search using DuckDuckGo Lite.

## Features

- **Web Search** — Search DuckDuckGo via MCP tools
- **Fast** — Async Rust with efficient HTML parsing
- **Privacy** — Uses DuckDuckGo (no tracking, no API key required)
- **MCP Compliant** — Works with any MCP client
- **Well Tested** — 111+ tests across unit, integration, and E2E

## Installation

### From crates.io (recommended)

```bash
cargo install mcp-websearch
```

### From Source

```bash
git clone https://github.com/guan-tends/mcp-websearch.git
cd mcp-websearch
cargo build --release
```

The binary will be at `target/release/mcp-websearch`.

## Usage

### Stdio Transport (Default)

```bash
mcp-websearch --transport stdio
```

### HTTP Transport (Experimental)

The HTTP transport is currently experimental and accepts TCP connections.
Full StreamableHTTP implementation is planned for a future release.
For now, use `--transport stdio` for full functionality.

```bash
mcp-websearch --transport http --host 127.0.0.1 --port 3000
```

### Configuration File

Create `config.toml`:

```toml
[transport]
mode = "stdio"

[transport.http]
host = "127.0.0.1"
port = 3000

[ddg]
timeout = 15
max_results = 5
user_agent = "Mozilla/5.0 (compatible; mcp-websearch/1.0)"
base_url = "https://lite.duckduckgo.com/lite/"

[logging]
level = "info"
format = "json"
```

Then run:

```bash
mcp-websearch --config config.toml
```

### Environment Variables

```bash
MCP_TRANSPORT_MODE=stdio
MCP_DDG_TIMEOUT=15
MCP_DDG_MAX_RESULTS=5
MCP_LOGGING_LEVEL=info
```

## MCP Tool

### web_search

Search the web for current information. Returns titles, URLs, and snippets.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "query": {
      "type": "string",
      "description": "The search query"
    }
  },
  "required": ["query"]
}
```

**Example Usage:**

```json
{
  "name": "web_search",
  "arguments": {
    "query": "Rust programming language"
  }
}
```

**Response:**

```json
{
  "success": true,
  "results": [
    {
      "title": "Rust Programming Language",
      "url": "https://www.rust-lang.org",
      "snippet": "A language empowering everyone..."
    }
  ],
  "message": "Found 5 results"
}
```

## Testing

### Quick Start

```bash
cargo test
```

Network-dependent tests (which hit real DuckDuckGo) are marked `#[ignore]`
and excluded from the default test run. Run them explicitly:

```bash
cargo test -- --ignored
```

### Test Structure

- **Unit Tests** — `tests/unit/` — config, parser, client, error, tool
- **Integration Tests** — `tests/integration/` — MCP protocol, stdio transport
- **E2E Tests** — `tests/e2e/` — full search workflows

See [TESTING.md](TESTING.md) for comprehensive testing documentation.

## Architecture

```
┌─────────────────────────────────────┐
│         MCP WebSearch Server        │
├─────────────────────────────────────┤
│  ┌──────────┐     ┌─────────────┐  │
│  │  Config  │────▶│  WebSearch  │  │
│  │ (Figment)│     │    Tool     │  │
│  └──────────┘     └──────┬──────┘  │
│                          │         │
│                   ┌──────▼──────┐  │
│                   │  DDG Client │  │
│                   └──────┬──────┘  │
│                          │         │
│                   ┌──────▼──────┐  │
│                   │   Parser    │  │
│                   └─────────────┘  │
├─────────────────────────────────────┤
│        MCP Transport (stdio)        │
└─────────────────────────────────────┘
```

## Development

### Prerequisites

- Rust 1.85+
- cargo

### Build

```bash
cargo build
cargo build --release
```

### Lint

```bash
cargo fmt
cargo clippy -- -D warnings
```

### Run

```bash
cargo run -- --transport stdio
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Write tests for your changes
4. Ensure CI passes
5. Submit a pull request

## License

MIT License — see [LICENSE](LICENSE) for details.

## Support

If this project is useful to you, consider supporting development:

- **EVM**: `0x2733ff7c865C56d565a99BE1DC11B81cc76850A5`
- **Solana**: `Eu8wQcW68TKMs1a6eqzZu8znzU52QLqQugAMG8uCD6y6`
- **XRP**: `r4X6e7McAQj7e8vBCeued1RYu4mCJrREDG`

## Acknowledgments

- Built with [rmcp](https://github.com/modelcontextprotocol/rust-sdk)
- Powered by DuckDuckGo's privacy-first search

---

Crafted with ❤️ by [Sage Labs](https://sagelabs.dev)
