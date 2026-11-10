<script setup lang="ts">
import { computed } from 'vue'

import { useShoppingStore } from '@/stores/shopping'

const shopping = useShoppingStore()
const inCart = computed(() => shopping.lists.inCart)
const tripSize = computed(
  () => shopping.toBuyCount + inCart.value.length + shopping.lists.fillIn.length,
)
</script>

<template>
  <p class="text-body-2 text-medium-emphasis mb-3">
    Tap or swipe right once it's in the cart. Swipe left, or tap the purple button, if you can't
    find it.
  </p>

  <v-card v-if="shopping.toBuyCount === 0" variant="tonal" class="pa-6 mb-4 text-center">
    All done here.
  </v-card>

  <section v-for="aisle in shopping.lists.toBuy" :key="aisle.name" class="mb-4">
    <h2 class="aisle-heading text-uppercase font-weight-bold text-medium-emphasis mb-1">
      {{ aisle.name }}
    </h2>
    <v-card
      v-for="item in aisle.items"
      :key="item.id"
      v-touch="{
        left: () => shopping.move(item, 'missing'),
        right: () => shopping.move(item, 'got'),
      }"
      class="mb-2 d-flex align-center"
    >
      <v-btn
        variant="text"
        height="64"
        class="flex-grow-1 justify-start text-none text-h6 font-weight-regular"
        prepend-icon="mdi-checkbox-blank-circle-outline"
        :aria-label="`Tick off ${item.name}`"
        @click="shopping.move(item, 'got')"
      >
        {{ item.name }}
      </v-btn>
      <v-btn
        icon="mdi-magnify-close"
        color="secondary"
        variant="tonal"
        rounded="lg"
        class="mr-2"
        :aria-label="`Can't find ${item.name}`"
        @click="shopping.move(item, 'missing')"
      />
    </v-card>
  </section>

  <template v-if="inCart.length">
    <h2 class="aisle-heading text-uppercase font-weight-bold text-medium-emphasis mb-1">
      In the cart
    </h2>
    <v-btn
      v-for="item in inCart"
      :key="item.id"
      block
      variant="tonal"
      color="primary"
      height="52"
      class="mb-2 justify-start text-none"
      prepend-icon="mdi-check-circle"
      :aria-label="`Put ${item.name} back on the list`"
      @click="shopping.move(item, 'needed')"
    >
      <s>{{ item.name }}</s>
    </v-btn>
  </template>

  <v-footer app class="px-4 py-3 d-flex align-center ga-3" elevation="4">
    <span class="flex-grow-1 text-body-1">
      <strong class="text-h6">{{ inCart.length }}</strong> of {{ tripSize }} in the cart
    </span>
    <v-btn
      color="primary"
      rounded="pill"
      class="text-none"
      :disabled="inCart.length === 0"
      @click="shopping.finishShop"
    >
      Finish shop
    </v-btn>
  </v-footer>
</template>
