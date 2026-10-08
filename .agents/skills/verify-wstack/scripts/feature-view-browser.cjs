// Real file:// rendering and clipboard readback. Playwright is verification-only.
"use strict";
const fs = require("node:fs/promises"), path = require("node:path");
const { execFileSync } = require("node:child_process");
const { createHash } = require("node:crypto"), { pathToFileURL } = require("node:url");
const assert = require("node:assert/strict"), { chromium, webkit } = require("playwright");
const root = path.resolve(process.argv[2]), base = process.argv[3] || "HEAD";
const destination = path.resolve(process.argv[4] || path.join(root, ".evidence", `feature-view-${Date.now()}`));
const binary = process.env.WSTACK_PROOF_BIN || path.join(root, "cli/target/debug/wstack");
const report = { feature: "feature-view", status: "incomplete", cleanup: false, coverage: [], browsers: [] };
const browsers = [];
let active = false, state;
const digest = bytes => createHash("sha256").update(bytes).digest("hex");
function cli(args) {
  const stdout = execFileSync(binary, args, { cwd: root, encoding: "utf8" });
  report.coverage.push({ command: [binary, ...args], stdout, status: "pass" });
  return stdout;
}
async function cleanup() {
  for (const browser of browsers) await browser.close();
  for (const browser of report.browsers) browser.closed = true;
  if (state) await fs.rm(state, { recursive: true });
  report.cleanup = true;
}
async function retain() {
  await fs.writeFile(path.join(destination, "report.json"), JSON.stringify(report, null, 2) + "\n");
}
for (const signal of ["SIGINT", "SIGTERM"]) process.once(signal, async () => {
  if (!active) process.exit(1);
  report.status = "incomplete"; report.error = signal;
  try { await cleanup(); await retain(); } finally { process.exit(1); }
});
async function paste(page, expected, artifact) {
  const field = await page.evaluateHandle(() => {
    const input = document.createElement("textarea"); document.body.append(input); input.focus(); return input;
  });
  await page.keyboard.press(process.platform === "darwin" ? "Meta+V" : "Control+V");
  const actual = await field.evaluate(input => input.value);
  assert.equal(actual, expected, "Actual clipboard paste must match source text");
  await fs.writeFile(artifact, actual);
  await field.evaluate(input => input.remove());
}
async function run() {
  await fs.mkdir(path.dirname(destination), { recursive: true });
  await fs.mkdir(destination, { recursive: false }); active = true;
  report.evidence = destination;
  report.identity = JSON.parse(cli(["info", "--json", "--base", base]));
  report.binary = { path: binary, sha256: digest(await fs.readFile(binary)) };
  state = path.join(destination, "state"); await fs.mkdir(state);
  const map = path.join(state, "map"); await fs.mkdir(map);
  const title = 'Find <img src=x onerror=alert(1)> & "text"';
  const description = 'A passage belongs to one edition. </script><script>alert(1)</script>';
  const goals = "- find: keep edition identity.\n- open: open the result in the reader.\n- retain: keep the complete passage reference.\n- copy-last: include this goal below the visible cell.";
  const source = `# ${title}\n\n${description}\n\n## Sub-features (goals)\n\n${goals}\n\n## How to get to it\n\nSearch.\n\n## Driving it with browser\n\nPreconditions: ready.\n\n## Proof\n\nRead the result.\n\n## Gotchas\n\nNone known yet.\n`;
  await fs.writeFile(path.join(map, "README.md"), "# Features\n- [Find](find.md)\n");
  await fs.writeFile(path.join(map, "find.md"), source);
  const expected = `Feature: ${title} (find)\nStatus: implemented\nDescription: ${description}\nGoals:\n${goals}\nSource: ${path.join(map, "find.md")}`;
  const fixtureHTML = path.join(destination, "fixture.html"), projectHTML = path.join(destination, "project.html");
  cli(["features", "view", "--root", map, "--no-open", "--output", fixtureHTML]);
  cli(["features", "view", "--no-open", "--output", projectHTML]);
  report.html_bytes = (await fs.stat(projectHTML)).size;
  report.template_bytes = (await fs.stat(path.join(root, "cli/src/feature_view.html"))).size;
  assert.ok(report.template_bytes < 4096, "UI shell must stay below 4 KiB");
  const artifactExpectations = [];
  for (const [engine, launcher] of Object.entries({ chromium, webkit })) {
    const browser = await launcher.launch({ headless: true }); browsers.push(browser);
    report.browsers.push({ engine, version: browser.version(), owned: true });
    const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
    const requests = [], errors = [];
    page.on("request", request => requests.push(request.url()));
    page.on("pageerror", error => errors.push(error.message));
    page.on("dialog", () => { throw Error("Source Markdown executed"); });
    await page.goto(pathToFileURL(fixtureHTML).href);
    assert.equal(await page.locator("tbody tr").count(), 1);
    assert.equal(await page.locator("tbody b").textContent(), title);
    assert.equal(await page.locator(".description").textContent(), description);
    assert.equal(await page.locator(".goals").textContent(), goals);
    assert.equal(await page.locator(".goals>div").evaluate(cell => cell.scrollHeight > cell.clientHeight), true, "Copy fixture must include goals below the visible cell");
    assert.equal(await page.locator("img").count(), 0);
    assert.equal(await page.locator("button").textContent(), "");
    const search = page.getByRole("searchbox");
    await search.fill("NO SUCH FEATURE"); assert.equal(await page.locator("tbody tr:visible").count(), 0);
    await search.fill("EDITION"); assert.equal(await page.locator("tbody tr:visible").count(), 1);
    await search.fill("");
    for (const fallback of [false, true]) {
      if (fallback) await page.evaluate(() => Object.defineProperty(navigator, "clipboard", { configurable: true, value: { writeText: async () => { throw Error("denied"); } } }));
      await page.getByRole("button", { name: "Copy " + title, exact: true }).click();
      await page.getByRole("button", { name: "Copied " + title, exact: true }).waitFor();
      const artifact = path.join(destination, `${engine}-${fallback ? "fallback" : "clipboard"}.txt`);
      await paste(page, expected, artifact); artifactExpectations.push({ artifact, expected });
      await page.getByRole("button", { name: "Copy " + title, exact: true }).waitFor();
    }
    await page.evaluate(() => { document.execCommand = () => false; });
    await page.getByRole("button", { name: "Copy " + title, exact: true }).click();
    const selected = await page.getByRole("textbox", { name: "Feature context to copy" }).evaluate(field => ({
      text: field.value.substring(field.selectionStart, field.selectionEnd), focused: document.activeElement === field
    }));
    assert.deepEqual(selected, { text: expected, focused: true });
    await page.goto(pathToFileURL(projectHTML).href);
    assert.deepEqual(await page.locator("thead th").allTextContents(), ["Feature", "Description", "Goals", "Status", ""]);
    assert.equal(await page.locator("footer").count(), 0);
    for (const width of [1440, 390]) {
      await page.setViewportSize({ width, height: 900 });
      const bounds = await page.locator("tbody tr").evaluateAll(rows => rows.map(row => row.getBoundingClientRect().height));
      if (width === 1440) assert.ok(bounds.every(height => height < 95), `Compact row heights: ${bounds}`);
      await page.screenshot({ path: path.join(destination, `${engine}-${width}.png`), fullPage: true });
    }
    await fs.writeFile(path.join(destination, `${engine}.html`), await page.content());
    assert.deepEqual(errors, []); assert.ok(requests.every(url => url.startsWith("file:")), "No external assets");
    report.coverage.push({ case: engine, surface: "file", rendering: true, search: true, clipboard_paste: true, denied_clipboard_fallback: true, manual_selection: true, requests, status: "pass" });
    await page.close();
  }
  assert.equal(await fs.readFile(path.join(map, "find.md"), "utf8"), source);
  report.final_identity = JSON.parse(cli(["info", "--json", "--base", base]));
  assert.deepEqual(report.final_identity, report.identity);
  assert.equal(digest(await fs.readFile(binary)), report.binary.sha256);
  report.status = report.identity.dirty ? "development" : "pass";
  await cleanup();
  report.readbacks = [];
  for (const { artifact, expected } of artifactExpectations) {
    assert.equal(await fs.readFile(artifact, "utf8"), expected);
    report.readbacks.push({ artifact, sha256: digest(expected), status: "pass" });
  }
  report.retained_after_cleanup = true;
}
(async () => {
  try { await run(); }
  catch (error) {
    report.status = "failed"; report.error = error.stack;
    if (!active) { console.error(error.message); process.exitCode = 1; return; }
    try { await cleanup(); } catch (cleanupError) { report.cleanup = false; report.cleanup_error = cleanupError.message; }
  }
  await retain();
  assert.equal(JSON.parse(await fs.readFile(path.join(destination, "report.json"))).status, report.status);
  console.log(JSON.stringify(report));
  if (report.status === "failed") process.exitCode = 1;
})();
