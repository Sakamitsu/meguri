import { ref, computed } from 'vue'

export type TimerState = 'idle' | 'running' | 'confirming'

export function useTimer(
  onConfirm: () => Promise<void>,
) {
  const timerState = ref<TimerState>('idle')
  const remainingSeconds = ref(0)
  const totalSeconds = ref(0)
  let intervalId: ReturnType<typeof setInterval> | null = null

  const progress = computed(() => {
    if (totalSeconds.value === 0) return 0
    return 1 - remainingSeconds.value / totalSeconds.value
  })

  const displayTime = computed(() => {
    const m = Math.floor(remainingSeconds.value / 60)
    const s = remainingSeconds.value % 60
    return `${m}:${s.toString().padStart(2, '0')}`
  })

  function clearTimer() {
    if (intervalId !== null) {
      clearInterval(intervalId)
      intervalId = null
    }
  }

  function tick(confirmationMinutes: number) {
    remainingSeconds.value--

    if (remainingSeconds.value <= 0) {
      clearTimer()

      if (timerState.value === 'running') {
        timerState.value = 'confirming'
        totalSeconds.value = confirmationMinutes * 60
        remainingSeconds.value = totalSeconds.value
        intervalId = setInterval(() => tick(confirmationMinutes), 1000)
      } else if (timerState.value === 'confirming') {
        timerState.value = 'idle'
      }
    }
  }

  function start(timerMinutes: number, confirmationMinutes: number) {
    clearTimer()
    timerState.value = 'running'
    totalSeconds.value = timerMinutes * 60
    remainingSeconds.value = totalSeconds.value
    intervalId = setInterval(() => tick(confirmationMinutes), 1000)
  }

  function stop() {
    clearTimer()
    timerState.value = 'idle'
    remainingSeconds.value = 0
  }

  async function confirm(timerMinutes: number, confirmationMinutes: number) {
    if (timerState.value !== 'confirming') return
    clearTimer()
    await onConfirm()
    start(timerMinutes, confirmationMinutes)
  }

  return {
    timerState,
    remainingSeconds,
    totalSeconds,
    progress,
    displayTime,
    start,
    stop,
    confirm,
  }
}
