<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'

let unlisten: (() => void) | null = null

async function resetSize() {
  // Read widget_position from main app state via load_data
  const data = await invoke<{ settings: { widget_position: string } }>('load_data')
  await invoke('reset_tiktok_size', { widgetPosition: data.settings.widget_position })
  await getCurrentWindow().close()
}

onMounted(async () => {
  const win = getCurrentWindow()
  unlisten = await win.onFocusChanged(({ payload: focused }) => {
    if (!focused) win.close()
  })
})

onUnmounted(() => {
  if (unlisten) unlisten()
})
</script>

<template>
  <div class="menu">
    <button class="menu-item" @click="resetSize">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <polyline points="15 3 21 3 21 9" />
        <polyline points="9 21 3 21 3 15" />
        <line x1="21" y1="3" x2="14" y2="10" />
        <line x1="3" y1="21" x2="10" y2="14" />
      </svg>
      <span>Reset size</span>
    </button>
  </div>
</template>

<style scoped>
.menu {
  background: var(--ctp-mantle, #181825);
  border: 1px solid var(--ctp-surface0, #313244);
  border-radius: 8px;
  padding: 4px;
  width: 100vw;
  height: 100vh;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
}

.menu-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 10px;
  background: none;
  border: none;
  border-radius: 6px;
  color: var(--ctp-text, #cdd6f4);
  font-size: 12px;
  cursor: pointer;
  width: 100%;
}

.menu-item:hover {
  background: var(--ctp-surface0, #313244);
}
</style>
