import angreal
import sys

# Create command group
call_testing_group = angreal.group(name="call-testing", about="Complex command group for testing MCP server")

@call_testing_group
@angreal.command(name="command-1", about="First test command that requires --option flag")
@angreal.argument(name='option', long='option', takes_value=False, is_flag=True, help='Required option flag', required=True)
def command_1(option=False):
    """First test command"""
    if not option:
        print("ERROR: command-1 requires --option flag", file=sys.stderr)
        sys.exit(1)

    print("✓ Successfully executed call-testing command-1 with --option flag")
    print("This demonstrates handling of boolean flags in complex commands")

@call_testing_group
@angreal.command(name="command-2", about="Second test command that requires --parameter <value>")
@angreal.argument(name='parameter', long='parameter', help='Required parameter with value', required=True)
def command_2(parameter=None):
    """Second test command"""
    if not parameter:
        print("ERROR: command-2 requires --parameter <value>", file=sys.stderr)
        sys.exit(1)

    print(f"✓ Successfully executed call-testing command-2 with parameter: {parameter}")
    print("This demonstrates handling of value parameters in complex commands")
