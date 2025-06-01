import angreal
import subprocess

@angreal.command(name='lint', about='Run clippy linter with warnings as errors')
def lint():
    """Run clippy linter with warnings as errors"""
    subprocess.run(["cargo", "clippy", "--", "-D", "warnings"])