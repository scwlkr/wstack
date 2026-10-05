"""Drive the public CLI and read effects independently; owns only its scratch state."""
import json
import os
import shutil
import subprocess
import sys
import time
from pathlib import Path

stage = sys.argv[1]
title = sys.argv[2] if len(sys.argv) > 2 else 'Plan trip'
mode = os.environ.get('EYES_TEST_MODE', '')
run = Path(os.environ['WSTACK_EVIDENCE_DIR']).parent.name if 'WSTACK_EVIDENCE_DIR' in os.environ else 'manual'
state = Path('.state') / run
file = state / 'draft.json'


def app(*args):
    return subprocess.run([sys.executable, 'app.py', *args], capture_output=True, text=True)


if stage == 'setup':
    state.mkdir(parents=True)
    file.write_text(json.dumps({'title': 'Original'}))
elif stage == 'observe':
    result = app('show')
    assert result.returncode == 0
    assert json.loads(result.stdout)['title'] == 'Original'
    print(result.stdout)
elif stage == 'act':
    if mode == 'fail':
        print('driver cannot reach the save operation', file=sys.stderr)
        sys.exit(9)
    if mode == 'hang':
        # The marker lets the cancellation test signal a running stage deterministically.
        (state / 'acting').touch()
        subprocess.Popen([sys.executable, '-c',
                          'import time,os; from pathlib import Path; time.sleep(0.7); '
                          'Path(os.environ["WSTACK_EVIDENCE_DIR"], "orphan").touch()'])
        time.sleep(30)
    saved = app('save', title)
    assert saved.returncode == 0, saved.stderr
    invalid = app('save', '')
    assert invalid.returncode == 2 and 'title required' in invalid.stderr
    print(saved.stdout)
elif stage == 'assert':
    result = app('show')
    assert result.returncode == 0
    actual = json.loads(file.read_text())
    assert actual == {'title': title}, actual
    assert json.loads(result.stdout) == actual
    print(json.dumps({'persisted': actual}))
elif stage == 'cleanup':
    if mode == 'cleanup-fail':
        print('cleanup failed', file=sys.stderr)
        sys.exit(17)
    shutil.rmtree(state, ignore_errors=True)
    try:
        state.parent.rmdir()
    except OSError:
        pass
    if mode == 'erase-evidence':
        Path(os.environ['WSTACK_EVIDENCE_DIR'], 'observe.stdout.log').unlink()
    print('owned state removed')
else:
    sys.exit(2)
