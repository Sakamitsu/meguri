# Meguri

Pomodoro-style desktop widget with random images, activity tracking, and an embedded TikTok reward system.

Portable binary, no installation needed. All data stored in a single `meguri_data.json` file next to the executable.

Full AI slop and bullshit, maybe I rewrite it in the future.

## How it works

1. Launch the app — a small 160x160 square widget appears, always on top
2. Open settings (gear icon on hover), configure an images folder and create actions (activities)
3. Press **Start** — a Pomodoro timer begins (default: 10 min)
4. When the timer ends, a pulsing confirmation overlay appears (default: 1 min window)
5. **Confirm** — session is logged, a new random image appears, timer restarts automatically
6. **Miss the confirmation or press Stop** — nothing is logged, timer resets

## Widget controls

**Hover** the widget to reveal the overlay:

- **Top-left**: active action name — click to open a dropdown and switch between actions (keyboard: arrows + Enter)
- **Top-right**: drag handle + settings gear
- **Center**: Start / Stop button with timer countdown
- **Bottom-right** (if TikTok mode enabled): TikTok button with earned views count

**Right-click** for context menu:

- **Force complete** — logs elapsed time (rounded up to the nearest minute), earns a TikTok view
- **Refresh image** — loads a new random image from the configured folder
- **Position selector** — move widget to any screen corner (bottom-left, top-left, top-right, bottom-right)

## Settings

Three tabs: **General**, **Actions**, **Statistics**.

### General

- **Images folder** — path to a directory with images (PNG, JPG, GIF, WebP, BMP)
- **Timer / Confirmation** — duration in minutes
- **Widget position** — screen corner placement
- **TikTok mode** — enables the TikTok reward feature (see below)
- **Global hotkeys** — system-wide keyboard shortcuts, works without app focus:
  - Start / Stop / Confirm (single hotkey cycles through all timer states)
  - Move widget to each of the four corners

### Actions

Create, rename, and delete activities. Select the active one — all sessions are logged under it.

### Statistics

- **All-time total** — sessions count and total time
- **Monthly heatmap** — GitHub-style calendar grid with green intensity levels, month selector dropdown
- **Daily breakdown** — per-day session list with per-action detail, sorted newest first

## TikTok reward

An embedded TikTok viewer that unlocks after completing Pomodoro sessions.

- Each confirmed session (or force complete) earns one **view**
- Click the TikTok button on the widget to spend a view and open the panel
- The widget expands to show a titlebar + embedded TikTok feed below (or above, depending on widget position)
- **Titlebar controls**: burger menu, view counter (click minus to spend a view), prev/next video arrows, close
- **Burger menu**: Reset size, Toggle TikTok UI buttons (show/hide native navigation)
- The TikTok webview is hidden (not destroyed) on close, preserving scroll position and auth
- URL is configurable (defaults to `tiktok.com/foryou`)
- A JS bridge injects a MutationObserver to hide TikTok's native UI elements (search, nav arrows, mini-player, etc.)

## Build

Requires: [Rust](https://rustup.rs/), [Node.js](https://nodejs.org/), [pnpm](https://pnpm.io/)

```bash
pnpm install
pnpm tauri build
```

Output: `src-tauri/target/release/meguri.exe`

## Dev

```bash
pnpm tauri dev
```

## Stack

- **Frontend**: Vue 3 + TypeScript (Composition API, no router, no Pinia)
- **Backend**: Tauri 2 + Rust
- **Theme**: Catppuccin Mocha
- **Plugins**: tauri-plugin-dialog, tauri-plugin-global-shortcut
- **Multi-webview**: Tauri unstable API for embedded TikTok viewer
