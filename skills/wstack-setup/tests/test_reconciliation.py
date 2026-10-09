"""Setup acceptance through disposable projects and their actual public CLI."""
import json
import shutil
import subprocess
import sys
from pathlib import Path

from support import ProjectFixture, SKILL, PROJECT


class ReconciliationTests(ProjectFixture):
    def test_missing_map_blocks_then_reconciled_map_and_repeat_apply_pass(self):
        self.setup("apply", "--team", "Example", "--linear-project", PROJECT)
        self.assertNotEqual(self.cli("doctor", "--json").returncode, 0)
        checked = self.setup("check", ok=False)
        self.assertIn("canonical", checked["operational_discovery"]["error"])
        feature = self.reconcile_map()
        planned = feature / "planned.md"
        planned.write_text((feature / "add.md").read_text().replace("# Add item", "# Planned outcome\n\nStatus: planned"))
        index = feature / "README.md"
        index.write_text(index.read_text() + "\n- [Planned outcome](./planned.md) Future behavior.\n")
        checked = self.setup("check")
        self.assertTrue(checked["ready"])
        self.assertEqual(checked["operational_discovery"]["feature_count"], 3)
        self.assertGreater(checked["checks"]["features:view"]["html_bytes"], 0)
        self.assertFalse(Path(checked["checks"]["features:view"]["output"]).exists())
        self.assertEqual(json.loads(self.cli("features:show", "planned", "--json").stdout)["feature"]["status"], "planned")
        before = self.snapshot()
        self.assertEqual(self.setup()["changed"], [])
        self.assertEqual(self.snapshot(), before)

    def test_existing_map_refresh_preserves_recipes_and_evidence(self):
        self.first_apply()
        feature = self.reconcile_map()
        evidence = feature.parent / "evidence/owner.txt"
        evidence.parent.mkdir()
        evidence.write_text("Retained owner evidence.\n")
        original = self.snapshot()
        self.assertEqual(self.setup()["changed"], [])
        self.assertEqual(self.snapshot(), original)
        recipe = feature / "add.md"
        recipe.write_text(recipe.read_text().replace("Add item lets", "Updated add behavior lets"))
        checked = self.setup("check")
        self.assertIn("Updated add behavior", checked["checks"]["features:show:add"]["output"])
        before = self.snapshot()
        self.assertEqual(self.setup()["changed"], [])
        self.assertEqual(self.snapshot(), before)
        self.assertEqual(evidence.read_text(), "Retained owner evidence.\n")

    def test_advertised_but_broken_commands_cannot_pass(self):
        self.first_apply()
        main = self.root / "tools/project-cli/src/main.rs"
        source = main.read_text()
        cases = {
            "features:list": 'println!("{{}}")',
            "features:show": 'println!("{{}}")',
            "features:check": 'println!("{{}}")',
            "features:view": 'println!("nothing generated")',
            "doctor": 'println!("{{}}")',
        }
        for command, body in cases.items():
            with self.subTest(command=command):
                main.write_text(source.replace("    if let Some(command) = commands::find(&name) {",
                    f'    if name == "{command}" {{ {body}; return 0; }}\n'
                    '    if let Some(command) = commands::find(&name) {'))
                checked = self.setup("check", ok=False)
                self.assertFalse(checked["operational_discovery"]["ready"])
                self.assertIn("error", checked["operational_discovery"])
        main.write_text(source)
        self.assertTrue(self.setup("check")["ready"])
        commands = main.parent / "commands.rs"
        commands.write_text(commands.read_text().replace('        "features:view",', '        "missing:view",'))
        checked = self.setup("check", ok=False)
        self.assertIn("Incomplete", checked["operational_discovery"]["error"])

    def test_old_metadata_gets_supplier_revision_and_repeat_is_unchanged(self):
        self.first_apply()
        state_path = self.root / "tools/project-cli/setup.json"
        state = json.loads(state_path.read_text())
        state.pop("template")
        state_path.write_text(json.dumps(state) + "\n")
        inspected = self.setup("inspect")
        self.assertIsNone(inspected["template"])
        revision = subprocess.check_output(["git", "-C", str(SKILL), "rev-parse", "HEAD"], text=True).strip()
        self.assertEqual(self.setup()["template"]["revision"], revision)
        state = json.loads(state_path.read_text())
        self.assertEqual(state["template"]["revision"], revision)
        self.assertIn("tools/project-cli/src/main.rs", state["hashes"])
        before = self.snapshot()
        self.assertEqual(self.setup()["changed"], [])
        self.assertEqual(self.snapshot(), before)

    def test_installed_template_without_supplier_git_reports_unknown(self):
        installed = Path(self.temp.name) / "installed/wstack-setup"
        shutil.copytree(SKILL, installed, ignore=shutil.ignore_patterns("__pycache__"))
        applied = subprocess.run([sys.executable, str(installed / "scripts/setup.py"), "apply", str(self.root),
                                  "--team", "Example", "--linear-project", PROJECT], text=True, capture_output=True)
        self.assertEqual(applied.returncode, 0, applied.stderr)
        provenance = json.loads(applied.stdout)["template"]
        self.assertIsNone(provenance["revision"])
        self.assertIsNone(provenance["dirty"])
        self.assertIn("unavailable", provenance["reason"])
