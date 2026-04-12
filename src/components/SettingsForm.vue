<script setup lang="ts">
import { reactive, watch } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { useAppState } from '../composables/useAppState'

const { state, saveSettings } = useAppState()

const form = reactive({
  images_path: state.settings.images_path,
  timer_minutes: state.settings.timer_minutes,
  confirmation_minutes: state.settings.confirmation_minutes,
  widget_position: state.settings.widget_position,
  tiktok_mode: state.settings.tiktok_mode,
})

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

    <button class="save-btn" @click="handleSave">Save</button>
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

.save-btn {
  align-self: flex-end;
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
