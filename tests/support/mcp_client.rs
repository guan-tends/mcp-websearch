//! MCP Test Client
//!
//! Spawns MCP server as subprocess and communicates via stdio

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::time::Duration;
use tokio::time::timeout;

/// MCP protocol message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpMessage {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
}

impl McpMessage {
    /// Create a request message
    pub fn request(id: u64, method: &str, params: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: Some(id),
            method: Some(method.to_string()),
            params: Some(params),
            result: None,
            error: None,
        }
    }

    /// Create a notification message (no id)
    pub fn notification(method: &str, params: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: None,
            method: Some(method.to_string()),
            params: Some(params),
            result: None,
            error: None,
        }
    }
}

/// MCP test client that spawns server as subprocess
pub struct McpTestClient {
    child: Child,
    stdin: ChildStdin,
    reader: BufReader<std::process::ChildStdout>,
    next_id: u64,
}

impl McpTestClient {
    /// Spawn the MCP server and create client
    pub async fn new() -> anyhow::Result<Self> {
        // Build the server first
        let status = tokio::process::Command::new("cargo")
            .args(["build", "--release"])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .status()
            .await?;

        if !status.success() {
            anyhow::bail!("Failed to build MCP server");
        }

        let server_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target/release/mcp-websearch");

        let mut child = Command::new(server_path)
            .arg("--transport")
            .arg("stdio")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null()) // Silence logs in test mode
            .spawn()?;

        let stdin = child.stdin.take().expect("Failed to get stdin");
        let stdout = child.stdout.take().expect("Failed to get stdout");
        let reader = BufReader::new(stdout);

        Ok(Self {
            child,
            stdin,
            reader,
            next_id: 1,
        })
    }

    /// Send a request and wait for response
    pub async fn request(&mut self, method: &str, params: Value) -> anyhow::Result<McpMessage> {
        let id = self.next_id;
        self.next_id += 1;

        let msg = McpMessage::request(id, method, params);
        self.send(&msg).await?;

        // Wait for response with matching id
        loop {
            let response = self.receive().await?;
            if response.id == Some(id) {
                return Ok(response);
            }
        }
    }

    /// Send a notification (no response expected)
    pub async fn notification(&mut self, method: &str, params: Value) -> anyhow::Result<()> {
        let msg = McpMessage::notification(method, params);
        self.send(&msg).await
    }

    /// Send a message to the server
    async fn send(&mut self, msg: &McpMessage) -> anyhow::Result<()> {
        let json = serde_json::to_string(msg)?;
        let line = format!("{}\n", json);
        self.stdin.write_all(line.as_bytes())?;
        self.stdin.flush()?;
        Ok(())
    }

    /// Receive a message from the server
    async fn receive(&mut self) -> anyhow::Result<McpMessage> {
        let mut line = String::new();

        // Use tokio's async read_line with timeout
        let result = timeout(Duration::from_secs(5), async {
            self.reader.read_line(&mut line)?;
            Ok::<_, anyhow::Error>(line)
        })
        .await;

        match result {
            Ok(Ok(line)) => {
                let msg: McpMessage = serde_json::from_str(&line)?;
                Ok(msg)
            }
            Ok(Err(e)) => Err(e),
            Err(_) => anyhow::bail!("Timeout waiting for MCP response"),
        }
    }

    /// Perform full initialize handshake
    pub async fn initialize(&mut self) -> anyhow::Result<()> {
        // Send initialize request
        let params = serde_json::json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "test-client",
                "version": "1.0.0"
            }
        });

        let response = self.request("initialize", params).await?;
        if response.error.is_some() {
            anyhow::bail!("Initialize failed: {:?}", response.error);
        }

        // Send initialized notification
        self.notification("notifications/initialized", serde_json::json!({}))
            .await?;

        Ok(())
    }
}

impl Drop for McpTestClient {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
