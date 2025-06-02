Check if the current directory is an angreal project and get project status. Angreal is a project automation tool that helps standardize and automate development workflows.

Use this tool when:
• Starting any new interaction or conversation
• Entering a new directory
• Before suggesting any angreal commands
• Before using angreal_run or angreal_tree
• When project status is unknown
• When unsure about available automation
• When troubleshooting project issues
• When planning next actions

This tool will:
• Detect if angreal is installed and available
• Check if current directory is an angreal project (.angreal/ folder exists)
• Determine project initialization status
• Provide guidance on next steps if project is not initialized
• Return available commands if project is properly initialized

The response includes:
1. Project status (installation, initialization)
2. Available commands (if project is initialized)
3. Next steps if project needs setup

{context_hint}

This tool provides all necessary information in one call - no need to call angreal_tree separately if commands are available. Use this tool proactively at the start of any interaction to understand the project context. If the project status changes (e.g., after initialization or setup), use this tool again to get updated information.

Project Structure:
• Angreal projects have a `.angreal` directory at the root
• Task files are Python scripts starting with `task_` and ending in `.py`
• Tasks are organized using command groups and decorators
• Commands are defined with `@angreal.command()` decorator
• Arguments are defined with `@angreal.argument()` decorator
• Task groups are created with `@command.group` decorator
• Documentation is provided through "help" and "about" attributes
