"""Install the skill bundled in the npm package, without downloading main."""
import importlib.util
import json
import os
import sys
from pathlib import Path


def main():
    root = Path(__file__).resolve().parents[1]
    spec = importlib.util.spec_from_file_location("wstack_installer", root / "install.py")
    installer = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(installer)
    skill = root / "skills/wstack"
    installer.validate_skill(skill)
    version = json.loads((root / "package.json").read_text(encoding="utf-8"))["version"]
    installer.install(skill, Path.home(), os.environ, version=version)


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError) as error:
        print(f"Wstack install failed: {error}", file=sys.stderr)
        sys.exit(1)
