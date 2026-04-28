---
name: Ethereal Harmony
colors:
  surface: '#151310'
  surface-dim: '#151310'
  surface-bright: '#3b3936'
  surface-container-lowest: '#100e0b'
  surface-container-low: '#1d1b18'
  surface-container: '#211f1c'
  surface-container-high: '#2c2a27'
  surface-container-highest: '#373431'
  on-surface: '#e7e1dd'
  on-surface-variant: '#cfc5b8'
  inverse-surface: '#e7e1dd'
  inverse-on-surface: '#32302d'
  outline: '#989084'
  outline-variant: '#4c463c'
  surface-tint: '#d9c49c'
  primary: '#ebd5ac'
  on-primary: '#3c2f12'
  primary-container: '#ceb992'
  on-primary-container: '#58492a'
  inverse-primary: '#6c5c3c'
  secondary: '#adcfb7'
  on-secondary: '#183626'
  secondary-container: '#2f4d3b'
  on-secondary-container: '#9cbda6'
  tertiary: '#ffc8e3'
  on-tertiary: '#4c223b'
  tertiary-container: '#e7aac9'
  on-tertiary-container: '#6b3c57'
  error: '#ffb4ab'
  on-error: '#690005'
  error-container: '#93000a'
  on-error-container: '#ffdad6'
  primary-fixed: '#f7e0b7'
  primary-fixed-dim: '#d9c49c'
  on-primary-fixed: '#251a02'
  on-primary-fixed-variant: '#534526'
  secondary-fixed: '#c8ebd3'
  secondary-fixed-dim: '#adcfb7'
  on-secondary-fixed: '#022112'
  on-secondary-fixed-variant: '#2f4d3b'
  tertiary-fixed: '#ffd8ea'
  tertiary-fixed-dim: '#f3b4d4'
  on-tertiary-fixed: '#340c25'
  on-tertiary-fixed-variant: '#663852'
  background: '#151310'
  on-background: '#e7e1dd'
  surface-variant: '#373431'
typography:
  h1:
    fontFamily: Inter
    fontSize: 48px
    fontWeight: '700'
    lineHeight: '1.2'
    letterSpacing: -0.02em
  h2:
    fontFamily: Inter
    fontSize: 32px
    fontWeight: '600'
    lineHeight: '1.3'
    letterSpacing: -0.01em
  h3:
    fontFamily: Inter
    fontSize: 24px
    fontWeight: '600'
    lineHeight: '1.4'
    letterSpacing: '0'
  body-lg:
    fontFamily: beVietnamPro
    fontSize: 18px
    fontWeight: '400'
    lineHeight: '1.6'
  body-md:
    fontFamily: beVietnamPro
    fontSize: 16px
    fontWeight: '400'
    lineHeight: '1.6'
  label-sm:
    fontFamily: Inter
    fontSize: 12px
    fontWeight: '500'
    lineHeight: '1.0'
    letterSpacing: 0.05em
rounded:
  sm: 0.25rem
  DEFAULT: 0.5rem
  md: 0.75rem
  lg: 1rem
  xl: 1.5rem
  full: 9999px
spacing:
  base: 8px
  xs: 4px
  sm: 12px
  md: 24px
  lg: 40px
  xl: 64px
  gutter: 24px
  margin: 32px
---

## Brand & Style

The design system is built upon a foundation of "Melancholic Elegance." It targets a sophisticated audience that views music as an emotional sanctuary. The brand personality is introspective, calm, and premium, avoiding the high-energy neon tropes of typical streaming services in favor of a grounded, organic aesthetic.

The design style follows a **Modern Minimalist** approach with **Tonal Layering**. It prioritizes heavy whitespace (or "color-space" using the neutral base) and high-quality typography. Visual complexity is reduced to allow album art and typography to lead the user experience. The interface should feel like a well-curated digital gallery rather than a utility tool.

## Colors

The palette deviates from traditional dark modes by using a warm, desaturated parchment (`#CEB992`) as the primary canvas, creating a "dimmed light" effect that is easy on the eyes. 

- **Neutral Base:** Used for the primary background, providing a calm, organic feel.
- **Containers:** The deep slate-purple (`#585563`) creates a sophisticated contrast for cards, sidebars, and player controls.
- **Secondary Highlights:** The muted sage (`#73937E`) is reserved for success states, secondary icons, or organic growth elements like "added to library."
- **Accents:** Deep plum and burgundy tones (`#5B2E48`, `#471323`) are used sparingly for critical interactive elements, selection states, and hover feedback to maintain the "dark" emotional undertone.

## Typography

The design system employs **Inter** for headlines to provide a crisp, modern structure that feels engineered and precise. For body copy, **beVietnamPro** is utilized to introduce warmth and approachability, balancing the technical feel of the headings.

Hierarchy is established through significant scale shifts and the use of uppercase labels for utility text. To maintain the calm aesthetic, avoid using pure black for text; instead, use the deep accent colors at high opacity for primary text and the container color at lower opacity for secondary metadata.

## Layout & Spacing

This design system utilizes a **Fluid Grid** model with a focus on generous internal padding to create a sense of "breathability." 

- **Grid:** A 12-column system for desktop, 4-column for mobile.
- **Rhythm:** All spacing must be a multiple of the 8px base unit. 
- **Philosophy:** Content should never feel cramped. Use `lg` (40px) or `xl` (64px) spacing between major sections (e.g., "Recently Played" vs "Recommended Mixes") to enforce a slow, deliberate browsing pace.

## Elevation & Depth

Depth is achieved through **Tonal Layering** and **Ambient Shadows**. 

1. **Surface 0 (Base):** The primary neutral base (`#CEB992`).
2. **Surface 1 (Cards/Sidebar):** The container color (`#585563`) at 100% opacity.
3. **Surface 2 (Hover/Floating):** The container color with an ambient shadow: `0px 10px 30px rgba(71, 19, 35, 0.15)`. 

Shadows should be soft, diffused, and slightly tinted with the strong accent color to maintain a cohesive color story. Avoid hard borders; use subtle value shifts to define boundaries.

## Shapes

The design system uses **Rounded** geometry to evoke comfort and fluidity. 

- **Standard Components:** 0.5rem (8px) for buttons, small inputs, and tags.
- **Containers/Cards:** 1rem (16px) for album art containers and main content modules.
- **Full Round:** Used exclusively for play/pause buttons and user avatars to signify primary action points and identity.

## Components

### Buttons
- **Primary:** Background of `#5B2E48`, text in `#CEB992`. 8px border radius.
- **Secondary:** Ghost style with `#5B2E48` border or `#73937E` text.
- **Hover States:** Shift background to `#471323` with a subtle scale increase (1.02x).

### Cards (Album/Playlist)
- Use the 16px radius. Album art should have a very subtle inner 1px border of white at 10% opacity to separate it from dark backgrounds.
- Metadata (Title/Artist) should follow the `body-md` and `label-sm` typographic patterns.

### Navigation & Lists
- Active menu items use the `#5B2E48` accent as a soft background pill or a left-side indicator.
- List items should have generous vertical padding (16px) to ensure touch targets are accessible and the layout remains "calm."

### Input Fields
- Backgrounds should use a slightly darkened version of the neutral base or a 10% opacity version of the container color. 
- Focus states are indicated by a 2px solid border of `#73937E`.

### Audio Player
- The bottom bar should be the container color (`#585563`).
- The progress bar uses the accent `#5B2E48` for the "filled" state and a low-opacity version of the base for the "unfilled" track.