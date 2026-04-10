<script setup lang="ts">
defineProps<{
  src: string
}>()
</script>

<template>
  <div class="image-display">
    <img v-if="src" :src="src" alt="" />
    <div v-else class="placeholder">
      <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <rect x="3" y="3" width="18" height="18" rx="2" />
        <circle cx="8.5" cy="8.5" r="1.5" />
        <path d="m21 15-5-5L5 21" />
      </svg>
    </div>
  </div>
</template>

<style scoped>
.image-display {
  width: 100%;
  height: 100%;
  position: absolute;
  inset: 0;
}

.image-display img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--ctp-mantle);
  color: var(--ctp-overlay0);
}
</style>
