#!/usr/bin/env node
'use strict';

const path = require('node:path');
const { spawnSync } = require('node:child_process');
const { version } = require('../package.json');

const help = `wstack [install|update|--help|--version]
Install or replace the global Wstack skill with the version in this package.
Requires Python 3.8+; Windows can use the py launcher.
Get the newest published version with: npx wstack@latest
`;
const args = process.argv.slice(2);
const command = args[0] || 'install';

if (args.length > 1 || !['install', 'update', '--help', '-h', '--version', '-V'].includes(command)) {
  console.error('Unknown arguments. Use wstack --help.');
  process.exitCode = 1;
} else if (['--help', '-h'].includes(command)) {
  process.stdout.write(help);
} else if (['--version', '-V'].includes(command)) {
  console.log(`wstack ${version}`);
} else {
  const candidates = process.platform === 'win32'
    ? [['py', ['-3']], ['python3', []], ['python', []]]
    : [['python3', []], ['python', []]];
  const python = candidates.find(([executable, prefix]) => {
    const result = spawnSync(executable, [...prefix, '-B', '-c',
      'import sys; sys.exit(0 if sys.version_info >= (3, 8) else 1)'],
    { stdio: 'ignore', windowsHide: true, timeout: 5000 });
    return !result.error && result.status === 0;
  });
  if (!python) {
    console.error('Wstack requires Python 3.8+. Install Python, then run npx wstack@latest again.');
    process.exitCode = 1;
  } else {
    const [executable, prefix] = python;
    const result = spawnSync(executable, [...prefix, '-B', '-X', 'utf8', path.join(__dirname, 'install.py')], {
      stdio: 'inherit', windowsHide: true,
      env: { ...process.env, PYTHONDONTWRITEBYTECODE: '1' },
    });
    if (result.error) console.error(`Wstack could not start Python: ${result.error.message}`);
    process.exitCode = result.status === null ? 1 : result.status;
  }
}
