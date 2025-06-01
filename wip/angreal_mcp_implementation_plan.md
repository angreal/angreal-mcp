# Angreal MCP Server Implementation Plan

## Overview
Create a standalone MCP (Model Context Protocol) server that provides AI assistants with discovery capabilities for angreal projects. The server will be a lightweight stdio-based tool that exposes angreal's command tree structure to MCP-compatible clients.

## Project Structure
```
angreal_mcp/
├── Cargo.toml
├── README.md
├── LICENSE
├── src/
│   ├── main.rs
│   ├── mcp.rs
│   └── angreal.rs
├── examples/
│   ├── claude_desktop_config.json
│   ├── cline_config.json
│   └── usage_examples.md
└── tests/
    ├── integration_tests.rs
    └── fixtures/
        └── sample_angreal_output.json
```

## Core Components

### 1. MCP Protocol Handler (`src/mcp.rs`)
- Implement stdio-based MCP server
- Handle standard MCP messages:
  - `initialize` - Server capabilities
  - `tools/list` - Available tools
  - `tools/call` - Tool execution
- Error handling and validation
- JSON-RPC 2.0 compliance

### 2. Angreal Integration (`src/angreal.rs`)
- Shell out to `angreal` binary
- Parse command output
- Handle cases where angreal is not installed
- Handle cases where not in angreal project
- Validate angreal is available and working

### 3. Main Server (`src/main.rs`)
- stdio loop for MCP communication
- Route messages to appropriate handlers
- Logging (optional, to stderr)

## MCP Tools Definition

### Tool: `angreal_tree`
```json
{
  "name": "angreal_tree",
  "description": "Get the command tree structure of the current angreal project, showing available tasks and command groups",
  "inputSchema": {
    "type": "object",
    "properties": {
      "format": {
        "type": "string",
        "enum": ["human", "json"],
        "default": "json",
        "description": "Output format - 'json' for structured data, 'human' for readable tree"
      }
    }
  }
}
```

## Implementation Steps

### Phase 1: Basic MCP Server
1. **Setup Rust project**
   - Create `Cargo.toml` with dependencies:
     - `serde` + `serde_json` for JSON handling
     - `tokio` for async (if needed)
     - `anyhow` for error handling
   
2. **Implement MCP protocol basics**
   - Message parsing and validation
   - Standard MCP responses
   - Error handling

3. **Basic angreal integration**
   - Execute `angreal tree --json`
   - Parse and return output
   - Handle error cases (angreal not found, not in project)

### Phase 2: Robust Implementation
1. **Enhanced error handling**
   - Graceful degradation when angreal unavailable
   - Clear error messages for different failure modes
   - Proper MCP error responses

2. **Working directory awareness**
   - Server operates in the directory where launched
   - Detect if current directory is an angreal project
   - Provide helpful context in responses

3. **Input validation**
   - Validate format parameter
   - Sanitize inputs before shell execution

### Phase 3: Production Ready
1. **Testing**
   - Unit tests for MCP protocol handling
   - Integration tests with mock angreal output
   - Error case testing

2. **Documentation**
   - Comprehensive README with setup instructions
   - Usage examples for different AI assistants
   - Troubleshooting guide

3. **Distribution**
   - GitHub releases with binaries
   - Installation instructions
   - Integration examples

## Dependencies

### Rust Crates
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
tokio = { version = "1.0", features = ["io-std", "macros", "rt"] }
```

### External Dependencies
- `angreal` binary must be installed and in PATH
- Current working directory should be an angreal project for meaningful output

## Error Handling Strategy

### Error Types
1. **Angreal not installed** - Clear message about installing angreal
2. **Not in angreal project** - Explain what angreal projects are
3. **Angreal execution failed** - Pass through angreal's error message
4. **Invalid MCP message** - Standard MCP error response
5. **Invalid parameters** - Validation error with helpful message

### Error Response Format
```json
{
  "error": {
    "code": -32602,
    "message": "Angreal not found",
    "data": {
      "details": "The 'angreal' command is not available. Please install angreal first.",
      "help_url": "https://github.com/angreal/angreal#installation"
    }
  }
}
```

## Usage Examples

### With Claude Desktop
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

### With Cline
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
```bash
# Test the MCP server manually
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/list"}' | angreal_mcp

# Test angreal_tree tool
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "angreal_tree", "arguments": {"format": "json"}}}' | angreal_mcp
```

## Distribution Strategy

### GitHub Releases
- Cross-platform binaries (Windows, macOS, Linux)
- Clear versioning aligned with MCP protocol updates
- Release notes with integration examples

### Package Managers (Future)
- Homebrew formula
- Cargo install
- npm package (for Node.js ecosystem)

## Integration Examples

### AI Assistant Workflow
1. **Project Discovery**: AI calls `angreal_tree` to understand available commands
2. **Task Execution**: AI uses standard shell tools to run `angreal <command>`
3. **Context Awareness**: AI can suggest relevant tasks based on project structure

### Example AI Interaction
```
User: "Run the tests for this project"

AI: 
1. Calls angreal_tree to discover available commands
2. Sees "test" group with "python", "rust", "all" subcommands  
3. Suggests/executes: `angreal test all`
```

## Security Considerations

### Input Validation
- Strict validation of MCP messages
- No arbitrary command execution beyond `angreal tree`
- Sanitize any user inputs

### Principle of Least Privilege
- Only read access to file system (through angreal)
- No network access required
- Minimal dependencies

### Error Information Disclosure
- Don't expose sensitive file paths in errors
- Generic error messages for security failures
- Log security events to stderr only

## Future Enhancements (Optional)

### Enhanced Discovery
- Parse command arguments and options from angreal
- Provide command descriptions and help text
- Support for angreal project metadata

### Caching
- Cache tree output for better performance
- Invalidate cache on project file changes
- Configurable cache duration

### Advanced Integration
- Support for multiple angreal projects in workspace
- Project detection and switching
- Integration with angreal templates/init

## Success Metrics

### Technical
- MCP protocol compliance
- Error-free operation in common scenarios
- Fast response times (< 100ms for tree command)

### Adoption
- Integration examples for major AI assistants
- Clear documentation and setup process
- Community feedback and contributions

## Timeline Estimate

- **Phase 1**: 2-3 days (basic working server)
- **Phase 2**: 2-3 days (robust implementation) 
- **Phase 3**: 2-3 days (testing, docs, distribution)

**Total**: ~1-2 weeks for production-ready MCP server

## Repository Setup

### Initial Files
```
README.md - Project overview and setup instructions
LICENSE - Choose appropriate license (MIT/Apache-2.0)
Cargo.toml - Rust project configuration
.gitignore - Standard Rust gitignore
src/main.rs - Basic MCP server stub
examples/ - Integration examples for AI assistants
```

### Documentation Structure
- Installation and setup
- Configuration for different AI assistants
- Troubleshooting common issues
- Contributing guidelines
- API reference for the MCP tools