# CNCF/KubeCon Theme Setup Guide

This guide will help you set up and use the custom CNCF/KubeCon branded Slidev theme.

## Installation

1. **Install dependencies** (if not already done):
```bash
npm install
```

2. **The theme is already configured** in your `slides.md`:
```yaml
---
theme: ./theme
---
```

## Running the Presentation

### Development Mode
```bash
npm run dev
```

This will start Slidev in development mode with hot reload.

### Build for Production
```bash
npm run build
```

This creates a production-ready static site in `dist/`.

### Export to PDF
```bash
npm run export
```

## Theme Features

### Official CNCF Branding
- ✅ Official CNCF color palette
- ✅ Clarity City font (official CNCF typeface)
- ✅ Official logos from CNCF artwork repository
- ✅ Follows all brand guidelines

### Layout Options

#### 1. Cover Layout (Title Slide)
```markdown
---
layout: cover
---

# Your Title

**Presenter:** Your Name

<div slot="meta">
  <div>Event details</div>
</div>
```

#### 2. Center Layout
```markdown
---
layout: center
---

# Centered Content

Content is automatically centered.
```

#### 3. Default Layout
```markdown
---
layout: default
---

# Standard Slide

Left-aligned content with CNCF branding.
```

#### 4. End Layout (Thank You)
```markdown
---
layout: end
---

# Thank You!

Contact information here.
```

## Styling Options

### Using CNCF Colors

In your markdown or Vue components:

```html
<div class="cncf-blue">CNCF Blue text</div>
<div class="bg-cncf-turquoise">Turquoise background</div>
```

### Info Boxes

```html
<div class="info-box">
  Important information with blue accent
</div>

<div class="warning-box">
  Warning or caution with pink accent
</div>

<div class="success-box">
  Success message with turquoise accent
</div>
```

### Buttons

```html
<button class="cncf-button">Primary Button</button>
<button class="cncf-button-secondary">Secondary Button</button>
```

### Two-Column Layouts

```html
<div class="two-column">
  <div>Left column content</div>
  <div>Right column content</div>
</div>
```

### Three-Column Layouts

```html
<div class="three-column">
  <div>Column 1</div>
  <div>Column 2</div>
  <div>Column 3</div>
</div>
```

## Available CSS Variables

```css
/* Primary Colors */
--cncf-blue: #0086FF
--cncf-black: #000000

/* Secondary Colors */
--cncf-turquoise: #93EAFF
--cncf-pink: #D62293
--cncf-stone: #E5E5E5
--cncf-cloud-white: #FFFFFF
```

## Typography

### Headlines
Headlines automatically use **Clarity City Bold**:
```markdown
# H1 Headline
## H2 Headline
### H3 Headline
```

### Body Text
Body text uses **Clarity City Regular** automatically.

### Code Blocks
Code blocks use **Fira Code** with syntax highlighting:

```javascript
const hello = 'world';
```

## Best Practices

### 1. Content Safety
- Keep text within the slide margins
- Use responsive font sizes (already configured)
- Test on different screen sizes

### 2. Logo Usage
- Don't modify logos
- Logos are automatically placed in layouts
- Maintain proper spacing (built-in)

### 3. Writing Conventions
Per CNCF Style Guide:
- ✅ "cloud native" (no hyphen)
- ✅ "open source" (no hyphen)
- ✅ "Kubernetes" (proper capitalization)
- ✅ "K8s" (preferred abbreviation)
- ✅ "KubeCon + CloudNativeCon" (full name)

### 4. Accessibility
- Use sufficient color contrast (built-in)
- Provide alt text for images
- Use semantic HTML structure
- Test with reduced motion settings

## Customization

### Adding Custom Styles
Create a file in `styles/custom.css`:

```css
/* Your custom styles */
.my-custom-class {
  color: var(--cncf-blue);
}
```

Then import it in `theme/styles/index.css`:
```css
@import './custom.css';
```

### Creating Custom Layouts
Create a new Vue file in `theme/layouts/`:

```vue
<template>
  <div class="slidev-layout my-layout">
    <slot />
  </div>
</template>

<style scoped>
.my-layout {
  /* Your layout styles */
}
</style>
```

## Troubleshooting

### Fonts Not Loading
If Clarity City doesn't load, check your internet connection. The fonts are loaded from CDN.

### Logos Not Showing
Verify you have internet access to load logos from the CNCF GitHub repository.

### Content Overflowing
- Use the built-in layout classes
- Check for hardcoded widths/heights
- Use responsive units (rem, %, vw, vh)

### Build Errors
```bash
# Clear cache and rebuild
rm -rf .slidev dist node_modules/.vite
npm run build
```

## Support & Resources

### Documentation
- [Slidev Documentation](https://sli.dev/)
- [CNCF Brand Guidelines](https://www.cncf.io/brand-guidelines/)
- [KubeCon Branding Guidelines](https://www.cncf.io/kubecon-cloudnativecon-branding-guidelines/)
- [CNCF Style Guide](https://github.com/cncf/foundation/blob/main/style-guide.md)

### Assets
- [CNCF Artwork Repository](https://github.com/cncf/artwork)
- [Clarity City Font](https://github.com/vmware/clarity-city)

## Examples

See the included `slides.md` for a complete example presentation using all theme features.

## License & Attribution

This theme uses official CNCF brand assets subject to the [Linux Foundation Trademark Usage Guidelines](https://www.linuxfoundation.org/legal/trademark-usage).

When using this theme, please include appropriate attribution:
> "CNCF and the CNCF logo design are registered trademarks of the Cloud Native Computing Foundation."

---

**Theme Version**: 1.0.0  
**Last Updated**: October 13, 2025  
**Slidev Compatibility**: 0.48.0+
