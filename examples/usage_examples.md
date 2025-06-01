# Angreal MCP Server Usage Examples

This document provides examples and instructions for testing and using the Angreal MCP server with various AI assistants.

## Command Line Testing

You can test the MCP server manually using the command line to verify it's working correctly:

### Test Basic Server Connection
```bash
# Test the MCP server initialization
echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"clientInfo": {"name": "test", "version": "1.0"}}}' | angreal_mcp
```

### List Available Tools
```bash
# Get list of available MCP tools
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/list"}' | angreal_mcp
```

### Test angreal_tree Tool
```bash
# Get angreal command tree in JSON format
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "angreal_tree", "arguments": {"format": "json"}}}' | angreal_mcp

# Get angreal command tree in human-readable format
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "angreal_tree", "arguments": {"format": "human"}}}' | angreal_mcp
```

## Integration with Claude Desktop

1. **Locate Claude Desktop configuration**:
   - macOS: `~/Library/Application Support/Claude/claude_desktop_config.json`
   - Windows: `%APPDATA%\Claude\claude_desktop_config.json`
   - Linux: `~/.config/Claude/claude_desktop_config.json`

2. **Add the angreal MCP server** to your configuration:
   ```json
   {
     "mcpServers": {
       "angreal": {
         "command": "angreal_mcp",
         "args": [],
         "cwd": "/path/to/your/angreal/project"
       }
     }
   }
   ```

3. **Restart Claude Desktop** for changes to take effect.

4. **Test the integration** by asking Claude about available angreal commands in your project.

## Integration with Cline (VS Code Extension)

1. **Open VS Code settings** for Cline extension.

2. **Add MCP server configuration**:
   ```json
   {
     "mcp": {
       "servers": [
         {
           "name": "angreal",
           "command": ["angreal_mcp"],
           "cwd": "${workspaceFolder}"
         }
       ]
     }
   }
   ```

3. **Reload VS Code window** or restart the Cline extension.

4. **Test by asking Cline** to list available angreal tasks in your project.

## Example AI Assistant Interactions

### Project Discovery
```
User: "What commands are available in this angreal project?"

AI: Let me check the available angreal commands for this project...
[Calls angreal_tree tool]
This project has the following command structure:
- build: Commands for building the project
  - docker: Build Docker images
  - binary: Build project binaries
- test: Testing commands
  - unit: Run unit tests
  - integration: Run integration tests
  - all: Run all tests
- deploy: Deployment commands
  - staging: Deploy to staging
  - production: Deploy to production
```

### Task Execution Workflow
```
User: "Run the tests for this project"

AI: 
1. First, let me discover what test commands are available...
   [Calls angreal_tree to discover available commands]
2. I can see there are test commands available: unit, integration, and all
3. I'll run all tests for comprehensive coverage:
   [Executes: angreal test all]
```

### Context-Aware Suggestions
```
User: "I need to deploy this application"

AI: Let me check what deployment options are available...
[Calls angreal_tree]
I see you have two deployment environments configured:
- staging: angreal deploy staging
- production: angreal deploy production

Which environment would you like to deploy to?
```

## Troubleshooting

### Common Issues

1. **"Angreal not found" error**
   - Ensure angreal is installed: `pip install angreal` or follow project-specific installation
   - Verify angreal is in PATH: `which angreal`

2. **"Not in an angreal project" error**
   - Make sure the MCP server's working directory is set to an angreal project
   - Check for `.angreal` directory or `angreal.yaml` configuration file

3. **No response from MCP server**
   - Verify the server binary is executable: `chmod +x angreal_mcp`
   - Check server is in PATH or use full path in configuration
   - Test manually with command line examples above

### Debug Mode

To enable debug logging, set the environment variable:
```bash
ANGREAL_MCP_DEBUG=1 angreal_mcp
```

Debug logs will be written to stderr and won't interfere with the MCP protocol on stdout.

## Advanced Usage

### Multiple Projects

If you work with multiple angreal projects, you can configure multiple MCP server instances:

```json
{
  "mcpServers": {
    "angreal-project1": {
      "command": "angreal_mcp",
      "args": [],
      "cwd": "/path/to/project1"
    },
    "angreal-project2": {
      "command": "angreal_mcp",
      "args": [],
      "cwd": "/path/to/project2"
    }
  }
}
```

### Custom Angreal Executable

If your angreal executable has a different name or location:

```json
{
  "mcpServers": {
    "angreal": {
      "command": "angreal_mcp",
      "args": ["--angreal-path", "/custom/path/to/angreal"],
      "cwd": "/path/to/your/project"
    }
  }
}
```

## Security Notes

- The MCP server only executes `angreal tree` commands
- No arbitrary command execution is possible through the MCP interface
- All file system access is read-only through angreal
- The server operates with the permissions of the user running it

## Getting Help

- Check the [Angreal MCP GitHub repository](https://github.com/yourusername/angreal-mcp) for updates
- Report issues or request features via GitHub Issues
- Join the discussion in the Angreal community channels