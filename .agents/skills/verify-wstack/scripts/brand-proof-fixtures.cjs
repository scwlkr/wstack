"use strict";
const { execFileSync } = require("node:child_process");
const { fs, path, assert, files, digest, snapshot } = require("./brand-proof-files.cjs");
const enhancement = `
<section class="section" id="resource-tools" data-wstack-catalog>
<p class="eyebrow">Portable resource tools</p><h2>Find a resource.<br>Carry the style.</h2>
<div class="catalog-controls"><input type="search" data-wstack-search aria-label="Search resources">
<select data-wstack-category aria-label="Resource category"></select><select data-wstack-scope aria-label="Resource scope"></select></div>
<p data-wstack-count role="status"></p><div data-wstack-assets></div><p data-wstack-empty hidden>No matching resources.</p>
<label>Image-prompt style JSON<textarea data-wstack-style readonly></textarea></label>
<button type="button" data-wstack-style-copy>Copy style JSON</button><p data-wstack-status role="status"></p>
</section><script src=".wstack-brand/catalog.js"></script><script src=".wstack-brand/integration.js"></script>
`;
async function prepare(root, state, destination) {
  const specs = process.env.WSTACK_BRAND_KIT ? [] : [
    { name: "generated", fixture: "brand", authored: false, target: "symbols/sunrise.svg" },
    { name: "alder", fixture: "brand-authored-alder", authored: true, target: "symbols/sunrise.svg", localFont: true },
    { name: "orbit", fixture: "brand-authored-orbit", authored: true, target: "marks/constellation.svg", localFont: true }
  ];
  if (process.env.WSTACK_BRAND_KIT) {
    assert.ok(path.isAbsolute(process.env.WSTACK_BRAND_KIT), "WSTACK_BRAND_KIT must be an absolute project root");
    const brandDir = process.env.WSTACK_BRAND_DIR || "brand";
    assert.ok(!path.isAbsolute(brandDir) && !brandDir.split(/[\\/]/).includes(".."), "WSTACK_BRAND_DIR must stay inside the selected project");
    let checks = {};
    if (process.env.WSTACK_BRAND_CHECKS) {
      checks = JSON.parse(await fs.readFile(process.env.WSTACK_BRAND_CHECKS));
      assert.ok(checks && !Array.isArray(checks) && typeof checks === "object", "WSTACK_BRAND_CHECKS must be a JSON object");
      for (const key of Object.keys(checks)) assert.ok(["color", "preview", "download", "bundle", "font", "target", "search"].includes(key), `Unknown selected-kit check: ${key}`);
      if (checks.search) {
        assert.deepEqual(Object.keys(checks.search).sort(), ["paths", "query"]);
        assert.equal(typeof checks.search.query, "string"); assert.ok(Array.isArray(checks.search.paths));
        for (const item of checks.search.paths) assert.equal(typeof item, "string");
      }
      for (const key of ["font", "target"]) if (checks[key] !== undefined) assert.equal(typeof checks[key], "string");
      for (const key of ["color", "preview", "download", "bundle"]) if (checks[key]) {
        assert.equal(typeof checks[key], "object");
        const allowed = key === "color" ? ["selector", "value", "status"] : key === "preview" ? ["open", "dialog", "close"] : ["selector", "path"];
        for (const [name, value] of Object.entries(checks[key])) { assert.ok(allowed.includes(name)); assert.equal(typeof value, "string"); }
        for (const name of allowed) assert.ok(checks[key][name], `Selected ${key} check requires ${name}`);
      }
    }
    specs.push({ name: "selected", source: path.resolve(process.env.WSTACK_BRAND_KIT, brandDir), brandDir, authored: true, checks, target: checks.target, localFont: checks.font });
  }
  for (const spec of specs) {
    spec.project = path.join(state, spec.name);
    spec.brandDir ||= "brand";
    spec.brand = path.join(spec.project, spec.brandDir);
    await fs.mkdir(spec.project, { recursive: true });
    if (spec.source) {
      spec.sourceBefore = await snapshot(spec.source);
      await fs.cp(spec.source, spec.brand, { recursive: true });
    } else await fs.cp(path.join(root, "tests/fixtures", spec.fixture), spec.project, { recursive: true });
    if (!spec.source) {
      spec.bundle = "proof-bundle.zip";
      execFileSync("python3", ["-c", "import pathlib,sys,zipfile; p=pathlib.Path(sys.argv[1]); z=zipfile.ZipFile(p/'proof-bundle.zip','w'); i=zipfile.ZipInfo('original.svg',(2026,1,1,0,0,0)); z.writestr(i,(p/sys.argv[2]).read_bytes()); z.close()", spec.brand, spec.target]);
      await fs.writeFile(path.join(spec.brand, spec.bundle + ".meta.json"), JSON.stringify({ title: "Editable masters bundle", category: "Bundles", role: "support", description: "Owned proof bundle containing the unchanged original SVG." }) + "\n");
    } else spec.bundle = spec.checks.bundle?.path;
    spec.original = await files(spec.brand);
    spec.original_snapshot = await snapshot(spec.brand);
    const config = JSON.parse(await fs.readFile(path.join(spec.brand, "brand.json")));
    if (spec.name === "selected" && process.env.WSTACK_BRAND_GUIDE) {
      const guide = process.env.WSTACK_BRAND_GUIDE;
      assert.ok(!path.isAbsolute(guide) && !guide.split(/[\\/]/).includes(".."), "WSTACK_BRAND_GUIDE must stay inside the brand folder");
      config.guide = guide;
      await fs.writeFile(path.join(spec.brand, "brand.json"), JSON.stringify(config, null, 2) + "\n");
      spec.original = await files(spec.brand); spec.original_snapshot = await snapshot(spec.brand);
    }
    spec.config = config;
    spec.guideRelative = spec.authored ? config.guide : ".wstack-brand/index.html";
    assert.ok(spec.guideRelative, `${spec.name}: selected authored guide must declare brand.json guide`);
    spec.guide = path.join(spec.brand, spec.guideRelative);
    spec.before = path.join(destination, `${spec.name}-before`);
    if (spec.authored) await fs.cp(spec.brand, spec.before, { recursive: true });
    if (spec.source) {
      const guide = await fs.readFile(spec.guide, "utf8");
      const oldIndex = path.relative(path.dirname(spec.guide), path.join(spec.brand, ".wstack-brand/index.html")).split(path.sep).join("/");
      const adapted = guide.split('href="' + oldIndex + '#catalog"').join('href="#resource-tools"')
        .split('href="' + oldIndex + '#prompt"').join('href="#resource-tools"')
        .split('href="' + oldIndex + '"').join('href="#resource-tools"');
      if (adapted !== guide) { await fs.writeFile(spec.guide, adapted); spec.adaptations = ["Retained legacy catalog/prompt navigation now targets the appended resource tools in the owned copy."]; }
      spec.original = await files(spec.brand); spec.original_snapshot = await snapshot(spec.brand);
    }
    spec.retained = path.join(destination, `${spec.name}-guide`);
    spec.style = JSON.stringify(JSON.parse(await fs.readFile(path.join(spec.brand, "style.json"))));
    await fs.writeFile(path.join(destination, `${spec.name}-style.json`), spec.style);
  }
  return specs;
}
function args(spec, action) {
  return ["brand", action, "--root", spec.project, "--brand-dir", spec.brandDir];
}
async function refresh(spec, cli, report) {
  const first = JSON.parse(cli([...args(spec, "refresh"), "--json"], `${spec.name}-refresh`));
  spec.refresh = first;
  const after = await files(spec.brand);
  const originalAfter = await snapshot(spec.brand);
  for (const [name, record] of Object.entries(spec.original_snapshot)) if (!name.startsWith(".wstack-brand/")) assert.deepEqual(originalAfter[name], record, `${spec.name} untouched source hash/mtime ${name}`);
  for (const [name, hash] of Object.entries(spec.original)) if (!name.startsWith(".wstack-brand/")) assert.equal(after[name], hash, `${spec.name} authored source ${name}`);
  if (spec.authored) {
    const guide = await fs.readFile(spec.guide, "utf8");
    assert.ok(/<\/body>/i.test(guide), "Authored guide needs a closing body for bounded fixture enhancement");
    const retainedHooks = spec.checks ? [spec.checks.download, spec.checks.bundle].filter(Boolean) : [];
    const hooks = retainedHooks.length ? '<script>(()=>{const bind=()=>{for(const row of ' + JSON.stringify(retainedHooks).replace(/</g, "\\u003c") + '){if(!WstackBrand.assets.some(asset=>asset.path===row.path))throw Error("Configured retained download is absent from catalog: "+row.path);for(const link of document.querySelectorAll(row.selector))link.dataset.wstackDownload=row.path;}};if(document.readyState==="loading")document.addEventListener("DOMContentLoaded",bind,{once:true});else bind();})();</script>' : "";
    const selectedStyle = spec.source ? '<style>#resource-tools{overflow-wrap:anywhere}#resource-tools [data-wstack-assets]{display:grid;grid-template-columns:repeat(auto-fit,minmax(min(100%,220px),1fr));gap:24px}#resource-tools [data-wstack-path]{padding:20px;border:1px solid currentColor;min-width:0}#resource-tools .catalog-controls{display:flex;flex-wrap:wrap;gap:16px;margin-block:24px}#resource-tools input,#resource-tools select{font:inherit;max-width:100%;min-width:0}#resource-tools textarea{width:100%;min-height:110px;font:13px/1.6 monospace}#resource-tools [data-wstack-preview]{max-width:100%}#resource-tools code{overflow-wrap:anywhere}</style>' : "";
    const scriptNames = ["catalog.js", "integration.js"];
    let tools = enhancement;
    for (const name of scriptNames) {
      const relative = path.relative(path.dirname(spec.guide), path.join(spec.brand, ".wstack-brand", name)).split(path.sep).join("/");
      const sources = [...guide.matchAll(/<script\b[^>]*src=["']([^"']+)["']/gi)].map(match => path.resolve(path.dirname(spec.guide), match[1]));
      const existing = sources.includes(path.join(spec.brand, ".wstack-brand", name));
      tools = tools.replace(`<script src=".wstack-brand/${name}"></script>`, existing ? "" : `<script src="${relative}"></script>`);
    }
    // A previously adopted guide keeps its catalog; the proof adds a separate bounded tool section.
    assert.ok(!guide.includes('id="resource-tools"'), "Selected guide already owns #resource-tools; choose a kit without the proof-only section");
    const extension = tools + hooks + selectedStyle;
    const enhanced = guide.includes("<!--wstack-proof-enhancement-->") ?
      guide.replace("<!--wstack-proof-enhancement-->", extension) : guide.replace(/<\/body>/i, extension + "</body>");
    await fs.writeFile(spec.guide, enhanced);
    spec.enhanced = await files(spec.brand);
    if (spec.original["theme.css"]) assert.equal(spec.enhanced["theme.css"], spec.original["theme.css"]);
    cli([...args(spec, "refresh"), "--json"], `${spec.name}-authored-enhancement`);
  }
  for (let index = 0; index < 2; index++) {
    const before = await snapshot(spec.brand);
    const result = JSON.parse(cli([...args(spec, "refresh"), "--json"]));
    assert.deepEqual(result.changed, [], `${spec.name}: repeated refresh ${index + 1}`);
    assert.deepEqual(await snapshot(spec.brand), before);
  }
  assert.equal(cli(args(spec, "style")).trim(), spec.style);
  await fs.cp(spec.brand, spec.retained, { recursive: true });
  report.coverage.push({ case: `${spec.name}-preservation`, status: "pass", actions: ["all original source hashes preserved by refresh", "authored guide retained", "two no-op refreshes byte-identical", "CLI/style source equality"] });
}
async function lifecycle(spec, cli, report) {
  if (!spec.authored || !spec.target) return;
  const authoredBefore = await snapshot(spec.brand);
  const guide = await fs.readFile(spec.guide, "utf8");
  const revision = '<p class="micro" data-author-revision>Field guide revision 02: make the next step useful.</p>';
  const revisedGuide = guide.includes("</h1>") ? guide.replace("</h1>", "</h1>" + revision) : guide.replace(/<\/body>/i, revision + "</body>");
  const stylesheetHref = spec.source ? (guide.match(/<link\b[^>]*href=["']([^"']+\.css)["']/i)?.[1] || "proof-authored.css") : "theme.css";
  const cssPath = path.resolve(path.dirname(spec.guide), stylesheetHref);
  assert.ok(cssPath.startsWith(spec.brand + path.sep), "Lifecycle stylesheet must resolve inside the copied brand folder");
  const stylesheet = path.relative(spec.brand, cssPath);
  await fs.writeFile(spec.guide, authoredBefore[stylesheet] ? revisedGuide : revisedGuide.replace(/<\/head>/i, `<link rel="stylesheet" href="${stylesheetHref}"></head>`));
  if (!authoredBefore[stylesheet]) await fs.writeFile(cssPath, "");
  await fs.appendFile(cssPath, spec.source ? "\n/* Owned proof revision of the added tools and authored annotation. */\n#resource-tools{padding-block:48px}[data-author-revision]{font-size:14px;font-style:italic}\n" : "\n/* Authored revision: a wider first application panel. */\n@media(min-width:651px){.examples{grid-template-columns:1.3fr 1fr;gap:28px}.examples .micro{letter-spacing:.025em}}\n");
  const authoredEdited = await snapshot(spec.brand);
  assert.notEqual(authoredEdited[spec.guideRelative].sha256, authoredBefore[spec.guideRelative].sha256);
  assert.notEqual(authoredEdited[stylesheet].sha256, authoredBefore[stylesheet]?.sha256);
  const guideHash = digest(await fs.readFile(spec.guide));
  const cssHash = digest(await fs.readFile(cssPath));
  const original = await fs.readFile(path.join(spec.brand, spec.target));
  const revisionSVG = !spec.source ? original.toString("utf8").replace(/#244938/gi, "#ad5837").replace(/#d9fd70/gi, "#fa8669") : original.toString("utf8") + "\n<!-- owned proof revision of original bytes -->\n";
  await fs.writeFile(path.join(spec.brand, spec.target), revisionSVG);
  const metadataName = spec.config.metadata;
  if (metadataName) {
    const manifestPath = path.join(spec.brand, metadataName), manifest = JSON.parse(await fs.readFile(manifestPath));
    let row = manifest.assets.find(item => item.file === spec.target);
    if (!row) { row = { file: spec.target }; manifest.assets.push(row); }
    row.title = "Revised retained resource";
    await fs.writeFile(manifestPath, JSON.stringify(manifest, null, 2) + "\n");
  }
  const sidecarPath = path.join(spec.brand, spec.target + ".meta.json");
  const sidecar = await fs.readFile(sidecarPath, "utf8").then(JSON.parse, () => ({}));
  if (!metadataName || "title" in sidecar) {
    sidecar.title = "Revised retained resource";
    await fs.writeFile(sidecarPath, JSON.stringify(sidecar, null, 2) + "\n");
  }
  const addition = "proof-added.svg";
  await fs.writeFile(path.join(spec.brand, addition), '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64"><path d="M8 32 32 8 56 32 32 56Z" fill="#244938"/></svg>\n');
  cli([...args(spec, "refresh"), "--json"], `${spec.name}-asset-metadata-add`);
  let rows = JSON.parse(cli([...args(spec, "list"), "--json"])).assets;
  assert.equal(rows.find(row => row.path === spec.target).title, "Revised retained resource");
  assert.ok(rows.some(row => row.path === addition));
  await fs.rm(path.join(spec.brand, addition));
  cli([...args(spec, "refresh"), "--json"], `${spec.name}-asset-remove`);
  rows = JSON.parse(cli([...args(spec, "list"), "--json"])).assets;
  assert.ok(!rows.some(row => row.path === addition));
  for (let index = 0; index < 2; index++) {
    const before = await snapshot(spec.brand);
    assert.deepEqual(JSON.parse(cli([...args(spec, "refresh"), "--json"])).changed, []);
    assert.deepEqual(await snapshot(spec.brand), before);
  }
  assert.equal(digest(await fs.readFile(spec.guide)), guideHash, "Authored HTML survives catalog changes");
  assert.equal(digest(await fs.readFile(cssPath)), cssHash, "Authored CSS survives catalog changes");
  const authoredAfter = await snapshot(spec.brand);
  for (const [name, record] of Object.entries(authoredEdited)) {
    if (name.startsWith(".wstack-brand/") || name === spec.target || name === metadataName || name === spec.target + ".meta.json") continue;
    assert.deepEqual(authoredAfter[name], record, `Unrelated edited source hash/mtime preserved: ${name}`);
  }
  report.authored_revisions ||= [];
  report.authored_revisions.push({ name: spec.name, before: authoredBefore, edited: authoredEdited, after: authoredAfter });
  spec.revised = path.join(spec.retained, spec.target);
  const oldDownloads = Object.keys(await files(spec.retained)).filter(name => name.startsWith(".wstack-brand/downloads/"));
  await fs.rm(spec.retained, { recursive: true });
  await fs.cp(spec.brand, spec.retained, { recursive: true });
  const refreshedFiles = await files(spec.brand);
  for (const old of oldDownloads) if (!(old in refreshedFiles)) assert.ok(!(old in await files(spec.retained)), `Retained obsolete payload: ${old}`);
  report.coverage.push({ case: `${spec.name}-lifecycle`, status: "pass", actions: ["asset byte revision", "manifest title revision", "asset addition/removal", "edited authored HTML/CSS layout/content preserved with unrelated hashes/mtimes", "two subsequent byte-identical no-op refreshes"] });
}
module.exports = { prepare, refresh, lifecycle, args };
