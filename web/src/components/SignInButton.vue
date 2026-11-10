<script setup lang="ts">
import { ref } from 'vue'

import { signInWithGoogle } from '@/account'

const error = ref<string | null>(null)

async function signIn() {
  error.value = null
  try {
    await signInWithGoogle()
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e)
  }
}
</script>

<template>
  <v-btn prepend-icon="mdi-google" variant="text" size="default" @click="signIn">Sync</v-btn>
  <v-snackbar :model-value="Boolean(error)" color="error" @update:model-value="error = null">
    {{ error }}
  </v-snackbar>
</template>
