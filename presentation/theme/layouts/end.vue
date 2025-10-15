<script setup lang="ts">
import { onMounted, ref } from 'vue'
import QRCodeStyling from 'qr-code-styling'

const qrCodeContainer = ref<HTMLElement>()

onMounted(() => {
  if (qrCodeContainer.value) {
    const isDark = document.documentElement.classList.contains('dark')
    
    const qrCode = new QRCodeStyling({
      width: 120,
      height: 120,
      data: 'https://mralias.github.io/kubecon-na-2025',
      margin: 0,
      qrOptions: {
        typeNumber: 0,
        mode: 'Byte',
        errorCorrectionLevel: 'M'
      },
      imageOptions: {
        hideBackgroundDots: true,
        imageSize: 0.4,
        margin: 0
      },
      dotsOptions: {
        type: 'rounded',
        color: isDark ? '#ffffff' : '#000000'
      },
      backgroundOptions: {
        color: 'transparent'
      },
      cornersSquareOptions: {
        type: 'extra-rounded',
        color: isDark ? '#ffffff' : '#000000'
      },
      cornersDotOptions: {
        type: 'dot',
        color: isDark ? '#ffffff' : '#000000'
      }
    })
    
    qrCode.append(qrCodeContainer.value)
    
    // Watch for theme changes
    const observer = new MutationObserver((mutations) => {
      mutations.forEach((mutation) => {
        if (mutation.attributeName === 'class') {
          const isDark = document.documentElement.classList.contains('dark')
          qrCode.update({
            dotsOptions: {
              type: 'rounded',
              color: isDark ? '#ffffff' : '#000000'
            },
            cornersSquareOptions: {
              type: 'extra-rounded',
              color: isDark ? '#ffffff' : '#000000'
            },
            cornersDotOptions: {
              type: 'dot',
              color: isDark ? '#ffffff' : '#000000'
            }
          })
        }
      })
    })
    
    observer.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ['class']
    })
  }
})
</script>

<template>
  <div class="slidev-layout end cncf-end">
    <!-- Header with KubeCon Logo and QR Code -->
    <div class="end-header">
      <!-- KubeCon + CloudNativeCon Logo on left -->
      <div class="kubecon-logo-container">
        <!-- Dark mode logo -->
        <img 
          src="https://raw.githubusercontent.com/cncf/artwork/refs/heads/main/other/kubecon-cloudnativecon/2025-na/white/kccnc-na-2025-white.svg" 
          alt="KubeCon + CloudNativeCon" 
          class="kubecon-logo dark-logo"
        />
        <!-- Light mode logo -->
        <img 
          src="https://raw.githubusercontent.com/cncf/artwork/refs/heads/main/other/kubecon-cloudnativecon/2025-na/color/kccnc-na-2025-color.svg" 
          alt="KubeCon + CloudNativeCon" 
          class="kubecon-logo light-logo"
        />
      </div>
      
      <!-- QR Code on right -->
      <div class="qr-code-container" ref="qrCodeContainer"></div>
    </div>
    
    <!-- Main content -->
    <div class="end-content">
      <slot />
    </div>
    
    <!-- CNCF Logo in footer -->
    <div class="slide-footer">
      <div class="presentation-meta">
        <span class="hashtag">#KubeCon #CloudNativeCon</span>
      </div>
      <!-- Dark mode logo -->
      <img 
        src="https://raw.githubusercontent.com/cncf/artwork/refs/heads/main/other/cncf/horizontal/color-whitetext/cncf-color-whitetext.svg" 
        alt="CNCF" 
        class="logo-small dark-logo"
      />
      <!-- Light mode logo -->
      <img 
        src="https://raw.githubusercontent.com/cncf/artwork/main/other/cncf/horizontal/color/cncf-color.svg" 
        alt="CNCF" 
        class="logo-small light-logo"
      />
    </div>
  </div>
</template>

<style scoped>
.cncf-end {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 0;
  margin: 0;
  box-sizing: border-box;
}

/* Header with KubeCon logo and QR code */
.end-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: 2rem 3rem;
  flex-shrink: 0;
  gap: 2rem;
}

/* KubeCon logo container */
.kubecon-logo-container {
  flex-shrink: 0;
  order: 1;
}

.kubecon-logo {
  width: 280px;
  height: auto;
  display: block;
}

/* QR Code container */
.qr-code-container {
  width: 120px;
  height: 120px;
  flex-shrink: 0;
  order: 2;
  margin-left: auto;
}

/* Logo visibility control */
.dark-logo {
  display: none;
}

.light-logo {
  display: block;
}

.dark .dark-logo {
  display: block;
}

.dark .light-logo {
  display: none;
}

.light .dark-logo {
  display: none;
}

.light .light-logo {
  display: block;
}

.end-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  padding: 2rem 4rem;
  max-width: 100%;
  box-sizing: border-box;
  overflow-y: auto;
}

.end-content :deep(h1) {
  font-size: clamp(2.5rem, 6vw, 3.5rem);
  margin-bottom: 1.5rem;
  text-align: center;
  width: 100%;
  box-sizing: border-box;
}

.end-content :deep(h2) {
  margin-bottom: 0.75rem;
  font-size: 1.5rem;
}

.end-content :deep(p) {
  margin-bottom: 0.5rem;
}

/* Footer with CNCF logo */
.slide-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 3rem 1.5rem 3rem;
  border-top: 2px solid #0086FF;
  flex-shrink: 0;
  gap: 2rem;
}

.presentation-meta {
  flex: 1;
}

.hashtag {
  font-weight: 700;
  font-size: 0.95rem;
}

.logo-small {
  height: 40px;
  width: auto;
  flex-shrink: 0;
}

/* Dark mode styles */
.dark .cncf-end {
  background: linear-gradient(135deg, #000000 0%, #0a0a0a 100%);
}

.dark .end-content :deep(h1) {
  color: #ffffff;
}

.dark .end-content :deep(h2) {
  color: var(--cncf-turquoise);
}

.dark .end-content :deep(p),
.dark .end-content :deep(div) {
  color: var(--cncf-stone);
}

.dark .hashtag {
  color: #93EAFF;
}

/* Light mode styles */
.light .cncf-end {
  background: linear-gradient(135deg, #f8f8f8 0%, #ffffff 100%);
}

.light .end-content :deep(h1) {
  color: #000000;
}

.light .end-content :deep(h2) {
  color: #005bcc;
}

.light .end-content :deep(p),
.light .end-content :deep(div) {
  color: rgba(0, 0, 0, 0.7);
}

.light .hashtag {
  color: #005bcc;
}
</style>
