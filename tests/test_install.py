"""Installer behavior: replacement, selective cleanup, retries and rollback."""
import importlib.util
import io
import json
import os
import subprocess
import tempfile
import unittest
import zipfile
from contextlib import redirect_stdout
from pathlib import Path
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("installer", ROOT / "install.py")
installer = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(installer)


class InstallTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="wstack-install-test-")
        self.addCleanup(self.temp.cleanup)
        self.home = Path(self.temp.name) / "home with spaces"
        self.skill = ROOT / "skills/wstack"
        self.roots = installer.global_roots(self.home, {})
        self.lock_path = self.home / ".agents/.skill-lock.json"

    def write_skill(self, root, name, text="old skill"):
        path = root / name
        path.mkdir(parents=True, exist_ok=True)
        (path / "SKILL.md").write_text(text)
        return path

    def write_lock(self, path=None, **skills):
        path = path or self.lock_path
        path.parent.mkdir(parents=True, exist_ok=True)
        data = {"version": 3, "skills": skills, "dismissed": {"findSkillsPrompt": True}}
        path.write_text(json.dumps(data))
        return data

    def apply(self, environment=None):
        with redirect_stdout(io.StringIO()):
            installer.install(self.skill, self.home, environment or {})

    def snapshot(self):
        result = {}
        if self.home.exists():
            for path in self.home.rglob("*"):
                key = str(path.relative_to(self.home))
                if path.is_symlink():
                    result[key] = ("link", os.readlink(path))
                elif path.is_file():
                    result[key] = ("file", path.read_bytes())
        return result

    def test_fresh_install_and_repeat_replace_stale_files(self):
        self.apply()
        for root in self.roots:
            installer.validate_skill(root / "wstack")
        (self.roots[0] / "wstack/stale.md").write_text("obsolete")
        first = json.loads(self.lock_path.read_text())["skills"]["wstack"]
        self.apply()
        self.assertFalse((self.roots[0] / "wstack/stale.md").exists())
        second = json.loads(self.lock_path.read_text())["skills"]["wstack"]
        self.assertEqual(first["installedAt"], second["installedAt"])
        self.assertEqual(first["skillFolderHash"], second["skillFolderHash"])
        self.assertFalse(list(self.home.rglob(".wstack-install-*")))

    def test_retires_known_members_and_preserves_unrelated_work(self):
        for root in self.roots:
            for name in installer.RETIRED:
                self.write_skill(root, name)
            self.write_skill(root, "wstack-custom", "owner extension")
            self.write_skill(root, "plain-english", "other publisher")
        original = self.write_lock(
            unrelated={"source": "owner/other", "custom": 7},
            **{"wstack-brand": {"source": installer.SOURCE}})
        project = self.home / "projects/repo"
        self.write_skill(project / ".agents/skills", "wstack-brand")
        (project / "AGENTS.md").write_text("owner notes")
        self.apply()
        for root in self.roots:
            for name in installer.RETIRED:
                self.assertFalse(os.path.lexists(root / name))
            self.assertEqual((root / "wstack-custom/SKILL.md").read_text(), "owner extension")
            self.assertEqual((root / "plain-english/SKILL.md").read_text(), "other publisher")
        lock = json.loads(self.lock_path.read_text())
        self.assertEqual(lock["skills"]["unrelated"], original["skills"]["unrelated"])
        self.assertEqual(lock["dismissed"], original["dismissed"])
        self.assertNotIn("wstack-brand", lock["skills"])
        self.assertTrue((project / ".agents/skills/wstack-brand").exists())
        self.assertEqual((project / "AGENTS.md").read_text(), "owner notes")

    def test_generic_names_need_provenance_at_each_location(self):
        for name in installer.SHARED:
            self.write_skill(self.roots[0], name, "older copy without markers")
            self.write_skill(self.roots[1], name, f"<!-- wstack:begin shared/{name}.md -->")
            self.write_skill(self.roots[2], name, "other publisher")
        self.write_lock(**{name: {"source": installer.SOURCE} for name in installer.SHARED})
        self.apply()
        for name in installer.SHARED:
            self.assertFalse((self.roots[0] / name).exists())
            self.assertFalse((self.roots[1] / name).exists())
            self.assertEqual((self.roots[2] / name / "SKILL.md").read_text(), "other publisher")
        lock = json.loads(self.lock_path.read_text())
        self.assertFalse(installer.SHARED & lock["skills"].keys())

    def test_symlinks_are_unlinked_without_removing_external_sources(self):
        external = Path(self.temp.name) / "external"
        self.write_skill(external, "wstack", "source Wstack")
        self.write_skill(external, "wstack-brand", "source brand")
        self.roots[0].mkdir(parents=True)
        (self.roots[0] / "wstack").symlink_to(external / "wstack")
        (self.roots[0] / "wstack-brand").symlink_to(external / "wstack-brand")
        (self.roots[0] / "wstack-debug").symlink_to(external / "missing")
        self.apply()
        self.assertEqual((external / "wstack/SKILL.md").read_text(), "source Wstack")
        self.assertEqual((external / "wstack-brand/SKILL.md").read_text(), "source brand")
        self.assertFalse(os.path.lexists(self.roots[0] / "wstack-debug"))

    def test_configured_paths_and_directory_aliases(self):
        codex = self.home / "custom codex"
        claude = self.home / "custom claude"
        state = self.home / "state"
        environment = {"CODEX_HOME": str(codex), "CLAUDE_CONFIG_DIR": str(claude),
                       "XDG_STATE_HOME": str(state)}
        self.roots[0].mkdir(parents=True)
        self.roots[1].parent.mkdir(parents=True)
        self.roots[1].symlink_to(self.roots[0], target_is_directory=True)
        self.write_skill(codex / "skills", "wstack-setup")
        self.write_skill(claude / "skills", "wstack-brand")
        self.write_lock(**{"wstack-setup": {"source": installer.SOURCE}})
        self.write_lock(state / "skills/.skill-lock.json", keep={"source": "owner/keep"})
        self.apply(environment)
        for root in installer.global_roots(self.home, environment):
            installer.validate_skill(root / "wstack")
        self.assertTrue(self.roots[1].is_symlink())
        self.assertFalse((codex / "skills/wstack-setup").exists())
        self.assertFalse((claude / "skills/wstack-brand").exists())
        self.assertIn("keep", json.loads((state / "skills/.skill-lock.json").read_text())["skills"])
        self.assertNotIn("wstack-setup", json.loads(self.lock_path.read_text())["skills"])

    def test_invalid_lock_preserves_existing_install(self):
        self.write_skill(self.roots[0], "wstack")
        self.lock_path.write_text("{invalid json")
        before = self.snapshot()
        with self.assertRaises(ValueError):
            self.apply()
        self.assertEqual(self.snapshot(), before)

    def test_nested_configured_root_is_rejected_before_replacement(self):
        self.write_skill(self.roots[0], "wstack")
        before = self.snapshot()
        with self.assertRaisesRegex(ValueError, "Overlapping install paths"):
            self.apply({"CODEX_HOME": str(self.roots[0] / "wstack/codex")})
        self.assertEqual(self.snapshot(), before)

    def test_failed_replacements_and_lock_write_restore_previous_state(self):
        for root in self.roots:
            self.write_skill(root, "wstack", "old Wstack")
            self.write_skill(root, "wstack-brand", "old brand")
        self.write_lock(unrelated={"source": "owner/keep"})
        before = self.snapshot()
        original = os.replace
        # Fail installing a new core, an agent link, retiring a member, or updating the lock.
        for failure in (2, 4, 9, 13, 14):
            count = 0

            def fail_once(source, destination):
                nonlocal count
                count += 1
                if count == failure:
                    raise OSError("simulated replacement failure")
                return original(source, destination)

            with self.subTest(failure=failure), patch.object(installer.os, "replace", fail_once):
                with self.assertRaises(OSError):
                    self.apply()
            self.assertEqual(self.snapshot(), before)
            self.assertFalse(list(self.home.rglob(".wstack-install-*")))

    def test_folder_hash_matches_git(self):
        expected = subprocess.check_output(["git", "rev-parse", "HEAD:skills/wstack"], cwd=ROOT, text=True).strip()
        self.assertEqual(installer.folder_hash(self.skill).hex(), expected)

    def test_bad_archive_fails_before_install_and_keeps_existing_files(self):
        self.write_skill(self.roots[0], "wstack")
        before = self.snapshot()
        data = io.BytesIO()
        with zipfile.ZipFile(data, "w") as archive:
            archive.writestr("wstack-main/skills/wstack/../../escaped", "bad")
        with patch.object(installer.urllib.request, "urlopen", return_value=io.BytesIO(data.getvalue())):
            with self.assertRaisesRegex(ValueError, "Unsafe archive"):
                installer.download_skill(Path(self.temp.name) / "download")
        self.assertEqual(self.snapshot(), before)
        self.assertFalse((Path(self.temp.name) / "escaped").exists())


if __name__ == "__main__":
    unittest.main()
