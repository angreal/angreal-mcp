import angreal
import subprocess
import sys
import os
import json

def run_mcp_tests():
    """Run the MCP server tests"""
    print("\n=== Running MCP Server Tests ===")
    
    # Build the release version first
    print("\nBuilding release version...")
    subprocess.run(["cargo", "build", "--release"], check=True)
    
    # Get the absolute path to the binary using angreal.get_root()
    project_root = os.path.dirname(angreal.get_root())  # Get parent of .angreal directory
    binary_path = os.path.join(project_root, "target", "release", "angreal_mcp")
    print(f"\nUsing binary at: {binary_path}")
    
    # Test MCP server initialization
    print("\nTesting MCP server initialization...")
    init_cmd = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "1.0",
            "capabilities": {
                "tools": {
                    "callTool": True
                }
            }
        }
    }
    result = subprocess.run(f"echo '{json.dumps(init_cmd)}' | {binary_path}", shell=True, capture_output=True, text=True)
    try:
        response = json.loads(result.stdout)
        if "result" not in response or "protocolVersion" not in str(response["result"]):
            print("FAILED: MCP server initialization failed")
            print(f"Response: {result.stdout}")
            sys.exit(1)
        print("PASSED: MCP server initialization successful")
    except json.JSONDecodeError:
        print("FAILED: Invalid JSON response from MCP server")
        print(f"Response: {result.stdout}")
        sys.exit(1)
    
    # Test tools list
    print("\nTesting tools list...")
    list_cmd = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/list"
    }
    result = subprocess.run(f"echo '{json.dumps(list_cmd)}' | {binary_path}", shell=True, capture_output=True, text=True)
    try:
        response = json.loads(result.stdout)
        if "result" not in response or "tools" not in response["result"]:
            print("FAILED: Tools list failed")
            print(f"Response: {result.stdout}")
            sys.exit(1)
        tools = response["result"]["tools"]
        if not any(tool["name"] == "angreal_check" for tool in tools):
            print("FAILED: angreal_check tool not found in tools list")
            print(f"Available tools: {[tool['name'] for tool in tools]}")
            sys.exit(1)
        print("PASSED: Tools list successful")
    except json.JSONDecodeError:
        print("FAILED: Invalid JSON response from MCP server")
        print(f"Response: {result.stdout}")
        sys.exit(1)
    
    # Test angreal_check tool
    print("\nTesting angreal_check tool...")
    check_cmd = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "angreal_check",
            "arguments": {}
        }
    }
    result = subprocess.run(f"echo '{json.dumps(check_cmd)}' | {binary_path}", shell=True, capture_output=True, text=True)
    try:
        response = json.loads(result.stdout)
        if "result" not in response or "content" not in response["result"]:
            print("FAILED: angreal_check tool failed")
            print(f"Response: {result.stdout}")
            sys.exit(1)
        content = response["result"]["content"][0]["text"]
        if "angreal project" not in content:
            print("FAILED: angreal_check tool did not return expected content")
            print(f"Content: {content}")
            sys.exit(1)
        print("PASSED: angreal_check tool successful")
    except json.JSONDecodeError:
        print("FAILED: Invalid JSON response from MCP server")
        print(f"Response: {result.stdout}")
        sys.exit(1)
    
    # Test angreal_tree tool with JSON format
    print("\nTesting angreal_tree tool (JSON format)...")
    tree_json_cmd = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "angreal_tree",
            "arguments": {
                "format": "json"
            }
        }
    }
    result = subprocess.run(f"echo '{json.dumps(tree_json_cmd)}' | {binary_path}", shell=True, capture_output=True, text=True)
    try:
        response = json.loads(result.stdout)
        if "result" not in response or "content" not in response["result"]:
            print("FAILED: angreal_tree tool (JSON) failed")
            print(f"Response: {result.stdout}")
            sys.exit(1)
        content = response["result"]["content"][0]["text"]
        tree_data = json.loads(content)
        # Check for presence of our test commands in the tree
        if "children" not in tree_data or "call-testing" not in tree_data["children"]:
            print("FAILED: angreal_tree tool did not return expected command structure")
            print(f"Content: {content}")
            sys.exit(1)
        call_testing = tree_data["children"]["call-testing"]
        if "children" not in call_testing or "command-1" not in call_testing["children"] or "command-2" not in call_testing["children"]:
            print("FAILED: angreal_tree tool did not return expected test commands")
            print(f"Content: {content}")
            sys.exit(1)
        print("PASSED: angreal_tree tool (JSON) successful")
    except json.JSONDecodeError:
        print("FAILED: Invalid JSON response from MCP server")
        print(f"Response: {result.stdout}")
        sys.exit(1)
    
    # Test angreal_tree tool with human format
    print("\nTesting angreal_tree tool (human format)...")
    tree_human_cmd = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "angreal_tree",
            "arguments": {
                "format": "human"
            }
        }
    }
    result = subprocess.run(f"echo '{json.dumps(tree_human_cmd)}' | {binary_path}", shell=True, capture_output=True, text=True)
    try:
        response = json.loads(result.stdout)
        if "result" not in response or "content" not in response["result"]:
            print("FAILED: angreal_tree tool (human) failed")
            print(f"Response: {result.stdout}")
            sys.exit(1)
        content = response["result"]["content"][0]["text"]
        if "call-testing" not in content:
            print("FAILED: angreal_tree tool did not return expected content")
            print(f"Content: {content}")
            sys.exit(1)
        print("PASSED: angreal_tree tool (human) successful")
    except json.JSONDecodeError:
        print("FAILED: Invalid JSON response from MCP server")
        print(f"Response: {result.stdout}")
        sys.exit(1)
    
    # Test complex angreal commands
    print("\nTesting complex angreal commands...")
    
    # Test command-1 with boolean flag
    result = subprocess.run(["angreal", "call-testing", "command-1", "--option"], capture_output=True, text=True)
    if "Successfully executed" not in result.stdout:
        print("FAILED: command-1 test failed")
        print(f"Output: {result.stdout}")
        print(f"Error: {result.stderr}")
        sys.exit(1)
    print("PASSED: command-1 test successful")
    
    # Test command-2 with parameter value
    result = subprocess.run(["angreal", "call-testing", "command-2", "--parameter", "test-value"], capture_output=True, text=True)
    if "Successfully executed" not in result.stdout:
        print("FAILED: command-2 test failed")
        print(f"Output: {result.stdout}")
        print(f"Error: {result.stderr}")
        sys.exit(1)
    print("PASSED: command-2 test successful")
    
    # Test MCP server with complex commands
    print("\nTesting MCP server with complex commands...")
    
    # Test command-1 through MCP
    mcp_cmd1 = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "angreal_run",
            "arguments": {
                "command": "call-testing command-1",
                "args": ["--option"]
            }
        }
    }
    result = subprocess.run(f"echo '{json.dumps(mcp_cmd1)}' | {binary_path}", shell=True, capture_output=True, text=True)
    try:
        response = json.loads(result.stdout)
        if "result" not in response or "content" not in response["result"]:
            print("FAILED: MCP command-1 test failed")
            print(f"Response: {result.stdout}")
            sys.exit(1)
        content = response["result"]["content"][0]["text"]
        if "Successfully executed" not in content:
            print("FAILED: MCP command-1 did not return expected content")
            print(f"Content: {content}")
            sys.exit(1)
        print("PASSED: MCP command-1 test successful")
    except json.JSONDecodeError:
        print("FAILED: Invalid JSON response from MCP server")
        print(f"Response: {result.stdout}")
        sys.exit(1)
    
    # Test command-2 through MCP
    mcp_cmd2 = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "angreal_run",
            "arguments": {
                "command": "call-testing command-2",
                "args": ["--parameter", "test-value"]
            }
        }
    }
    result = subprocess.run(f"echo '{json.dumps(mcp_cmd2)}' | {binary_path}", shell=True, capture_output=True, text=True)
    try:
        response = json.loads(result.stdout)
        if "result" not in response or "content" not in response["result"]:
            print("FAILED: MCP command-2 test failed")
            print(f"Response: {result.stdout}")
            sys.exit(1)
        content = response["result"]["content"][0]["text"]
        if "Successfully executed" not in content:
            print("FAILED: MCP command-2 did not return expected content")
            print(f"Content: {content}")
            sys.exit(1)
        print("PASSED: MCP command-2 test successful")
    except json.JSONDecodeError:
        print("FAILED: Invalid JSON response from MCP server")
        print(f"Response: {result.stdout}")
        sys.exit(1)
    
    # Test error handling
    print("\nTesting error handling...")
    error_cmd = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "invalid/method"
    }
    result = subprocess.run(f"echo '{json.dumps(error_cmd)}' | {binary_path}", shell=True, capture_output=True, text=True)
    try:
        response = json.loads(result.stdout)
        if "error" not in response or response["error"]["message"] != "Method not found":
            print("FAILED: Error handling test failed")
            print(f"Response: {result.stdout}")
            sys.exit(1)
        print("PASSED: Error handling test successful")
    except json.JSONDecodeError:
        print("FAILED: Invalid JSON response from MCP server")
        print(f"Response: {result.stdout}")
        sys.exit(1)
    
    print("\n=== MCP Server Tests Completed Successfully ===")

@angreal.command(name='test', about='Run the test suite')
def test():
    """Run the test suite"""
    # Run cargo tests first
    print("\n=== Running Cargo Tests ===")
    subprocess.run(["cargo", "test", "--", "--nocapture"], check=True)
    
    # Run MCP server tests
    run_mcp_tests()