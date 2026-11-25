<script setup lang="ts">
import { onMounted } from 'vue'

import { isFirebaseConfigured } from '@/firebase'
import { useAccountStore } from '@/stores/account'
import { useShoppingStore } from '@/stores/shopping'
import AccountMenu from '@/components/AccountMenu.vue'
import ConflictCard from '@/components/ConflictCard.vue'
import SignInButton from '@/components/SignInButton.vue'

const account = useAccountStore()
const shopping = useShoppingStore()
const configured = isFirebaseConfigured()
// Without a cloud project the list simply stays on this phone.
onMounted(() => (configured ? account.follow() : shopping.load()))
</script>

<template>
  <v-app>
    <v-app-bar color="primary" density="comfortable">
      <v-app-bar-title>Shopping list</v-app-bar-title>
      <template v-if="configured" #append>
        <AccountMenu v-if="account.account" :account="account.account" />
        <SignInButton v-else />
      </template>
      <template #extension>
        <v-tabs grow bg-color="primary" aria-label="Stages">
          <v-tab to="/pick">Pick</v-tab>
          <v-tab to="/shop">
            Shop
            <v-badge
              v-if="shopping.toBuyCount"
              inline
              color="white"
              :content="shopping.toBuyCount"
            />
          </v-tab>
          <v-tab to="/fill-in">
            Fill-in
            <v-badge
              v-if="shopping.lists.fillIn.length"
              inline
              color="secondary"
              :content="shopping.lists.fillIn.length"
            />
          </v-tab>
        </v-tabs>
      </template>
    </v-app-bar>

    <v-main>
      <v-alert v-if="shopping.loadError" type="error" class="ma-4" :text="shopping.loadError" />
      <v-container v-else-if="shopping.loaded" class="pa-3" style="max-width: 600px">
        <v-alert
          v-if="shopping.saveError"
          type="error"
          variant="tonal"
          class="mb-4"
          :text="`Your last change wasn't saved: ${shopping.saveError}`"
        />
        <ConflictCard
          v-for="conflict in shopping.conflicts"
          :key="`${conflict.kind}:${conflict.itemId}`"
          :conflict="conflict"
          :item-name="shopping.find(conflict.itemId)?.name ?? 'Something'"
          @choose="(op) => shopping.choose(conflict.itemId, op)"
        />
        <router-view />
      </v-container>
      <div v-else class="d-flex justify-center pa-8"><v-progress-circular indeterminate /></div>
    </v-main>
  </v-app>
</template>

<style>
.aisle-heading {
  font-size: 0.8rem;
  letter-spacing: 0.08em;
}
</style>
