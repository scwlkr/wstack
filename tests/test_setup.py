"""Focused behavior checks for the standalone setup helper."""
import json
import shutil
import subprocess
import sys
import tempfile
import time
import unittest
from pathlib import Path

SCRIPT = Path(__file__).resolve().parents[1] / "skills/wstack/scripts/setup.py"


class SetupTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="wstack-setup-test-")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name) / "project with spaces"
        self.root.mkdir()

    def apply(self, success=True):
        result = subprocess.run([sys.executable, str(SCRIPT), str(self.root), "--apply"],
                                capture_output=True, text=True, timeout=10)
        self.assertEqual(result.returncode, 0 if success else 1, result.stderr)
        return json.loads(result.stdout)

    def test_fresh_setup_cli_and_owner_preserving_upgrade(self):
        self.assertEqual(self.apply()["status"], "ready")
        cli = subprocess.run([str(self.root / "project"), "--help"], cwd=self.temp.name,
                             capture_output=True, text=True, timeout=20)
        self.assertEqual(cli.returncode, 0, cli.stderr)
        self.assertIn("Usage:", cli.stdout)
        source = self.root / "tools/project-cli/main.rs"
        source.write_text(source.read_text() + "\n// Owner addition\n")
        home = self.root / "w.md"
        home.write_text(home.read_text().replace("Wstack: 0.2.0", "Wstack: 0.1.0") + "\nOwner decision\n")
        agents = self.root / "AGENTS.md"
        agents.write_text(agents.read_text() + "\nOwner rule\n")
        self.assertEqual(self.apply()["status"], "ready")
        self.assertIn("Owner decision", home.read_text())
        self.assertIn("Owner rule", agents.read_text())
        self.assertIn("Owner addition", source.read_text())
        before = home.read_bytes()
        self.assertEqual(self.apply()["status"], "current")
        self.assertEqual(home.read_bytes(), before)
        home.write_text(home.read_text().replace("Wstack: 0.2.0", "Wstack: 9.0.0"))
        self.assertEqual(self.apply()["status"], "current")
        self.assertIn("Wstack: 9.0.0", home.read_text())
        self.assertFalse((self.root / ".wstack-setup.lock").exists())

    def test_existing_commands_and_documents_survive(self):
        launcher = self.root / "project"
        launcher.write_text("#!/bin/sh\nprintf '%s\\n' owner-command\nexit 7\n")
        launcher.chmod(0o755)
        (self.root / "w.md").write_text("Owner notes\n")
        (self.root / "AGENTS.md").write_text("Owner rule\n<!-- wstack-setup:start -->\nOld generated receipt rules\n<!-- wstack-setup:end -->\n")
        (self.root / "FEATURES.md").write_text("# Bookmarks\nOwner feature\n")
        original = launcher.read_bytes()
        self.apply()
        self.assertEqual(launcher.read_bytes(), original)
        out = subprocess.run([str(launcher)], capture_output=True, text=True)
        self.assertEqual(out.returncode, 7)
        self.assertEqual(out.stdout.strip(), "owner-command")
        self.assertTrue((self.root / "w.md").read_text().startswith("Owner notes\n"))
        self.assertIn("Owner feature", (self.root / "FEATURES.md").read_text())
        self.assertIn("Owner rule", (self.root / "AGENTS.md").read_text())
        self.assertNotIn("Old generated receipt rules", (self.root / "AGENTS.md").read_text())
        self.assertFalse((self.root / "tools/project-cli").exists())
        launcher.unlink()
        (self.root / "package.json").write_text(json.dumps({"scripts": {"dev": "vite"}, "packageManager": "pnpm@10"}))
        (self.root / "w.md").unlink()
        self.assertEqual(self.apply()["cli"], "pnpm run")
        self.assertFalse(launcher.exists())

    def test_malformed_marker_and_symlink_leave_owner_work_untouched(self):
        home = self.root / "w.md"
        text = "Owner text\n<!-- wstack:begin setup -->\nUnclosed\n"
        home.write_text(text)
        self.assertEqual(self.apply(success=False)["status"], "failed")
        self.assertEqual(home.read_text(), text)
        self.assertFalse((self.root / "project").exists())
        self.assertFalse((self.root / ".wstack-setup.lock").exists())
        home.unlink()
        external = Path(self.temp.name) / "shared-notes.md"
        external.write_text("Shared owner notes\n")
        home.symlink_to(external)
        self.apply(success=False)
        self.assertEqual(external.read_text(), "Shared owner notes\n")
        self.assertTrue(home.is_symlink())

    def test_background_launch_returns_before_worker_and_reports_result(self):
        start = time.monotonic()
        out = subprocess.run([sys.executable, str(SCRIPT), str(self.root)],
                             capture_output=True, text=True, timeout=2)
        self.assertEqual(out.returncode, 0, out.stderr)
        self.assertLess(time.monotonic() - start, 2)
        report = Path(out.stdout.strip())
        self.addCleanup(shutil.rmtree, report.parent, ignore_errors=True)
        deadline = time.monotonic() + 5
        while time.monotonic() < deadline:
            try:
                result = json.loads(report.read_text())
            except json.JSONDecodeError:
                result = {"status": "running"}
            if result["status"] != "running":
                break
            time.sleep(0.01)
        self.assertEqual(result["status"], "ready", result)
        self.assertIn("Wstack: 0.2.0", (self.root / "w.md").read_text())
        self.assertFalse((self.root / ".wstack-setup.lock").exists())


if __name__ == "__main__":
    unittest.main()
