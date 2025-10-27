# CNCF/KubeCon Theme - Quick Reference

## 🎨 Color Palette

```css
--cncf-blue: #0086ff /* Primary brand color */ --cncf-black: #000000
  /* Primary text */ --cncf-turquoise: #93eaff /* Accent color */
  --cncf-pink: #d62293 /* Highlight color */ --cncf-stone: #e5e5e5
  /* Secondary text */ --cncf-white: #ffffff /* Background */;
```

## 📝 Typography

| Element | Font         | Weight        | Size          |
| ------- | ------------ | ------------- | ------------- |
| H1      | Clarity City | Bold (700)    | 2-3.5rem      |
| H2      | Clarity City | Bold (700)    | 1.5-2.5rem    |
| H3      | Clarity City | Bold (700)    | 1.25-1.875rem |
| Body    | Clarity City | Regular (400) | 1-1.25rem     |
| Code    | Fira Code    | Regular       | 0.9em         |

## 📐 Layouts

### Cover (Title Slide)

```yaml
---
layout: cover
---
```

- KubeCon logo (top left)
- CNCF logo (footer)
- Dark gradient background

### Center

```yaml
---
layout: center
---
```

- CNCF icon (top right)
- Centered content
- Flexible height

### Default

```yaml
---
layout: default
---
```

- CNCF icon (top right)
- Left-aligned content
- Hashtag footer

### End (Thank You)

```yaml
---
layout: end
---
```

- KubeCon logo (top left)
- CNCF logo (footer)
- Centered content

## 🧩 Components

### Info Boxes

```html
<div class="info-box">Blue accent box</div>
<div class="warning-box">Pink accent box</div>
<div class="success-box">Turquoise accent box</div>
```

### Buttons

```html
<button class="cncf-button">Primary</button>
<button class="cncf-button-secondary">Secondary</button>
```

### Grids

```html
<div class="two-column">
  <div>Left</div>
  <div>Right</div>
</div>

<div class="three-column">
  <div>1</div>
  <div>2</div>
  <div>3</div>
</div>
```

### Hashtags

```html
<span class="hashtag">#KubeCon #CloudNativeCon</span>
```

## ✍️ Writing Style (CNCF Style Guide)

| ✅ Correct               | ❌ Incorrect           |
| ------------------------ | ---------------------- |
| cloud native             | cloud-native           |
| open source              | open-source            |
| Kubernetes               | kubernetes, KUBERNETES |
| K8s                      | K8, k8s, K8S           |
| KubeCon + CloudNativeCon | KubeCon, Kubecon       |
| OpenTelemetry            | Open Telemetry         |
| eBPF                     | EBPF, ebpf             |

## 🖼️ Logo Guidelines

### CNCF Logo

- ✅ Use official SVG from GitHub
- ✅ Maintain clear space (1rem padding)
- ✅ Available: color, black, white
- ❌ Don't modify, stretch, or recolor

### KubeCon Logo

- ✅ Always use full name
- ✅ Don't separate KubeCon from CloudNativeCon
- ✅ Use official conference year logo
- ❌ Never use "KubeCon" alone

## 📏 Safe Content Area

```diagram
┌────────────────────────────┐
│  3rem margin left/right    │
│  2rem margin top/bottom    │
│                            │
│  ┌──────────────────────┐ │
│  │                      │ │
│  │   Safe Content Area  │ │
│  │                      │ │
│  └──────────────────────┘ │
│                            │
└────────────────────────────┘
```

## 🎯 Quick Commands

```bash
# Development
npm run dev

# Build
npm run build

# Export PDF
npm run export

# Install dependencies
npm install
```

## 🔧 Common Customizations

### Custom Color Text

```html
<span style="color: var(--cncf-blue)">Blue text</span>
```

### Custom Background

```html
<div style="background: var(--cncf-turquoise)">Content</div>
```

### Responsive Image

```html
<img src="path/to/image" style="max-width: 100%; height: auto;" />
```

## 📱 Responsive Breakpoints

| Size    | Max Width | Font Adjustment |
| ------- | --------- | --------------- |
| Mobile  | 768px     | Reduced 10-20%  |
| Tablet  | 1024px    | Standard        |
| Desktop | 1920px+   | Clamped max     |

## ⚡ Performance Tips

1. **Optimize images** before adding
2. **Use SVG** for logos and icons
3. **Lazy load** heavy content with v-click
4. **Minimize** custom JavaScript
5. **Test** on target display resolution

## 🎬 Animation Classes

```html
<!-- Fade in -->
<div v-click>Content</div>

<!-- Multiple clicks -->
<div v-click="1">First</div>
<div v-click="2">Second</div>

<!-- Click range -->
<div v-click="[1, 3]">Visible from click 1-3</div>
```

## 🌐 Browser Support

- ✅ Chrome/Edge 90+
- ✅ Firefox 88+
- ✅ Safari 14+
- ⚠️ IE not supported

## 📞 Support

- **Theme Issues**: Check THEME_SETUP.md
- [**Slidev Questions**](https://sli.dev/)
- [**Brand Guidelines**](https://www.cncf.io/brand-guidelines/)
- [**CNCF Style**](https://github.com/cncf/foundation/blob/main/style-guide.md)

---

**Quick Print**: Save this file or print for quick reference during development!
