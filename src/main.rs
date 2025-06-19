pub mod angreal;
pub mod mcp;

use anyhow::Result;
use mcp::{JsonRpcRequest, JsonRpcResponse, McpServer};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[tokio::main]
async fn main() -> Result<()> {
    let server = McpServer::new();
    let stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();
    let mut reader = BufReader::new(stdin);

    // Server initialization complete - ready for MCP communication

    let mut line = String::new();

    loop {
        line.clear();

        match reader.read_line(&mut line).await {
            Ok(0) => {
                // EOF received, shutting down gracefully
                break;
            }
            Ok(_) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }

                match serde_json::from_str::<JsonRpcRequest>(trimmed) {
                    Ok(request) => {
                        // Process request

                        match server.handle_request(request).await {
                            Ok(response) => {
                                let response_str = serde_json::to_string(&response)?;
                                stdout.write_all(response_str.as_bytes()).await?;
                                stdout.write_all(b"\n").await?;
                                stdout.flush().await?;
                            }
                            Err(e) => {
                                // Error handling request
                                let error_response = JsonRpcResponse {
                                    jsonrpc: "2.0".to_string(),
                                    id: None,
                                    result: None,
                                    error: Some(mcp::JsonRpcError {
                                        code: -32603,
                                        message: "Internal error".to_string(),
                                        data: Some(serde_json::json!({
                                            "details": e.to_string()
                                        })),
                                    }),
                                };
                                let response_str = serde_json::to_string(&error_response)?;
                                stdout.write_all(response_str.as_bytes()).await?;
                                stdout.write_all(b"\n").await?;
                                stdout.flush().await?;
                            }
                        }
                    }
                    Err(_e) => {
                        // Failed to parse JSON-RPC request
                        let error_response = JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: None,
                            result: None,
                            error: Some(mcp::JsonRpcError {
                                code: -32700,
                                message: "Parse error".to_string(),
                                data: None,
                            }),
                        };
                        let response_str = serde_json::to_string(&error_response)?;
                        stdout.write_all(response_str.as_bytes()).await?;
                        stdout.write_all(b"\n").await?;
                        stdout.flush().await?;
                    }
                }
            }
            Err(_e) => {
                // Error reading from stdin - terminate gracefully
                break;
            }
        }
    }

    Ok(())
}
