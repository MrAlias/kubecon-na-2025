<template>
  <div class="slidev-layout cncf-cover">
    <!-- Header with KubeCon Logo -->
    <div class="cover-header">
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
    
    <!-- Main content - Centered -->
    <div class="cover-content">
      <slot />
    </div>
    
    <!-- Footer with metadata and CNCF Logo -->
    <div class="cover-footer">
      <div class="presentation-meta">
        <slot name="meta" />
      </div>
      <!-- Dark mode logo -->
      <img 
        src="https://raw.githubusercontent.com/cncf/artwork/refs/heads/main/other/cncf/horizontal/color-whitetext/cncf-color-whitetext.svg" 
        alt="CNCF" 
        class="cncf-logo dark-logo"
      />
      <!-- Light mode logo -->
      <img 
        src="https://raw.githubusercontent.com/cncf/artwork/main/other/cncf/horizontal/color/cncf-color.svg" 
        alt="CNCF" 
        class="cncf-logo light-logo"
      />
    </div>
  </div>
</template>

<style scoped>
.cncf-cover {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 0;
  margin: 0;
  box-sizing: border-box;
}

/* Header - fixed height for logo */
.cover-header {
  padding: 2rem 3rem 1rem 3rem;
  flex-shrink: 0;
}

.kubecon-logo {
  width: 280px;
  height: auto;
  display: block;
}

.cncf-logo {
  height: 50px;
  width: auto;
  flex-shrink: 0;
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

/* Main content - takes remaining space and centers content */
.cover-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
  padding: 2rem 4rem;
  min-height: 0;
}

.cover-content :deep(h1) {
  font-size: 3.5rem;
  margin: 0 0 2rem 0;
  line-height: 1.2;
  font-weight: 700;
}

.cover-content :deep(p) {
  font-size: 1.5rem;
  margin: 0;
  line-height: 1.6;
}

.cover-content :deep(strong) {
  color: #0086FF;
  font-weight: 700;
}

/* Footer - fixed height */
.cover-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem 3rem 2rem 3rem;
  border-top: 3px solid #0086FF;
  flex-shrink: 0;
  gap: 2rem;
}

.presentation-meta {
  font-size: 1rem;
  line-height: 1.8;
  flex: 1;
}

.presentation-meta :deep(br) {
  display: block;
  content: "";
  margin: 0.25rem 0;
}

.presentation-meta :deep(strong) {
  color: #0086FF;
  font-weight: 700;
}

/* Dark mode styles */
.dark .cncf-cover {
  background: linear-gradient(135deg, #000000 0%, #0a0a0a 100%);
}

.dark .cover-content :deep(h1),
.dark .cover-content :deep(p) {
  color: #ffffff;
}

.dark .presentation-meta {
  color: #E5E5E5;
}

/* Light mode styles */
.light .cncf-cover {
  background: linear-gradient(135deg, #f8f8f8 0%, #ffffff 100%);
}

.light .cover-content :deep(h1),
.light .cover-content :deep(p) {
  color: #000000;
}

.light .presentation-meta {
  color: rgba(0, 0, 0, 0.87);
}

.light .cover-footer {
  border-top: 3px solid #0086FF;
}
</style>
