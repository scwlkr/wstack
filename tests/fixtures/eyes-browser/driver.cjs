const fs = require('node:fs/promises');
const path = require('node:path');
const assert = require('node:assert/strict');
const {stateDir, start} = require('./server.cjs');

async function drive(stage, title) {
  const {chromium} = require(process.env.WSTACK_PLAYWRIGHT_MODULE || 'playwright');
  const instance = await start();
  let browser;
  try {
    browser = await chromium.launch({headless: true});
    const context = await browser.newContext();
    const page = await context.newPage();
    const errors = [];
    const consoleMessages = [];
    const network = [];
    page.on('pageerror', error => errors.push(String(error)));
    page.on('console', message => consoleMessages.push({type: message.type(), text: message.text()}));
    const cdp = await context.newCDPSession(page);
    await cdp.send('Network.enable');
    cdp.on('Network.responseReceived', event => network.push({
      url: new URL(event.response.url).pathname, status: event.response.status
    }));
    await page.goto(instance.url);
    const input = page.getByRole('textbox', {name: 'Title'});
    await input.waitFor({state: 'visible'});
    await page.waitForFunction(() => !document.querySelector('#title').disabled);
    if (stage === 'observe') {
      assert.equal(await input.inputValue(), 'Original');
      const layout = await input.evaluate(element => ({
        width: element.getBoundingClientRect().width,
        display: getComputedStyle(element).display
      }));
      assert.ok(layout.width > 0);
      console.log(JSON.stringify({initialTitle: await input.inputValue(), layout}));
    } else if (stage === 'act') {
      await input.fill(title);
      const saved = page.waitForResponse(response => response.url().endsWith('/draft') && response.request().method() === 'POST');
      await page.getByRole('button', {name: 'Save'}).click();
      assert.equal((await saved).status(), 200);
      await page.waitForFunction(() => document.querySelector('[role="status"]').textContent === 'Saved');
      await input.fill('');
      const invalid = page.waitForResponse(response => response.url().endsWith('/draft') && response.request().method() === 'POST');
      await page.getByRole('button', {name: 'Save'}).click();
      assert.equal((await invalid).status(), 400);
      await page.waitForFunction(() => document.querySelector('[role="status"]').textContent === 'title required');
      console.log(JSON.stringify({validation: await page.getByRole('status').textContent()}));
    } else if (stage === 'assert') {
      assert.equal(await input.inputValue(), title, 'fresh UI must read the persisted title');
      const persisted = JSON.parse(await fs.readFile(path.join(stateDir(), 'draft.json'), 'utf8'));
      assert.deepEqual(persisted, {title}, 'independent disk read must match');
      console.log(JSON.stringify({reopened: await input.inputValue(), persisted}));
    } else {
      throw new Error(`Unknown browser action: ${stage}`);
    }
    assert.deepEqual(errors, []);
    const evidence = process.env.WSTACK_EVIDENCE_DIR;
    await page.screenshot({path: path.join(evidence, `${stage}.png`)});
    await fs.writeFile(path.join(evidence, `${stage}.devtools.json`), JSON.stringify({network, errors, consoleMessages}, null, 2));
    console.log(JSON.stringify({network, errors, consoleMessages}));
  } finally {
    if (browser) await browser.close();
    await new Promise(resolve => instance.server.close(resolve));
  }
}

async function main() {
  const [stage, title = 'Plan trip'] = process.argv.slice(2);
  const state = stateDir();
  if (stage === 'setup') {
    await fs.mkdir(state, {recursive: true});
    await fs.writeFile(path.join(state, 'draft.json'), JSON.stringify({title: 'Original'}));
  } else if (stage === 'cleanup') {
    await fs.rm(state, {recursive: true, force: true});
    try { await fs.rmdir(path.dirname(state)); } catch (error) {
      if (!['ENOTEMPTY', 'ENOENT'].includes(error.code)) throw error;
    }
  } else {
    await drive(stage, title);
  }
}

main().catch(error => {console.error(error); process.exitCode = 1;});
