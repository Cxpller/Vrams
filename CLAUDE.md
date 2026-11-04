# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Tauri v2 desktop app with a Vue 3 + TypeScript frontend (Vite) and a Rust backend. Package manager is **pnpm**.

## Commands

```bash
# Frontend dev server (Vite only, port 1420)
pnpm dev

# Type-check + build frontend for production
pnpm build

# Preview production build
pnpm preview

# Tauri dev (launches desktop app with hot-reload)
pnpm tauri dev

# Tauri production build
pnpm tauri build
```

- `pnpm build` runs `vue-tsc --noEmit` (type-checking) then `vite build`.
- The Tauri dev server uses **fixed port 1420** — it fails if the port is unavailable.

## Architecture

### Frontend (`src/`)

- **`src/main.ts`** — Entry point. Mounts the root Vue component (`App.vue`) onto `#app` in `index.html`.
- **`src/App.vue`** — Root SFC using `<script setup lang="ts">`. All Vue components use this pattern.
- **`vite.config.ts`** — Vite config tailored for Tauri: port 1420, strict port, ignores `src-tauri/` from file watching, HMR host detection for Tauri's webview.

### Backend (`src-tauri/`)

- **`src-tauri/src/main.rs`** — Binary entry point. Calls `vrams_lib::run()`.
- **`src-tauri/src/lib.rs`** — Application logic. Registers Tauri `#[tauri::command]` functions via `tauri::generate_handler!` (currently 4 tag commands in `commands/tag_commands.rs`), registers plugins, opens the database in the `.setup()` hook and injects `AppState { db: Mutex<Connection> }` via `app.manage()`, and runs the Tauri app.
- **Library path** (`src-tauri/src/utils/path.rs`) — `VRAMS_LIBRARY_PATH` env var if set, otherwise `{app_data_dir}/library` (Windows: `%APPDATA%\com.cxpller.vrams\library`). Never next to the exe: Program Files is read-only and uninstall wipes the install dir.
- **`src-tauri/Cargo.toml`** — Rust crate config. The library crate is named `vrams_lib` (`crate-type = ["staticlib", "cdylib", "rlib"]`). Dependencies: `tauri` v2, `tauri-plugin-opener`/`tauri-plugin-dialog`/`tauri-plugin-fs`, `rusqlite` (bundled SQLite), `serde`/`serde_json`, `zip`/`tar`/`flate2` (package parsing, Phase 4), `scraper` (HTML parsing, Phase 7), `reqwest` (downloads).
- **`src-tauri/build.rs`** — Standard Tauri build script (`tauri_build::build()`).
- **`src-tauri/tauri.conf.json`** — Tauri configuration: product name, identifier (`com.cxpller.vrams`), build commands, dev URL (`http://localhost:1420`), window settings (1200×800), CSP, bundle icons.

### IPC (Frontend → Rust)

Frontend calls Rust commands via `invoke()` from `@tauri-apps/api/core`:

```ts
import { invoke } from "@tauri-apps/api/core";
// Invokes the Rust #[tauri::command] fn get_tags(state: State<AppState>) -> Result<Vec<Tag>, String>
const tags = await invoke("get_tags");
// Note: JS parameter names use camelCase; Tauri maps them to the Rust snake_case args automatically.
```

### Capabilities (`src-tauri/capabilities/default.json`)

Permissions model for the main window. Currently grants `core:default`, `opener:default`, `dialog:default`, `fs:default`, `core:window:default`.

### TypeScript Config

- **`tsconfig.json`** — Strict mode, ES2020 target, bundler module resolution, no emit. Includes `src/**/*.ts`, `src/**/*.vue`.
- **`tsconfig.node.json`** — References `vite.config.ts` (Node/bundler context).
