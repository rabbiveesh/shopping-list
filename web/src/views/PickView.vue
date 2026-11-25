<script setup lang="ts">
import { ref } from 'vue'

import { isOnList, type Item } from '@/core'
import { useShoppingStore } from '@/stores/shopping'

const shopping = useShoppingStore()
const draft = ref('')

function add() {
  shopping.add(draft.value)
  draft.value = ''
}

// The aisle whose own add box is open, if any.
const addingTo = ref<string | null>(null)
const aisleDraft = ref('')

function openAisle(aisle: string) {
  addingTo.value = aisle
  aisleDraft.value = ''
}

function addToAisle() {
  if (addingTo.value) shopping.addToAisle(aisleDraft.value, addingTo.value)
  addingTo.value = null
}

// On the list: filled green with a tick. Couldn't find last time: purple. Otherwise plain.
function look(item: Item) {
  if (isOnList(item)) return { variant: 'flat', color: 'primary', icon: 'mdi-check' } as const
  if (item.status === 'missing')
    return { variant: 'tonal', color: 'secondary', icon: 'mdi-magnify-close' } as const
  return { variant: 'outlined', color: undefined, icon: undefined } as const
}
</script>

<template>
  <v-form class="d-flex ga-2 mb-5" @submit.prevent="add">
    <v-text-field
      v-model="draft"
      label="Add something else"
      variant="outlined"
      bg-color="surface"
      hide-details
    />
    <v-btn
      type="submit"
      color="primary"
      icon="mdi-plus"
      aria-label="Add to the list"
      height="56"
      width="56"
      rounded="lg"
      :disabled="!draft.trim()"
    />
  </v-form>

  <section v-for="aisle in shopping.lists.catalogue" :key="aisle.name" class="mb-5">
    <h2 class="aisle-heading text-uppercase font-weight-bold text-medium-emphasis mb-1">
      {{ aisle.name }}
    </h2>
    <div class="d-flex flex-wrap ga-2">
      <v-btn
        v-for="item in aisle.items"
        :key="item.id"
        rounded="pill"
        size="default"
        height="44"
        class="text-none"
        :variant="look(item).variant"
        :color="look(item).color"
        :prepend-icon="look(item).icon"
        :aria-pressed="isOnList(item)"
        @click="shopping.pick(item)"
      >
        {{ item.name }}
      </v-btn>
      <v-btn
        v-if="addingTo !== aisle.name"
        icon="mdi-plus"
        size="small"
        variant="tonal"
        :aria-label="`Add something to ${aisle.name}`"
        @click="openAisle(aisle.name)"
      />
    </div>
    <v-form
      v-if="addingTo === aisle.name"
      class="d-flex ga-2 mt-2"
      @submit.prevent="addToAisle"
      @keydown.esc="addingTo = null"
    >
      <v-text-field
        v-model="aisleDraft"
        :label="`New in ${aisle.name}`"
        variant="outlined"
        density="comfortable"
        bg-color="surface"
        autofocus
        hide-details
      />
      <v-btn
        type="submit"
        color="primary"
        icon="mdi-check"
        rounded="lg"
        :aria-label="`Add to ${aisle.name}`"
        :disabled="!aisleDraft.trim()"
      />
      <v-btn icon="mdi-close" variant="text" aria-label="Cancel" @click="addingTo = null" />
    </v-form>
  </section>

  <v-footer app class="px-4 py-3 d-flex align-center ga-3" elevation="4">
    <span class="flex-grow-1 text-body-1">
      <strong class="text-h6">{{ shopping.toBuyCount }}</strong> on the list
    </span>
    <v-btn color="primary" rounded="pill" class="text-none" to="/shop">Go shopping</v-btn>
  </v-footer>
</template>
