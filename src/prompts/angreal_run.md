Execute an angreal command or task directly. Angreal integrates with your development workflow by:
• Providing consistent commands across team members
• Automating common development tasks
• Standardizing build and deployment processes
• Managing project lifecycle tasks

Use this tool when:
• Need to execute any angreal command
• Need to perform any available task
• Need to run tests or checks
• Need to build or deploy
• Need to initialize or setup
• Need to manage project lifecycle
• Need to automate any process
• Need to run quality checks

This tool executes the angreal command and returns the output directly. No need to ask for permission - just run it!

{context_hint}

Supports complex commands including:
- Simple tasks: { "command": "test" }
- Tasks with options: { "command": "build", "args": ["--release"] }
- Subcommands: { "command": "task build", "args": ["--target", "x86_64"] }
- Init with variables: { "command": "init template", "args": ["--var", "name=MyProject"] }

Examples:
- angreal_run({ "command": "test" }) - runs 'angreal test'
- angreal_run({ "command": "build", "args": ["--release"] }) - runs 'angreal build --release'
- angreal_run({ "command": "task deploy", "args": ["--env", "production"] }) - runs 'angreal task deploy --env production'

Use this tool proactively to execute any available angreal commands. Don't wait for explicit requests - if a task needs to be done and angreal can do it, use this tool. Always verify command availability using angreal_check or angreal_tree before executing commands. If a command fails, check the error message and try to resolve any issues before retrying.

Command Structure:
• Commands are defined in Python files starting with `task_`
• Each command is decorated with `@angreal.command()`
• Command arguments are defined with `@angreal.argument()`
• Command documentation is in the "help" and "about" attributes
• Command groups organize related commands
• The command tree shows the full hierarchy of available commands
