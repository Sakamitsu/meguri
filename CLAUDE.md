# Meguri

Pomodoro-style desktop widget built with Tauri 2 + Vue 3 + TypeScript. Catppuccin Mocha theme.

## Commands

- `pnpm tauri dev` - run dev server (port 1420 + Rust binary)
- `pnpm tauri build` - production build, output: `src-tauri/target/release/meguri.exe`
- `npx vue-tsc --noEmit` - type-check frontend
- `cd src-tauri && cargo check` - check Rust compilation

## Architecture

### Data flow

All data lives in a single JSON file (`meguri_data.json`) next to the executable (in dev: `src-tauri/target/debug/meguri_data.json`). The Rust backend holds data in `Mutex<AppData>` managed state, flushing to disk on every write via atomic tmp+rename.

### Rust backend (`src-tauri/src/`)

- `data.rs` - structs: `AppData`, `Settings`, `Action`, `StatEntry`. File I/O: `load_or_create()`, `save()`, `data_file_path()`.
- `commands.rs` - 9 Tauri commands: `load_data`, `save_settings`, `get_random_image`, `add_action`, `update_action`, `delete_action`, `set_active_action`, `log_session`, `get_stats`. All use `State<Mutex<AppData>>`.
- `images.rs` - reads random image from configured directory, returns base64 data URL. Filters by extension (png/jpg/jpeg/gif/webp/bmp).
- `lib.rs` - wires modules, registers state and commands.

### Frontend (`src/`)

No router, no Pinia. Two views toggled via `state.currentView` reactive ref.

- `composables/useAppState.ts` - singleton reactive state, wraps all `invoke()` calls. Shared across components.
- `composables/useTimer.ts` - timer state machine: `idle` -> `running` -> `confirming` -> `idle`. Uses `setInterval` (1s tick). Returns `timerState`, `displayTime`, `progress`, `start()`, `stop()`, `confirm()`.
- `App.vue` - loads data on mount, switches between WidgetView and SettingsView.

### Widget view components

- `WidgetView.vue` - 160x160 square, tracks hover, orchestrates timer. `data-tauri-drag-region` for dragging.
- `ImageDisplay.vue` - shows base64 image or placeholder SVG.
- `HoverOverlay.vue` - shown on widget hover. Two corner hover zones (top-left: action name, top-right: settings gear). Center: start/stop button with timer display above it (absolutely positioned to not shift the button).
- `ConfirmationPulse.vue` - pulsing overlay during confirmation period, click to confirm.

### Settings view components

- `SettingsView.vue` - resizes window to 350x450 on mount, back to 160x160 on close. Three collapsible `<details>` sections. Close (X) button quits app.
- `SettingsForm.vue` - image path, timer minutes, confirmation minutes.
- `ActionsManager.vue` - CRUD list with active selection (radio buttons). Double-click or edit button to rename.
- `StatsDisplay.vue` - groups stats by action, shows sessions count and total time.

## Key decisions

- Images loaded as base64 data URLs via custom Rust command (avoids Tauri asset protocol scope complexity).
- Timer runs in frontend JS (no need to survive app restarts, all transitions are UI-driven).
- No window decorations. Window: 160x160, always on top, transparent, skip taskbar.
- Tauri capabilities in `src-tauri/capabilities/default.json` - must add permissions for any new window API usage.
- When adding new fields to `Settings` struct, use `#[serde(default = "...")]` for backwards compatibility with existing data files.

## Style guide

- Catppuccin Mocha palette via CSS variables (`--ctp-*`) defined in `src/assets/theme.css`.
- Inline SVG icons throughout (no icon library).
- Vue `<script setup lang="ts">` with Composition API.
- Scoped styles in every component.
