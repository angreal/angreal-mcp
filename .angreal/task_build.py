import angreal
import subprocess

@angreal.command(name='build', about='Build the project in release mode')
def build():
    """Build the project in release mode"""
    subprocess.run(["cargo", "build", "--release"])
