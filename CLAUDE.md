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

- `data.rs` - structs: `AppData`, `Settings`, `Action`, `StatEntry`. File I/O: `load_or_create()`, `save()`, `data_file_path()`. Settings includes `tiktok_mode`, `tiktok_url`, `hotkeys_enabled`, and `hotkey_*` fields.
- `commands.rs` - 18 Tauri commands:
  - Data: `load_data`, `save_settings`, `get_random_image` (returns `ImageResponse { data_url, path }`), `get_stats`.
  - Actions: `add_action`, `update_action`, `delete_action`, `set_active_action`.
  - Sessions: `log_session`.
  - TikTok: `open_tiktok`, `close_tiktok`, `destroy_tiktok`, `tiktok_eval` (JS injection into webview), `sync_tiktok_webview`, `open_tiktok_menu`, `reset_tiktok_size`.
  - TikTok UI toggle: `get_tiktok_buttons_hidden`, `toggle_tiktok_buttons_hidden` (state stored in `TiktokButtonsHidden(Mutex<bool>)`).
- `images.rs` - reads random image from configured directory, returns `ImageResult { data_url, path }`. Filters by extension (png/jpg/jpeg/gif/webp/bmp).
- `lib.rs` - wires modules, registers plugins (opener, dialog, global-shortcut), state and commands.

### Frontend (`src/`)

No router, no Pinia. Two views toggled via `state.currentView` reactive ref.

- `composables/useAppState.ts` - singleton reactive state, wraps all `invoke()` calls. State includes `tiktokViews` (earned view count, not persisted), `tiktokOpen` (panel visibility), `imagePath` (current image file path). Exports `openTikTok()`, `closeTikTok()`, `destroyTikTok()`, `incrementTiktokViews()`, `decrementTiktokViews()`.
- `composables/useTimer.ts` - timer state machine: `idle` -> `running` -> `confirming` -> `idle`. Uses `setInterval` (1s tick). Returns `timerState`, `displayTime`, `progress`, `start()`, `stop()`, `confirm()`.
- `composables/useHotkeys.ts` - global shortcut registration via `@tauri-apps/plugin-global-shortcut`. `refreshHotkeys()` unregisters all and re-registers from settings. `onHotkeyAction(fn)` sets a handler called on shortcut press.
- `composables/useTiktokBridge.ts` - JS injection into TikTok webview. `init()` injects MutationObserver that hides UI elements (HIDE_SELECTORS). `exec(action)` runs named JS snippets (prev/next video clicks). `setHidden(bool)` and `toggleHidden()` control UI visibility. The observer respects `window.__meguri_hide` flag.
- `App.vue` - loads data on mount, switches between WidgetView and SettingsView. When TikTok is open, renders expanded layout with custom titlebar (burger menu, view counter, prev/next arrows, close button) and positions widget via computed styles based on `widget_position`.

### Widget view components

- `WidgetView.vue` - 160x160 square, tracks hover, orchestrates timer. Right-click context menu with force complete, refresh image, and position selector (4 SVG corner icons in a row). `data-tauri-drag-region` for dragging. Registers global hotkey handler on mount.
- `ImageDisplay.vue` - shows base64 image or placeholder SVG.
- `HoverOverlay.vue` - shown on widget hover. Three corner hover zones: top-left (action name, clickable to open action picker with keyboard navigation), top-right (drag handle + settings gear), bottom-right (TikTok button with view count, only if `tiktok_mode` enabled). Center: start/stop button with timer display.
- `ConfirmationPulse.vue` - pulsing overlay during confirmation period, click to confirm.

### Settings view components

- `SettingsView.vue` - resizes window to 350x450 on mount, back to 160x160 on close. Three tabs (General, Actions, Statistics). Back arrow returns to widget, close (X) button quits app.
- `SettingsForm.vue` - image path with browse button, current image path (clickable to copy), timer minutes, confirmation minutes, widget position dropdown, TikTok mode checkbox, TikTok URL input (shown when mode enabled), global hotkeys toggle with per-action key recorder. Save button with "Saved" feedback.
- `ActionsManager.vue` - CRUD list with active selection (radio buttons). Double-click or edit button to rename.
- `StatsDisplay.vue` - GitHub-style contribution grid: all-time total, custom month dropdown, 7-column calendar heatmap (Mon-Sun, green intensity levels, CSS tooltips on hover), legend, daily breakdown with per-action session/time detail.
- `TiktokMenu.vue` - separate Tauri window (220x74), closes on focus loss. Reset size button, toggle TikTok UI buttons (show/hide). State synced via Rust-side `TiktokButtonsHidden`.

### TikTok feature

Embedded TikTok viewer as reward for completed Pomodoro sessions.

- **Earning views**: `tiktokViews` counter increments on session confirm and force complete (if `tiktok_mode` enabled). Counter is frontend-only, resets on app restart.
- **Opening**: expands main window from 160x160 to 244x430. Creates a child webview (`tauri::webview::WebviewBuilder`) inside the main window using Tauri's unstable multi-webview API. URL defaults to `https://www.tiktok.com/foryou`, configurable via `tiktok_url` setting. Injects `useTiktokBridge.init()` to set up MutationObserver.
- **Titlebar**: burger menu, minus button (decrements view count), prev/next video arrows, close button.
- **JS Bridge**: `tiktok_eval` command runs arbitrary JS in the webview. Init script sets up `MutationObserver` to hide elements matching `HIDE_SELECTORS` (nav arrows, search, mini-player, etc.). Supports toggling via `window.__meguri_hide` flag.
- **Closing (hide)**: shrinks child webview to 0x0, collapses window back. Webview stays alive to preserve page state (scroll position, auth).
- **Destroying**: `destroy_tiktok` actually closes the webview. Called when entering settings to avoid layout conflicts.
- **Resize**: window is resizable when TikTok is open. `on_window_event(Resized)` listener syncs child webview size automatically. Disabled on close.
- **Layout**: titlebar (28px) rendered by Vue in App.vue. Child webview positioned below titlebar. Widget stays in its corner via absolute positioning. Position varies by `widget_position` (TikTok above widget for bottom-* positions, below for top-*).

## Key decisions

- Images loaded as base64 data URLs via custom Rust command (avoids Tauri asset protocol scope complexity). `get_random_image` returns both data URL and file path.
- Timer runs in frontend JS (no need to survive app restarts, all transitions are UI-driven).
- No window decorations. Window: 160x160, always on top, transparent, skip taskbar.
- Tauri `unstable` feature enabled for multi-webview support (`Window::add_child`, `app.get_webview()`, `app.get_window()`).
- Tauri capabilities in `src-tauri/capabilities/default.json` - must add permissions for any new window API usage (includes global-shortcut permissions).
- When adding new fields to `Settings` struct, use `#[serde(default)]` for backwards compatibility with existing data files.
- TikTok webview hidden (size 0x0) instead of destroyed on close, to preserve page state across open/close cycles.
- TikTok UI toggle state stored in Rust-side `TiktokButtonsHidden(Mutex<bool>)` to stay in sync across the main window and the separate menu window.
- Global hotkeys use `tauri-plugin-global-shortcut`, registered/unregistered dynamically on settings save and app mount.

## Style guide

- Catppuccin Mocha palette via CSS variables (`--ctp-*`) defined in `src/assets/theme.css`.
- Inline SVG icons throughout (no icon library).
- Vue `<script setup lang="ts">` with Composition API.
- Scoped styles in every component.
