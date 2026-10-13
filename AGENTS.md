## Repository

- Coding start → `2brain`; extra tool/project context only when needed.
- One skill: `skills/wstack/`. Canonical rules: `shared/`; edit there → `cargo run -- sync` in `cli/`.
- Skill references stay inside its folder. Update README/catalog when packaging changes.
- Rust CLI: `cli/`; checks: `tests/`. Std-first, minimal dependencies.
- ≤300 lines default; split when useful. Clear names, small functions.
- Before commit, in `cli/`: `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `cargo run -- check`. Setup changes → focused Python checks (`./project ci` includes them).
- README prose: no em dashes.

<!-- wstack:begin setup -->
Wstack project notes → [w.md](w.md).
<!-- wstack:end setup -->
