"use strict";
const { fs, path, assert, digest, decode, screenshot, matchedScreenshot, expectations } = require("./brand-proof-files.cjs");
const { pathToFileURL } = require("node:url");
const hooks = authored => authored ? {
  root: "#resource-tools", search: "[data-wstack-search]", category: "[data-wstack-category]", scope: "[data-wstack-scope]",
  cards: "[data-wstack-path]", empty: "[data-wstack-empty]", copy: "[data-wstack-style-copy]", field: "[data-wstack-style]", status: "[data-wstack-status]"
} : { root: "body", search: "#search", category: "#category", scope: "#scope", cards: ".asset", empty: "#empty", copy: "#copy", field: "#style-json", status: "#copy-status" };
function guideURL(spec, mode, origin, destination, before = false) {
  const directory = before ? spec.before : spec.retained;
  const local = path.join(directory, spec.guideRelative);
  return mode === "file" ? pathToFileURL(local).href : origin + "/" + path.relative(destination, local).split(path.sep).map(encodeURIComponent).join("/");
}
async function clipboard(page, button, expected, artifact, readAPI, statusSelector) {
  await button.click();
  const color = await button.getAttribute("data-color");
  if (statusSelector) await page.waitForFunction(({ selector, value }) => /copied/i.test(document.querySelector(selector).textContent) && document.querySelector(selector).textContent.includes(value), { selector: statusSelector, value: expected });
  else await page.waitForFunction(value => [...document.querySelectorAll('[role="status"]')].some(node => value ? node.textContent.includes("Color copied: " + value) : node.textContent === "Style JSON copied."), color);
  const paste = await page.evaluateHandle(() => {
    const field = document.createElement("textarea"); field.id = "proof-paste"; field.setAttribute("aria-label", "Clipboard readback");
    document.body.append(field); field.focus(); return field;
  });
  await page.keyboard.press(process.platform === "darwin" ? "Meta+V" : "Control+V");
  const actual = await paste.evaluate(node => node.value);
  assert.equal(actual, expected, "Actual clipboard paste differs from source/CLI");
  if (readAPI) assert.equal(await page.evaluate(() => navigator.clipboard.readText()), expected);
  await fs.writeFile(artifact, actual);
  expectations.push({ artifact, expected_text: expected, expected_sha256: digest(expected), source: color ? "Authored color value" : "Source style.json validated against CLI style output" });
  await paste.evaluate(node => node.remove());
}
async function download(page, target, expected, filename, requests, selector) {
  const before = requests.length;
  const event = page.waitForEvent("download");
  await page.locator(selector || `[data-wstack-download="${target}"]`).first().click();
  const result = await event;
  await result.saveAs(filename);
  assert.equal(digest(await fs.readFile(filename)), digest(expected), "Downloaded original bytes differ");
  expectations.push({ artifact: filename, expected_sha256: digest(expected), source: `Original resource bytes at request time: ${target}` });
  return requests.slice(before);
}
async function browserCase(browser, engine, spec, mode, origin, destination, report) {
  const prefix = `${spec.name}-${engine}-${mode}`;
  const context = await browser.newContext({ viewport: { width: 1440, height: 1000 }, acceptDownloads: true });
  if (engine === "chromium") await context.grantPermissions(["clipboard-read", "clipboard-write"]);
  const page = await context.newPage(), errors = [], requests = [];
  page.on("pageerror", error => errors.push(error.message));
  page.on("response", response => { if (response.status() >= 400 && !response.url().endsWith("favicon.ico")) errors.push(`${response.status()} ${response.url()}`); });
  page.on("request", request => { if (/\/downloads\/[a-f0-9]+\.js$/.test(request.url())) requests.push(request.url()); });
  const settings = hooks(spec.authored), url = guideURL(spec, mode, origin, destination);
  if (spec.authored) {
    await page.goto(guideURL(spec, mode, origin, destination, true));
    await decode(page, spec.localFont);
    for (const width of [320, 390, 1440]) {
      await page.setViewportSize({ width, height: 1000 });
      await page.screenshot({ path: path.join(destination, `${prefix}-${width}-baseline.png`) });
    }
  }
  await page.goto(url); await decode(page, spec.localFont);
  assert.deepEqual(requests, [], "No download payload may be requested on initial guide load");
  const library = page.locator(settings.root), cards = library.locator(settings.cards);
  const assets = await page.evaluate(() => window.WstackBrand.assets);
  const current = assets.filter(row => row.role === "current");
  assert.equal(await library.locator(`${settings.cards}:visible`).count(), current.length);
  assert.equal(await library.locator(settings.scope).inputValue(), "current");
  if (spec.name !== "selected") {
    await page.keyboard.press(engine === "webkit" && process.platform === "darwin" ? "Alt+Tab" : "Tab");
    assert.equal(await page.evaluate(() => document.activeElement.textContent), "Skip to guide");
    await page.keyboard.press("Enter");
    assert.equal(await page.evaluate(() => document.activeElement.id), "main");
  }
  for (const width of [320, 390, 1440]) {
    await page.goto(url); await decode(page, spec.localFont);
    await screenshot(page, prefix, destination, width);
    if (spec.authored) {
      report.visual_matches ||= [];
      report.visual_matches.push(await matchedScreenshot(path.join(destination, `${prefix}-${width}-baseline.png`), path.join(destination, `${prefix}-${width}-overview.png`)));
    }
  }
  await library.locator(settings.copy).scrollIntoViewIfNeeded();
  await clipboard(page, library.locator(settings.copy), spec.style, path.join(destination, `${prefix}-clipboard.json`), engine === "chromium");
  if (spec.authored && spec.name !== "selected") {
    const color = page.locator("[data-color]").first();
    await clipboard(page, color, await color.getAttribute("data-color"), path.join(destination, `${prefix}-color.txt`), engine === "chromium");
    await page.locator("[data-preview]").first().click();
    assert.equal(await page.locator("#art-preview").isVisible(), true);
    await page.locator("#close-preview").click();
    assert.equal(await page.locator("#art-preview").isVisible(), false);
  }
  if (spec.checks?.color) {
    const check = spec.checks.color;
    await clipboard(page, page.locator(check.selector).first(), check.value, path.join(destination, `${prefix}-color.txt`), engine === "chromium", check.status);
  }
  if (spec.checks?.preview) {
    const check = spec.checks.preview;
    await page.locator(check.open).first().click(); assert.equal(await page.locator(check.dialog).isVisible(), true);
    await page.locator(check.close).click(); assert.equal(await page.locator(check.dialog).isVisible(), false);
  }
  const selected = spec.target ? current.find(row => row.path === spec.target) : current.find(row => row.preview && (!spec.source || row.path.endsWith(".svg")));
  assert.ok(selected, "Guide proof requires a current preview/download resource (selected kits: editable SVG)");
  if (spec.source) assert.ok(selected.path.endsWith(".svg"), "Selected proof target must be a current editable SVG");
  spec.target ||= selected.path;
  const query = spec.checks?.search?.query || (spec.source ? selected.path : selected.title);
  await library.locator(settings.search).focus(); await page.keyboard.type(query);
  if (spec.checks?.search) {
    const observed = await library.locator(`${settings.cards}:visible`).evaluateAll(nodes => nodes.map(node => node.dataset.wstackPath || node.querySelector("[data-asset]").dataset.asset).sort());
    assert.deepEqual(observed, [...spec.checks.search.paths].sort());
  } else if (spec.source) assert.equal(await library.locator(`[data-wstack-path="${selected.path}"]`).isVisible(), true);
  else assert.equal(await library.locator(`${settings.cards}:visible`).count(), 1);
  await library.locator(settings.search).fill("no-matching-brand-resource-3849");
  assert.equal(await library.locator(`${settings.cards}:visible`).count(), 0);
  assert.equal(await library.locator(settings.empty).isVisible(), true);
  await library.locator(settings.search).fill("");
  await library.locator(settings.category).selectOption(selected.category);
  assert.equal(await library.locator(`${settings.cards}:visible`).count(), current.filter(row => row.category === selected.category).length);
  await library.locator(settings.category).selectOption("");
  await library.locator(settings.scope).selectOption("all");
  assert.equal(await library.locator(`${settings.cards}:visible`).count(), assets.length);
  const legacy = assets.find(row => row.role === "legacy");
  if (legacy) {
    await library.locator(settings.search).fill(legacy.path);
    assert.equal(await library.locator(`${settings.cards}:visible`).count(), 1);
    await library.locator(settings.scope).selectOption("current");
    assert.equal(await library.locator(`${settings.cards}:visible`).count(), 0);
  }
  await library.locator(settings.search).fill(""); await library.locator(settings.scope).selectOption("current");
  const preview = library.locator(`[data-wstack-preview="${selected.path}"]`).first();
  await preview.focus(); await page.keyboard.press("Enter");
  await page.locator("[data-wstack-dialog]").waitFor({ state: "visible" });
  await page.keyboard.press("Escape");
  assert.equal(await preview.evaluate(node => document.activeElement === node), true);
  assert.deepEqual(requests, [], "Search, copy, previews and category/scope changes must not eagerly load download payloads");
  const bytes = await fs.readFile(path.join(spec.retained, selected.path));
  const first = await download(page, selected.path, bytes, path.join(destination, `${prefix}-first-download.bin`), requests);
  assert.equal(first.length, 1, "First download loads exactly its own payload");
  const later = await download(page, selected.path, bytes, path.join(destination, `${prefix}-later-download.bin`), requests);
  assert.equal(later.length, 0, "Later download reuses the selected resource payload");
  if (spec.checks?.download) {
    const check = spec.checks.download;
    assert.ok(assets.some(row => row.path === check.path), "Configured retained download must be a validated catalog resource");
    const expected = await fs.readFile(path.join(spec.retained, check.path));
    await download(page, check.path, expected, path.join(destination, `${prefix}-retained-download.bin`), requests, check.selector);
  }
  if (spec.bundle) {
    assert.ok(assets.some(row => row.path === spec.bundle), "Configured bundle must be a validated catalog resource");
    await library.locator(settings.scope).selectOption("all");
    const bundle = await fs.readFile(path.join(spec.retained, spec.bundle));
    const loaded = await download(page, spec.bundle, bundle, path.join(destination, `${prefix}-bundle.zip`), requests, spec.checks?.bundle?.selector);
    assert.equal(loaded.length, 1, "Existing ZIP bundle loads only when requested");
    await fs.writeFile(path.join(destination, `${prefix}-bundle-source.zip`), bundle);
  }
  const payload = selected.download;
  assert.ok(first[0].endsWith("/" + payload));
  await fs.writeFile(path.join(destination, `${prefix}-requests.json`), JSON.stringify(requests, null, 2));
  assert.deepEqual(errors, []);
  report.coverage.push({ case: prefix, status: "pass", actions: ["320/390/1440 layout and local resources", "actual system clipboard paste (plus Chromium API read)", "existing color copy/artwork preview", "keyboard search/category/scope/empty state", "keyboard preview/focus return", "lazy first/later original download bytes", ...(spec.authored ? ["matched first-screen authored baseline screenshots"] : [])], requests });
  await context.close();
  return { spec, engine, mode, payload, bytes: digest(bytes), prefix };
}
async function deniedCase(browser, engine, spec, mode, origin, destination, report) {
  const context = await browser.newContext();
  await context.addInitScript(() => {
    Object.defineProperty(navigator, "clipboard", { configurable: true, value: { writeText: () => Promise.reject(Error("denied")) } });
    document.execCommand = () => false;
  });
  const page = await context.newPage();
  await page.goto(guideURL(spec, mode, origin, destination));
  const settings = hooks(spec.authored), library = page.locator(settings.root);
  await library.locator(settings.copy).click();
  await library.locator(settings.status).filter({ hasText: "Copy unavailable" }).waitFor();
  const selected = await library.locator(settings.field).evaluate(node => ({ text: node.value.substring(node.selectionStart, node.selectionEnd), focused: node === document.activeElement }));
  assert.deepEqual(selected, { text: spec.style, focused: true });
  report.coverage.push({ case: `${spec.name}-${engine}-${mode}-clipboard-denied`, status: "pass", observation: "Exact style selected with manual keyboard-copy instructions" });
  await context.close();
}
async function revisedCase(browser, previous, origin, destination, report) {
  const { spec, engine, mode, prefix } = previous;
  if (!spec.revised) return;
  const page = await browser.newPage(), requests = [];
  page.on("request", request => { if (/\/downloads\/[a-f0-9]+\.js$/.test(request.url())) requests.push(request.url()); });
  await page.goto(guideURL(spec, mode, origin, destination));
  const assets = await page.evaluate(() => window.WstackBrand.assets), row = assets.find(item => item.path === spec.target);
  assert.equal(row.title, "Revised retained resource"); assert.notEqual(row.download, previous.payload);
  assert.ok(!assets.some(item => item.path === "proof-added.svg")); assert.deepEqual(requests, []);
  const expected = await fs.readFile(spec.revised); assert.notEqual(digest(expected), previous.bytes);
  await download(page, spec.target, expected, path.join(destination, `${prefix}-revised-download.bin`), requests);
  await decode(page, spec.localFont);
  for (const width of [320, 390, 1440]) await screenshot(page, `${prefix}-authored-revised`, destination, width);
  assert.equal(requests.length, 1); assert.ok(requests[0].endsWith("/" + row.download));
  report.coverage.push({ case: `${prefix}-revised`, status: "pass", actions: ["fresh revised metadata", "removed asset absent", "only revised payload requested", "revised original download bytes"], requests });
  await page.close();
}
module.exports = { browserCase, deniedCase, revisedCase };
