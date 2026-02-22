---
name: color-matching
description: Applies the 60-30-10 Blue-Purple-Orange color palette and modern UX accessibility rules.
---
# Color Matching & UI Palette Strategy

This document outlines the core color strategy for the project, grounded in the 60-30-10 design rule and modern UX best practices.

## Core Palette

Our brand identity relies on a tri-color system supported by a robust grayscale foundation.

| Role | Color Family | Example Hex | Primary Use Case |
| :--- | :--- | :--- | :--- |
| **Primary** | Blue | `#3187d1` | Brand identity, primary navigation, active states. |
| **Secondary** | Purple | `#8a31b3` | Supporting elements, secondary buttons, cards, visual interest. |
| **Accent** | Orange | `#e77a20` | Call-to-Action (CTA) buttons, notifications, highlight badges. |

### The Grayscale Foundation
Grayscale colors are used for backgrounds, text, and borders to ensure the primary palette stands out without overwhelming the user.

| Shade | Example Hex | Usage |
| :--- | :--- | :--- |
| **Light** | `#F8FAFC` | App backgrounds, card surfaces (Light Mode). |
| **Mid** | `#94A3B8` | Borders, disabled states, placeholder text. |
| **Dark** | `#0F172A` | Primary text, headings, backgrounds (Dark Mode). |

---

## Application: The 60-30-10 Rule



To maintain visual balance, colors are distributed across the UI using the 60-30-10 proportion:

* **60% - The Foundation (Backgrounds & Primary Anchor):** Dominant space is occupied by the grayscale backgrounds and subtle, desaturated tints of the **Blue** primary color. This creates a calm, usable canvas.
* **30% - The Structure (Secondary Forms):** The **Purple** and bolder **Blue** shades are used to break up space, guide the eye, and establish hierarchy (e.g., sidebars, secondary buttons, headers, or active menu items).
* **10% - The Spark (Accent):** The **Orange** is strictly reserved for critical conversion points. It must be used sparingly to draw the eye immediately to primary CTAs (e.g., "Sign Up," "Buy Now," or unread notification dots).

---

## Modern UX Considerations

To ensure our application is inclusive, usable, and modern, the following rules apply to our color implementation:

### 1. Accessibility & Contrast (WCAG Compliance)
* All text and essential UI elements must meet a minimum contrast ratio of **4.5:1** against their background (WCAG AA). 
* Avoid placing white text on light Orange or light Purple; use dark text or deepen the background shade to ensure readability.
* Color is never the *only* visual means of conveying information. Always pair color changes with text labels or iconography.

### 2. Dark Mode Adaptation
* **Avoid Pure Black:** Use deep, cool grays (e.g., `#0F172A`) for dark mode backgrounds to reduce eye strain.
* **Desaturate Accents:** The vibrant Blue, Purple, and Orange used in Light Mode must be slightly desaturated and brightened in Dark Mode to prevent them from vibrating against dark backgrounds and causing visual fatigue.

### 3. Semantic System Colors

Do not mix the brand palette with system feedback colors. These colors have distinct, universal meanings:
* **Success:** Green (e.g., Form saved, payment complete)
* **Error:** Red (e.g., Invalid input, destructive actions)
* **Warning:** Yellow (e.g., Irreversible action ahead)
* **Info:** Standard Blue (Can overlap with the primary brand blue if styled distinctly, usually with lighter backgrounds).

### 4. Interactive States
Every interactive element must have a defined color shift to provide immediate user feedback:
* **Hover:** 10% darker or lighter than the base color.
* **Active/Pressed:** 20% darker than the base color.
* **Focus Rings:** Essential for keyboard navigation. A highly visible ring (often using the Primary Blue or Accent Orange) must appear around focused elements.
* **Disabled:** Shift to Mid-Grayscale (`#94A3B8`) and reduce opacity to 50% to indicate inactivity.