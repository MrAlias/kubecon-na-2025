# CNCF/KubeCon Dynamic Theme Features

## Color Scheme Adaptability

The theme now fully supports dynamic switching between `dark`, `light`, and `both` color schemes. Simply change the `colorSchema` value in your slides.md frontmatter:

```yaml
---
colorSchema: dark  # or 'light' or 'both'
---
```

## Dark Mode Features

### Background
- Solid color: `#0a0f1a` (dark navy)
- Subtle static radial gradient overlays with CNCF colors (blue, turquoise, pink)
- Overlays set at 50% opacity for visual interest without distraction

### Colors
- **Primary text**: `#ffffff` (white)
- **Secondary text**: `rgba(255, 255, 255, 0.9)`
- **Headings (H2)**: `#93EAFF` (CNCF Turquoise)
- **Headings (H3)**: `#ffffff` (white)
- **Links**: `#93EAFF` (turquoise), hover: `#0086FF` (blue)
- **Code backgrounds**: `rgba(0, 134, 255, 0.1)` with turquoise text
- **Footer backgrounds**: `rgba(0, 134, 255, 0.1)`

### Accessibility
- High contrast ratios (WCAG AAA compliant)
- Distinct visual hierarchy
- Clear hover states

## Light Mode Features

### Background
- Solid color: `#f8f8f8` (light gray)
- Subtle static radial gradient overlays with CNCF colors (reduced opacity)
- Maintains visual interest without animation overhead

### Colors
- **Primary text**: `#000000` (black)
- **Secondary text**: `rgba(0, 0, 0, 0.87)`
- **Headings (H2)**: `#005bcc` (darker blue for contrast)
- **Headings (H3)**: `rgba(0, 0, 0, 0.87)`
- **Links**: `#0086FF` (CNCF blue), hover: `#D62293` (pink)
- **Code backgrounds**: `rgba(0, 134, 255, 0.08)` with dark blue text
- **Footer backgrounds**: `rgba(0, 134, 255, 0.05)`

### Accessibility
- Minimum 4.5:1 contrast ratio for body text
- 3:1 for large text and UI components
- Softer colors to reduce eye strain in bright environments

## Shared Features

### Static Backgrounds
- Solid color backgrounds for optimal performance
- Subtle radial gradient overlays for visual depth
- No CPU-intensive animations
- Respects `prefers-reduced-motion` media query for accessibility

### Component Boxes
- **Info boxes**: Blue-themed, clear border-left
- **Warning boxes**: Pink-themed, attention-grabbing
- **Success boxes**: Turquoise/blue-themed, positive feedback
- All boxes have proper contrast in both modes

### Navigation
- Visible navigation buttons in both modes
- Color-coded hover states
- Progress bar always uses CNCF blue (`#0086FF`)

### Typography
- Clarity City font throughout
- Responsive sizing with clamp()
- Proper line-height for readability
- Consistent spacing and hierarchy

## CNCF Brand Compliance

### Colors Used
- **CNCF Blue**: `#0086FF` - Primary actions, links, accents
- **CNCF Turquoise**: `#93EAFF` - Highlights, hover states (dark mode)
- **CNCF Pink**: `#D62293` - Warnings, alternate highlights
- **CNCF Stone**: `#E5E5E5` - Metadata, secondary text
- **CNCF Black/White**: Context-dependent backgrounds

### Logo Usage
- Official CNCF and KubeCon logos from GitHub artwork repository
- Proper clear space maintained around logos
- Consistent sizing across layouts
- Never distorted or recolored

### Typography Standards
- Clarity City as primary typeface (CNCF official)
- Appropriate font weights: 400, 500, 700
- Hierarchy follows brand guidelines
- Consistent spacing following 8px grid

## Accessibility Standards

### WCAG Compliance
- **Level AAA** for critical content in dark mode
- **Level AA** minimum for all content
- Proper focus indicators on interactive elements
- Sufficient color contrast ratios

### Motion Sensitivity
- Respects `prefers-reduced-motion` media query
- Animations can be disabled system-wide
- Fallback to instant transitions when needed

### Screen Reader Support
- Semantic HTML structure
- Proper alt text on all images
- Logical tab order
- ARIA labels where appropriate

## Testing Checklist

- [x] Dark mode background visible and attractive
- [x] Light mode background visible with sufficient contrast
- [x] All text readable in both modes
- [x] Links clearly distinguishable
- [x] Code blocks readable in both modes
- [x] Component boxes have proper contrast
- [x] Logos visible in both modes
- [x] Static backgrounds performant and visually appealing
- [x] Footer hashtags visible in both modes
- [x] 4:3 aspect ratio maintained
- [x] Content stays within safe viewing area
- [x] Reduced motion preferences honored

## Usage Examples

### Switching to Light Mode
Edit your slides.md:
```yaml
---
colorSchema: light
---
```

### Using Component Boxes
```markdown
::: info-box
This is important information that adapts to the color scheme.
:::

::: warning-box
Pay attention to this warning!
:::

::: success-box
Everything is working correctly!
:::
```

### Custom Styling
All CNCF colors are available as CSS variables:
```css
color: var(--cncf-blue);
background: var(--cncf-turquoise);
border-color: var(--cncf-pink);
```

## Browser Compatibility

Tested and working in:
- Chrome/Edge (Chromium)
- Firefox
- Safari
- Presenter view
- Export to PDF

## Performance

- Static backgrounds with no CPU overhead
- Subtle visual effects using opacity and static gradients
- Optimized for battery life and system resources
- Instant rendering with no animation processing
