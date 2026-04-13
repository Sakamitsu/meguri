<script setup lang="ts">
import { onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useAppState } from './composables/useAppState'
import { useTiktokBridge } from './composables/useTiktokBridge'
import WidgetView from './components/WidgetView.vue'
import SettingsView from './components/SettingsView.vue'
import TiktokMenu from './components/TiktokMenu.vue'

const isMenuView = window.location.search.includes('view=tiktok-menu')

const { state, loadData, getRandomImage, applyWidgetPosition, closeTikTok, decrementTiktokViews } = useAppState()
const { prevVideo, nextVideo } = useTiktokBridge()

function openMenu() {
  invoke('open_tiktok_menu', { widgetPosition: state.settings.widget_position })
}

const WIDGET_SIZE = 160

const tiktokAbove = computed(() => state.settings.widget_position.includes('bottom'))
const widgetRight = computed(() => state.settings.widget_position.includes('right'))

const rootStyle = computed(() => {
  if (!state.tiktokOpen) return { width: `${WIDGET_SIZE}px`, height: `${WIDGET_SIZE}px` }
  return {
    width: '100vw',
    height: '100vh',
    position: 'relative' as const,
  }
})

const widgetStyle = computed(() => {
  if (!state.tiktokOpen) return {}
  return {
    position: 'absolute' as const,
    [tiktokAbove.value ? 'bottom' : 'top']: '0',
    [widgetRight.value ? 'right' : 'left']: '0',
  }
})

const titlebarStyle = computed(() => ({
  position: 'absolute' as const,
  left: '0',
  top: tiktokAbove.value ? '0' : `${WIDGET_SIZE}px`,
  width: '100%',
  height: '28px',
}))

onMounted(async () => {
  if (isMenuView) return
  await loadData()
  await getRandomImage()
  await applyWidgetPosition()
})
</script>

<template>
  <TiktokMenu v-if="isMenuView" />

  <template v-else>
    <div v-if="state.currentView === 'widget'" :style="rootStyle">
      <!-- TikTok titlebar -->
      <div v-if="state.tiktokOpen" class="tiktok-titlebar" :style="titlebarStyle">
        <span class="tiktok-title">TikTok</span>
        <button class="tiktok-burger" @click="openMenu">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
            <line x1="4" y1="6" x2="20" y2="6" />
            <line x1="4" y1="12" x2="20" y2="12" />
            <line x1="4" y1="18" x2="20" y2="18" />
          </svg>
        </button>
        <div style="flex: 1" />
        <button class="tiktok-views-btn" title="Use a view" @click="decrementTiktokViews">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="10" />
            <path d="M8 12h8" />
          </svg>
          <span class="views-count">{{ state.tiktokViews }}</span>
        </button>
        <button class="tiktok-nav-btn" title="Previous video" @click="prevVideo">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="m18 15-6-6-6 6" />
          </svg>
        </button>
        <button class="tiktok-nav-btn" title="Next video" @click="nextVideo">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="m6 9 6 6 6-6" />
          </svg>
        </button>
        <button class="tiktok-close" @click="closeTikTok">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M18 6 6 18" /><path d="m6 6 12 12" />
          </svg>
        </button>
      </div>

      <!-- Widget -->
      <div :style="widgetStyle">
        <WidgetView />
      </div>
    </div>

    <SettingsView v-else />
  </template>
</template>

<style scoped>
.tiktok-titlebar {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 0 8px;
  background: var(--ctp-mantle);
  border-radius: 12px 12px 0 0;
  border-bottom: 1px solid var(--ctp-surface0);
  z-index: 20;
}

.tiktok-title {
  font-size: 11px;
  font-weight: 600;
  color: var(--ctp-subtext0);
}

.tiktok-burger {
  background: none;
  border: none;
  color: var(--ctp-overlay0);
  cursor: pointer;
  padding: 4px;
  border-radius: 6px;
  display: flex;
  align-items: center;
}

.tiktok-burger:hover {
  color: var(--ctp-text);
  background: var(--ctp-surface0);
}

.tiktok-views-btn {
  background: none;
  border: none;
  color: var(--ctp-overlay0);
  cursor: pointer;
  padding: 4px 6px;
  border-radius: 6px;
  display: flex;
  align-items: center;
  gap: 3px;
  transition: color 0.1s, background 0.1s;
  margin-right: 2px;
}

.tiktok-views-btn:hover {
  color: var(--ctp-peach);
  background: rgba(250, 179, 135, 0.1);
}

.views-count {
  font-size: 10px;
  font-weight: 600;
  font-variant-numeric: tabular-nums;
}

.tiktok-nav-btn {
  background: none;
  border: none;
  color: var(--ctp-overlay0);
  cursor: pointer;
  padding: 4px;
  border-radius: 6px;
  display: flex;
  align-items: center;
  transition: color 0.1s, background 0.1s;
}

.tiktok-nav-btn:hover {
  color: var(--ctp-text);
  background: var(--ctp-surface0);
}

.tiktok-close {
  background: none;
  border: none;
  color: var(--ctp-overlay0);
  cursor: pointer;
  padding: 4px;
  border-radius: 6px;
  display: flex;
  align-items: center;
}

.tiktok-close:hover {
  color: var(--ctp-red);
  background: rgba(243, 139, 168, 0.1);
}
</style>
