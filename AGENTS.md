## wstack repo rules

- Layout: `skills/<name>/SKILL.md` (suite), `shared/` (canonical rules), `cli/` (Rust `wstack` binary), `tests/` (Rust tests + fixtures).
- Edit rules in `shared/` only. Skills carry vendored copies (and `wstack/SKILL.md` an inline principles block between `wstack:begin/end` markers) listed in `shared/sync.json`; run `wstack sync` after editing (`cargo run -- sync` from `cli/`).
- Skill links stay inside their own skill folder so each skill installs alone. Members refer to `wstack` by name.
- New skill: folder + `SKILL.md` (name = folder), README table row, `skills.sh.json` grouping.
- Files ≤300 lines; small functions; std-first, minimal dependencies.
- Before commit: `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings` in `cli/`, and `cargo run -- check` from `cli/` must pass.
- No em dashes in README prose.
