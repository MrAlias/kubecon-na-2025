# CNCF/KubeCon Slidev Theme

Custom Slidev presentation theme following official CNCF and KubeCon + CloudNativeCon branding guidelines.

## ✨ Features

- ✅ **Official CNCF Branding**: Uses exact colors, fonts, and logos from CNCF brand guidelines
- ✅ **Clarity City Font**: Official CNCF typeface for headlines and body copy
- ✅ **Logo Compliance**: Official CNCF and KubeCon logos with proper spacing
- ✅ **Safe Content Area**: Ensures all content stays within viewport during presentation
- ✅ **Responsive Design**: Scales appropriately for different display sizes
- ✅ **Style Guide Compliant**: Follows CNCF writing conventions (cloud native, open source)
- ✅ **Accessibility**: WCAG AA color contrast, reduced motion support
- ✅ **Multiple Layouts**: Cover, center, default, and end layouts included

## 🎨 Brand Colors

| Color            | Hex       | Usage                   |
| ---------------- | --------- | ----------------------- |
| CNCF Blue        | `#0086FF` | Primary brand color     |
| CNCF Black       | `#000000` | Text, backgrounds       |
| CNCF Turquoise   | `#93EAFF` | Accent, highlights      |
| CNCF Pink        | `#D62293` | Warnings, emphasis      |
| CNCF Stone       | `#E5E5E5` | Secondary text          |
| CNCF Cloud White | `#FFFFFF` | Backgrounds, light text |

## 📖 Documentation

- **[THEME_SETUP.md](./THEME_SETUP.md)** - Complete setup and usage guide
- **[QUICK_REFERENCE.md](./theme/QUICK_REFERENCE.md)** - Quick reference for common tasks
- **[COMPLIANCE.md](./theme/COMPLIANCE.md)** - Brand compliance checklist
- **[README.md](./README.md)** - Theme overview

## 🚀 Quick Start

1. Theme is already configured in `slides.md`:

   ```yaml
   ---
   theme: ./theme
   ---
   ```

2. Install dependencies:

   ```bash
   npm install
   ```

3. Start development server:

   ```bash
   npm run dev
   ```

## 📁 Theme Structure

```console
theme/
├── layouts/           # Vue layout components
│   ├── cover.vue     # Title slide layout
│   ├── center.vue    # Centered content layout
│   ├── default.vue   # Standard content layout
│   └── end.vue       # Thank you slide layout
├── styles/           # CSS stylesheets
│   ├── index.css     # Main entry point
│   ├── fonts.css     # Clarity City font loading
│   ├── cncf-colors.css   # Brand color variables
│   ├── typography.css    # Text styles
│   ├── layout.css    # Layout and spacing
│   └── components.css    # Reusable components
├── setup/
│   └── fonts.ts      # Font configuration
├── index.ts          # Theme entry point
└── package.json      # Theme metadata
```

## 🎯 Key Guidelines

### Logo Usage

- ✅ Always use official logos from CNCF artwork repository
- ✅ Maintain proper clear space (1rem minimum)
- ❌ Never modify, stretch, or recolor logos
- ❌ Never separate "KubeCon" from "CloudNativeCon"

### Typography

- Headlines: **Clarity City Bold** (700)
- Body: **Clarity City Regular** (400)
- Code: **Fira Code** (monospace)

### Writing Style

Per [CNCF Style Guide](https://github.com/cncf/foundation/blob/main/style-guide.md):

- "cloud native" (no hyphen)
- "open source" (no hyphen)
- "Kubernetes" (proper capitalization)
- "K8s" (preferred abbreviation)

## 🔗 Official Resources

- [CNCF Brand Guidelines](https://www.cncf.io/brand-guidelines/)
- [KubeCon Branding Guidelines](https://www.cncf.io/kubecon-cloudnativecon-branding-guidelines/)
- [CNCF Style Guide](https://github.com/cncf/foundation/blob/main/style-guide.md)
- [CNCF Artwork Repository](https://github.com/cncf/artwork)
- [Clarity City Font](https://github.com/vmware/clarity-city)

## 📄 License

This theme uses official CNCF brand assets subject to the [Linux Foundation Trademark Usage Guidelines](https://www.linuxfoundation.org/legal/trademark-usage).

**Attribution Required**:

> "CNCF and the CNCF logo design are registered trademarks of the Cloud Native Computing Foundation."

## 🤝 Contributing

This is a presentation-specific theme. For issues or suggestions:

1. Check brand guidelines for compliance
2. Test changes across all layouts
3. Ensure content stays within safe viewing area
4. Verify accessibility standards

---

**Version**: 1.0.0  
**Last Updated**: October 13, 2025  
**Compatibility**: Slidev 0.48.0+  
**Status**: ✅ Fully compliant with CNCF brand guidelines
