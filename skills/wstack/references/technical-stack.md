# Stack

Defaults for new work; needed layers only. Preserve working stack/managers/lockfiles; practical alternatives → choose + explain.
- Backend → PostgreSQL + Rust/Axum + OpenAPI + Zod. Generate/reuse one contract source where practical.
- Web-only → React + TypeScript + Tailwind + shadcn.
- Shared web/mobile → Expo/Expo Router + React Native + NativeWind.
- Shared web/desktop → Electron/Forge; native when OS integration makes it practical.
- TUI → OpenTUI.
- JS/TS → pnpm; Rust → Cargo; services → Docker/Compose.
- Platform-compatible UI: shadcn → React DOM; React Native → native primitives. Preserve approved tokens/components.
- Read current official docs when compatibility matters; no framework migration merely to use a tool.
- Electron → narrow preload/IPC; context isolation on, renderer Node off.
