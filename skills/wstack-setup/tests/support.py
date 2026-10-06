"""Reusable disposable project fixture; public setup and CLI interfaces only."""
import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

SKILL = Path(__file__).resolve().parents[1]
SCRIPT = SKILL / "scripts/setup.py"
PROJECT = "https://linear.app/example/project/sample-123"

class ProjectFixture(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="wstack-setup-")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name) / "project with spaces"
        self.root.mkdir()

    def setup(self, action="apply", *args, ok=True, env=None):
        result = subprocess.run([sys.executable, str(SCRIPT), action, str(self.root), *args],
                                text=True, capture_output=True, env=env)
        self.assertEqual(result.returncode, 0 if ok else 1, result.stdout + result.stderr)
        return json.loads(result.stdout if result.stdout else result.stderr)

    def first_apply(self):
        return self.setup("apply", "--team", "Example", "--linear-project", PROJECT)

    def cli(self, *args, env=None):
        return subprocess.run([str(self.root / "project"), *args], cwd=self.temp.name,
                              text=True, capture_output=True, env=env)

    def snapshot(self):
        return {str(path.relative_to(self.root)): (path.read_bytes(), path.stat().st_mtime_ns)
                for path in self.root.rglob("*") if path.is_file() and "target" not in path.parts}
