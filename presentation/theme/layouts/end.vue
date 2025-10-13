<template>
  <div class="slidev-layout end cncf-end">
    <!-- KubeCon + CloudNativeCon Logo -->
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
  position: relative;
}

/* KubeCon logo container at top */
.kubecon-logo-container {
  padding: 2rem 3rem 1rem 3rem;
  flex-shrink: 0;
}

.kubecon-logo {
  width: 280px;
  height: auto;
  display: block;
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
