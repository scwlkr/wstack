"""Reusable disposable project fixture; public setup and CLI interfaces only."""
import json
import os
import shutil
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
        self.env = {**os.environ, "WSTACK_BIN": os.environ.get("WSTACK_BIN", str(SKILL.parent.parent / "cli/target/debug/wstack"))}

    def setup(self, action="apply", *args, ok=True, env=None):
        result = subprocess.run([sys.executable, str(SCRIPT), action, str(self.root), *args],
                                text=True, capture_output=True, env=env or self.env)
        self.assertEqual(result.returncode, 0 if ok else 1, result.stdout + result.stderr)
        return json.loads(result.stdout if result.stdout else result.stderr)

    def first_apply(self):
        applied = self.setup("apply", "--team", "Example", "--linear-project", PROJECT)
        self.reconcile_map()
        return applied

    def reconcile_map(self):
        # Model the agent's bounded map-creation phase using the existing format fixture.
        feature = self.root / ".agents/skills/verify-fixture/features"
        if not feature.exists():
            shutil.copytree(SKILL.parent.parent / "tests/fixtures/features-good", feature)
            (feature.parent / "SKILL.md").write_text("# Fixture verification\n")
        return feature

    def cli(self, *args, env=None):
        return subprocess.run([str(self.root / "project"), *args], cwd=self.temp.name,
                              text=True, capture_output=True, env=env or self.env)

    def snapshot(self):
        return {str(path.relative_to(self.root)): (path.read_bytes(), path.stat().st_mtime_ns)
                for path in self.root.rglob("*") if path.is_file() and "target" not in path.parts}
