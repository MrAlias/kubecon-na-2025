<template>
  <div ref="qrCodeRef" class="qr-code-container"></div>
</template>

<script setup lang="ts">
import { onMounted, ref, computed, watch } from 'vue'
import QRCodeStyling, { type Options } from 'qr-code-styling'

interface QRCodeProps {
  value: string
  width?: number
  height?: number
  margin?: number
  type?: 'svg' | 'canvas'
  errorCorrectionLevel?: 'L' | 'M' | 'Q' | 'H'
}

const props = withDefaults(defineProps<QRCodeProps>(), {
  width: 300,
  height: 300,
  margin: 0,
  type: 'svg',
  errorCorrectionLevel: 'Q'
})

const qrCodeRef = ref<HTMLElement | null>(null)

// Check if we're in dark mode by looking at the document classes
const isDark = computed(() => {
  if (typeof document === 'undefined') return true
  return document.documentElement.classList.contains('dark')
})

// Create QR code options based on theme
const getQROptions = (): Options => {
  const fgColor = isDark.value ? '#FFFFFF' : '#000000'
  const bgColor = isDark.value ? 'transparent' : '#FFFFFF'
  
  return {
    type: props.type,
    width: props.width,
    height: props.height,
    data: props.value,
    margin: props.margin,
    qrOptions: {
      typeNumber: 0,
      mode: undefined,
      errorCorrectionLevel: props.errorCorrectionLevel
    },
    dotsOptions: {
      type: 'square',
      color: fgColor
    },
    backgroundOptions: {
      color: bgColor
    },
    imageOptions: {
      hideBackgroundDots: true,
      imageSize: 0.4,
      margin: 0
    }
  }
}

let qrCode: QRCodeStyling | null = null

const renderQRCode = () => {
  if (!qrCodeRef.value) return
  
  // Clear existing QR code
  qrCodeRef.value.innerHTML = ''
  
  // Create new QR code with current theme
  qrCode = new QRCodeStyling(getQROptions())
  qrCode.append(qrCodeRef.value)
}

onMounted(() => {
  renderQRCode()
  
  // Watch for theme changes
  if (typeof document !== 'undefined') {
    const observer = new MutationObserver(() => {
      renderQRCode()
    })
    
    observer.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ['class']
    })
  }
})

// Watch for value changes
watch(() => props.value, () => {
  if (qrCode) {
    qrCode.update({ data: props.value })
  }
})
</script>

<style scoped>
.qr-code-container {
  display: inline-block;
  padding: 1rem;
  border-radius: 8px;
}

/* Dark mode background */
.dark .qr-code-container {
  background: rgba(255, 255, 255, 0.1);
}

/* Light mode background */
.light .qr-code-container {
  background: rgba(0, 0, 0, 0.05);
}
</style>
