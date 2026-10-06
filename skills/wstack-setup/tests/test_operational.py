"""Operational discovery and preservation through real generated CLI processes."""
import hashlib
import json
import os
import shutil
import subprocess
from support import ProjectFixture, SKILL, PROJECT


class OperationalTests(ProjectFixture):
    def test_identity_and_map_discovery_do_not_claim_app_readiness(self):
        subprocess.run(["git", "init", self.temp.name], check=True, capture_output=True)
        subprocess.run(["git", "-C", self.temp.name, "-c", "user.name=Fixture", "-c", "user.email=fixture@example.test", "commit", "--allow-empty", "-m", "parent"], check=True, capture_output=True)
        self.setup("apply", "--name", 'A "quoted" \\ project\tΩ', "--team", "Example", "--linear-project", PROJECT)
        info = json.loads(self.cli("info", "--json").stdout)
        self.assertEqual(info["project"], 'A "quoted" \\ project\tΩ')
        self.assertIsNone(info["commit"])
        self.assertIsNone(info["dirty"])
        self.assertIsNone(info["feature_map"])
        self.assertIsNone(json.loads(self.cli("doctor", "--json").stdout)["app_ready"])
        self.assertNotEqual(self.cli("features:list", "--json").returncode, 0)
        repo = SKILL.parent.parent
        feature = self.root / ".agents/skills/verify-fixture/features"
        shutil.copytree(repo / "tests/fixtures/features-good", feature)
        (feature.parent / "SKILL.md").write_text("# Fixture verification\n")
        env = {**os.environ, "WSTACK_BIN": os.environ.get("WSTACK_BIN", str(repo / "cli/target/debug/wstack"))}
        listed = self.cli("features:list", "--json", env=env)
        self.assertEqual(listed.returncode, 0, listed.stderr)
        catalog = json.loads(listed.stdout)
        self.assertEqual(len(catalog["features"]), 2)
        self.assertEqual(self.cli("features:check", env=env).returncode, 0)
        mapped = json.loads(self.cli("info", "--json").stdout)
        self.assertEqual(mapped["feature_map"], str(feature))
        self.assertEqual(mapped["verification_skill"], str(feature.parent / "SKILL.md"))
        self.assertTrue(json.loads(self.cli("doctor", "--json", env=env).stdout)["feature_map_ready"])
        second = self.root / ".claude/skills/verify-duplicate/features"
        shutil.copytree(feature, second)
        self.assertNotEqual(self.cli("features:list", "--json", env=env).returncode, 0)
        self.assertEqual(json.loads(self.cli("info", "--json").stdout)["map_count"], 2)
        self.assertNotEqual(self.cli("doctor", "--json", env=env).returncode, 0)

    def test_custom_info_route_keeps_its_behavior_and_builtin_remains_discoverable(self):
        (self.root / "Cargo.toml").write_text('[package]\nname="fixture-app"\nversion="0.1.0"\nedition="2021"\n[workspace]\n')
        (self.root / "src").mkdir()
        (self.root / "src/main.rs").write_text('fn main() { println!("{}", std::env::args().nth(1).unwrap()); }\n')
        self.first_apply()
        routes = self.root / "tools/project-cli/src/routes.rs"
        routes.write_text(routes.read_text().replace(
            "];", '    Route { name: "info", program: "cargo", args: &["run", "--"] },\n'
            '    Route { name: "doctor", program: "cargo", args: &["run", "--"] },\n'
            + ''.join(f'    Route {{ name: "padding-{n}", program: "cargo", args: &[] }},\n' for n in range(120)) + "];"))
        result = self.cli("info", "owner argument")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stdout.strip(), "owner argument")
        self.assertEqual(self.cli("doctor", "owner doctor").stdout.strip(), "owner doctor")
        info = json.loads(self.cli("wstack:info", "--json").stdout)
        name = next(c["name"] for c in info["capabilities"] if c["id"] == "info")
        self.assertEqual(name, "wstack:info")
        self.assertIn("wstack:info --json", self.cli("--help").stdout)
        self.assertTrue(self.setup("check")["ready"])
        before = self.snapshot()
        self.assertIn("tools/project-cli/src/routes.rs", self.setup()["preserved"])
        self.assertEqual(self.snapshot(), before)
        self.assertEqual(self.cli("info", "still owned").stdout.strip(), "still owned")

    def legacy_installation(self, customized):
        (self.root / "Cargo.toml").write_text('[package]\nname="legacy-app"\nversion="0.1.0"\nedition="2021"\n[workspace]\n')
        (self.root / "src").mkdir()
        (self.root / "src/main.rs").write_text('fn main() { println!("{}", std::env::args().nth(1).unwrap()); }\n')
        self.first_apply()
        main = self.root / "tools/project-cli/src/main.rs"
        legacy = (SKILL / "tests/fixtures/legacy-main.rs").read_text()
        state_path = self.root / "tools/project-cli/setup.json"
        state = json.loads(state_path.read_text())
        state["hashes"]["tools/project-cli/src/main.rs"] = hashlib.sha256(legacy.encode()).hexdigest()
        content = legacy + ("// Owner-specific implementation.\n" if customized else "")
        main.write_text(content)
        for name in ["commands.rs", "json.rs", "metadata.rs", "operational.rs"]:
            (main.parent / name).unlink()
            state["hashes"].pop("tools/project-cli/src/" + name)
        state_path.write_text(json.dumps(state, indent=2) + "\n")
        self.assertEqual(self.cli("run", "owner behavior").stdout.strip(), "owner behavior")
        return main, content

    def test_unchanged_legacy_cli_upgrades_and_owner_routes_survive(self):
        main, legacy = self.legacy_installation(customized=False)
        result = self.setup()
        self.assertIn("tools/project-cli/src/main.rs", result["changed"])
        self.assertNotEqual(main.read_text(), legacy)
        self.assertTrue((main.parent / "operational.rs").exists())
        self.assertEqual(self.cli("run", "owner behavior").stdout.strip(), "owner behavior")
        self.assertEqual(json.loads(self.cli("info", "--json").stdout)["project"], self.root.name)
        self.assertTrue(self.setup("check")["ready"])
        before = self.snapshot()
        self.assertEqual(self.setup()["changed"], [])
        self.assertEqual(self.snapshot(), before)

    def test_legacy_custom_main_is_preserved_and_missing_adoption_is_blocked(self):
        main, custom = self.legacy_installation(customized=True)
        result = self.setup()
        self.assertIn("tools/project-cli/src/main.rs", result["preserved"])
        self.assertEqual(main.read_text(), custom)
        self.assertFalse((main.parent / "operational.rs").exists())
        self.assertIn("setup:operational-cli", (self.root / "SETUP-TODO.md").read_text())
        checked = self.setup("check", ok=False)
        self.assertFalse(checked["operational_discovery"]["ready"])
        self.assertEqual(self.cli("run", "owner behavior").stdout.strip(), "owner behavior")
        before = self.snapshot()
        self.assertEqual(self.setup()["changed"], [])
        self.assertEqual(self.snapshot(), before)

    def test_partial_or_nonobject_metadata_cannot_pass_adoption(self):
        self.first_apply()
        main = self.root / "tools/project-cli/src/main.rs"
        for response in ('{"schema":1}', '[]'):
            main.write_text('fn main() { match std::env::args().nth(1).as_deref() {\n'
                            'Some("--help") => println!("info --json [id:info]"),\n'
                            'Some("info") => println!("{}", ' + json.dumps(response) + '),\n'
                            '_ => () } }\n')
            checked = self.setup("check", ok=False)
            self.assertFalse(checked["ready"])
            self.assertFalse(checked["operational_discovery"]["ready"])
            self.assertIn("Incomplete", checked["operational_discovery"]["error"])
