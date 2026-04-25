# MCP WebSearch Server 🔍

[![CI](https://img.shields.io/github/actions/workflow/status/freeman/mcp-websearch/ci.yml?branch=main&label=CI&logo=github)](https://github.com/freeman/mcp-websearch/actions)
[![Coverage](https://img.shields.io/codecov/c/github/freeman/mcp-websearch?label=coverage&logo=codecov)](https://codecov.io/gh/freeman/mcp-websearch)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85+-orange.svg?logo=rust)](https://www.rust-lang.org)

A Rust MCP (Model Context Protocol) server for web search using DuckDuckGo Lite.

## Features ✨

- 🔍 **Web Search** - Search DuckDuckGo via MCP tools
- ⚡ **Fast** - Async Rust with efficient HTML parsing
- 🔒 **Privacy** - Uses DuckDuckGo (no tracking)
- 🌐 **MCP Compliant** - Works with any MCP client
- 🛠️ **Well Tested** - 111+ tests across unit, integration, and E2E

## Installation 📦

### From Source
```bash
git clone https://github.com/freeman/mcp-websearch.git
cd mcp-websearch
cargo build --release
```

The binary will be at `target/release/mcp-websearch`.

## Usage 🚀

### Stdio Transport (Default)
```bash
mcp-websearch --transport stdio
```

### HTTP Transport
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
user_agent = "Mozilla/5.0 (compatible; Kai-MCP/1.0)"

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

## MCP Tool 🔧

### web_search

Search the web for current information.

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

## Testing 🧪

### Quick Start
```bash
cargo test
```

### Test Structure
- **Unit Tests**: `cargo test --lib`
- **Integration Tests**: `cargo test --test integration`
- **E2E Tests**: `cargo test --test e2e`

### Coverage
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --all-features --out Html
```

See [TESTING.md](TESTING.md) for comprehensive testing documentation.

## Architecture 🏗️

```
┌─────────────────────────────────────┐
│         MCP WebSearch Server        │
├─────────────────────────────────────┤
│  ┌──────────┐     ┌─────────────┐  │
│  │  Config  │────▶│  WebSearch  │  │
│  │  (Fig)  │     │    Tool     │  │
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

## Development 🛠️

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

## CI/CD 🔄

Automated testing on every push/PR:
- **Format**: `cargo fmt --check`
- **Lint**: `cargo clippy -- -D warnings`
- **Test**: Ubuntu + macOS matrix
- **Build**: Release artifacts
- **Coverage**: Tarpaulin + Codecov

## Contributing 🤝

1. Fork the repository
2. Create a feature branch
3. Write tests for your changes
4. Ensure CI passes
5. Submit a pull request

See [TESTING.md](TESTING.md) for testing guidelines.

## License 📄

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments 🙏

- Built with [rmcp](https://github.com/modelcontextprotocol/rust-sdk)
- Inspired by DuckDuckGo's privacy-first search
- Created by Freeman & Guan 🪷

---

**Questions?** Open an issue or see our documentation.
