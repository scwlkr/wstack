// Owned local browser proof. NODE_PATH may locate the owner's existing Playwright/sharp.
"use strict";
const fs = require("node:fs/promises");
const path = require("node:path");
const http = require("node:http");
const { execFileSync } = require("node:child_process");
const { createHash } = require("node:crypto");
const assert = require("node:assert/strict");
const { pathToFileURL } = require("node:url");
const root = path.resolve(process.argv[2]);
const base = process.argv[3] || "HEAD";
const destination = path.resolve(process.argv[4] || path.join(root, ".evidence", `brand-${Date.now()}`));
const binary = process.env.WSTACK_PROOF_BIN || path.join(root, "cli/target/debug/wstack");
const report = { feature: "brand", status: "incomplete", coverage: [], cleanup: false };
let browser, server, state, active = false;
const digest = data => createHash("sha256").update(data).digest("hex");
function cli(args, suffix) {
  try {
    const result = execFileSync(binary, args, { cwd: root, encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] });
    if (suffix) report.coverage.push({ case: suffix, command: [binary, ...args], stdout: result, status: "pass" });
    return result;
  } catch (error) {
    throw new Error(`${args.join(" ")}: ${error.stderr || error.message}`);
  }
}
async function files(dir, prefix = "") {
  const found = {};
  for (const entry of await fs.readdir(dir, { withFileTypes: true })) {
    if (entry.name === "__pycache__" || entry.name.endsWith(".pyc")) continue;
    const relative = path.join(prefix, entry.name);
    const target = path.join(dir, entry.name);
    if (entry.isDirectory()) Object.assign(found, await files(target, relative));
    else found[relative] = digest(await fs.readFile(target));
  }
  return found;
}
async function cleanup() {
  if (browser) { await browser.close(); report.browser_closed = true; browser = null; }
  if (server) {
    await new Promise(resolve => server.close(resolve));
    report.preview_closed = true; server = null;
  }
  if (state) { await fs.rm(state, { recursive: true, force: true }); state = null; }
  report.cleanup = report.browser_closed === true && report.preview_closed === true;
}
async function retain() {
  await fs.writeFile(path.join(destination, "report.json"), JSON.stringify(report, null, 2) + "\n");
}
for (const signal of ["SIGINT", "SIGTERM"]) process.once(signal, async () => {
  if (!active) process.exit(1);
  report.status = "incomplete"; report.error = signal;
  try { await cleanup(); await retain(); } finally { process.exit(1); }
});
async function run() {
  await fs.mkdir(destination, { recursive: false });
  active = true;
  report.evidence = destination;
  report.identity = JSON.parse(cli(["info", "--json", "--base", base]));
  report.binary_sha256 = digest(await fs.readFile(binary));
  report.source_sha256 = await files(path.join(root, "skills/wstack-brand"));
  const { chromium } = require("playwright");
  const sharp = require("sharp");
  state = path.join(destination, "state");
  await fs.mkdir(state);
  for (const name of ["alder", "harbor"]) {
    const project = path.join(state, name);
    await fs.cp(path.join(root, "tests/fixtures/brand"), project, { recursive: true });
    const brand = path.join(project, "brand");
    if (name === "harbor") {
      const config = JSON.parse(await fs.readFile(path.join(brand, "brand.json")));
      config.name = "Harbor Tools";
      config.summary = "Clear tools for useful work.";
      await fs.writeFile(path.join(brand, "brand.json"), JSON.stringify(config));
      await fs.rm(path.join(brand, "guidance.html"));
      await fs.rm(path.join(brand, "legacy-guide.html"));
    }
    const before = await files(project);
    cli(["brand", "refresh", "--root", project, "--json"], `${name}-refresh`);
    const after = await files(project);
    for (const [file, hash] of Object.entries(before)) assert.equal(after[file], hash, file);
    assert.deepEqual(JSON.parse(cli(["brand", "refresh", "--root", project, "--json"])).changed, []);
    assert.deepEqual(await files(project), after);
    const stored = await fs.readFile(path.join(brand, "style.json"), "utf8");
    const compact = cli(["brand", "style", "--root", project], `${name}-style`).trim();
    assert.deepEqual(JSON.parse(compact), JSON.parse(stored));
    await fs.writeFile(path.join(destination, `${name}-style.json`), compact);
    await fs.cp(brand, path.join(destination, `${name}-guide`), { recursive: true });
  }
  // Compare each reconstructed vector with its independent original raster crop.
  const brand = path.join(state, "alder/brand");
  const comparisons = [];
  for (const [index, name] of ["sprout", "sunrise", "cairn"].entries()) {
    const original = await sharp(path.join(brand, "references/original-icon-sheet.png"))
      .extract({ left: index * 256, top: 0, width: 256, height: 256 }).removeAlpha().raw().toBuffer();
    const rendered = await sharp(path.join(brand, `symbols/${name}.svg`))
      .flatten({ background: "#f1efe6" }).removeAlpha().raw().toBuffer();
    assert.equal(rendered.length, original.length);
    let delta = 0;
    for (let i = 0; i < original.length; i++) delta += Math.abs(original[i] - rendered[i]);
    const mean = delta / original.length / 255;
    assert.ok(mean < .015, `${name}: raster/vector delta ${mean}`);
    comparisons.push({ name, normalized_mean_error: mean, threshold: .015 });
  }
  report.vector_comparisons = comparisons;
  const comparisonHTML = '<!doctype html><title>Original and reconstructed icons</title>' +
    '<body style="background:#f1efe6;font:16px system-ui"><h1>Original raster / editable SVGs</h1>' +
    '<img width="768" src="alder-guide/references/original-icon-sheet.png"><div>' +
    ["sprout", "sunrise", "cairn"].map(n => `<img width="256" src="alder-guide/symbols/${n}.svg">`).join("") + '</div></body>';
  await fs.writeFile(path.join(destination, "comparison.html"), comparisonHTML);
  const types = { ".html": "text/html", ".css": "text/css", ".js": "text/javascript", ".svg": "image/svg+xml", ".png": "image/png" };
  server = http.createServer(async (request, response) => {
    try {
      const target = path.resolve(destination, "." + decodeURIComponent(new URL(request.url, "http://localhost").pathname));
      if (!target.startsWith(destination + path.sep)) throw new Error("outside preview");
      const content = await fs.readFile(target);
      response.writeHead(200, { "Content-Type": types[path.extname(target)] || "application/octet-stream" });
      response.end(content);
    } catch { response.writeHead(404); response.end(); }
  });
  await new Promise(resolve => server.listen(0, "127.0.0.1", resolve));
  const origin = `http://127.0.0.1:${server.address().port}`;
  report.preview = { origin, pid: process.pid, owned: true, directory: destination };
  browser = await chromium.launch({ headless: true });
  report.browser = { version: browser.version(), owned: true, engine: "Chromium" };
  const context = await browser.newContext({ permissions: ["clipboard-read", "clipboard-write"], viewport: { width: 1440, height: 1000 } });
  const page = await context.newPage();
  const errors = [];
  page.on("pageerror", error => errors.push(error.message));
  page.on("response", response => { if (response.status() >= 400) errors.push(`${response.status()} ${response.url()}`); });
  for (const name of ["alder", "harbor"]) {
    const url = `${origin}/${name}-guide/.wstack-brand/index.html`;
    await page.goto(url);
    await page.keyboard.press("Tab");
    assert.equal(await page.evaluate(() => document.activeElement.textContent), "Skip to guide");
    await page.keyboard.press("Enter");
    assert.equal(await page.evaluate(() => document.activeElement.id), "main");
    await page.locator("#copy").click();
    await page.getByText("Style JSON copied.", { exact: true }).waitFor();
    const copied = await page.evaluate(() => navigator.clipboard.readText());
    assert.equal(copied, await fs.readFile(path.join(destination, `${name}-style.json`), "utf8"));
    await fs.writeFile(path.join(destination, `${name}-clipboard.json`), copied);
    await page.keyboard.press("Tab");
    await page.locator("#search").focus();
    await page.keyboard.type("sunrise");
    assert.equal(await page.locator(".asset:visible").count(), 1);
    await page.locator("#search").fill("no-such-resource");
    assert.equal(await page.locator(".asset:visible").count(), 0);
    assert.equal(await page.locator("#empty").isVisible(), true);
    await page.locator("#search").fill("");
    await page.locator("#category").selectOption("Symbols");
    assert.equal(await page.locator(".asset:visible").count(), 5);
    await page.locator("#category").selectOption("");
    await page.getByRole("link", { name: "Assets", exact: true }).focus();
    await page.keyboard.press("Enter");
    assert.equal(new URL(page.url()).hash, "#catalog");
    const card = page.locator('.asset').filter({ has: page.getByRole("heading", { name: "Sunrise", exact: true }) });
    const downloadWait = page.waitForEvent("download");
    await card.getByRole("link", { name: "Download", exact: true }).click();
    const download = await downloadWait;
    const saved = path.join(destination, `${name}-download.svg`);
    await download.saveAs(saved);
    assert.equal(digest(await fs.readFile(saved)), digest(await fs.readFile(path.join(brand, "symbols/sunrise.svg"))));
    await page.locator("#prompt").scrollIntoViewIfNeeded();
    for (const width of [1440, 390]) {
      await page.setViewportSize({ width, height: 1000 });
      await page.goto(url);
      await page.evaluate(async () => {
        for (const img of document.images) { img.loading = "eager"; await img.decode(); }
      });
      assert.equal(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), true);
      await page.screenshot({ path: path.join(destination, `${name}-${width}.png`), fullPage: true });
      await page.screenshot({ path: path.join(destination, `${name}-${width}-overview.png`) });
      await page.locator("#catalog").evaluate(node => node.scrollIntoView({ block: "start" }));
      await page.screenshot({ path: path.join(destination, `${name}-${width}-catalog.png`) });
      await fs.writeFile(path.join(destination, `${name}-${width}.html`), await page.content());
    }
    report.coverage.push({ case: `${name}-browser`, status: "pass", actions: ["clipboard equality", "keyboard search", "empty results", "category filter", "keyboard navigation", "download bytes", "image decode", "1440/390 layout"] });
    // Exercise direct local-file use and the actual local-file copy path too.
    await page.goto(pathToFileURL(path.join(destination, `${name}-guide/.wstack-brand/index.html`)).href);
    await page.locator("#copy").click();
    await page.getByText("Style JSON copied.", { exact: true }).waitFor();
    assert.equal(await page.evaluate(() => navigator.clipboard.readText()), copied);
    report.coverage.push({ case: `${name}-file-guide`, status: "pass", actions: ["local file loading", "actual clipboard equality"] });
  }
  await page.setViewportSize({ width: 900, height: 650 });
  await page.goto(`${origin}/comparison.html`);
  await page.screenshot({ path: path.join(destination, "vector-comparison.png"), fullPage: true });
  assert.deepEqual(errors, []);
  report.final_identity = JSON.parse(cli(["info", "--json", "--base", base]));
  assert.deepEqual(report.final_identity, report.identity);
  assert.equal(digest(await fs.readFile(binary)), report.binary_sha256);
  report.status = report.identity.dirty ? "development" : "pass";
}
(async () => {
  try { await run(); }
  catch (error) {
    report.status = "failed"; report.error = error.stack;
    if (!active) { console.error(error.message); process.exitCode = 1; return; }
  }
  try {
    await cleanup();
    if (!report.cleanup) report.status = "failed";
    // Read retained outputs after owned teardown; missing/empty artifacts cannot pass.
    const retained = await files(destination);
    if (report.status === "pass" || report.status === "development") {
      for (const name of ["alder", "harbor"]) {
        for (const file of [`${name}-clipboard.json`, `${name}-download.svg`, `${name}-390.png`, `${name}-1440.png`]) {
          assert.ok((await fs.stat(path.join(destination, file))).size > 0, file);
        }
        assert.equal(await fs.readFile(path.join(destination, `${name}-clipboard.json`), "utf8"),
          await fs.readFile(path.join(destination, `${name}-style.json`), "utf8"));
      }
      assert.ok((await fs.stat(path.join(destination, "vector-comparison.png"))).size > 0);
    }
    delete retained["report.json"];
    report.artifacts = retained;
    await retain();
    assert.equal(JSON.parse(await fs.readFile(path.join(destination, "report.json"))).status, report.status);
    console.log(JSON.stringify(report));
  } catch (error) {
    report.status = "failed"; report.error = error.stack;
    await retain(); console.log(JSON.stringify(report));
  }
})();
