<script setup lang="ts">
import { computed } from 'vue'
import { useAppState } from '../composables/useAppState'

const { state } = useAppState()

interface ActionStats {
  name: string
  sessions: number
  totalMinutes: number
}

const grouped = computed<ActionStats[]>(() => {
  const map = new Map<string, ActionStats>()
  for (const entry of state.stats) {
    const existing = map.get(entry.action_name)
    if (existing) {
      existing.sessions++
      existing.totalMinutes += entry.duration_minutes
    } else {
      map.set(entry.action_name, {
        name: entry.action_name,
        sessions: 1,
        totalMinutes: entry.duration_minutes,
      })
    }
  }
  return Array.from(map.values()).sort((a, b) => b.totalMinutes - a.totalMinutes)
})

function formatTime(minutes: number): string {
  const h = Math.floor(minutes / 60)
  const m = minutes % 60
  if (h > 0) return `${h}h ${m}m`
  return `${m}m`
}
</script>

<template>
  <div class="stats-body">
    <div v-if="grouped.length === 0" class="empty">No data yet</div>

    <div v-for="item in grouped" :key="item.name" class="stat-row">
      <span class="stat-name">{{ item.name }}</span>
      <span class="stat-detail">
        {{ item.sessions }} {{ item.sessions === 1 ? 'session' : 'sessions' }}
        &middot; {{ formatTime(item.totalMinutes) }}
      </span>
    </div>

    <div v-if="grouped.length > 0" class="stat-total">
      Total: {{ state.stats.length }} sessions &middot;
      {{ formatTime(state.stats.reduce((s, e) => s + e.duration_minutes, 0)) }}
    </div>
  </div>
</template>

<style scoped>
.stats-body {
  padding: 8px 12px 12px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.empty {
  font-size: 11px;
  color: var(--ctp-overlay0);
  text-align: center;
  padding: 8px;
}

.stat-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 5px 8px;
  background: var(--ctp-surface0);
  border-radius: 6px;
}

.stat-name {
  font-size: 12px;
  color: var(--ctp-text);
  font-weight: 500;
}

.stat-detail {
  font-size: 11px;
  color: var(--ctp-subtext0);
}

.stat-total {
  font-size: 11px;
  color: var(--ctp-overlay1);
  text-align: right;
  padding-top: 4px;
  border-top: 1px solid var(--ctp-surface0);
  margin-top: 2px;
}
</style>
