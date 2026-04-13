<script setup lang="ts">
import { ref, computed } from 'vue'
import { useAppState } from '../composables/useAppState'

const { state } = useAppState()

const now = new Date()
const selectedMonth = ref(`${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}`)
const dropdownOpen = ref(false)

const availableMonths = computed(() => {
  const months: { value: string; label: string }[] = []
  const earliest = state.stats.reduce((min, e) => {
    const d = new Date(e.timestamp)
    return d < min ? d : min
  }, new Date())

  const start = new Date(earliest.getFullYear(), earliest.getMonth(), 1)
  const end = new Date(now.getFullYear(), now.getMonth(), 1)

  if (state.stats.length === 0) {
    return [{ value: selectedMonth.value, label: formatMonthLabel(now.getFullYear(), now.getMonth()) }]
  }

  const cursor = new Date(end)
  while (cursor >= start) {
    const y = cursor.getFullYear()
    const m = cursor.getMonth()
    months.push({
      value: `${y}-${String(m + 1).padStart(2, '0')}`,
      label: formatMonthLabel(y, m),
    })
    cursor.setMonth(cursor.getMonth() - 1)
  }
  return months
})

const selectedLabel = computed(() => {
  const found = availableMonths.value.find(m => m.value === selectedMonth.value)
  return found ? found.label : ''
})

function selectMonth(value: string) {
  selectedMonth.value = value
  dropdownOpen.value = false
}

function formatMonthLabel(year: number, month: number): string {
  const names = ['January', 'February', 'March', 'April', 'May', 'June',
    'July', 'August', 'September', 'October', 'November', 'December']
  return `${names[month]} ${year}`
}

/** Map date string (YYYY-MM-DD) -> total minutes for selected month */
const dailyMap = computed(() => {
  const [y, m] = selectedMonth.value.split('-').map(Number)
  const map = new Map<string, number>()

  for (const entry of state.stats) {
    const d = new Date(entry.timestamp)
    if (d.getFullYear() !== y || d.getMonth() + 1 !== m) continue
    const key = `${y}-${String(m).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`
    map.set(key, (map.get(key) || 0) + entry.duration_minutes)
  }
  return map
})

/** Max minutes in a single day for the selected month (for color scaling) */
const maxMinutes = computed(() => {
  let max = 0
  for (const v of dailyMap.value.values()) {
    if (v > max) max = v
  }
  return max
})

interface GridCell {
  date: string      // YYYY-MM-DD
  day: number        // day of month (0 = empty placeholder)
  minutes: number
}

const weekdayHeaders = ['Mo', 'Tu', 'We', 'Th', 'Fr', 'Sa', 'Su']

/** Build grid: rows = weeks, 7 columns (Mon..Sun) — traditional calendar layout */
const grid = computed(() => {
  const [y, m] = selectedMonth.value.split('-').map(Number)
  const firstDay = new Date(y, m - 1, 1)
  const daysInMonth = new Date(y, m, 0).getDate()

  // Monday = 0, Sunday = 6
  let startDow = firstDay.getDay() - 1
  if (startDow < 0) startDow = 6

  const cells: GridCell[] = []

  // leading empty cells
  for (let i = 0; i < startDow; i++) {
    cells.push({ date: '', day: 0, minutes: 0 })
  }

  for (let d = 1; d <= daysInMonth; d++) {
    const key = `${y}-${String(m).padStart(2, '0')}-${String(d).padStart(2, '0')}`
    cells.push({ date: key, day: d, minutes: dailyMap.value.get(key) || 0 })
  }

  // trailing empty cells to fill last week
  while (cells.length % 7 !== 0) {
    cells.push({ date: '', day: 0, minutes: 0 })
  }

  // Split into rows of 7 (each row = one week)
  const rows: GridCell[][] = []
  for (let i = 0; i < cells.length; i += 7) {
    rows.push(cells.slice(i, i + 7))
  }
  return rows
})

function cellLevel(minutes: number): number {
  if (minutes === 0 || maxMinutes.value === 0) return 0
  const ratio = minutes / maxMinutes.value
  if (ratio <= 0.25) return 1
  if (ratio <= 0.5) return 2
  if (ratio <= 0.75) return 3
  return 4
}

function cellTooltip(cell: GridCell): string {
  if (!cell.day) return ''
  const [, m, d] = cell.date.split('-').map(Number)
  const monthNames = ['January', 'February', 'March', 'April', 'May', 'June',
    'July', 'August', 'September', 'October', 'November', 'December']
  const label = `${monthNames[m - 1]} ${d}`
  if (cell.minutes === 0) return `${label} — No activity`
  return `${label} — ${formatTime(cell.minutes)}`
}

interface DayEntry {
  date: string
  totalMinutes: number
  totalSessions: number
  actions: { name: string; sessions: number; minutes: number }[]
}

/** Daily list with per-action breakdown, sorted most recent first */
const dailyList = computed(() => {
  const [y, m] = selectedMonth.value.split('-').map(Number)
  const dayMap = new Map<string, Map<string, { sessions: number; minutes: number }>>()

  for (const entry of state.stats) {
    const d = new Date(entry.timestamp)
    if (d.getFullYear() !== y || d.getMonth() + 1 !== m) continue
    const key = `${y}-${String(m).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`
    if (!dayMap.has(key)) dayMap.set(key, new Map())
    const actions = dayMap.get(key)!
    const existing = actions.get(entry.action_name)
    if (existing) {
      existing.sessions++
      existing.minutes += entry.duration_minutes
    } else {
      actions.set(entry.action_name, { sessions: 1, minutes: entry.duration_minutes })
    }
  }

  const result: DayEntry[] = []
  for (const [date, actions] of dayMap) {
    let totalMinutes = 0
    let totalSessions = 0
    const actionList: DayEntry['actions'] = []
    for (const [name, data] of actions) {
      actionList.push({ name, ...data })
      totalMinutes += data.minutes
      totalSessions += data.sessions
    }
    actionList.sort((a, b) => b.minutes - a.minutes)
    result.push({ date, totalMinutes, totalSessions, actions: actionList })
  }
  return result.sort((a, b) => b.date.localeCompare(a.date))
})

const allTimeTotal = computed(() => {
  return {
    sessions: state.stats.length,
    minutes: state.stats.reduce((sum, e) => sum + e.duration_minutes, 0),
  }
})

const monthTotal = computed(() => {
  let minutes = 0
  let sessions = 0
  for (const day of dailyList.value) {
    minutes += day.totalMinutes
    sessions += day.totalSessions
  }
  return { minutes, sessions }
})

function formatTime(minutes: number): string {
  const h = Math.floor(minutes / 60)
  const m = minutes % 60
  if (h > 0) return `${h}h ${m}m`
  return `${m}m`
}

function formatDateLabel(dateStr: string): string {
  const [, m, d] = dateStr.split('-').map(Number)
  const monthNames = ['January', 'February', 'March', 'April', 'May', 'June',
    'July', 'August', 'September', 'October', 'November', 'December']
  return `${monthNames[m - 1]} ${d}`
}
</script>

<template>
  <div class="stats-body" @click.self="dropdownOpen = false">
    <div class="all-time">
      <span class="all-time-label">All time</span>
      <span class="all-time-value">
        {{ allTimeTotal.sessions }} {{ allTimeTotal.sessions === 1 ? 'session' : 'sessions' }}
        &middot; {{ formatTime(allTimeTotal.minutes) }}
      </span>
    </div>

    <!-- Custom select -->
    <div class="custom-select">
      <div v-if="dropdownOpen" class="select-backdrop" @click="dropdownOpen = false" />
      <button class="select-trigger" @click="dropdownOpen = !dropdownOpen" :class="{ open: dropdownOpen }">
        <span>{{ selectedLabel }}</span>
        <svg class="chevron" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="m6 9 6 6 6-6" />
        </svg>
      </button>
      <Transition name="dropdown">
        <div v-if="dropdownOpen" class="select-dropdown">
          <button
            v-for="m in availableMonths"
            :key="m.value"
            class="select-option"
            :class="{ active: m.value === selectedMonth }"
            @click="selectMonth(m.value)"
          >
            {{ m.label }}
          </button>
        </div>
      </Transition>
    </div>

    <div class="grid-section">
      <div class="weekday-headers">
        <span v-for="d in weekdayHeaders" :key="d" class="weekday-header">{{ d }}</span>
      </div>
      <div class="grid">
        <div v-for="(week, wi) in grid" :key="wi" class="grid-row">
          <div
            v-for="(cell, ci) in week"
            :key="ci"
            class="grid-cell"
            :class="[cell.day ? `level-${cellLevel(cell.minutes)}` : 'empty']"
            :data-tooltip="cellTooltip(cell)"
          />
        </div>
      </div>
      <div class="grid-footer">
        <span class="month-total">
          Total: {{ monthTotal.sessions }} {{ monthTotal.sessions === 1 ? 'session' : 'sessions' }}
          &middot; {{ formatTime(monthTotal.minutes) }}
        </span>
        <div class="grid-legend">
          <span class="legend-label">Less</span>
          <div class="grid-cell level-0" />
          <div class="grid-cell level-1" />
          <div class="grid-cell level-2" />
          <div class="grid-cell level-3" />
          <div class="grid-cell level-4" />
          <span class="legend-label">More</span>
        </div>
      </div>
    </div>

    <div class="daily-list">
      <div v-if="dailyList.length === 0" class="empty">No activity this month</div>
      <div v-for="(entry, i) in dailyList" :key="entry.date" class="daily-entry" :class="{ first: i === 0 }">
        <div class="daily-header">
          <span class="daily-date">{{ formatDateLabel(entry.date) }}</span>
          <span class="daily-total">
            {{ entry.totalSessions }} {{ entry.totalSessions === 1 ? 'session' : 'sessions' }}
            &middot; {{ formatTime(entry.totalMinutes) }}
          </span>
        </div>
        <div class="daily-actions">
          <div v-for="action in entry.actions" :key="action.name" class="daily-action">
            <span class="action-dot" />
            <span class="action-name">{{ action.name }}</span>
            <span class="action-detail">
              {{ action.sessions }} {{ action.sessions === 1 ? 'session' : 'sessions' }}
              &middot; {{ formatTime(action.minutes) }}
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.stats-body {
  padding: 8px 12px 12px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

/* ── All Time ── */
.all-time {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  padding: 6px 10px;
  background: var(--ctp-surface0);
  border-radius: 6px;
}

.all-time-label {
  font-size: 12px;
  font-weight: 600;
  color: var(--ctp-text);
}

.all-time-value {
  font-size: 12px;
  color: var(--ctp-subtext0);
}

/* ── Custom Select ── */
.custom-select {
  position: relative;
}

.select-backdrop {
  position: fixed;
  inset: 0;
  z-index: 99;
}

.select-trigger {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 10px;
  font-size: 12px;
  font-weight: 600;
  color: var(--ctp-text);
  background: var(--ctp-surface0);
  border: 1px solid var(--ctp-surface1);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.15s;
}

.select-trigger:hover {
  background: var(--ctp-surface1);
  border-color: var(--ctp-surface2);
}

.select-trigger.open {
  border-color: var(--ctp-green);
  background: var(--ctp-surface1);
}

.chevron {
  transition: transform 0.2s;
  color: var(--ctp-overlay0);
  flex-shrink: 0;
}

.select-trigger.open .chevron {
  transform: rotate(180deg);
  color: var(--ctp-green);
}

.select-dropdown {
  position: absolute;
  top: calc(100% + 4px);
  left: 0;
  right: 0;
  background: var(--ctp-surface0);
  border: 1px solid var(--ctp-surface1);
  border-radius: 6px;
  padding: 3px;
  z-index: 100;
  max-height: 160px;
  overflow-y: auto;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

.select-option {
  width: 100%;
  padding: 5px 8px;
  font-size: 12px;
  color: var(--ctp-subtext1);
  background: none;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  text-align: left;
  transition: all 0.1s;
}

.select-option:hover {
  background: var(--ctp-surface1);
  color: var(--ctp-text);
}

.select-option.active {
  color: var(--ctp-green);
  font-weight: 600;
}

/* Dropdown transition */
.dropdown-enter-active,
.dropdown-leave-active {
  transition: all 0.15s ease;
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}

/* ── Grid ── */
.grid-section {
  background: var(--ctp-mantle);
  border-radius: 8px;
  padding: 10px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
}

.weekday-headers {
  display: grid;
  grid-template-columns: repeat(7, 18px);
  gap: 3px;
}

.weekday-header {
  font-size: 9px;
  color: var(--ctp-overlay0);
  text-align: center;
  padding-bottom: 2px;
}

.grid {
  display: flex;
  flex-direction: column;
  gap: 3px;
}

.grid-row {
  display: grid;
  grid-template-columns: repeat(7, 18px);
  gap: 3px;
}

.grid-cell {
  width: 18px;
  height: 18px;
  border-radius: 3px;
  position: relative;
  transition: transform 0.1s, outline-color 0.1s;
}

.grid-cell.empty {
  background: transparent;
}

.grid-cell.level-0 {
  background: var(--ctp-surface0);
}

.grid-cell.level-1 {
  background: rgba(166, 227, 161, 0.25);
}

.grid-cell.level-2 {
  background: rgba(166, 227, 161, 0.5);
}

.grid-cell.level-3 {
  background: rgba(166, 227, 161, 0.75);
}

.grid-cell.level-4 {
  background: var(--ctp-green);
}

/* Cell hover effect */
.grid-cell:not(.empty):hover {
  transform: scale(1.4);
  outline: 2px solid var(--ctp-overlay0);
  outline-offset: 0px;
  z-index: 10;
}

/* CSS tooltip */
.grid-cell[data-tooltip]:not(.empty)::after {
  content: attr(data-tooltip);
  position: absolute;
  bottom: calc(100% + 6px);
  left: 50%;
  transform: translateX(-50%) translateY(4px);
  background: var(--ctp-surface1);
  color: var(--ctp-text);
  font-size: 10px;
  font-weight: 500;
  white-space: nowrap;
  padding: 4px 8px;
  border-radius: 4px;
  pointer-events: none;
  opacity: 0;
  transition: opacity 0.15s, transform 0.15s;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  z-index: 20;
}

.grid-cell[data-tooltip]:not(.empty)::before {
  content: '';
  position: absolute;
  bottom: calc(100% + 2px);
  left: 50%;
  transform: translateX(-50%) translateY(4px);
  border: 4px solid transparent;
  border-top-color: var(--ctp-surface1);
  pointer-events: none;
  opacity: 0;
  transition: opacity 0.15s, transform 0.15s;
  z-index: 20;
}

.grid-cell[data-tooltip]:not(.empty):hover::after,
.grid-cell[data-tooltip]:not(.empty):hover::before {
  opacity: 1;
  transform: translateX(-50%) translateY(0);
}

.grid-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  align-self: stretch;
}

.month-total {
  font-size: 10px;
  color: var(--ctp-overlay0);
}

.grid-legend {
  display: flex;
  align-items: center;
  gap: 3px;
}

.legend-label {
  font-size: 9px;
  color: var(--ctp-overlay0);
}

.grid-legend .grid-cell {
  width: 10px;
  height: 10px;
  aspect-ratio: unset;
}

.grid-legend .grid-cell:hover {
  transform: none;
  outline: none;
}

.daily-list {
  display: flex;
  flex-direction: column;
}

.empty {
  font-size: 11px;
  color: var(--ctp-overlay0);
  text-align: center;
  padding: 8px;
}

.daily-entry {
  padding: 10px 0;
  border-top: 1px solid var(--ctp-surface0);
}

.daily-entry.first {
  border-top: none;
}

.daily-header {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  margin-bottom: 6px;
}

.daily-date {
  font-size: 14px;
  color: var(--ctp-text);
  font-weight: 600;
}

.daily-total {
  font-size: 12px;
  color: var(--ctp-overlay1);
}

.daily-actions {
  display: flex;
  flex-direction: column;
  gap: 3px;
  padding-left: 2px;
}

.daily-action {
  display: flex;
  align-items: center;
  gap: 6px;
}

.action-dot {
  width: 5px;
  height: 5px;
  border-radius: 50%;
  background: var(--ctp-green);
  flex-shrink: 0;
}

.action-name {
  font-size: 13px;
  color: var(--ctp-subtext1);
}

.action-detail {
  font-size: 12px;
  color: var(--ctp-overlay0);
  margin-left: auto;
}
</style>
