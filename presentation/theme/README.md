# CNCF/KubeCon Slidev Theme

A custom Slidev theme following the official CNCF and KubeCon + CloudNativeCon branding guidelines.

## Features

### Official Branding Compliance
- ✅ Uses official CNCF color palette
  - Primary: CNCF Blue (#0086FF), CNCF Black (#000000)
  - Secondary: CNCF Turquoise (#93EAFF), CNCF Pink (#D62293), CNCF Stone (#E5E5E5)
- ✅ Clarity City font family (official CNCF typeface)
- ✅ Official CNCF and KubeCon logos with proper spacing
- ✅ Follows CNCF Style Guide conventions

### Layout Safety
- Content padding ensures nothing goes off-screen
- Responsive typography with clamp() for all screen sizes
- Safe viewing area with proper margins
- Overflow protection for all content types

### Professional Design
- Clean, modern layouts (cover, center, default, end)
- Smooth transitions and animations
- Dark mode optimized
- Accessible color contrasts
- Print-friendly styles

## Usage

This theme is configured in your `slides.md` frontmatter:

```yaml
---
theme: ./theme
---
```

## Layouts

### Cover Layout
Use for title slides with KubeCon + CloudNativeCon branding:
```yaml
---
layout: cover
---
```

### Center Layout
Centered content with CNCF logo:
```yaml
---
layout: center
---
```

### Default Layout
Standard content layout with header space:
```yaml
---
layout: default
---
```

### End Layout
Thank you / closing slides:
```yaml
---
layout: end
---
```

## Customization

### Colors
All CNCF brand colors are available as CSS variables:
- `--cncf-blue`
- `--cncf-black`
- `--cncf-turquoise`
- `--cncf-pink`
- `--cncf-stone`
- `--cncf-cloud-white`

### Typography
- Headlines: Clarity City Bold
- Body: Clarity City Regular
- Code: Fira Code (monospace)

## Brand Guidelines Compliance

This theme follows:
- [CNCF Brand Guidelines](https://www.cncf.io/brand-guidelines/)
- [KubeCon + CloudNativeCon Branding Guidelines](https://www.cncf.io/kubecon-cloudnativecon-branding-guidelines/)
- [CNCF Style Guide](https://github.com/cncf/foundation/blob/main/style-guide.md)

### Key Requirements Met
- ✅ No logo modifications
- ✅ Proper logo clear space
- ✅ Official color usage only
- ✅ Clarity City typography
- ✅ "cloud native" and "open source" (no hyphen) per style guide
- ✅ Content stays within safe viewing area

## License

This theme uses official CNCF brand assets which are subject to the [Linux Foundation Trademark Usage Guidelines](https://www.linuxfoundation.org/legal/trademark-usage).
