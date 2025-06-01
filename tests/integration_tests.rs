use serde_json::json;

#[tokio::test]
async fn test_mcp_initialize() {
    let request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "1.0",
            "capabilities": {
                "tools": {
                    "callTool": true
                }
            }
        }
    });

    let response_str = handle_request_string(request.to_string()).await;
    let response: serde_json::Value = serde_json::from_str(&response_str).unwrap();

    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], 1);
    assert!(response["result"]["protocolVersion"].is_string());
    assert!(response["result"]["capabilities"]["tools"]["static"].is_array());
}

#[tokio::test]
async fn test_tools_list() {
    let request = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list"
    });

    let response_str = handle_request_string(request.to_string()).await;
    let response: serde_json::Value = serde_json::from_str(&response_str).unwrap();

    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], 2);

    let tools = &response["result"]["tools"];
    assert!(tools.is_array());
    assert_eq!(tools.as_array().unwrap().len(), 3);
    assert_eq!(tools[0]["name"], "angreal_check");
}

#[tokio::test]
async fn test_invalid_method() {
    let request = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "invalid/method"
    });

    let response_str = handle_request_string(request.to_string()).await;
    let response: serde_json::Value = serde_json::from_str(&response_str).unwrap();

    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], 3);
    assert!(response["error"].is_object());
    assert_eq!(response["error"]["code"], -32601);
    assert_eq!(response["error"]["message"], "Method not found");
}

#[tokio::test]
async fn test_malformed_json() {
    let response_str = handle_request_string("not valid json".to_string()).await;
    let response: serde_json::Value = serde_json::from_str(&response_str).unwrap();

    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["error"].is_object());
    assert_eq!(response["error"]["code"], -32700);
    assert_eq!(response["error"]["message"], "Parse error");
}

// Helper function to simulate request handling without running the full server
async fn handle_request_string(request_str: String) -> String {
    use angreal_mcp::mcp::{JsonRpcRequest, McpServer};

    let server = McpServer::new();

    match serde_json::from_str::<JsonRpcRequest>(&request_str) {
        Ok(request) => match server.handle_request(request).await {
            Ok(response) => serde_json::to_string(&response).unwrap(),
            Err(e) => json!({
                "jsonrpc": "2.0",
                "id": null,
                "error": {
                    "code": -32603,
                    "message": "Internal error",
                    "data": {
                        "details": e.to_string()
                    }
                }
            })
            .to_string(),
        },
        Err(_) => json!({
            "jsonrpc": "2.0",
            "id": null,
            "error": {
                "code": -32700,
                "message": "Parse error"
            }
        })
        .to_string(),
    }
}

