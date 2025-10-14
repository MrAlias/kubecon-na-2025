// UnoCSS configuration for CNCF theme
import { defineConfig, presetUno } from 'unocss'

export default defineConfig({
  shortcuts: {
    // CNCF color utilities
    'cncf-blue': 'text-[#0086FF]',
    'cncf-turquoise': 'text-[#93EAFF]',
    'cncf-pink': 'text-[#D62293]',
    'cncf-stone': 'text-[#E5E5E5]',
    
    'bg-cncf-blue': 'bg-[#0086FF]',
    'bg-cncf-turquoise': 'bg-[#93EAFF]',
    'bg-cncf-pink': 'bg-[#D62293]',
    'bg-cncf-stone': 'bg-[#E5E5E5]',
    
    // Layout utilities
    'safe-content': 'px-12 py-8 max-w-full',
    'slide-title': 'text-4xl font-bold mb-8 cncf-blue',
    
    // Component utilities
    'cncf-card': 'p-6 rounded-lg bg-opacity-10',
    'info-card': 'cncf-card bg-cncf-blue border-l-4 border-[#0086FF]',
    'warning-card': 'cncf-card bg-cncf-pink border-l-4 border-[#D62293]',
    'success-card': 'cncf-card bg-cncf-turquoise border-l-4 border-[#93EAFF]',
  },
  presets: [
    presetUno(),
  ],
})
