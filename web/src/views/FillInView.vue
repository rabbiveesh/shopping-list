<script setup lang="ts">
import { useShoppingStore } from '@/stores/shopping'

const shopping = useShoppingStore()
</script>

<template>
  <p class="text-body-2 text-medium-emphasis mb-3">
    Things you couldn't find. Grab them at the next shop.
  </p>

  <v-card v-if="shopping.lists.fillIn.length === 0" variant="tonal" class="pa-6 text-center">
    Nothing missing.
  </v-card>

  <div v-else>
    <v-card
      v-for="item in shopping.lists.fillIn"
      :key="item.id"
      variant="outlined"
      color="secondary"
      class="mb-2 pa-2 pl-4 d-flex align-center ga-2"
    >
      <span class="flex-grow-1 text-h6 font-weight-regular text-high-emphasis">
        {{ item.name }}
      </span>
      <v-btn
        variant="tonal"
        size="default"
        class="text-none"
        :aria-label="`Put ${item.name} back on the list`"
        @click="shopping.move(item, 'needed')"
      >
        To list
      </v-btn>
      <v-btn
        color="secondary"
        variant="flat"
        size="default"
        class="text-none"
        :aria-label="`Got ${item.name}`"
        @click="shopping.move(item, 'off')"
      >
        Got it
      </v-btn>
    </v-card>
  </div>
</template>
