import { defineStore } from 'pinia'
import { computed, ref } from 'vue'

import {
  finishShopEdits,
  isOnList,
  itemNamed,
  ledger,
  lists,
  newItemEdits,
  pickEdit,
  recordItemEdits,
  statusEdit,
  type Item,
  type ItemChange,
  type ItemOp,
  type Status,
} from '@/core'
import { appendChanges, deviceId, loadChanges } from '@/data/repository'
import { MY_ITEMS, starterItems } from '@/data/starter'

export const useShoppingStore = defineStore('shopping', () => {
  /** The change log; everything else is worked out from it. */
  const changes = ref<ItemChange[]>([])
  const device = ref('')
  let lastAt = 0
  const loaded = ref(false)
  const loadError = ref<string | null>(null)
  const saveError = ref<string | null>(null)

  const current = computed(() => ledger(starterItems, changes.value))
  const items = computed(() => current.value.items)
  const all = computed(() => lists(items.value))
  const conflicts = computed(() => current.value.conflicts)
  const toBuyCount = computed(() => all.value.toBuy.reduce((n, a) => n + a.items.length, 0))

  async function load() {
    loaded.value = false
    loadError.value = null
    try {
      device.value = await deviceId()
      changes.value = await loadChanges()
      loaded.value = true
    } catch (e) {
      loadError.value = e instanceof Error ? e.message : String(e)
    }
  }

  // The lists change at once; the save finishes in the background. Offline, the cloud
  // only confirms a save once the phone is back online.
  function apply(itemId: string, edits: ItemOp[]) {
    if (edits.length === 0) return
    // Strictly increasing, so two edits in the same millisecond still get distinct ids.
    lastAt = Math.max(Date.now(), lastAt + 1)
    const stamp = { at: lastAt, device: device.value }
    const added = recordItemEdits(changes.value, itemId, edits, stamp)
    changes.value = [...changes.value, ...added]
    appendChanges(added).then(
      () => (saveError.value = null),
      (e: unknown) => (saveError.value = e instanceof Error ? e.message : String(e)),
    )
  }

  const find = (id: string) => items.value.find((i) => i.id === id)

  /** Tapping an item while picking: onto the list, or off it again. */
  const pick = (item: Item) => apply(item.id, [pickEdit(item)])
  /** Ticked off, can't find, back on the list, or done with. */
  const move = (item: Item, status: Status) => apply(item.id, [statusEdit(status)])

  /** Puts something on the list by name, adding it to the catalogue if it's new. */
  function add(name: string) {
    if (!name.trim()) return
    const existing = itemNamed(items.value, name)
    if (!existing) apply(crypto.randomUUID(), newItemEdits(name, MY_ITEMS))
    else if (!isOnList(existing)) move(existing, 'needed')
  }

  /** Takes what's in the cart off the list. */
  function finishShop() {
    for (const { itemId, edits } of finishShopEdits(items.value)) apply(itemId, edits)
  }

  /** Settles a conflict by recording the chosen value as the newest edit. */
  const choose = (itemId: string, op: ItemOp) => apply(itemId, [op])

  return {
    changes,
    loaded,
    loadError,
    saveError,
    lists: all,
    toBuyCount,
    conflicts,
    load,
    find,
    pick,
    move,
    add,
    finishShop,
    choose,
  }
})
