# Maintain

Use for release artifacts, resource ownership or cleanup questions. The core
Public/Private responsibilities apply throughout work without loading this guide.
Choose useful records and methods proportional to the task's impact.

## Public

- Match affected feature descriptions, downloads and other public surfaces to delivered decisions and behavior.
- Check actual distribution artifacts against the intended release; labels or a source build alone do not establish package contents. Preserve needed release inputs and evidence.

## Private

- Plan ownership/lifecycle before creating a resource. Disposable isolation is useful when cleanup is reliable; one global simulator is not required.
- Preserve useful diagnostics before removing temporary resources; clean previews when review ends. Tidy encountered stale merged leftovers while preserving active/shared/unrelated/useful work.
- Continuing work needs a durable, recoverable handoff with intentional ownership, rather than an orphaned checkout. Keep useful private decisions available for later work.
- Browser sessions follow project rules: reuse existing automation browser; bundled Chromium/Chrome for Testing for disposable tests; graceful `finally` closure, preserve user browser/profiles.
