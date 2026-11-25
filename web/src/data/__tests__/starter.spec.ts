import { describe, expect, it } from 'vitest'

import { starterItems } from '../starter'

describe('the starter list', () => {
  it('lists each item once, so tapping one never ticks another', () => {
    const ids = starterItems.map((item) => item.id)
    expect(new Set(ids).size).toBe(ids.length)
  })
})
