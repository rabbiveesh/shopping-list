<script setup lang="ts">
// When a new version of the app has downloaded, offer to switch to it. It waits for a tap
// rather than reloading by itself, so an update never interrupts a shop.
import { useRegisterSW } from 'virtual:pwa-register/vue'

const { needRefresh, updateServiceWorker } = useRegisterSW()
</script>

<template>
  <v-snackbar v-model="needRefresh" :timeout="-1" location="top" color="primary">
    An update is ready
    <template #actions>
      <v-btn variant="flat" class="text-none" @click="updateServiceWorker()">Update</v-btn>
    </template>
  </v-snackbar>
</template>
