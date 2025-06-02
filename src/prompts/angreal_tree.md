Get a specific format of available tasks and commands in an angreal project. Angreal is particularly useful for:
• Software development projects that need standardized workflows
• Teams that want consistent build, test, and deployment processes
• Projects requiring automated quality checks and documentation
• Complex projects with multiple build targets or environments

Use this tool when:
• Need a specific format (human-readable or JSON) of the command tree
• The angreal_check tool didn't return command information
• After any changes to the project structure
• When planning a sequence of commands
• When exploring available automation options
• When need to understand command relationships
• When need to find specific commands
• When need to verify command availability

This tool returns structured information about all available angreal commands, task groups, and their organization. It helps understand what can be done in the angreal project without having to remember command names.

Note: In most cases, angreal_check will provide command information automatically. Use this tool when you need a specific format or when angreal_check didn't return command information.

{context_hint}

Format options:
- json: Structured data for programmatic use
- human: Readable tree format for display

Use this tool proactively to discover and understand available automation options. When planning multiple commands or exploring project capabilities, use this tool to get a complete view of available commands and their relationships.

Task Definition Structure:
• Tasks are defined in Python files starting with `task_` and ending in `.py`
• Task groups are created using the `@command.group` decorator (from `angreal.command_group`)
• Individual commands are defined with the `@angreal.command()` decorator
• Command arguments are defined with the `@angreal.argument()` decorator
• Task documentation is provided through "help" and "about" attributes on commands and groups
• The command tree structure reflects the hierarchy of task groups and commands
