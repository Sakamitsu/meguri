import { reactive, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow, currentMonitor } from '@tauri-apps/api/window'
import { LogicalPosition } from '@tauri-apps/api/dpi'

interface Settings {
  images_path: string
  timer_minutes: number
  confirmation_minutes: number
  widget_position: 'bottom-left' | 'top-left' | 'top-right' | 'bottom-right'
}

interface Action {
  id: string
  name: string
  active: boolean
}

interface StatEntry {
  action_name: string
  duration_minutes: number
  timestamp: string
}

interface AppData {
  settings: Settings
  actions: Action[]
  stats: StatEntry[]
}

const state = reactive({
  settings: {
    images_path: '',
    timer_minutes: 10,
    confirmation_minutes: 1,
    widget_position: 'bottom-left',
  } as Settings,
  actions: [] as Action[],
  stats: [] as StatEntry[],
  currentView: 'widget' as 'widget' | 'settings',
  imageDataUrl: '' as string,
})

export function useAppState() {
  const activeAction = computed(() =>
    state.actions.find((a) => a.active)
  )

  async function loadData() {
    const data = await invoke<AppData>('load_data')
    state.settings = data.settings
    state.actions = data.actions
    state.stats = data.stats
  }

  async function saveSettings(settings: Settings) {
    await invoke('save_settings', { settings })
    state.settings = settings
  }

  async function getRandomImage() {
    const url = await invoke<string | null>('get_random_image')
    if (url) {
      state.imageDataUrl = url
    }
  }

  async function addAction(name: string) {
    const action = await invoke<Action>('add_action', { name })
    state.actions.push(action)
  }

  async function updateAction(id: string, name: string) {
    await invoke('update_action', { id, name })
    const action = state.actions.find((a) => a.id === id)
    if (action) action.name = name
  }

  async function deleteAction(id: string) {
    await invoke('delete_action', { id })
    state.actions = state.actions.filter((a) => a.id !== id)
    if (state.actions.length > 0 && !state.actions.some((a) => a.active)) {
      state.actions[0].active = true
    }
  }

  async function setActiveAction(id: string) {
    await invoke('set_active_action', { id })
    for (const action of state.actions) {
      action.active = action.id === id
    }
  }

  async function logSession(actionName: string, durationMinutes: number) {
    await invoke('log_session', {
      actionName,
      durationMinutes,
    })
  }

  async function loadStats() {
    state.stats = await invoke<StatEntry[]>('get_stats')
  }

  const WIDGET_SIZE = 160
  const MARGIN = 10

  async function applyWidgetPosition() {
    const win = getCurrentWindow()
    const monitor = await currentMonitor()
    if (!monitor) return
    const { width, height } = monitor.size
    const scale = monitor.scaleFactor
    const logicalW = width / scale
    const logicalH = height / scale

    let x: number
    let y: number

    switch (state.settings.widget_position) {
      case 'top-left':
        x = MARGIN
        y = MARGIN
        break
      case 'top-right':
        x = logicalW - WIDGET_SIZE - MARGIN
        y = MARGIN
        break
      case 'bottom-right':
        x = logicalW - WIDGET_SIZE - MARGIN
        y = logicalH - WIDGET_SIZE - MARGIN
        break
      case 'bottom-left':
      default:
        x = MARGIN
        y = logicalH - WIDGET_SIZE - MARGIN
        break
    }

    await win.setPosition(new LogicalPosition(x, y))
  }

  function goToSettings() {
    state.currentView = 'settings'
  }

  function goToWidget() {
    state.currentView = 'widget'
  }

  return {
    state,
    activeAction,
    loadData,
    saveSettings,
    getRandomImage,
    addAction,
    updateAction,
    deleteAction,
    setActiveAction,
    logSession,
    loadStats,
    applyWidgetPosition,
    goToSettings,
    goToWidget,
  }
}
