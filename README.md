# Angreal MCP Server

A lightweight MCP (Model Context Protocol) server that provides AI assistants with discovery capabilities for [angreal](https://github.com/angreal/angreal) projects.

## Overview

This server exposes angreal's command tree structure to MCP-compatible clients, enabling AI assistants to:
- Discover available commands and tasks in your angreal project
- Understand project structure and capabilities
- Suggest appropriate commands based on context

## Prerequisites

- [angreal](https://github.com/angreal/angreal) must be installed and available in your PATH
- Rust toolchain (for building from source)

## Installation

### Method 1: Using Claude CLI (Easiest)

If you have Claude Code installed, you can use the built-in MCP management:

```bash
# Install and configure in one command
claude mcp add angreal -s user -- angreal_mcp

# Or if angreal_mcp is not in PATH, use full path
claude mcp add angreal -s user -- ~/.cargo/bin/angreal_mcp
```

This automatically:
- Installs the MCP server configuration
- Sets up the proper permissions
- Restarts Claude services

### Method 2: Using Cargo + Manual Config

```bash
# First install the binary
cargo install angreal_mcp

# Then manually configure (see configuration sections below)
```

### Alternative Installation Methods

```bash
# Install from git (latest development version)
cargo install --git https://github.com/yourusername/angreal-mcp

# Install with cargo-binstall (faster, uses pre-compiled binaries)
cargo binstall angreal_mcp

# Update to latest version
cargo install angreal_mcp --force

# Using community MCP manager
npm install -g mcpm
mcpm install angreal_mcp
```

## Usage

### With Claude Code (Recommended)

Claude Code has built-in MCP support. Here's how to set it up:

#### Option A: One-Command Setup (Recommended)

```bash
# Install binary and configure automatically
cargo install angreal_mcp
claude mcp add angreal -s user -- angreal_mcp
```

#### Option B: Manual Setup

**1. Install the Server**
```bash
cargo install angreal_mcp
```

**2. Configure for Claude Code**
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

#### 3. Usage in Claude Code

Once configured, Claude Code will automatically:
- Launch the angreal MCP server when you work in angreal projects
- Detect `.angreal/` directories and offer angreal-specific assistance
- Provide access to `angreal_check` and `angreal_tree` tools

**Example interaction:**
```
You: "What can I run in this project?"

Claude Code will:
1. Use angreal_check to detect if it's an angreal project
2. Use angreal_tree to show available commands
3. Suggest appropriate tasks based on your project structure
```

#### 4. Verify Setup

Test the configuration:
```bash
# In an angreal project directory
claude-code --test-mcp angreal
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

See `examples/claude_desktop_config.json` for a complete example.

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

See `examples/cline_config.json` for a complete example.

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

# Or directly with cargo (will use angreal via profile.test.runner)
cargo test
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

# Run linting
angreal lint

# Format code
angreal fmt
```

## Error Handling

The server provides clear error messages for common scenarios:

- **Angreal not installed**: Prompts to install angreal first
- **Not in angreal project**: Explains the need to run from within an angreal project
- **Command execution failed**: Passes through angreal's error messages

## Security

- The server only executes `angreal tree` commands
- No arbitrary command execution
- Read-only access to project structure
- All inputs are validated before use

## Troubleshooting

### Claude Code Specific Issues

#### MCP Server Not Loading
```bash
# Check if the binary is in your PATH
which angreal_mcp

# Verify the configuration file exists and is valid JSON
cat ~/.config/claude-code/mcp_servers.json | jq .

# Test the server manually
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/list"}' | angreal_mcp
```

#### Tools Not Available in Claude Code
1. Restart Claude Code after configuration changes
2. Check that you're in an angreal project directory
3. Use the `angreal_check` tool first to verify project status

#### Permission Issues
```bash
# Make sure the binary is executable
chmod +x ~/.local/bin/angreal_mcp

# Or if installed globally
ls -la /usr/local/bin/angreal_mcp
```

### General Issues

#### "Angreal not found" error

Ensure angreal is installed and in your PATH:
```bash
which angreal
angreal --version

# Install if missing
pip install angreal
```

#### "Not in an angreal project" error

Use the `angreal_check` tool to understand project status:
- Look for `.angreal/` directory
- Check if project needs initialization

#### Debug Mode

Run with debug output to stderr:
```bash
RUST_LOG=debug angreal_mcp
```

#### Manual Testing

Test the MCP server directly:
```bash
# Test project detection
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "angreal_check", "arguments": {}}}' | angreal_mcp

# Test command discovery
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "angreal_tree", "arguments": {"format": "human"}}}' | angreal_mcp
```

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

This project is dual-licensed under MIT OR Apache-2.0.