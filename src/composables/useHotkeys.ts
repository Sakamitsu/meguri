import { register, unregister, isRegistered } from '@tauri-apps/plugin-global-shortcut'
import { useAppState } from './useAppState'

type HotkeyAction =
  | 'move_top_left'
  | 'move_top_right'
  | 'move_bottom_left'
  | 'move_bottom_right'
  | 'start_stop'

type HotkeyHandler = (action: HotkeyAction) => void

const registeredShortcuts: string[] = []
let handler: HotkeyHandler | null = null

const hotkeyFieldMap: { field: string; action: HotkeyAction }[] = [
  { field: 'hotkey_move_top_left', action: 'move_top_left' },
  { field: 'hotkey_move_top_right', action: 'move_top_right' },
  { field: 'hotkey_move_bottom_left', action: 'move_bottom_left' },
  { field: 'hotkey_move_bottom_right', action: 'move_bottom_right' },
  { field: 'hotkey_start_stop', action: 'start_stop' },
]

async function unregisterAll() {
  for (const shortcut of registeredShortcuts) {
    try {
      if (await isRegistered(shortcut)) {
        await unregister(shortcut)
      }
    } catch { /* ignore */ }
  }
  registeredShortcuts.length = 0
}

async function registerAll() {
  const { state } = useAppState()
  if (!state.settings.hotkeys_enabled) return

  for (const { field, action } of hotkeyFieldMap) {
    const shortcut = (state.settings as any)[field] as string
    if (!shortcut) continue

    try {
      if (await isRegistered(shortcut)) continue
      await register(shortcut, (event) => {
        if (event.state === 'Pressed' && handler) {
          handler(action)
        }
      })
      registeredShortcuts.push(shortcut)
    } catch (e) {
      console.warn(`Failed to register shortcut "${shortcut}":`, e)
    }
  }
}

export async function refreshHotkeys() {
  await unregisterAll()
  await registerAll()
}

export function onHotkeyAction(fn: HotkeyHandler) {
  handler = fn
}
