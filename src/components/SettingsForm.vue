<script setup lang="ts">
import { reactive, watch } from 'vue'
import { useAppState } from '../composables/useAppState'

const { state, saveSettings } = useAppState()

const form = reactive({
  images_path: state.settings.images_path,
  timer_minutes: state.settings.timer_minutes,
  confirmation_minutes: state.settings.confirmation_minutes,
  widget_position: state.settings.widget_position,
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
  },
  { immediate: true },
)

async function handleSave() {
  await saveSettings({ ...form })
}
</script>

<template>
  <div class="form-body">
    <label class="field">
      <span class="field-label">Images folder</span>
      <input
        v-model="form.images_path"
        type="text"
        placeholder="/path/to/images"
        class="field-input"
      />
    </label>

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
