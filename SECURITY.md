# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability in mcp-websearch, please report it responsibly.

**Do not open a public GitHub issue for security vulnerabilities.**

Instead, please email the maintainers directly. Include:
- A description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

You will receive a response within 48 hours.

## Scope

This project scrapes DuckDuckGo Lite HTML pages. It does not:
- Store search queries or results
- Send telemetry or analytics
- Require API keys or credentials
- Make outbound requests beyond DuckDuckGo (or a configured `base_url`)

## Security Considerations

- **No tracking**: DuckDuckGo is used specifically for its privacy properties
- **No storage**: Search results are returned to the MCP client and not persisted
- **Configurable endpoint**: The `base_url` can be pointed at a self-hosted mirror for air-gapped environments
- **rustls**: Pure Rust TLS implementation — no OpenSSL dependency, reducing attack surface

## Known Vulnerabilities

- **rmcp 0.16.0 — DNS rebinding (RUSTSEC-2026-0189)**: Affects the Streamable HTTP
  server transport only. The stdio transport (primary) is not affected. The HTTP
  transport is marked experimental. Fix: upgrade to rmcp >=1.4.0 (planned for v1.1.0).
