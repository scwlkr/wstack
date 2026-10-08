"""Brand forwarding through actual generated project CLIs, preserving owner routes."""
import json
import os
from support import ProjectFixture, SKILL


class BrandForwardingTests(ProjectFixture):
    def test_brand_forwarder_keeps_custom_command_and_error_status(self):
        self.first_apply()
        brand = self.root / "brand"
        brand.mkdir()
        style = {"colors": ["navy"], "type": ["sans"], "treatment": ["flat"], "exclude": ["gloss"]}
        (brand / "style.json").write_text(json.dumps(style))
        (brand / "brand.json").write_text(json.dumps({"guide": "index.html"}))
        guide = "<!doctype html><title>Owner guide</title><img src='mark.svg'>"
        (brand / "index.html").write_text(guide)
        (brand / "mark.svg").write_text(
            '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><path d="M0 0h10v10H0Z"/></svg>')
        routes = self.root / "tools/project-cli/src/routes.rs"
        routes.write_text(routes.read_text().replace(
            "];", '    Route { name: "brand", program: "python3", args: &["-c", "import sys; print(sys.argv[1])"] },\n];'))
        repo = SKILL.parent.parent
        env = {**os.environ, "WSTACK_BIN": os.environ.get("WSTACK_BIN", str(repo / "cli/target/debug/wstack"))}
        result = self.cli("wstack:brand", "style", env=env)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(json.loads(result.stdout), style)
        refreshed = self.cli("wstack:brand", "refresh", "--json", env=env)
        self.assertEqual(refreshed.returncode, 0, refreshed.stderr)
        self.assertEqual(json.loads(refreshed.stdout)["guide"], str((brand / "index.html").resolve()))
        self.assertEqual((brand / "index.html").read_text(), guide)
        listed = self.cli("wstack:brand", "list", "--query", "mark.svg", "--json", env=env)
        self.assertEqual(listed.returncode, 0, listed.stderr)
        self.assertEqual(json.loads(listed.stdout)["assets"][0]["path"], "mark.svg")
        repeated = self.cli("wstack:brand", "refresh", "--json", env=env)
        self.assertEqual(repeated.returncode, 0, repeated.stderr)
        self.assertEqual(json.loads(repeated.stdout)["changed"], [])
        self.assertEqual(self.cli("brand", "owner argument").stdout.strip(), "owner argument")
        self.assertIn("wstack:brand refresh|list|style", self.cli("--help").stdout)
        (brand / "style.json").write_text("{}")
        self.assertEqual(self.cli("wstack:brand", "style", env=env).returncode, 1)
        before = self.snapshot()
        self.assertEqual(self.setup()["changed"], [])
        self.assertEqual(self.snapshot(), before)
