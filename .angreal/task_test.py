import angreal
import subprocess

@angreal.command(name='test', about='Run the test suite')
def test():
    """Run the test suite"""
    subprocess.run(["cargo", "test", "--", "--nocapture"])