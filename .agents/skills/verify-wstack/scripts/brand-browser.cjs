// Owned local browser proof. NODE_PATH may locate existing Playwright/sharp packages.
"use strict";
const { execFileSync } = require("node:child_process");
const { fs, path, assert, files, snapshot, digest, expectations, serve, vectorProof } = require("./brand-proof-files.cjs");
const { prepare, refresh, lifecycle } = require("./brand-proof-fixtures.cjs");
const { browserCase, deniedCase, revisedCase } = require("./brand-proof-surfaces.cjs");
const root = path.resolve(process.argv[2]);
const base = process.argv[3] || "HEAD";
const destination = path.resolve(process.argv[4] || path.join(root, ".evidence", `brand-${Date.now()}`));
const binary = process.env.WSTACK_PROOF_BIN || path.join(root, "cli/target/debug/wstack");
const report = { feature: "brand", status: "incomplete", coverage: [], cleanup: false };
const browsers = {};
let server, state, active = false;
function cli(args, label) {
  try {
    const stdout = execFileSync(binary, args, { cwd: root, encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] });
    if (label) report.coverage.push({ case: label, command: [binary, ...args], stdout, status: "pass" });
    return stdout;
  } catch (error) { throw Error(`${args.join(" ")}: ${error.stderr || error.message}`); }
}
async function cleanup() {
  report.cleanup_errors = [];
  for (const [engine, browser] of Object.entries(browsers)) {
    try { await browser.close(); report.browsers.find(item => item.engine === engine).closed = true; }
    catch (error) { report.cleanup_errors.push(`${engine}: ${error.message}`); }
    finally { delete browsers[engine]; }
  }
  if (server) {
    try { server.closeAllConnections?.(); await new Promise((resolve, reject) => server.close(error => error ? reject(error) : resolve())); report.preview_closed = true; }
    catch (error) { report.cleanup_errors.push(`preview: ${error.message}`); }
    finally { server = null; }
  }
  if (state) {
    try { await fs.rm(state, { recursive: true, force: true }); report.state_removed = true; }
    catch (error) { report.cleanup_errors.push(`state: ${error.message}`); }
    finally { state = null; }
  }
  report.cleanup = report.browsers?.length === 2 && report.browsers.every(item => item.closed) && report.preview_closed === true && report.state_removed === true && report.cleanup_errors.length === 0;
}
async function retain() { await fs.writeFile(path.join(destination, "report.json"), JSON.stringify(report, null, 2) + "\n"); }
for (const signal of ["SIGINT", "SIGTERM"]) process.once(signal, async () => {
  if (!active) process.exit(1);
  report.status = "incomplete"; report.error = signal;
  try { await cleanup(); await retain(); } finally { process.exit(1); }
});
async function run() {
  await fs.mkdir(destination, { recursive: false }); active = true;
  report.evidence = destination;
  report.identity = JSON.parse(cli(["info", "--json", "--base", base]));
  report.binary = { path: binary, sha256: digest(await fs.readFile(binary)) };
  report.helper_sha256 = await files(path.join(root, "skills/wstack-brand"));
  report.recipe_sha256 = await files(path.join(root, ".agents/skills/verify-wstack/scripts"));
  const { chromium, webkit } = require("playwright"), sharp = require("sharp");
  state = path.join(destination, "state"); await fs.mkdir(state);
  const specs = await prepare(root, state, destination);
  report.inputs = specs.map(spec => ({ name: spec.name, source: spec.source || `tests/fixtures/${spec.fixture}`, authored: spec.authored, adaptations: spec.adaptations || [], files: spec.original_snapshot }));
  for (const spec of specs) await refresh(spec, cli, report);
  await vectorProof(root, destination, sharp, report);
  const preview = await serve(destination, report); server = preview.server;
  const { origin } = preview;
  report.browsers = [];
  for (const [engine, launcher] of Object.entries({ chromium, webkit })) {
    browsers[engine] = await launcher.launch({ headless: true });
    report.browsers.push({ engine, version: browsers[engine].version(), owned: true, closed: false });
  }
  const observations = [];
  for (const [engine, browser] of Object.entries(browsers)) {
    for (const spec of specs) for (const mode of ["loopback", "file"]) {
      observations.push(await browserCase(browser, engine, spec, mode, origin, destination, report));
      await deniedCase(browser, engine, spec, mode, origin, destination, report);
    }
  }
  for (const spec of specs) await lifecycle(spec, cli, report);
  for (const observation of observations) await revisedCase(browsers[observation.engine], observation, origin, destination, report);
  const comparison = await browsers.chromium.newPage({ viewport: { width: 900, height: 650 } });
  await comparison.goto(`${origin}/comparison.html`);
  await comparison.screenshot({ path: path.join(destination, "vector-comparison.png"), fullPage: true });
  await comparison.close();
  for (const spec of specs) if (spec.source) assert.deepEqual(await snapshot(spec.source), spec.sourceBefore, "Selected original project was not modified");
  report.final_identity = JSON.parse(cli(["info", "--json", "--base", base]));
  assert.deepEqual(report.final_identity, report.identity);
  assert.equal(digest(await fs.readFile(binary)), report.binary.sha256);
  assert.deepEqual(await files(path.join(root, "skills/wstack-brand")), report.helper_sha256);
  assert.deepEqual(await files(path.join(root, ".agents/skills/verify-wstack/scripts")), report.recipe_sha256);
  report.required_cases = specs.flatMap(spec => ["chromium", "webkit"].flatMap(engine => ["file", "loopback"].flatMap(mode => [ `${spec.name}-${engine}-${mode}`, `${spec.name}-${engine}-${mode}-clipboard-denied` ])));
  for (const required of report.required_cases) assert.ok(report.coverage.some(row => row.case === required && row.status === "pass"), required);
  report.visual_review = "Inspect matched baseline/overview pairs, full guides and vector-comparison.png. Matched-pixel bound plus preserved source hashes establishes the authored first-screen comparison; a person still reviews overall composition.";
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
    if (!report.cleanup && ["pass", "development"].includes(report.status)) report.status = "failed";
    const retained = await files(destination); delete retained["report.json"];
    if (["pass", "development"].includes(report.status)) {
      for (const row of report.coverage.filter(item => item.requests)) {
        for (const suffix of row.case.endsWith("-revised") ? ["-download.bin"] : ["-clipboard.json", "-first-download.bin", "-later-download.bin", "-320.png", "-390.png", "-1440.png"]) {
          assert.ok((await fs.stat(path.join(destination, row.case + suffix))).size > 0, row.case + suffix);
        }
      }
      assert.ok((await fs.stat(path.join(destination, "vector-comparison.png"))).size > 0);
    }
    report.expected_readbacks = expectations;
    report.readbacks = [];
    for (const expected of expectations) {
      const actual = await fs.readFile(expected.artifact);
      assert.equal(digest(actual), expected.expected_sha256, `Retained bytes after teardown: ${expected.artifact}`);
      if (expected.expected_text !== undefined) assert.equal(actual.toString("utf8"), expected.expected_text);
      report.readbacks.push({ artifact: expected.artifact, actual_sha256: digest(actual), expected_sha256: expected.expected_sha256, status: "pass" });
    }
    report.artifacts = retained;
    report.retained_after_cleanup = true;
    await retain();
    assert.equal(JSON.parse(await fs.readFile(path.join(destination, "report.json"))).status, report.status);
  } catch (error) { report.status = "failed"; report.error = error.stack; await retain(); }
  console.log(JSON.stringify(report));
  if (report.status === "failed" || report.status === "incomplete") process.exitCode = 1;
})();
