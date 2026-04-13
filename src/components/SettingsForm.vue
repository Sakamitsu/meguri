<script setup lang="ts">
import { reactive, ref, watch } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { useAppState } from '../composables/useAppState'
import { refreshHotkeys } from '../composables/useHotkeys'

const { state, saveSettings } = useAppState()

const copied = ref(false)
const saved = ref(false)
let copiedTimeout: ReturnType<typeof setTimeout> | null = null
let savedTimeout: ReturnType<typeof setTimeout> | null = null

async function copyImagePath() {
  await navigator.clipboard.writeText(state.imagePath)
  copied.value = true
  if (copiedTimeout) clearTimeout(copiedTimeout)
  copiedTimeout = setTimeout(() => { copied.value = false }, 2000)
}

const form = reactive({
  images_path: state.settings.images_path,
  timer_minutes: state.settings.timer_minutes,
  confirmation_minutes: state.settings.confirmation_minutes,
  widget_position: state.settings.widget_position,
  tiktok_mode: state.settings.tiktok_mode,
  tiktok_url: state.settings.tiktok_url,
  hotkeys_enabled: state.settings.hotkeys_enabled,
  hotkey_move_top_left: state.settings.hotkey_move_top_left,
  hotkey_move_top_right: state.settings.hotkey_move_top_right,
  hotkey_move_bottom_left: state.settings.hotkey_move_bottom_left,
  hotkey_move_bottom_right: state.settings.hotkey_move_bottom_right,
  hotkey_start_stop: state.settings.hotkey_start_stop,
})

const recording = ref<string | null>(null)

function startRecording(field: string) {
  recording.value = field
}

function handleKeyDown(e: KeyboardEvent, field: string) {
  if (recording.value !== field) return
  e.preventDefault()
  e.stopPropagation()

  if (e.key === 'Escape') {
    recording.value = null
    return
  }

  // Ignore standalone modifier keys
  if (['Control', 'Alt', 'Shift', 'Meta'].includes(e.key)) return

  const parts: string[] = []
  if (e.ctrlKey) parts.push('Ctrl')
  if (e.altKey) parts.push('Alt')
  if (e.shiftKey) parts.push('Shift')
  if (e.metaKey) parts.push('Super')

  let key = e.key
  if (key === ' ') key = 'Space'
  else if (key.length === 1) key = key.toUpperCase()
  else if (key === 'ArrowUp') key = 'Up'
  else if (key === 'ArrowDown') key = 'Down'
  else if (key === 'ArrowLeft') key = 'Left'
  else if (key === 'ArrowRight') key = 'Right'

  parts.push(key)
  ;(form as any)[field] = parts.join('+')
  recording.value = null
}

function clearHotkey(field: string) {
  ;(form as any)[field] = ''
}

const hotkeyFields = [
  { field: 'hotkey_start_stop', label: 'Start / Stop / Confirm', dot: null },
  { field: 'hotkey_move_top_left', label: 'Move to top-left', dot: { cx: 5, cy: 5 } },
  { field: 'hotkey_move_top_right', label: 'Move to top-right', dot: { cx: 11, cy: 5 } },
  { field: 'hotkey_move_bottom_left', label: 'Move to bottom-left', dot: { cx: 5, cy: 11 } },
  { field: 'hotkey_move_bottom_right', label: 'Move to bottom-right', dot: { cx: 11, cy: 11 } },
]

const positionOptions = [
  { value: 'bottom-left', label: 'Bottom left' },
  { value: 'top-left', label: 'Top left' },
  { value: 'top-right', label: 'Top right' },
  { value: 'bottom-right', label: 'Bottom right' },
] as const

watch(
  () => state.settings,
  (s) => {
    form.images_path = s.images_path
    form.timer_minutes = s.timer_minutes
    form.confirmation_minutes = s.confirmation_minutes
    form.widget_position = s.widget_position
    form.tiktok_mode = s.tiktok_mode
    form.tiktok_url = s.tiktok_url
    form.hotkeys_enabled = s.hotkeys_enabled
    form.hotkey_move_top_left = s.hotkey_move_top_left
    form.hotkey_move_top_right = s.hotkey_move_top_right
    form.hotkey_move_bottom_left = s.hotkey_move_bottom_left
    form.hotkey_move_bottom_right = s.hotkey_move_bottom_right
    form.hotkey_start_stop = s.hotkey_start_stop
  },
  { immediate: true },
)

async function browseFolder() {
  const selected = await open({ directory: true, multiple: false })
  if (selected) {
    form.images_path = selected as string
  }
}

async function handleSave() {
  await saveSettings({ ...form })
  await refreshHotkeys()
  saved.value = true
  if (savedTimeout) clearTimeout(savedTimeout)
  savedTimeout = setTimeout(() => { saved.value = false }, 1500)
}
</script>

<template>
  <div class="form-body">
    <div class="field">
      <span class="field-label">Images folder</span>
      <div class="path-row">
        <input
          v-model="form.images_path"
          type="text"
          placeholder="/path/to/images"
          class="field-input path-input"
        />
        <button class="browse-btn" @click="browseFolder">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z" />
          </svg>
        </button>
      </div>
    </div>

    <div v-if="state.imagePath" class="field">
      <span class="field-label">Current image:</span>
      <span class="image-path" @click="copyImagePath">{{ state.imagePath }}</span>
      <Transition name="fade">
        <span v-if="copied" class="copied-msg">Copied</span>
      </Transition>
    </div>

    <label class="field">
      <span class="field-label">Timer (minutes)</span>
      <input
        v-model.number="form.timer_minutes"
        type="number"
        min="1"
        class="field-input"
      />
    </label>

    <label class="field">
      <span class="field-label">Confirmation wait (minutes)</span>
      <input
        v-model.number="form.confirmation_minutes"
        type="number"
        min="1"
        class="field-input"
      />
    </label>

    <label class="field">
      <span class="field-label">Widget position</span>
      <select v-model="form.widget_position" class="field-input">
        <option
          v-for="opt in positionOptions"
          :key="opt.value"
          :value="opt.value"
        >
          {{ opt.label }}
        </option>
      </select>
    </label>

    <label class="field checkbox-field">
      <input
        v-model="form.tiktok_mode"
        type="checkbox"
        class="checkbox-input"
      />
      <span class="field-label">TikTok mode</span>
    </label>

    <label v-if="form.tiktok_mode" class="field">
      <span class="field-label">TikTok URL</span>
      <input
        v-model="form.tiktok_url"
        type="text"
        placeholder="https://www.tiktok.com/foryou"
        class="field-input"
      />
    </label>

    <label class="field checkbox-field">
      <input
        v-model="form.hotkeys_enabled"
        type="checkbox"
        class="checkbox-input"
      />
      <span class="field-label">Global hotkeys</span>
    </label>

    <template v-if="form.hotkeys_enabled">
      <div class="field" v-for="hk in hotkeyFields" :key="hk.field">
        <span class="field-label hotkey-label">
          {{ hk.label }}
          <svg v-if="hk.dot" width="14" height="14" viewBox="0 0 16 16">
            <rect x="1" y="1" width="14" height="14" rx="2" fill="none" stroke="currentColor" stroke-width="1.5"/>
            <circle :cx="hk.dot.cx" :cy="hk.dot.cy" r="2" fill="currentColor"/>
          </svg>
        </span>
        <div class="hotkey-row">
          <button
            class="hotkey-input"
            :class="{ recording: recording === hk.field }"
            @click="startRecording(hk.field)"
            @keydown="handleKeyDown($event, hk.field)"
          >
            {{ recording === hk.field ? 'Press keys...' : ((form as any)[hk.field] || 'Not set') }}
          </button>
          <button
            v-if="(form as any)[hk.field]"
            class="hotkey-clear"
            @click="clearHotkey(hk.field)"
          >
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
              <path d="M18 6 6 18" /><path d="m6 6 12 12" />
            </svg>
          </button>
        </div>
      </div>
    </template>

    <div class="save-row">
      <Transition name="fade">
        <span v-if="saved" class="saved-msg">Saved</span>
      </Transition>
      <button class="save-btn" @click="handleSave">Save</button>
    </div>
  </div>
</template>

<style scoped>
.form-body {
  padding: 8px 12px 12px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 3px;
}

.field-label {
  font-size: 11px;
  color: var(--ctp-subtext0);
}

.field-input {
  background: var(--ctp-surface0);
  border: 1px solid var(--ctp-surface1);
  border-radius: 6px;
  padding: 5px 8px;
  font-size: 12px;
  color: var(--ctp-text);
  outline: none;
  font-family: inherit;
}

.field-input:focus {
  border-color: var(--ctp-mauve);
}

.path-row {
  display: flex;
  gap: 4px;
}

.path-input {
  flex: 1;
  min-width: 0;
}

.browse-btn {
  background: var(--ctp-surface0);
  border: 1px solid var(--ctp-surface1);
  border-radius: 6px;
  padding: 4px 8px;
  color: var(--ctp-subtext0);
  cursor: pointer;
  display: flex;
  align-items: center;
  flex-shrink: 0;
}

.browse-btn:hover {
  background: var(--ctp-surface1);
  color: var(--ctp-text);
}

.image-path {
  font-size: 11px;
  color: var(--ctp-blue);
  cursor: pointer;
  word-break: break-all;
  line-height: 1.3;
}

.image-path:hover {
  color: var(--ctp-sapphire);
  text-decoration: underline;
}

.copied-msg {
  font-size: 10px;
  color: var(--ctp-green);
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.15s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.checkbox-field {
  flex-direction: row;
  align-items: center;
  gap: 8px;
}

.checkbox-input {
  width: 14px;
  height: 14px;
  accent-color: var(--ctp-mauve);
  cursor: pointer;
}

.hotkey-row {
  display: flex;
  gap: 4px;
}

.hotkey-input {
  flex: 1;
  background: var(--ctp-surface0);
  border: 1px solid var(--ctp-surface1);
  border-radius: 6px;
  padding: 5px 8px;
  font-size: 12px;
  color: var(--ctp-text);
  font-family: inherit;
  cursor: pointer;
  text-align: left;
  transition: border-color 0.15s;
}

.hotkey-input:hover {
  border-color: var(--ctp-surface2);
}

.hotkey-input.recording {
  border-color: var(--ctp-mauve);
  color: var(--ctp-mauve);
}

.hotkey-clear {
  background: var(--ctp-surface0);
  border: 1px solid var(--ctp-surface1);
  border-radius: 6px;
  padding: 4px 6px;
  color: var(--ctp-overlay0);
  cursor: pointer;
  display: flex;
  align-items: center;
  flex-shrink: 0;
}

.hotkey-clear:hover {
  background: var(--ctp-surface1);
  color: var(--ctp-red);
}

.hotkey-label {
  display: flex;
  align-items: center;
  gap: 4px;
}

.save-row {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 8px;
}

.saved-msg {
  font-size: 11px;
  color: var(--ctp-green);
  font-weight: 500;
}

.save-btn {
  background: var(--ctp-mauve);
  color: var(--ctp-crust);
  border: none;
  border-radius: 6px;
  padding: 5px 16px;
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
  font-family: inherit;
}

.save-btn:hover {
  opacity: 0.9;
}
</style>
