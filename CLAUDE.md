# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

VRAMS — a local asset manager for VRChat / Unity creators. Tauri v2 desktop app: **React 19 + TypeScript** frontend (Vite, Ant Design, Zustand, React Router) and a **Rust** backend with **SQLite** (rusqlite, bundled). Package manager is **pnpm**.

**v2 rewrite in progress** on branch `v2`. Scope, decisions and data model live in [`docs/v2-需求与框架.md`](docs/v2-需求与框架.md) — treat it as the source of truth. v1 (Vue, tag manager only) is archived at tag `v1-final`; its docs are in `docs/archive/v1/`.

The user is learning Tauri / Rust / React through this project: explain the principle before the code, and follow `.claude/skills/mentor/SKILL.md`. Reply in Chinese.

## Commands

```bash
pnpm dev            # Vite dev server only (port 1420)
pnpm build          # tsc --noEmit type-check + vite build
pnpm tauri dev      # desktop app with hot reload
pnpm tauri build    # installers (MSI / NSIS)

cd src-tauri
cargo test
cargo clippy --all-targets -- -D warnings   # CI treats warnings as errors
```

- Tauri dev uses **fixed port 1420** and fails if it's taken.
- Local Rust toolchain lives on E: (`RUSTUP_HOME=E:\DevTools\rustup`, `CARGO_HOME=E:\DevTools\cargo`); MSVC is VS 2022 Build Tools (C++ only).
- CI (`.github/workflows/ci.yml`, windows-latest) runs pnpm build, cargo test and strict clippy on every push to any branch.

## Architecture

### Frontend (`src/`)

- `main.tsx` — mounts `<App />` into `#root`, wrapped in antd `ConfigProvider` (zh_CN) and `App`.
- `App.tsx` — root component (currently a placeholder).
- `@/` alias → `src/` (configured in both `vite.config.ts` and `tsconfig.json` `paths`; TypeScript 7 has no `baseUrl`, so paths must start with `./`).

### Backend (`src-tauri/`)

- `src/lib.rs` — registers plugins (opener, dialog), opens the database in the `.setup()` hook and injects `AppState { db: Mutex<Connection> }` via `app.manage()`. No commands registered yet.
- `src/db/mod.rs` — `initialize_database()` + migration runner. Migrations are listed in the `MIGRATIONS` array (`include_str!` of `src-tauri/migrations/V00N__*.sql`); each runs in its own transaction and bumps `PRAGMA user_version`. Never edit a released migration — add a new one.
- `src/utils/path.rs` — library path: `VRAMS_LIBRARY_PATH` env var if set, otherwise `{app_data_dir}/library` (Windows: `%APPDATA%\com.cxpller.vrams\library`). Never next to the exe.
- Modules are added when first needed (scanner, commands, …); dependencies too (`cargo add`), e.g. zip/tar/flate2 for archive parsing.

### IPC

```ts
import { invoke } from "@tauri-apps/api/core";
const result = await invoke("command_name", { someArg: 1 });
// JS args are camelCase; Tauri maps them to Rust snake_case params.
// Return values are serialized by serde as-is (snake_case fields).
```

### Capabilities (`src-tauri/capabilities/default.json`)

`core:default`, `opener:default`, `dialog:default`, `core:window:default`. File system work is done in Rust, so there is no fs plugin.
