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

### `angreal_tree`

Get the command tree structure of the current angreal project.

**Parameters:**
- `format` (optional): Output format - `"json"` (default) or `"human"`

**Example Response (JSON format):**
```json
{
  "groups": [
    {
      "name": "test",
      "commands": ["python", "rust", "all"]
    }
  ],
  "commands": ["build", "deploy"]
}
```

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
