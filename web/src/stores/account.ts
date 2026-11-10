import { defineStore } from 'pinia'
import { ref } from 'vue'

import { watchAccount, type Account } from '@/account'
import { appendChanges, loadChanges } from '@/data/repository'
import { useAccountStorage, useDeviceStorage } from '@/storage'
import { useShoppingStore } from './shopping'

export const useAccountStore = defineStore('account', () => {
  const account = ref<Account | null>(null)
  const shopping = useShoppingStore()

  /**
   * Follows sign-in and sign-out. Signing in brings along what was done on this phone:
   * the log only ever gains changes, so adding this phone's to the account's is safe.
   */
  function follow() {
    watchAccount(async (signedIn) => {
      account.value = signedIn
      if (signedIn) {
        const onThisPhone = await loadChanges()
        useAccountStorage(signedIn.uid)
        if (onThisPhone.length) await appendChanges(onThisPhone)
      } else {
        useDeviceStorage()
      }
      await shopping.load()
    })
  }

  return { account, follow }
})
