import { defineSetupFont } from '@slidev/types'

export default defineSetupFont(() => {
  return {
    webfonts: {
      fonts: {
        // Clarity City font family
        sans: 'Clarity City',
        // Fallback to system fonts
        fallback: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif',
      },
      provider: 'none', // We'll load Clarity City manually via CSS
    },
  }
})
