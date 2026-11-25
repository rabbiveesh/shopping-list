<script setup lang="ts">
import { ref } from 'vue'
import { vOnLongPress } from '@vueuse/components'

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

// A long press shows a trash can next to an item; the tap that ends it doesn't pick it.
const armed = ref<string | null>(null)
let longPressed = false
const deleted = ref<Item | null>(null)

function arm(item: Item) {
  armed.value = item.id
  longPressed = true
}

function tap(item: Item) {
  if (longPressed) {
    longPressed = false
    return
  }
  armed.value = null
  shopping.pick(item)
}

function remove(item: Item) {
  armed.value = null
  shopping.remove(item)
  deleted.value = item
}

function undo() {
  if (deleted.value) shopping.restore(deleted.value.id)
  deleted.value = null
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
      <span v-for="item in aisle.items" :key="item.id" class="d-inline-flex align-center ga-1">
        <v-btn
          v-on-long-press="[() => arm(item), { delay: 500 }]"
          rounded="pill"
          size="default"
          height="44"
          class="text-none no-callout"
          :variant="look(item).variant"
          :color="look(item).color"
          :prepend-icon="look(item).icon"
          :aria-pressed="isOnList(item)"
          @click="tap(item)"
          @contextmenu.prevent
        >
          {{ item.name }}
        </v-btn>
        <v-btn
          v-if="armed === item.id"
          icon="mdi-delete"
          color="error"
          variant="flat"
          size="small"
          :aria-label="`Delete ${item.name}`"
          @click="remove(item)"
        />
      </span>
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

  <v-snackbar :model-value="deleted !== null" :timeout="6000" @update:model-value="deleted = null">
    Deleted {{ deleted?.name }}
    <template #actions>
      <v-btn variant="text" class="text-none font-weight-bold" @click="undo">Undo</v-btn>
    </template>
  </v-snackbar>

  <v-footer app class="px-4 py-3 d-flex align-center ga-3" elevation="4">
    <span class="flex-grow-1 text-body-1">
      <strong class="text-h6">{{ shopping.toBuyCount }}</strong> on the list
    </span>
    <v-btn color="primary" rounded="pill" class="text-none" to="/shop">Go shopping</v-btn>
  </v-footer>
</template>
