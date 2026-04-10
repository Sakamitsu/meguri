<script setup lang="ts">
import { ref } from 'vue'
import { useAppState } from '../composables/useAppState'
import { useTimer } from '../composables/useTimer'
import ImageDisplay from './ImageDisplay.vue'
import HoverOverlay from './HoverOverlay.vue'
import ConfirmationPulse from './ConfirmationPulse.vue'

const { state, activeAction, logSession, getRandomImage } = useAppState()
const hovered = ref(false)

const { timerState, displayTime, progress, start, stop, confirm } = useTimer(
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
  getRandomImage()
}
</script>

<template>
  <div
    class="widget"
    @mouseenter="hovered = true"
    @mouseleave="hovered = false"
    data-tauri-drag-region
  >
    <ImageDisplay :src="state.imageDataUrl" />

    <ConfirmationPulse
      v-if="timerState === 'confirming'"
      :display-time="displayTime"
      @confirm="handleConfirm"
    />

    <HoverOverlay
      v-if="hovered"
      :timer-state="timerState"
      :display-time="displayTime"
      :progress="progress"
      :active-action-name="activeAction?.name ?? ''"
      @start="handleStart"
      @stop="handleStop"
    />
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
</style>
