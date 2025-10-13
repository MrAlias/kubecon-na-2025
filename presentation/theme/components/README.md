# CNCF Theme Components

## CncfQRCode Component

A custom QR code component that automatically adapts to light and dark color schemes.

### Features

- **Theme-aware**: Automatically switches between white (dark mode) and black (light mode) QR codes
- **Responsive**: Configurable size and margin
- **Live updates**: Watches for theme changes and re-renders automatically
- **Clean styling**: Includes subtle background for better scannability

### Usage

```vue
<CncfQRCode 
  value="https://your-url.com" 
  :width="200"
  :height="200"
  :margin="2"
/>
```

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `string` | **required** | The URL or text to encode in the QR code |
| `width` | `number` | `300` | Width of the QR code in pixels |
| `height` | `number` | `300` | Height of the QR code in pixels |
| `margin` | `number` | `0` | Margin around the QR code |
| `type` | `'svg' \| 'canvas'` | `'svg'` | Rendering type |
| `errorCorrectionLevel` | `'L' \| 'M' \| 'Q' \| 'H'` | `'Q'` | Error correction level |

### Dark Mode Behavior

- **Foreground**: White (`#FFFFFF`)
- **Background**: Transparent
- **Container Background**: `rgba(255, 255, 255, 0.1)`

### Light Mode Behavior

- **Foreground**: Black (`#000000`)
- **Background**: White (`#FFFFFF`)
- **Container Background**: `rgba(0, 0, 0, 0.05)`

### Example

```markdown
---
layout: center
---

# Scan to Learn More

<CncfQRCode 
  value="https://opentelemetry.io/docs" 
  :width="250"
  :height="250"
  :margin="2"
/>
```

### Implementation Details

Based on the `qr-code-styling` library with custom Vue wrapper that:
- Monitors DOM changes to detect theme switches
- Automatically re-renders when the theme changes
- Uses computed properties to determine current color scheme
- Provides proper CNCF-branded styling

### Dependencies

- `qr-code-styling` - Already installed via `slidev-addon-qrcode`
- Vue 3 Composition API
