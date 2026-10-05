#!/usr/bin/env python3
"""Prove browser/DevTools access and detect a false-success save against a real UI."""
import argparse
import json
import os
import shutil
import subprocess
import tempfile
from pathlib import Path

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--wstack', type=Path, required=True)
parser.add_argument('--playwright-module', type=Path, required=True)
parser.add_argument('--output', type=Path, required=True)
args = parser.parse_args()
args.output.mkdir(parents=True, exist_ok=True)
fixture = Path(__file__).resolve().parents[1] / 'fixtures/eyes-browser'
summary = []
with tempfile.TemporaryDirectory(prefix='wstack-browser-') as temporary:
    for broken in (False, True):
        name = 'broken-save' if broken else 'working-save'
        root = Path(temporary) / name
        shutil.copytree(fixture, root)
        env = {**os.environ, 'WSTACK_PLAYWRIGHT_MODULE': str(args.playwright_module.resolve())}
        env.pop('EYES_FIXTURE_BREAK_SAVE', None)
        if broken:
            env['EYES_FIXTURE_BREAK_SAVE'] = '1'
        command = [str(args.wstack.resolve()), 'eyes-and-hands', 'check', '--root', str(root),
                   '--json', '--run', '--output', str(args.output.resolve() / name)]
        result = subprocess.run(command, env=env, text=True, capture_output=True, timeout=90)
        if not result.stdout:
            raise RuntimeError(result.stderr)
        report = json.loads(result.stdout)
        assert result.returncode == (1 if broken else 0), report
        assert report['ok'] is not broken, report
        assert not (root / '.state').exists(), 'owned state survived cleanup'
        steps = report['executions'][0]['steps']
        assert len(steps) == 5, report
        assert steps[4]['exit'] == 0, report
        if broken:
            assert steps[3]['exit'] == 1, report
            assert 'fresh UI must read the persisted title' in Path(steps[3]['stderr']).read_text()
        else:
            evidence = Path(steps[2]['stdout']).parent
            assert (evidence / 'assert.png').is_file()
            devtools = json.loads((evidence / 'act.devtools.json').read_text())
            statuses = [entry['status'] for entry in devtools['network'] if entry['url'] == '/draft']
            assert 200 in statuses and 400 in statuses, devtools
            assert 'Plan trip' in Path(steps[3]['stdout']).read_text()
        summary.append({'scenario': name, 'exit': result.returncode, 'receipt': report['receipt']})
        print(f'{name}: PASS', flush=True)
(args.output / 'qualification.json').write_text(json.dumps(summary, indent=2) + '\n')
