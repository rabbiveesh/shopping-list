// How a conflict is worded for the user.
import type { ItemEdit, ItemField, Status } from '@/core'

export const fieldLabel: Record<ItemField, string> = {
  name: 'Its name',
  aisle: 'Its aisle',
  status: 'Which list it is on',
}

export const statusLabel: Record<Status, string> = {
  off: 'Not needed',
  needed: 'On the list',
  got: 'In the cart',
  missing: "Couldn't find it",
}

export function editLabel(edit: ItemEdit): string {
  return edit.field === 'status' ? statusLabel[edit.value] : edit.value
}
