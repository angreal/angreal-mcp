# Claude Code Setup Guide

This guide walks you through setting up the Angreal MCP Server with Claude Code.

## Quick Start

### 1. Prerequisites

Ensure you have:
- [Claude Code](https://claude.ai/code) installed
- [Rust toolchain](https://rustup.rs/) for building the server
- [angreal](https://github.com/angreal/angreal) installed: `pip install angreal`

### 2. Install the MCP Server

```bash
# Install via cargo (recommended)
cargo install angreal_mcp

# Verify installation
angreal_mcp --help
which angreal_mcp
```

The binary will be installed to `~/.cargo/bin/angreal_mcp` which is automatically in your PATH.

### 3. Configure Claude Code

Create the MCP configuration directory and file:

```bash
# Create config directory if it doesn't exist
mkdir -p ~/.config/claude-code

# Create the MCP servers configuration
cat > ~/.config/claude-code/mcp_servers.json << 'EOF'
{
  "mcpServers": {
    "angreal": {
      "command": "angreal_mcp",
      "args": [],
      "env": {}
    }
  }
}
EOF
```

### 4. Test the Setup

```bash
# Verify the configuration is valid JSON
jq . ~/.config/claude-code/mcp_servers.json

# Test the server manually
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/list"}' | angreal_mcp
```

### 5. Use with Claude Code

1. **Start Claude Code** in an angreal project directory
2. **Verify detection**: Ask Claude "What type of project is this?"
3. **Explore commands**: Ask "What can I run in this project?"

## Expected Behavior

When working in an angreal project, Claude Code will:

### Automatic Project Detection
```
You: "What can I do in this project?"

Claude: I'll check what type of project this is first.
[Uses angreal_check tool]
I can see this is an angreal project with several available commands.
[Uses angreal_tree tool]
Here are the available commands:
- build: Build the project in release mode
- test: Run the test suite
- lint: Run clippy linter
- fmt: Format code
- check: Check code without building
```

### Context-Aware Assistance
```
You: "How do I run the tests?"

Claude: Since this is an angreal project, you can run tests using:
`angreal test`

This will execute the test suite with proper configuration.
```

## Troubleshooting

### Server Not Found
```bash
# Check if binary is in PATH
which angreal_mcp

# If not found, ensure ~/.local/bin is in your PATH
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Configuration Issues
```bash
# Check configuration syntax
jq . ~/.config/claude-code/mcp_servers.json

# View Claude Code logs (location may vary by OS)
tail -f ~/.local/share/claude-code/logs/mcp.log
```

### Tools Not Available
1. Restart Claude Code after configuration changes
2. Ensure you're in a directory with a `.angreal/` folder
3. Use manual testing commands from the troubleshooting section

## Advanced Configuration

### Custom Environment Variables
```json
{
  "mcpServers": {
    "angreal": {
      "command": "angreal_mcp",
      "args": [],
      "env": {
        "RUST_LOG": "debug",
        "ANGREAL_CONFIG_PATH": "/custom/path"
      }
    }
  }
}
```

### Project-Specific Configuration

For project-specific setups, you can create a local configuration:

```bash
# In your project root
mkdir -p .claude-code
cat > .claude-code/mcp_servers.json << 'EOF'
{
  "mcpServers": {
    "angreal": {
      "command": "angreal_mcp",
      "args": [],
      "cwd": "."
    }
  }
}
EOF
```

## Getting Help

If you encounter issues:

1. **Check the main README** for general troubleshooting
2. **Test manually** using the command-line examples
3. **Check Claude Code documentation** for MCP-specific issues
4. **Open an issue** on the GitHub repository with:
   - Your configuration file
   - Error messages
   - Output from manual testing commands

## Next Steps

Once set up successfully:
- Explore the `angreal_check` and `angreal_tree` tools
- Try asking Claude Code about project automation
- Use angreal commands through Claude Code's suggestions
- Share feedback to improve the MCP server