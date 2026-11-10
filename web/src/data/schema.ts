// The shape of what the app stores. The lists are never stored as such: they are the
// change log, which the Rust core folds over the starter items. Bump SCHEMA_VERSION and add
// a step to migrations.ts whenever the stored change format changes.
import type { ItemChange } from '@/core'

export const SCHEMA_VERSION = 1

/** One change as stored: its format version, so any version of the app can read it. */
export type StoredChange = { v: number; change: ItemChange }
