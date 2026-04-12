<script setup lang="ts">
import { onMounted } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useAppState } from '../composables/useAppState'
import SettingsForm from './SettingsForm.vue'
import ActionsManager from './ActionsManager.vue'
import StatsDisplay from './StatsDisplay.vue'

const { goToWidget, loadStats, applyWidgetPosition, clampToScreen } = useAppState()

onMounted(async () => {
  await getCurrentWindow().setSize(new (await import('@tauri-apps/api/dpi')).LogicalSize(350, 450))
  await clampToScreen(350, 450)
  await loadStats()
})

async function handleBack() {
  await getCurrentWindow().setSize(new (await import('@tauri-apps/api/dpi')).LogicalSize(160, 160))
  await applyWidgetPosition()
  goToWidget()
}
</script>

<template>
  <div class="settings-view">
    <header class="settings-header" data-tauri-drag-region>
      <button class="back-btn" @click="handleBack">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="m15 18-6-6 6-6" />
        </svg>
      </button>
      <span class="settings-title">Settings</span>
      <button class="close-btn" @click="getCurrentWindow().close()">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="M18 6 6 18" /><path d="m6 6 12 12" />
        </svg>
      </button>
    </header>

    <div class="settings-content">
      <details open>
        <summary>General</summary>
        <SettingsForm />
      </details>

      <details>
        <summary>Actions</summary>
        <ActionsManager />
      </details>

      <details>
        <summary>Statistics</summary>
        <StatsDisplay />
      </details>
    </div>
  </div>
</template>

<style scoped>
.settings-view {
  width: 100%;
  height: 100vh;
  background: var(--ctp-base);
  display: flex;
  flex-direction: column;
  border-radius: 12px;
  overflow: hidden;
}

.settings-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 12px;
  background: var(--ctp-mantle);
  border-bottom: 1px solid var(--ctp-surface0);
}

.close-btn {
  margin-left: auto;
  background: none;
  border: none;
  color: var(--ctp-overlay0);
  cursor: pointer;
  padding: 4px;
  border-radius: 6px;
  display: flex;
  align-items: center;
}

.close-btn:hover {
  color: var(--ctp-red);
  background: rgba(243, 139, 168, 0.1);
}

.back-btn {
  background: none;
  border: none;
  color: var(--ctp-subtext0);
  cursor: pointer;
  padding: 4px;
  border-radius: 6px;
  display: flex;
  align-items: center;
}

.back-btn:hover {
  color: var(--ctp-text);
  background: var(--ctp-surface0);
}

.settings-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--ctp-text);
}

.settings-content {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  padding: 8px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

details {
  background: var(--ctp-mantle);
  border-radius: 8px;
  flex-shrink: 0;
}

summary {
  padding: 8px 12px;
  cursor: pointer;
  font-size: 12px;
  font-weight: 600;
  color: var(--ctp-subtext1);
  list-style: none;
  display: flex;
  align-items: center;
  gap: 6px;
}

summary:hover {
  background: var(--ctp-surface0);
}

summary::before {
  content: '▸';
  font-size: 10px;
  transition: transform 0.15s;
}

details[open] summary::before {
  transform: rotate(90deg);
}
</style>
