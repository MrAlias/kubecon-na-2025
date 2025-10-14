import { defineAppSetup } from '@slidev/types'

export default defineAppSetup(({ app, router }) => {
  // Fix viewport scaling for GitHub Pages
  if (typeof window !== 'undefined') {
    // Ensure proper viewport meta tag
    const viewport = document.querySelector('meta[name="viewport"]')
    if (viewport) {
      viewport.setAttribute('content', 'width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no')
    }
  }
})
