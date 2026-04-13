<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'

let unlisten: (() => void) | null = null
const buttonsHidden = ref(true)

async function resetSize() {
  const data = await invoke<{ settings: { widget_position: string } }>('load_data')
  await invoke('reset_tiktok_size', { widgetPosition: data.settings.widget_position })
  await getCurrentWindow().close()
}

async function toggleButtons() {
  const newVal = await invoke<boolean>('toggle_tiktok_buttons_hidden')
  buttonsHidden.value = newVal
}

onMounted(async () => {
  buttonsHidden.value = await invoke<boolean>('get_tiktok_buttons_hidden')
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
    <button class="menu-item" @click="toggleButtons">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path v-if="buttonsHidden" d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24" />
        <line v-if="buttonsHidden" x1="1" y1="1" x2="23" y2="23" />
        <template v-else>
          <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" />
          <circle cx="12" cy="12" r="3" />
        </template>
      </svg>
      <span>{{ buttonsHidden ? 'Show buttons' : 'Hide buttons' }}</span>
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
