import angreal
import subprocess

@angreal.command(name='fmt', about='Format code using cargo fmt')
def fmt():
    """Format code using cargo fmt"""
    subprocess.run(["cargo", "fmt"])