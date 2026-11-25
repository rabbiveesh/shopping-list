import { beforeEach, describe, expect, it, vi } from 'vitest'
import { screen, within } from '@testing-library/dom'
import userEvent from '@testing-library/user-event'
import { flushPromises, mount } from '@vue/test-utils'
import { createPinia } from 'pinia'
import { createMemoryHistory, createRouter } from 'vue-router'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import unstorageMemory from 'unstorage/drivers/memory'

import App from '@/App.vue'
import { recordItemEdits, statusEdit } from '@/core'
import { appendChanges } from '@/data/repository'
import { routes } from '@/router'
import { useDriver } from '@/storage'

// No cloud project: the list stays on the phone, here in memory.
vi.mock('@/firebase', () => ({ isFirebaseConfigured: () => false }))

async function openApp(path = '/') {
  const router = createRouter({ history: createMemoryHistory(), routes })
  await router.push(path)
  mount(App, {
    global: { plugins: [createPinia(), router, createVuetify({ components, directives })] },
    attachTo: document.body,
  })
  await flushPromises()
  return router
}

const go = async (router: Awaited<ReturnType<typeof openApp>>, path: string) => {
  await router.push(path)
  await flushPromises()
}

describe('the app', () => {
  beforeEach(() => useDriver(unstorageMemory()))

  it('opens on the foods to pick from, with nothing on the list yet', async () => {
    await openApp()
    expect(await screen.findByRole('button', { name: 'Rice cakes' })).toHaveAttribute(
      'aria-pressed',
      'false',
    )
    expect(screen.getByText(/on the list/)).toHaveTextContent('0 on the list')
  })

  it('tapping a food puts it on the shopping list, and tapping again takes it off', async () => {
    const router = await openApp()
    const user = userEvent.setup()
    await user.click(await screen.findByRole('button', { name: 'Rice cakes' }))
    expect(screen.getByRole('button', { name: 'Rice cakes' })).toHaveAttribute(
      'aria-pressed',
      'true',
    )

    await go(router, '/shop')
    expect(await screen.findByRole('button', { name: 'Tick off Rice cakes' })).toBeInTheDocument()

    await go(router, '/pick')
    await user.click(await screen.findByRole('button', { name: 'Rice cakes' }))
    await go(router, '/shop')
    expect(await screen.findByText('All done here.')).toBeInTheDocument()
  })

  it('something the user types goes straight onto the list', async () => {
    const router = await openApp()
    const user = userEvent.setup()
    await user.type(await screen.findByLabelText('Add something else'), 'Hummus{Enter}')
    expect(await screen.findByRole('button', { name: 'Hummus' })).toHaveAttribute(
      'aria-pressed',
      'true',
    )
    await go(router, '/shop')
    expect(await screen.findByRole('button', { name: 'Tick off Hummus' })).toBeInTheDocument()
  })

  it('the plus at the end of an aisle adds something to that aisle, ready to pick', async () => {
    await openApp()
    const user = userEvent.setup()
    await user.click(
      await screen.findByRole('button', { name: 'Add something to Aisle 11 · Breads' }),
    )
    await user.type(screen.getByLabelText('New in Aisle 11 · Breads'), 'Challah{Enter}')

    const challah = await screen.findByRole('button', { name: 'Challah' })
    expect(challah).toHaveAttribute('aria-pressed', 'false')
    const breads = screen.getByText('Aisle 11 · Breads').closest('section')!
    expect(within(breads).getByRole('button', { name: 'Challah' })).toBe(challah)
  })

  it('ticking something off crosses it out in the cart, and finishing the shop clears it', async () => {
    const router = await openApp()
    const user = userEvent.setup()
    await user.click(await screen.findByRole('button', { name: 'Bamba' }))
    await user.click(screen.getByRole('button', { name: 'Eggs' }))
    await go(router, '/shop')

    await user.click(await screen.findByRole('button', { name: 'Tick off Bamba' }))
    const crossedOut = screen.getByRole('button', { name: 'Put Bamba back on the list' })
    expect(within(crossedOut).getByText('Bamba').tagName).toBe('S')
    expect(screen.getByText(/in the cart$/)).toHaveTextContent('1 of 2 in the cart')

    await user.click(screen.getByRole('button', { name: 'Finish shop' }))
    expect(screen.queryByText('Bamba')).not.toBeInTheDocument()
    // What wasn't got stays on the list.
    expect(screen.getByRole('button', { name: 'Tick off Eggs' })).toBeInTheDocument()
  })

  it("something you can't find moves to the fill-in list, ready for another shop", async () => {
    const router = await openApp()
    const user = userEvent.setup()
    await user.click(await screen.findByRole('button', { name: 'Instant coffee' }))
    await go(router, '/shop')
    await user.click(await screen.findByRole('button', { name: "Can't find Instant coffee" }))
    expect(
      screen.queryByRole('button', { name: 'Tick off Instant coffee' }),
    ).not.toBeInTheDocument()

    await go(router, '/fill-in')
    await user.click(
      await screen.findByRole('button', { name: 'Put Instant coffee back on the list' }),
    )
    expect(screen.getByText('Nothing missing.')).toBeInTheDocument()
    await go(router, '/shop')
    expect(
      await screen.findByRole('button', { name: 'Tick off Instant coffee' }),
    ).toBeInTheDocument()
  })

  it('remembers the list after the app is closed and opened again', async () => {
    await openApp()
    const user = userEvent.setup()
    await user.click(await screen.findByRole('button', { name: 'Honey' }))
    await flushPromises()
    document.body.innerHTML = ''

    await openApp('/shop')
    expect(await screen.findByRole('button', { name: 'Tick off Honey' })).toBeInTheDocument()
  })

  it('asks which list is right when two phones disagree', async () => {
    const onList = recordItemEdits([], 'starter:bamba', [statusEdit('needed')], {
      at: 1,
      device: 'phone',
    })
    const move = (status: 'got' | 'missing', device: string, at: number) =>
      recordItemEdits(onList, 'starter:bamba', [statusEdit(status)], { at, device })
    await appendChanges([...onList, ...move('got', 'phone', 2), ...move('missing', 'tablet', 3)])

    await openApp('/shop')
    expect(await screen.findByText(/changed on two phones at once/)).toBeInTheDocument()
    const user = userEvent.setup()
    await user.click(screen.getByRole('button', { name: 'In the cart' }))
    await flushPromises()
    expect(screen.queryByText(/changed on two phones at once/)).not.toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'Put Bamba back on the list' })).toBeInTheDocument()
  })
})
