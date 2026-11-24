# Design direction

## Tone

Calm, technical, and native. Use system SwiftUI controls and Material-free
iOS conventions. The product should feel like a trustworthy tool.

## Visual system

- Use `SmokeColor` and `SmokeTheme` from DesignSystem.
- Signal blue is for primary actions and status; ink is the dark surface.
- Prefer system typography with Dynamic Type.
- Use native navigation, tab, form, and button behaviour.
- Do not introduce custom chrome that fights system navigation or Liquid Glass.
- Use edge-to-edge content with safe-area-aware layouts.

## Accessibility

- Every actionable icon needs a label.
- Touch targets are at least 44pt.
- Support VoiceOver, Dynamic Type, Reduce Motion, and color differentiation.
- Keep error copy specific and actionable.
- Do not rely on color alone for status.

## Content

- Use sentence case.
- Keep auth copy direct and reassuring.
- No analytics or tracking in the base.
