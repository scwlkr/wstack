# Optional guide integrations

Author ordinary HTML/CSS/JS in any composition. Add these local classic scripts
in order (paths here assume an entry point at the brand-folder root):

```html
<script src=".wstack-brand/catalog.js" defer></script>
<script src=".wstack-brand/integration.js" defer></script>
```

They work on file URLs without fetch, modules or a server. Refresh resources
before opening the guide. The integration adds no global CSS. Style your own
controls/cards, preserve established interactions and supply labels/headings.

## Catalog

```html
<section aria-labelledby="library-title" data-wstack-catalog>
  <h2 id="library-title">Artwork library</h2>
</section>
```

The integration mounts once and adds search, category, Current assets / All
resources scope, live result count, cards and an empty state. Its selectors are
`[data-wstack-search]`, `category`, `scope`, `count`, `assets`, `empty`, `status`
with the same `data-wstack-` prefix. Preauthor controls/containers with those
attributes to place them in your layout. Mount preserves other authored content.
Category and scope selects receive options from the current catalog. Search is
case insensitive across row descriptions/metadata within the chosen scope.

Cards expose `data-wstack-path` and `data-wstack-role`, with headings, honest kind/
role labels, lazy image previews and downloads. Preview buttons open an enlarged
native dialog; Escape/Close restores focus. Hidden cards use `hidden`; ensure your
CSS respects it (`[hidden] { display: none !important; }`). You control spacing,
type, colors and responsive layout. Keep images contained at narrow widths.

For a wholly bespoke library, read `WstackBrand.assets` and compose your own
markup. `window.wstackBrandData` is the generated `{assets, style}` data, not an
editable authority. Rows add live `sha256` and a digest-based `download` URL to
the CLI catalog fields. `WstackBrand.mount(elementOrSelector)` mounts additional
catalog roots when needed. No per-project importer or framework is required.

## Downloads from an existing main guide

Keep the original anchor and stable asset URL; add the optional download hook:

```html
<a href="symbols/primary.svg" download data-wstack-download="symbols/primary.svg">Download logo</a>
<a href="vectors.zip" download data-wstack-download="vectors.zip">Download vectors</a>
```

Paths are relative to the brand folder, even from a nested guide. Any discovered
file, including an existing bundle, works. Clicks load only that file's byte chunk,
then save a Blob with its original basename. This repairs browsers that navigate
instead of downloading from file URLs. An adjacent live status reports failures.
Refresh after source revisions; reopen the guide to consume the new catalog.

`await WstackBrand.download("symbols/primary.svg")` provides the same behavior
for owner controls, returning `{path, bytes}`; callers handle/report rejection.
Existing bundle creation remains owner tooling; Wstack catalogs/downloads real
bundle files and does not invent an archive authority.

## Exact prompt copying

Place the prompt where it belongs in your guide:

```html
<section>
  <h2>Image direction</h2>
  <label for="prompt">Append after your subject and medium</label>
  <textarea id="prompt" data-wstack-style readonly rows="6"></textarea>
  <button type="button" data-wstack-copy="#prompt">Copy style JSON</button>
  <p data-wstack-status role="status" aria-live="polite"></p>
</section>
```

The field receives exactly the compact style source. Copy uses the browser
clipboard or local-file copy fallback; denied copying focuses/selects the exact
text and tells the user to press Ctrl+C or Command+C. The field stays editable
through the source file `style.json`; refresh after editing it.

`await WstackBrand.copy(fieldOrSelector, statusOrSelector)` returns true on copy,
false on the selected-text fallback. `WstackBrand.style` exposes the same compact
string for bespoke controls. Color-copy controls remain your authored interaction;
check their actual pasted text as well as prompt copying in both browser engines.

## Review and repeatability

Exercise main-guide links, previews, color copying and downloads alongside new
integrations. Compare saved bytes to sources and pasted JSON to CLI/source output.
Inspect file URLs and loopback at 320, 390 and 1440 pixels, keyboard/focus, local
fonts/images, categories/scope/empty state and denied copy. Observe network/resource
requests: no download chunk should load before a download click. Verify the first,
later and revised-source downloads. Review composition, identifying artwork,
primary/support color roles and useful examples; automated overflow checks do
not establish expressive design.

Use the repository's `./project verify brand` recipe for owned fixture copies and
its documented selected-kit mode for real projects. Never mutate an owner's kit
just to verify it. Preserve evidence after owned browser/preview teardown.
