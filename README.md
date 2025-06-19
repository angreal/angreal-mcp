# Angreal MCP Server

An MCP (Model Context Protocol) server that provides AI assistants with discovery capabilities for [angreal](https://github.com/angreal/angreal) projects.

## Overview

This server exposes angreal's command tree structure to MCP-compatible clients, enabling AI assistants to:
- Discover available commands and tasks in your angreal project
- Understand project structure and capabilities
- Suggest appropriate commands based on context

## Prerequisites

- [angreal](https://github.com/angreal/angreal) must be installed and available in your PATH
- Rust toolchain (for building from source)

## Installation

```bash
# Install from crates.io
cargo install angreal_mcp

# Or install from git (latest development version)
cargo install --git https://github.com/colliery-io/angreal-mcp
```

## Usage

### With Claude Code

Create or update your Claude Code MCP configuration file at `~/.config/claude-code/mcp_servers.json`:

```json
{
  "mcpServers": {
    "angreal": {
      "command": "angreal_mcp",
      "args": [],
      "env": {}
    }
  }
}
```

### With Claude Desktop

Add to your Claude Desktop configuration:

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

### With Cline (VS Code)

Add to your Cline configuration:

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

### Command Line Testing

You can test the MCP server directly via command line:

```bash
# List available tools
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/list"}' | angreal_mcp

# Get angreal command tree
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "angreal_tree", "arguments": {"format": "json"}}}' | angreal_mcp
```

## Available Tools

### `angreal_check`
Check if the current directory is an angreal project and get project status including available commands.

### `angreal_tree`
Get a structured view of all available angreal commands and tasks in the project.

**Parameters:**
- `format` (optional): Output format - `"json"` (default) or `"human"`

### `angreal_run`
Execute an angreal command or task with optional arguments.

**Parameters:**
- `command` (required): The angreal command/task to execute
- `args` (optional): Additional arguments and flags

## Agent Usage Guide

When working in angreal projects, use these tools for intelligent command discovery and execution:

### Tool Usage Workflow
1. **Start with discovery**: Use `angreal_check` to verify project status and capabilities
2. **Explore commands**: Use `angreal_tree` to see available commands with rich metadata
3. **Execute intelligently**: Use `angreal_run` with context-aware parameter selection

### Best Practices
- Always check `when_to_use` and `when_not_to_use` guidance from `angreal_tree`
- Use parameter recommendations (`recommended_when`) for better results
- Verify prerequisites before executing commands
- Let command output and exit codes guide success validation

### Common Patterns
- **Setup workflows**: Run setup/install commands before build/test commands
- **Development cycles**: Use test commands after code changes, build commands before deployment
- **Parameter selection**: Use verbose flags when debugging, production flags for releases

### Troubleshooting
- If MCP server becomes unavailable, restart Claude Code to reinitialize
- Check that angreal binary is installed and accessible in PATH
- Verify you're in an angreal project directory (contains .angreal/ folder)

The angreal MCP server provides enhanced command metadata including usage context, parameter guidance, and intelligent categorization to enable better automation decisions.

## Development

### Running Tests

This project uses angreal as its test runner:

```bash
# Run tests using angreal
angreal test
```

### Project Structure

```
angreal_mcp/
├── src/
│   ├── main.rs      # Main server loop
│   ├── mcp.rs       # MCP protocol implementation
│   └── angreal.rs   # Angreal integration
├── examples/        # Configuration examples
└── tests/          # Integration tests
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

```

## License

This project is dual-licensed under MIT OR Apache-2.0.
