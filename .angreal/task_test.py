import angreal
import subprocess
import sys
import os
import json
import traceback

def run_mcp_tests():
    """Run the MCP server tests"""
    try:
        print("\n=== Running MCP Server Tests ===")
        print(f"Current working directory: {os.getcwd()}")

        # Build the release version first
        print("\nBuilding release version...")
        subprocess.run(["cargo", "build", "--release"], check=True)

        # Get the absolute path to the binary using angreal.get_root()
        print("\nGetting project paths...")
        angreal_root = angreal.get_root()
        print(f"Angreal root: {angreal_root}")
        project_root = os.path.dirname(angreal_root)  # Get parent of .angreal directory
        print(f"Project root: {project_root}")
        binary_path = os.path.join(project_root, "target", "release", "angreal_mcp")
        print(f"Binary path: {binary_path}")

        if not os.path.exists(binary_path):
            print(f"ERROR: Binary not found at {binary_path}")
            print(f"Current directory: {os.getcwd()}")
            print(f"Project root: {project_root}")
            print("Directory contents:")
            subprocess.run(["ls", "-la", os.path.dirname(binary_path)], check=True)
            sys.exit(1)

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
        print(f"Sending init command: {json.dumps(init_cmd)}")
        result = subprocess.run(f"echo '{json.dumps(init_cmd)}' | {binary_path}", shell=True, capture_output=True, text=True)
        print(f"Init command stdout: {result.stdout}")
        print(f"Init command stderr: {result.stderr}")

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
            # Check for presence of our test commands in the new format
            if "commands" not in tree_data:
                print("FAILED: angreal_tree tool did not return expected command structure (missing 'commands')")
                print(f"Content: {content}")
                sys.exit(1)

            # Find our test commands in the commands array
            commands = tree_data["commands"]
            command1 = None
            command2 = None
            command3 = None

            for cmd in commands:
                if cmd.get("name") == "command-1":
                    command1 = cmd
                elif cmd.get("name") == "command-2":
                    command2 = cmd
                elif cmd.get("name") == "command-3":
                    command3 = cmd

            if not command1 or not command2 or not command3:
                print("FAILED: angreal_tree tool did not return expected test commands")
                print(f"Found commands: {[cmd.get('name') for cmd in commands]}")
                print(f"Content: {content}")
                sys.exit(1)

            # Verify the commands have the correct group metadata
            if command1.get("group") != "call-testing":
                print("FAILED: command-1 does not have correct group metadata")
                print(f"Content: {content}")
                sys.exit(1)
            if command2.get("group") != "call-testing":
                print("FAILED: command-2 does not have correct group metadata")
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
            if "command-1" not in content or "command-2" not in content:
                print("FAILED: angreal_tree tool did not return expected commands")
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

    except Exception as e:
        print(f"ERROR: Test failed with exception: {str(e)}")
        print("Traceback:")
        traceback.print_exc()
        sys.exit(1)

@angreal.command(name='test', about='Run the test suite')
def test():
    """Run the test suite"""
    try:
        print("\n=== Starting test suite ===")
        print(f"Current working directory: {os.getcwd()}")

        # Run cargo tests first
        print("\n=== Running Cargo Tests ===")
        subprocess.run(["cargo", "test", "--", "--nocapture"], check=True)

        # Run MCP server tests
        print("\n=== Running MCP Server Tests ===")
        run_mcp_tests()

        print("\n=== Test suite completed successfully ===")
    except Exception as e:
        print(f"ERROR: Test failed with exception: {str(e)}")
        print("Traceback:")
        traceback.print_exc()
        sys.exit(1)
