# Meguri

Pomodoro timer with your random images (full AI slop and bullshit, maybe I rewrite it in the future).

Portable binary, no installation needed.

## How it works

1. Launch the app — a small square widget appears on screen
2. Configure an images folder and create actions (activities) in settings
3. Press **Start** — a timer begins (default: 10 min)
4. When the timer ends, a pulsing confirmation appears (default: 1 min to confirm)
5. **Confirm** — the session is logged, a new image appears, and the timer restarts automatically
6. **Miss the confirmation or press Stop** — nothing is logged, timer resets

All data (settings, actions, statistics) is stored in `meguri_data.json` next to the executable. In dev `src-tauri/target/debug/meguri_data.json`

## Widget controls

- **Hover** to reveal the overlay
- **Center**: Start / Pause button
- **Top-left corner** (on hover): current active action name
- **Top-right corner** (on hover): settings button
- Settings page has a close (X) button to quit the app

## Build

Requires: [Rust](https://rustup.rs/), [Node.js](https://nodejs.org/), [pnpm](https://pnpm.io/)

```bash
pnpm install
pnpm tauri build
```

Output binary: `src-tauri/target/release/meguri.exe`

## Dev

```bash
pnpm tauri dev
```

## Stack

Tauri 2, Vue 3, TypeScript, Catppuccin Mocha theme
