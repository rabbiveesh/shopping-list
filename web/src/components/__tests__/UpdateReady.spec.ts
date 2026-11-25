import { describe, expect, it, vi } from 'vitest'
import { screen } from '@testing-library/dom'
import userEvent from '@testing-library/user-event'
import { mount } from '@vue/test-utils'
import { ref } from 'vue'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'

import UpdateReady from '../UpdateReady.vue'

const sw = vi.hoisted(() => ({ ready: false, update: vi.fn<() => Promise<void>>() }))
vi.mock('virtual:pwa-register/vue', () => ({
  useRegisterSW: () => ({ needRefresh: ref(sw.ready), updateServiceWorker: sw.update }),
}))

function open() {
  mount(
    { components: { UpdateReady }, template: '<v-app><UpdateReady /></v-app>' },
    { global: { plugins: [createVuetify({ components, directives })] }, attachTo: document.body },
  )
}

describe('a new version of the app', () => {
  it('says nothing while the app is up to date', () => {
    sw.ready = false
    open()
    expect(screen.queryByText('A new version of the app is ready.')).not.toBeInTheDocument()
  })

  it('offers a reload button once it has downloaded, and switches to it when tapped', async () => {
    sw.ready = true
    open()
    await userEvent.click(await screen.findByRole('button', { name: 'Reload' }))
    expect(screen.getByText('A new version of the app is ready.')).toBeInTheDocument()
    expect(sw.update).toHaveBeenCalled()
  })
})
