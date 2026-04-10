<script setup lang="ts">
import { onMounted } from 'vue'
import { useAppState } from './composables/useAppState'
import WidgetView from './components/WidgetView.vue'
import SettingsView from './components/SettingsView.vue'

const { state, loadData, getRandomImage } = useAppState()

onMounted(async () => {
  await loadData()
  await getRandomImage()
})
</script>

<template>
  <WidgetView v-if="state.currentView === 'widget'" />
  <SettingsView v-else />
</template>