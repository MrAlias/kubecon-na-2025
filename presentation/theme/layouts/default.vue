<template>
  <div class="slidev-layout cncf-default">
    <!-- CNCF Logo in corner -->
    <div class="cncf-logo-container">
      <!-- Dark mode logo -->
      <img 
        src="https://raw.githubusercontent.com/cncf/artwork/main/other/cncf/icon/white/cncf-icon-white.svg" 
        alt="CNCF" 
        class="cncf-logo dark-logo"
      />
      <!-- Light mode logo -->
      <img 
        src="https://raw.githubusercontent.com/cncf/artwork/main/other/cncf/icon/color/cncf-icon-color.svg" 
        alt="CNCF" 
        class="cncf-logo light-logo"
      />
    </div>
    
    <!-- Main content -->
    <div class="default-content">
      <slot />
    </div>
    
    <!-- Footer -->
    <div class="slide-footer">
      <div class="hashtag">#KubeCon #CloudNativeCon</div>
    </div>
  </div>
</template>

<style scoped>
.cncf-default {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 0;
  margin: 0;
  box-sizing: border-box;
}

.cncf-logo-container {
  position: absolute;
  top: 1.5rem;
  right: 2rem;
  width: 60px;
  z-index: 10;
}

.cncf-logo {
  width: 100%;
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

.default-content {
  flex: 1;
  padding: 3rem 4rem 2rem 4rem;
  max-width: 100%;
  overflow-y: auto;
  box-sizing: border-box;
}

.default-content :deep(h1) {
  color: #0086FF;
  margin-bottom: 2rem;
  padding-top: 0;
}

.default-content :deep(h2) {
  margin-bottom: 1.5rem;
}

.slide-footer {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 0.75rem 3rem;
  border-top: 2px solid #0086FF;
  flex-shrink: 0;
  min-height: 50px;
}

.hashtag {
  font-weight: 700;
  font-size: 0.95rem;
}

/* Dark mode styles */
.dark .cncf-default {
  background: transparent;
}

.dark .default-content :deep(h2) {
  color: #93EAFF;
}

.dark .default-content :deep(h3) {
  color: #ffffff;
}

.dark .default-content :deep(p),
.dark .default-content :deep(li) {
  color: rgba(255, 255, 255, 0.9);
}

.dark .slide-footer {
  background: rgba(0, 134, 255, 0.1);
}

.dark .hashtag {
  color: #93EAFF;
}

/* Light mode styles */
.light .cncf-default {
  background: transparent;
}

.light .default-content :deep(h2) {
  color: #005bcc;
}

.light .default-content :deep(h3) {
  color: rgba(0, 0, 0, 0.87);
}

.light .default-content :deep(p),
.light .default-content :deep(li) {
  color: rgba(0, 0, 0, 0.8);
}

.light .slide-footer {
  background: rgba(0, 134, 255, 0.05);
}

.light .hashtag {
  color: #005bcc;
}
</style>
