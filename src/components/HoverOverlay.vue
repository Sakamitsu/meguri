<script setup lang="ts">
import { ref } from 'vue'
import type { TimerState } from '../composables/useTimer'
import { useAppState } from '../composables/useAppState'
import { getCurrentWindow } from '@tauri-apps/api/window'

defineProps<{
  timerState: TimerState
  displayTime: string
  progress: number
  activeActionName: string
}>()

const emit = defineEmits<{
  start: []
  stop: []
}>()

const { state, saveSettings, applyWidgetPosition, goToSettings } = useAppState()

const cornerTopLeft = ref(false)
const cornerTopRight = ref(false)
const showMenu = ref(false)

const positionOptions = [
  { value: 'bottom-left', label: 'Bottom left' },
  { value: 'top-left', label: 'Top left' },
  { value: 'top-right', label: 'Top right' },
  { value: 'bottom-right', label: 'Bottom right' },
] as const

function openPositionMenu(e: MouseEvent) {
  e.preventDefault()
  showMenu.value = true
}

async function pickPosition(value: typeof state.settings.widget_position) {
  showMenu.value = false
  state.settings.widget_position = value
  await saveSettings({ ...state.settings })
  await applyWidgetPosition()
}

function startDrag() {
  getCurrentWindow().startDragging()
}
</script>

<template>
  <div class="hover-overlay">
    <!-- Top-left: action name -->
    <div
      class="corner top-left"
      @mouseenter="cornerTopLeft = true"
      @mouseleave="cornerTopLeft = false"
    >
      <Transition name="fade">
        <span v-if="cornerTopLeft" class="action-name">
          {{ activeActionName || 'No action' }}
        </span>
      </Transition>
    </div>

    <!-- Top-right: drag + settings -->
    <div
      class="corner top-right"
      @mouseenter="cornerTopRight = true"
      @mouseleave="cornerTopRight = false"
    >
      <Transition name="fade">
        <div v-if="cornerTopRight" class="top-right-buttons">
          <button class="icon-btn drag-btn" @mousedown="startDrag" @contextmenu="openPositionMenu">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <circle cx="9" cy="5" r="1.5" /><circle cx="15" cy="5" r="1.5" />
              <circle cx="9" cy="12" r="1.5" /><circle cx="15" cy="12" r="1.5" />
              <circle cx="9" cy="19" r="1.5" /><circle cx="15" cy="19" r="1.5" />
            </svg>
          </button>
          <button class="icon-btn settings-btn" @click="goToSettings">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/>
              <circle cx="12" cy="12" r="3"/>
            </svg>
          </button>
        </div>
      </Transition>
    </div>

    <!-- Center: start/stop button -->
    <div class="center">
      <span v-if="timerState === 'running'" class="timer-display">{{ displayTime }}</span>

      <button
        v-if="timerState === 'idle'"
        class="icon-btn start-btn"
        @click="emit('start')"
      >
        <svg width="36" height="36" viewBox="0 0 24 24" fill="currentColor">
          <polygon points="6,3 20,12 6,21" />
        </svg>
      </button>

      <button
        v-else-if="timerState === 'running'"
        class="icon-btn stop-btn"
        @click="emit('stop')"
      >
        <svg width="36" height="36" viewBox="0 0 24 24" fill="currentColor">
          <rect x="6" y="4" width="4" height="16" rx="1" />
          <rect x="14" y="4" width="4" height="16" rx="1" />
        </svg>
      </button>
    </div>

    <!-- Position context menu -->
    <Transition name="fade">
      <div v-if="showMenu" class="menu-backdrop" @click="showMenu = false">
        <div class="position-menu" @click.stop>
          <button
            v-for="opt in positionOptions"
            :key="opt.value"
            class="menu-item"
            :class="{ active: state.settings.widget_position === opt.value }"
            @click="pickPosition(opt.value)"
          >
            {{ opt.label }}
          </button>
        </div>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.hover-overlay {
  position: absolute;
  inset: 0;
  background: rgba(17, 17, 27, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10;
}

.corner {
  position: absolute;
  width: 50px;
  height: 40px;
  display: flex;
  align-items: flex-start;
  justify-content: center;
  padding-top: 6px;
}

.top-left {
  top: 0;
  left: 0;
  width: calc(100% - 70px);
  justify-content: flex-start;
  padding-left: 8px;
}

.top-right {
  top: 0;
  right: 0;
  width: 70px;
  justify-content: flex-end;
  padding-right: 6px;
}

.top-right-buttons {
  display: flex;
  align-items: center;
  gap: 2px;
}

.drag-btn {
  color: var(--ctp-overlay1);
  cursor: default !important;
}

.drag-btn:hover {
  color: var(--ctp-lavender);
  background: rgba(180, 190, 254, 0.1);
}

.action-name {
  font-size: 13px;
  color: var(--ctp-subtext0);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 100%;
}

.center {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

.timer-display {
  position: absolute;
  bottom: 100%;
  left: 50%;
  transform: translateX(-50%);
  margin-bottom: 6px;
  white-space: nowrap;
  font-size: 12px;
  color: var(--ctp-subtext0);
  font-variant-numeric: tabular-nums;
}


.icon-btn {
  background: none;
  border: none;
  cursor: pointer;
  color: var(--ctp-text);
  padding: 4px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: color 0.15s, background 0.15s;
}

.icon-btn:hover {
  color: var(--ctp-mauve);
  background: rgba(203, 166, 247, 0.1);
}

.start-btn {
  color: var(--ctp-green);
}

.start-btn:hover {
  color: var(--ctp-green);
  background: rgba(166, 227, 161, 0.15);
}

.stop-btn {
  color: var(--ctp-red);
}

.stop-btn:hover {
  color: var(--ctp-red);
  background: rgba(243, 139, 168, 0.15);
}

.settings-btn {
  color: var(--ctp-overlay1);
}

.settings-btn:hover {
  color: var(--ctp-lavender);
  background: rgba(180, 190, 254, 0.1);
}

.menu-backdrop {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(17, 17, 27, 0.4);
  z-index: 20;
}

.position-menu {
  background: var(--ctp-mantle);
  border: 1px solid var(--ctp-surface1);
  border-radius: 8px;
  padding: 4px;
  display: flex;
  flex-direction: column;
  min-width: 120px;
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

.menu-item:hover {
  background: var(--ctp-surface0);
  color: var(--ctp-text);
}

.menu-item.active {
  color: var(--ctp-mauve);
  font-weight: 600;
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
