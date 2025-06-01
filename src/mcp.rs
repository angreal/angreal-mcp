use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitializeParams {
    pub protocol_version: String,
    pub capabilities: ClientCapabilities,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientCapabilities {
    pub tools: Option<ToolsCapability>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolsCapability {
    pub call_tool: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerCapabilities {
    pub tools: ToolsServerCapability,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolsServerCapability {
    pub r#static: Vec<Tool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCallParams {
    pub name: String,
    pub arguments: Option<Value>,
}

pub struct McpServer {
    tools: Vec<Tool>,
}

impl McpServer {
    pub fn new() -> Self {
        let angreal_check_tool = Tool {
            name: "angreal_check".to_string(),
            description: "Check if the current directory is an angreal project and get project status. Use this tool when:
• You're unsure if you're in an angreal project
• User asks about project type or available automation
• You want to detect angreal capabilities before offering angreal-specific help
• Need to understand the project state (initialized vs uninitialized)

This tool will:
• Detect if angreal is installed and available
• Check if current directory is an angreal project (.angreal/ folder exists)
• Determine project initialization status
• Provide guidance on next steps if project is not initialized

Use this FIRST before using angreal_tree if you're uncertain about the project type.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {}
            }),
        };

        let angreal_tree_tool = Tool {
            name: "angreal_tree".to_string(),
            description: "Discover available tasks and commands in an angreal project. Use this tool when:
• User asks 'what can I run?', 'what tasks are available?', 'what commands exist?', or similar questions about project capabilities
• You've confirmed you're in an angreal project (use angreal_check first if unsure)
• User mentions running tasks, automation, or build commands in an angreal project
• User wants to understand what project-specific automation is available
• Need to discover project structure and available commands

This tool returns structured information about all available angreal commands, task groups, and their organization. It helps users understand what they can do in their angreal project without having to remember command names.

TIP: If this tool fails with 'not in angreal project' error, the project may need initialization. Check with angreal_check first.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "format": {
                        "type": "string",
                        "enum": ["human", "json"],
                        "default": "json",
                        "description": "Output format - 'json' for structured data, 'human' for readable tree"
                    }
                }
            }),
        };

        Self {
            tools: vec![angreal_check_tool, angreal_tree_tool],
        }
    }

    pub async fn handle_request(&self, request: JsonRpcRequest) -> Result<JsonRpcResponse> {
        match request.method.as_str() {
            "initialize" => self.handle_initialize(request.id).await,
            "tools/list" => self.handle_tools_list(request.id).await,
            "tools/call" => {
                let params: ToolCallParams = serde_json::from_value(
                    request.params.ok_or_else(|| anyhow::anyhow!("Missing params"))?,
                )?;
                self.handle_tool_call(request.id, params).await
            }
            _ => Ok(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: None,
                }),
            }),
        }
    }

    async fn handle_initialize(&self, id: Option<Value>) -> Result<JsonRpcResponse> {
        let capabilities = ServerCapabilities {
            tools: ToolsServerCapability {
                r#static: self.tools.clone(),
            },
        };

        Ok(JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({
                "protocolVersion": "1.0",
                "capabilities": capabilities,
            })),
            error: None,
        })
    }

    async fn handle_tools_list(&self, id: Option<Value>) -> Result<JsonRpcResponse> {
        Ok(JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({
                "tools": self.tools,
            })),
            error: None,
        })
    }

    async fn handle_tool_call(
        &self,
        id: Option<Value>,
        params: ToolCallParams,
    ) -> Result<JsonRpcResponse> {
        match params.name.as_str() {
            "angreal_check" => {
                match crate::angreal::check_angreal_project_status().await {
                    Ok(status_info) => Ok(JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id,
                        result: Some(json!({
                            "content": [
                                {
                                    "type": "text",
                                    "text": status_info
                                }
                            ]
                        })),
                        error: None,
                    }),
                    Err(e) => Ok(JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id,
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32603,
                            message: "Internal error".to_string(),
                            data: Some(json!({
                                "details": e.to_string(),
                            })),
                        }),
                    }),
                }
            }
            "angreal_tree" => {
                let format = params
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("format"))
                    .and_then(|f| f.as_str())
                    .unwrap_or("json");

                match crate::angreal::get_angreal_tree(format).await {
                    Ok(output) => Ok(JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id,
                        result: Some(json!({
                            "content": [
                                {
                                    "type": "text",
                                    "text": output
                                }
                            ]
                        })),
                        error: None,
                    }),
                    Err(e) => Ok(JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id,
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32603,
                            message: "Internal error".to_string(),
                            data: Some(json!({
                                "details": e.to_string(),
                            })),
                        }),
                    }),
                }
            }
            _ => Ok(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32602,
                    message: "Invalid params".to_string(),
                    data: Some(json!({
                        "details": format!("Unknown tool: {}", params.name),
                    })),
                }),
            }),
        }
    }
}