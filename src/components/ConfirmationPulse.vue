<script setup lang="ts">
defineProps<{
  displayTime: string
}>()

defineEmits<{
  confirm: []
}>()
</script>

<template>
  <div class="confirmation-pulse" @click="$emit('confirm')">
    <div class="pulse-ring" />
    <div class="pulse-content">
      <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
        <polyline points="20 6 9 17 4 12" />
      </svg>
      <span class="pulse-time">{{ displayTime }}</span>
    </div>
  </div>
</template>

<style scoped>
.confirmation-pulse {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  z-index: 20;
  background: rgba(17, 17, 27, 0.7);
}

.pulse-ring {
  position: absolute;
  inset: 4px;
  border-radius: 12px;
  border: 2px solid var(--ctp-mauve);
  animation: pulse 1.2s ease-in-out infinite;
}

.pulse-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  color: var(--ctp-green);
  animation: pulse-opacity 1.2s ease-in-out infinite;
}

.pulse-time {
  font-size: 11px;
  color: var(--ctp-subtext0);
  font-variant-numeric: tabular-nums;
}

@keyframes pulse {
  0%, 100% {
    opacity: 0.4;
    transform: scale(1);
  }
  50% {
    opacity: 1;
    transform: scale(1.02);
  }
}

@keyframes pulse-opacity {
  0%, 100% {
    opacity: 0.6;
  }
  50% {
    opacity: 1;
  }
}
</style>
