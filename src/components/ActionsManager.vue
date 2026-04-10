<script setup lang="ts">
import { ref } from 'vue'
import { useAppState } from '../composables/useAppState'

const { state, addAction, updateAction, deleteAction, setActiveAction } = useAppState()

const newName = ref('')
const editingId = ref<string | null>(null)
const editName = ref('')

async function handleAdd() {
  const name = newName.value.trim()
  if (!name) return
  await addAction(name)
  newName.value = ''
}

function startEdit(id: string, name: string) {
  editingId.value = id
  editName.value = name
}

async function finishEdit(id: string) {
  const name = editName.value.trim()
  if (name) {
    await updateAction(id, name)
  }
  editingId.value = null
}

function cancelEdit() {
  editingId.value = null
}
</script>

<template>
  <div class="actions-body">
    <div class="add-row">
      <input
        v-model="newName"
        type="text"
        placeholder="New action..."
        class="add-input"
        @keydown.enter="handleAdd"
      />
      <button class="add-btn" @click="handleAdd">+</button>
    </div>

    <div v-if="state.actions.length === 0" class="empty">No actions yet</div>

    <div
      v-for="action in state.actions"
      :key="action.id"
      class="action-row"
      :class="{ active: action.active }"
    >
      <button
        class="radio-btn"
        :class="{ selected: action.active }"
        @click="setActiveAction(action.id)"
      >
        <span class="radio-dot" />
      </button>

      <template v-if="editingId === action.id">
        <input
          v-model="editName"
          class="edit-input"
          @keydown.enter="finishEdit(action.id)"
          @keydown.escape="cancelEdit"
          @blur="finishEdit(action.id)"
        />
      </template>
      <template v-else>
        <span class="action-name" @dblclick="startEdit(action.id, action.name)">
          {{ action.name }}
        </span>
      </template>

      <div class="action-buttons">
        <button class="small-btn" @click="startEdit(action.id, action.name)">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z"/>
          </svg>
        </button>
        <button class="small-btn delete" @click="deleteAction(action.id)">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M18 6 6 18" /><path d="m6 6 12 12" />
          </svg>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.actions-body {
  padding: 8px 12px 12px;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.add-row {
  display: flex;
  gap: 4px;
}

.add-input {
  flex: 1;
  background: var(--ctp-surface0);
  border: 1px solid var(--ctp-surface1);
  border-radius: 6px;
  padding: 5px 8px;
  font-size: 12px;
  color: var(--ctp-text);
  outline: none;
  font-family: inherit;
}

.add-input:focus {
  border-color: var(--ctp-mauve);
}

.add-btn {
  background: var(--ctp-green);
  color: var(--ctp-crust);
  border: none;
  border-radius: 6px;
  width: 28px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
}

.add-btn:hover {
  opacity: 0.9;
}

.empty {
  font-size: 11px;
  color: var(--ctp-overlay0);
  text-align: center;
  padding: 8px;
}

.action-row {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 6px;
  border-radius: 6px;
  background: var(--ctp-surface0);
}

.action-row.active {
  background: rgba(203, 166, 247, 0.08);
}

.radio-btn {
  width: 16px;
  height: 16px;
  border-radius: 50%;
  border: 2px solid var(--ctp-surface2);
  background: none;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  padding: 0;
}

.radio-btn.selected {
  border-color: var(--ctp-mauve);
}

.radio-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: transparent;
}

.radio-btn.selected .radio-dot {
  background: var(--ctp-mauve);
}

.action-name {
  flex: 1;
  font-size: 12px;
  color: var(--ctp-text);
  cursor: default;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.edit-input {
  flex: 1;
  background: var(--ctp-base);
  border: 1px solid var(--ctp-mauve);
  border-radius: 4px;
  padding: 2px 6px;
  font-size: 12px;
  color: var(--ctp-text);
  outline: none;
  font-family: inherit;
}

.action-buttons {
  display: flex;
  gap: 2px;
  flex-shrink: 0;
}

.small-btn {
  background: none;
  border: none;
  color: var(--ctp-overlay0);
  cursor: pointer;
  padding: 3px;
  border-radius: 4px;
  display: flex;
  align-items: center;
}

.small-btn:hover {
  color: var(--ctp-text);
  background: var(--ctp-surface1);
}

.small-btn.delete:hover {
  color: var(--ctp-red);
}
</style>
