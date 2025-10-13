<template>
  <div class="slidev-layout cncf-center">
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
    
    <!-- Main centered content -->
    <div class="center-content">
      <slot />
    </div>
    
    <!-- Footer -->
    <div class="slide-footer">
      <div class="hashtag">#KubeCon #CloudNativeCon</div>
    </div>
  </div>
</template>

<style scoped>
.cncf-center {
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

.center-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  padding: 3rem 4rem;
  max-width: 100%;
  box-sizing: border-box;
}

.center-content :deep(h1) {
  color: #0086FF;
  margin-bottom: 2rem;
  text-align: center;
}

.center-content :deep(h2) {
  margin-bottom: 1.5rem;
  text-align: center;
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
.dark .cncf-center {
  background: transparent;
}

.dark .center-content :deep(h2) {
  color: #93EAFF;
}

.dark .center-content :deep(h3),
.dark .center-content :deep(p) {
  color: #ffffff;
}

.dark .slide-footer {
  background: rgba(0, 134, 255, 0.1);
}

.dark .hashtag {
  color: #93EAFF;
}

/* Light mode styles */
.light .cncf-center {
  background: transparent;
}

.light .center-content :deep(h2) {
  color: #005bcc;
}

.light .center-content :deep(h3),
.light .center-content :deep(p) {
  color: rgba(0, 0, 0, 0.87);
}

.light .slide-footer {
  background: rgba(0, 134, 255, 0.05);
}

.light .hashtag {
  color: #005bcc;
}
</style>
