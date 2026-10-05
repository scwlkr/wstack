"""A real file-backed draft CLI, separate from its verification driver."""
import json
import os
import sys
from pathlib import Path

run = Path(os.environ['WSTACK_EVIDENCE_DIR']).parent.name if 'WSTACK_EVIDENCE_DIR' in os.environ else 'manual'
state = Path('.state') / run / 'draft.json'
action = sys.argv[1]
if action == 'show':
    print(state.read_text())
elif action == 'save':
    title = sys.argv[2].strip()
    if not title:
        print('title required', file=sys.stderr)
        sys.exit(2)
    state.write_text(json.dumps({'title': title}))
    print('saved')
else:
    sys.exit(2)
