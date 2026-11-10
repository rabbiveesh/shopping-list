import '@mdi/font/css/materialdesignicons.css'
import 'vuetify/styles'
import { createVuetify } from 'vuetify'

// Green for what's on the list and in the cart; purple for what couldn't be found.
export default createVuetify({
  theme: {
    defaultTheme: 'system',
    themes: {
      light: {
        colors: { primary: '#1d6b4f', secondary: '#6a3ea1', background: '#f6f5ef' },
      },
      dark: { colors: { primary: '#5fbf97', secondary: '#b9a3db' } },
    },
  },
  defaults: {
    VBtn: { size: 'large' },
    VCard: { rounded: 'xl' },
  },
})
