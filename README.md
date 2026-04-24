# MCP WebSearch Server

MCP (Model Context Protocol) server for web search using DuckDuckGo Lite.

Ported from the Kotlin WebSearchTool with exact regex patterns and UTF-8 fix.

## Features

- **Dual transport**: stdio (for local clients) and HTTP POST/JSON (for remote)
- **Exact compatibility**: Uses same regex patterns as original Kotlin tool
- **UTF-8 fix**: Proper multi-byte character handling in URL decoding
- **Production ready**: Structured logging, error handling, configuration

## Usage

### Stdio mode (default)

```bash
mcp-websearch
```

### HTTP mode

```bash
mcp-websearch --transport http --port 3000
```

### Environment variables

```bash
MCP_TRANSPORT=http
MCP_HOST=0.0.0.0
MCP_PORT=3000
RUST_LOG=info
```

## Tool: web_search

Searches DuckDuckGo Lite and returns titles, URLs, and snippets.

## License

MIT
