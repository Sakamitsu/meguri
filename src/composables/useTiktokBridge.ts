import { invoke } from '@tauri-apps/api/core'

/**
 * TikTok bridge — executes actions inside the TikTok webview via JS injection.
 *
 * Each action is a named JS snippet that interacts with TikTok's DOM.
 * Extend `actions` to add new behaviors (like, mute, hide UI, etc).
 */

const actions: Record<string, string> = {
  prev: `document.querySelector('button[data-e2e="arrow-left"]')?.click()`,
  next: `document.querySelector('button[data-e2e="arrow-right"]')?.click()`,
}

async function exec(action: string): Promise<void> {
  const js = actions[action]
  if (!js) {
    console.warn(`TikTok bridge: unknown action "${action}"`)
    return
  }
  try {
    await invoke('tiktok_eval', { js })
  } catch (e) {
    console.warn(`TikTok bridge: failed to exec "${action}"`, e)
  }
}

export function useTiktokBridge() {
  return {
    exec,
    prevVideo: () => exec('prev'),
    nextVideo: () => exec('next'),
  }
}
