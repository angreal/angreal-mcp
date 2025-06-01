import angreal
import subprocess

@angreal.command(name='check', about='Check code without building')
def check():
    """Check code without building"""
    subprocess.run(["cargo", "check"])