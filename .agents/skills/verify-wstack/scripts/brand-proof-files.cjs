"use strict";
const fs = require("node:fs/promises");
const path = require("node:path");
const http = require("node:http");
const { createHash } = require("node:crypto");
const assert = require("node:assert/strict");
const expectations = [];
const digest = data => createHash("sha256").update(data).digest("hex");
async function files(dir, prefix = "") {
  assert.ok(!(await fs.lstat(dir)).isSymbolicLink(), `Proof inputs cannot be symlinks: ${dir}`);
  const found = {};
  for (const entry of await fs.readdir(dir, { withFileTypes: true })) {
    if (["__pycache__", ".DS_Store"].includes(entry.name) || entry.name.endsWith(".pyc")) continue;
    const relative = path.join(prefix, entry.name), target = path.join(dir, entry.name);
    assert.ok(!entry.isSymbolicLink(), `Proof inputs cannot be symlinks: ${target}`);
    if (entry.isDirectory()) Object.assign(found, await files(target, relative));
    else found[relative] = digest(await fs.readFile(target));
  }
  return found;
}
async function snapshot(dir) {
  const hashes = await files(dir), result = {};
  for (const [name, sha256] of Object.entries(hashes)) result[name] = { sha256, mtime_ms: (await fs.stat(path.join(dir, name))).mtimeMs };
  return result;
}
async function serve(destination, report) {
  const types = { ".html": "text/html", ".css": "text/css", ".js": "text/javascript", ".svg": "image/svg+xml", ".png": "image/png", ".woff2": "font/woff2" };
  const server = http.createServer(async (request, response) => {
    try {
      const target = path.resolve(destination, "." + decodeURIComponent(new URL(request.url, "http://localhost").pathname));
      if (!target.startsWith(destination + path.sep)) throw new Error("outside preview");
      const bytes = await fs.readFile(target);
      response.writeHead(200, { "Content-Type": types[path.extname(target)] || "application/octet-stream" });
      response.end(bytes);
    } catch { response.writeHead(404); response.end(); }
  });
  await new Promise(resolve => server.listen(0, "127.0.0.1", resolve));
  const origin = `http://127.0.0.1:${server.address().port}`;
  report.preview = { origin, pid: process.pid, owned: true, directory: destination };
  return { server, origin };
}
async function vectorProof(root, destination, sharp, report) {
  const source = path.join(root, "tests/fixtures/brand/brand");
  await fs.cp(source, path.join(destination, "vector-originals"), { recursive: true });
  report.vector_comparisons = [];
  for (const [index, name] of ["sprout", "sunrise", "cairn"].entries()) {
    const original = await sharp(path.join(source, "references/original-icon-sheet.png"))
      .extract({ left: index * 256, top: 0, width: 256, height: 256 }).removeAlpha().raw().toBuffer();
    const rendered = await sharp(path.join(source, `symbols/${name}.svg`))
      .flatten({ background: "#f1efe6" }).removeAlpha().raw().toBuffer();
    assert.equal(rendered.length, original.length);
    let delta = 0;
    for (let i = 0; i < original.length; i++) delta += Math.abs(original[i] - rendered[i]);
    const mean = delta / original.length / 255;
    assert.ok(mean < .015, `${name}: raster/vector delta ${mean}`);
    report.vector_comparisons.push({ name, normalized_mean_error: mean, threshold: .015 });
  }
  const html = '<!doctype html><title>Original and reconstructed icons</title>' +
    '<body style="background:#f1efe6;font:16px system-ui"><h1>Original raster / editable SVGs</h1>' +
    '<img width="768" src="vector-originals/references/original-icon-sheet.png"><div>' +
    ["sprout", "sunrise", "cairn"].map(n => `<img width="256" src="vector-originals/symbols/${n}.svg">`).join("") + '</div></body>';
  await fs.writeFile(path.join(destination, "comparison.html"), html);
}
async function decode(page, localFont) {
  await page.evaluate(async () => {
    await document.fonts.ready;
    for (const image of document.images) {
      if (!image.getAttribute("src")) continue;
      image.loading = "eager";
      await image.decode();
    }
  });
  if (localFont) assert.equal(await page.evaluate(expected => [...document.fonts].some(font => font.status === "loaded" && font.family.replace(/['"]/g, "") === expected), localFont === true ? "Geist" : localFont), true);
}
async function matchedScreenshot(before, after) {
  const sharp = require("sharp");
  const left = await sharp(before).raw().toBuffer(), right = await sharp(after).raw().toBuffer();
  assert.equal(left.length, right.length);
  let delta = 0;
  for (let index = 0; index < left.length; index++) delta += Math.abs(left[index] - right[index]);
  const mean = delta / left.length / 255;
  assert.ok(mean < .005, `Authored first-screen visual changed: normalized mean error ${mean}`);
  return { before, after, normalized_mean_error: mean, maximum: .005 };
}
async function screenshot(page, prefix, destination, width) {
  await page.setViewportSize({ width, height: 1000 });
  await page.evaluate(() => scrollTo({ left: 0, top: 0, behavior: "instant" }));
  await page.waitForFunction(() => scrollX === 0 && scrollY === 0);
  assert.equal(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), true, `${prefix}: ${width}px overflow`);
  await page.screenshot({ path: path.join(destination, `${prefix}-${width}.png`), fullPage: true });
  await page.screenshot({ path: path.join(destination, `${prefix}-${width}-overview.png`) });
  await fs.writeFile(path.join(destination, `${prefix}-${width}.html`), await page.content());
}
module.exports = { fs, path, assert, digest, files, snapshot, expectations, serve, vectorProof, decode, screenshot, matchedScreenshot };
