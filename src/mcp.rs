use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// Include prompt files at compile time
const ANGREAL_CHECK_PROMPT: &str = include_str!("prompts/angreal_check.md");
const ANGREAL_TREE_PROMPT: &str = include_str!("prompts/angreal_tree.md");
const ANGREAL_RUN_PROMPT: &str = include_str!("prompts/angreal_run.md");

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
        let is_angreal_project = std::path::Path::new(".angreal").exists();

        let context_hint = if is_angreal_project {
            "NOTE: The server detected a .angreal/ directory in the current working directory, indicating this IS an angreal project. You can confidently use angreal tools here."
        } else {
            "NOTE: The server did not detect a .angreal/ directory in the current working directory. This may not be an angreal project."
        };

        let angreal_check_tool = Tool {
            name: "angreal_check".to_string(),
            description: ANGREAL_CHECK_PROMPT.replace("{context_hint}", context_hint),
            input_schema: json!({
                "type": "object",
                "properties": {}
            }),
        };

        let angreal_tree_tool = Tool {
            name: "angreal_tree".to_string(),
            description: ANGREAL_TREE_PROMPT.replace("{context_hint}", context_hint),
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

        let angreal_run_tool = Tool {
            name: "angreal_run".to_string(),
            description: ANGREAL_RUN_PROMPT.replace("{context_hint}", context_hint),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "command": {
                        "type": "string",
                        "description": "The angreal command/task to execute. Can include subcommands (e.g., 'test', 'build', 'task deploy', 'init template-name')",
                        "examples": ["test", "build", "lint", "task deploy", "init my-template"]
                    },
                    "args": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Additional arguments, options, and flags to pass to the command",
                        "examples": [["--release"], ["--env", "production"], ["--var", "name=value"]],
                        "default": []
                    }
                },
                "required": ["command"]
            }),
        };

        Self {
            tools: vec![angreal_check_tool, angreal_tree_tool, angreal_run_tool],
        }
    }

    pub async fn handle_request(&self, request: JsonRpcRequest) -> Result<JsonRpcResponse> {
        match request.method.as_str() {
            "initialize" => self.handle_initialize(request.id).await,
            "tools/list" => self.handle_tools_list(request.id).await,
            "tools/call" => {
                let params: ToolCallParams = serde_json::from_value(
                    request
                        .params
                        .ok_or_else(|| anyhow::anyhow!("Missing params"))?,
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

        // Check project status during initialization
        let project_status = match crate::angreal::check_angreal_project_status().await {
            Ok(status) => status,
            Err(_) => "Unable to determine project status".to_string(),
        };

        let is_angreal_project = std::path::Path::new(".angreal").exists();
        let current_dir = std::env::current_dir()
            .map(|d| d.display().to_string())
            .unwrap_or_else(|_| "unknown".to_string());

        Ok(JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({
                "protocolVersion": "2024-11-05",
                "capabilities": capabilities,
                "serverInfo": {
                    "name": "angreal_mcp",
                    "version": "0.1.0",
                    "description": "MCP server for angreal project discovery and automation",
                    "context": {
                        "currentDirectory": current_dir,
                        "isAngrealProject": is_angreal_project,
                        "projectStatus": project_status
                    }
                }
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
            "angreal_check" => match crate::angreal::check_angreal_project_status().await {
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
            },
            "angreal_run" => {
                let command = params
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("command"))
                    .and_then(|c| c.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Missing required 'command' parameter"))?;

                let args: Vec<String> = params
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("args"))
                    .and_then(|a| a.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str())
                            .map(String::from)
                            .collect()
                    })
                    .unwrap_or_default();

                match crate::angreal::run_angreal_command(command, &args).await {
                    Ok(output) => Ok(JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id,
                        result: Some(json!({
                            "content": [
                                {
                                    "type": "text",
                                    "text": format!("$ angreal {}\n\n{}", command, output)
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
                            message: "Command execution failed".to_string(),
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

impl Default for McpServer {
    fn default() -> Self {
        Self::new()
    }
}
