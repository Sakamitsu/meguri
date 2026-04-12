<script setup lang="ts">
import { onMounted, computed } from 'vue'
import { useAppState } from './composables/useAppState'
import WidgetView from './components/WidgetView.vue'
import SettingsView from './components/SettingsView.vue'

const { state, loadData, getRandomImage, applyWidgetPosition, closeTikTok } = useAppState()

const TIKTOK_WIDTH = 244
const TIKTOK_HEIGHT = 270
const WIDGET_SIZE = 160
const TITLEBAR_HEIGHT = 28

const tiktokAbove = computed(() => state.settings.widget_position.includes('bottom'))
const widgetRight = computed(() => state.settings.widget_position.includes('right'))

const rootStyle = computed(() => {
  if (!state.tiktokOpen) return { width: `${WIDGET_SIZE}px`, height: `${WIDGET_SIZE}px` }
  return {
    width: `${TIKTOK_WIDTH}px`,
    height: `${WIDGET_SIZE + TIKTOK_HEIGHT}px`,
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
  width: `${TIKTOK_WIDTH}px`,
  height: `${TITLEBAR_HEIGHT}px`,
}))

onMounted(async () => {
  await loadData()
  await getRandomImage()
  await applyWidgetPosition()
})
</script>

<template>
  <div v-if="state.currentView === 'widget'" :style="rootStyle">
    <!-- TikTok titlebar -->
    <div v-if="state.tiktokOpen" class="tiktok-titlebar" :style="titlebarStyle">
      <span class="tiktok-title">TikTok</span>
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

<style scoped>
.tiktok-titlebar {
  display: flex;
  align-items: center;
  justify-content: space-between;
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
