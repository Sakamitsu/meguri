<script setup lang="ts">
import { ref } from 'vue'
import { useAppState } from '../composables/useAppState'
import { useTimer } from '../composables/useTimer'
import ImageDisplay from './ImageDisplay.vue'
import HoverOverlay from './HoverOverlay.vue'
import ConfirmationPulse from './ConfirmationPulse.vue'

const { state, activeAction, logSession, getRandomImage, incrementTiktokViews } = useAppState()
const hovered = ref(false)
const showContextMenu = ref(false)

const { timerState, totalSeconds, remainingSeconds, displayTime, progress, start, stop, confirm } = useTimer(
  async () => {
    const action = activeAction.value
    if (action) {
      await logSession(action.name, state.settings.timer_minutes)
    }
  },
)

function handleStart() {
  start(state.settings.timer_minutes, state.settings.confirmation_minutes)
}

function handleStop() {
  stop()
  getRandomImage()
}

async function handleConfirm() {
  await confirm(state.settings.timer_minutes, state.settings.confirmation_minutes)
  if (state.settings.tiktok_mode) {
    incrementTiktokViews()
  }
  getRandomImage()
}

function onWidgetContextMenu(e: MouseEvent) {
  e.preventDefault()
  showContextMenu.value = true
}

async function forceComplete() {
  showContextMenu.value = false
  const elapsed = totalSeconds.value - remainingSeconds.value
  const elapsedMinutes = Math.floor(elapsed / 60)
  stop()
  const action = activeAction.value
  if (action && elapsedMinutes > 0) {
    await logSession(action.name, elapsedMinutes)
  }
  if (state.settings.tiktok_mode) {
    incrementTiktokViews()
  }
  await getRandomImage()
}

async function refreshImage() {
  showContextMenu.value = false
  await getRandomImage()
}
</script>

<template>
  <div
    class="widget"
    @mouseenter="hovered = true"
    @mouseleave="hovered = false"
    @contextmenu="onWidgetContextMenu"
    data-tauri-drag-region
  >
    <ImageDisplay :src="state.imageDataUrl" />

    <ConfirmationPulse
      v-if="timerState === 'confirming'"
      :display-time="displayTime"
      @confirm="handleConfirm"
    />

    <HoverOverlay
      v-if="hovered && !showContextMenu"
      :timer-state="timerState"
      :display-time="displayTime"
      :progress="progress"
      :active-action-name="activeAction?.name ?? ''"
      :tiktok-mode="state.settings.tiktok_mode"
      :tiktok-views="state.tiktokViews"
      @start="handleStart"
      @stop="handleStop"
    />

    <Transition name="fade">
      <div v-if="showContextMenu" class="menu-backdrop" @click="showContextMenu = false">
        <div class="context-menu" @click.stop>
          <button
            class="menu-item"
            :class="{ disabled: timerState !== 'running' }"
            :disabled="timerState !== 'running'"
            @click="forceComplete"
          >
            Force complete
          </button>
          <button class="menu-item" @click="refreshImage">
            Refresh image
          </button>
        </div>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.widget {
  width: 160px;
  height: 160px;
  border-radius: 16px;
  overflow: hidden;
  position: relative;
  background: var(--ctp-base);
}

.menu-backdrop {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(17, 17, 27, 0.6);
  z-index: 20;
}

.context-menu {
  background: var(--ctp-mantle);
  border: 1px solid var(--ctp-surface1);
  border-radius: 8px;
  padding: 4px;
  display: flex;
  flex-direction: column;
  min-width: 130px;
}

.menu-item {
  background: none;
  border: none;
  color: var(--ctp-subtext1);
  font-size: 12px;
  font-family: inherit;
  padding: 6px 10px;
  border-radius: 5px;
  cursor: pointer;
  text-align: left;
  transition: background 0.1s, color 0.1s;
}

.menu-item:hover:not(:disabled) {
  background: var(--ctp-surface0);
  color: var(--ctp-text);
}

.menu-item.disabled {
  color: var(--ctp-overlay0);
  cursor: default;
  opacity: 0.5;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.15s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
