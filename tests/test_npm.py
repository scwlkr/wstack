"""Exercise the npm tarball and installed command outside the checkout."""
import importlib.util
import json
import os
import shutil
import subprocess
import tarfile
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("npm_installer", ROOT / "install.py")
INSTALLER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(INSTALLER)


class NpmPackageTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.node = shutil.which("node")
        cls.npm = shutil.which("npm")
        if not cls.node or not cls.npm:
            raise RuntimeError("Packed npm checks require Node.js and npm")
        cls.temp = tempfile.TemporaryDirectory(prefix="wstack-npm-test-")
        cls.addClassCleanup(cls.temp.cleanup)
        cls.workspace = Path(cls.temp.name)
        cls.cache = cls.workspace / "npm-cache"
        cls.environment = dict(os.environ, npm_config_cache=str(cls.cache),
                               npm_config_update_notifier="false")
        result = subprocess.run([cls.npm, "pack", "--json", "--ignore-scripts", "--offline",
                                 "--pack-destination", str(cls.workspace)],
                                cwd=ROOT, env=cls.environment, capture_output=True,
                                text=True, timeout=60, check=True)
        cls.metadata = json.loads(result.stdout)[0]
        cls.tarball = cls.workspace / cls.metadata["filename"]
        cls.extracted = cls.workspace / "package with spaces"
        with tarfile.open(cls.tarball, "r:gz") as archive:
            for member in archive.getmembers():
                parts = Path(member.name).parts
                if not parts or parts[0] != "package" or ".." in parts:
                    raise AssertionError(f"Unexpected packed path: {member.name}")
                target = cls.extracted.joinpath(*parts)
                if member.isdir():
                    target.mkdir(parents=True, exist_ok=True)
                elif member.isfile():
                    target.parent.mkdir(parents=True, exist_ok=True)
                    target.write_bytes(archive.extractfile(member).read())
                    target.chmod(member.mode)
                else:
                    raise AssertionError(f"Unexpected packed file type: {member.name}")
        cls.package = cls.extracted / "package"
        cls.cli = cls.package / "bin/wstack.cjs"
        cls.outside = cls.workspace / "working directory with spaces"
        cls.outside.mkdir()

    def setUp(self):
        self.home = self.workspace / self._testMethodName / "home with spaces"
        self.configured = self.home.parent / "configured roots"
        self.environment = dict(self.__class__.environment, HOME=str(self.home),
                                USERPROFILE=str(self.home),
                                CODEX_HOME=str(self.configured / "codex"),
                                CLAUDE_CONFIG_DIR=str(self.configured / "claude"),
                                XDG_STATE_HOME=str(self.configured / "state"),
                                PYTHONDONTWRITEBYTECODE="1")

    def run_cli(self, *args, success=True, environment=None):
        result = subprocess.run([self.node, str(self.cli), *args], cwd=self.outside,
                                env=environment or self.environment, capture_output=True,
                                text=True, timeout=30)
        self.assertEqual(result.returncode, 0 if success else 1, result.stdout + result.stderr)
        return result

    def assert_installed(self):
        expected = INSTALLER.folder_hash(self.package / "skills/wstack").hex()
        for root in INSTALLER.global_roots(self.home, self.environment):
            self.assertEqual(INSTALLER.folder_hash(root / "wstack").hex(), expected)
        for lock in INSTALLER.lock_paths(self.home, self.environment):
            self.assertEqual(INSTALLER.read_lock(lock)["skills"]["wstack"]["skillFolderHash"], expected)
        return expected

    def test_tarball_contains_exact_skill_and_only_distribution_files(self):
        packed = {entry["path"] for entry in self.metadata["files"]}
        source = ROOT / "skills/wstack"
        expected_skill = {"skills/wstack/" + path.relative_to(source).as_posix()
                          for path in source.rglob("*") if path.is_file()}
        expected = expected_skill | {"README.md", "LICENSE", "NOTICE", "package.json",
                                    "install.py", "bin/install.py", "bin/wstack.cjs"}
        self.assertEqual(packed, expected)
        self.assertEqual(INSTALLER.folder_hash(self.package / "skills/wstack"),
                         INSTALLER.folder_hash(source))
        self.assertTrue(self.cli.stat().st_mode & 0o111)
        package = json.loads((self.package / "package.json").read_text())
        self.assertEqual(package["bin"], {"wstack": "bin/wstack.cjs"})
        self.assertFalse({"preinstall", "install", "postinstall"} & package.get("scripts", {}).keys())

    def test_npx_package_installs_then_updates_without_fetching_github(self):
        canonical = self.home / ".agents/skills"
        unrelated = canonical / "unrelated"
        unrelated.mkdir(parents=True)
        (unrelated / "owner.txt").write_text("Keep me\n")
        retired = canonical / "wstack-restate"
        retired.mkdir()
        (retired / "SKILL.md").write_text("Retired Wstack\n")
        active_lock = Path(self.environment["XDG_STATE_HOME"]) / "skills/.skill-lock.json"
        active_lock.parent.mkdir(parents=True)
        owner_entry = {"source": "owner/unrelated", "installedAt": "original"}
        active_lock.write_text(json.dumps({"version": 3, "skills": {"unrelated": owner_entry}}))
        # The command must install its bundled skill even when network access fails.
        offline = dict(self.environment, HTTPS_PROXY="http://127.0.0.1:9",
                       HTTP_PROXY="http://127.0.0.1:9", ALL_PROXY="http://127.0.0.1:9",
                       NO_PROXY="")
        result = subprocess.run([self.npm, "exec", "--yes", "--offline", "--ignore-scripts",
                                 "--package", str(self.tarball), "--", "wstack"],
                                cwd=self.outside, env=offline, capture_output=True,
                                text=True, timeout=60)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn(f"Installed Wstack {self.metadata['version']}", result.stdout)
        self.assert_installed()
        self.assertFalse(retired.exists())
        self.assertEqual((unrelated / "owner.txt").read_text(), "Keep me\n")
        first_lock = INSTALLER.read_lock(active_lock)
        self.assertEqual(first_lock["skills"]["unrelated"], owner_entry)
        stale = canonical / "wstack/stale.txt"
        stale.write_text("Old package leftover\n")
        (canonical / "wstack/SKILL.md").write_text("Stale copy\n")
        self.run_cli("update", environment=offline)
        self.assert_installed()
        self.assertFalse(stale.exists())
        second_lock = INSTALLER.read_lock(active_lock)
        self.assertEqual(second_lock["skills"]["wstack"]["installedAt"],
                         first_lock["skills"]["wstack"]["installedAt"])
        self.assertEqual(second_lock["skills"]["unrelated"], owner_entry)
        self.assertEqual((unrelated / "owner.txt").read_text(), "Keep me\n")
        self.assertFalse(list(self.home.rglob(".wstack-install-*")))
        self.assertFalse(list(self.configured.rglob(".wstack-install-*")))
        self.assertFalse(list(self.package.rglob("__pycache__")))

    def test_help_version_and_bad_arguments_do_not_modify_home(self):
        self.assertIn("npx wstack@latest", self.run_cli("--help").stdout)
        self.assertEqual(self.run_cli("--version").stdout.strip(),
                         f"wstack {self.metadata['version']}")
        self.assertIn("Unknown arguments", self.run_cli("remove", success=False).stderr)
        self.assertIn("Unknown arguments", self.run_cli("install", "--bogus", success=False).stderr)
        self.assertFalse(self.home.exists())
        self.assertFalse(self.configured.exists())

    def test_missing_python_fails_before_home_mutation(self):
        without_python = dict(self.environment, PATH="")
        self.assertIn("requires Python 3.8+", self.run_cli(success=False, environment=without_python).stderr)
        self.assertIn("npx wstack@latest", self.run_cli("--help", environment=without_python).stdout)
        self.assertFalse(self.home.exists())
        self.assertFalse(self.configured.exists())


if __name__ == "__main__":
    unittest.main()
