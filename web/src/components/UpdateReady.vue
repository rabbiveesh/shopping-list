<script setup lang="ts">
// When a new version of the app has downloaded, offer to switch to it. It waits for a tap
// rather than reloading by itself, so an update never interrupts a shop. Same as the
// meal planner's: an open app also asks hourly whether a newer version is deployed.
import { useRegisterSW } from 'virtual:pwa-register/vue'

/** How often an open app asks the server whether a newer version is deployed. */
const CHECK_EVERY_MS = 60 * 60 * 1000

const { needRefresh, updateServiceWorker } = useRegisterSW({
  onRegisteredSW(_url, registration) {
    if (registration) setInterval(() => registration.update(), CHECK_EVERY_MS)
  },
})
</script>

<template>
  <v-snackbar v-model="needRefresh" :timeout="-1" location="top" color="primary">
    A new version of the app is ready.
    <template #actions>
      <v-btn variant="flat" class="text-none" @click="updateServiceWorker()">Reload</v-btn>
    </template>
  </v-snackbar>
</template>
