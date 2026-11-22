# Design direction

## Tone

Calm, technical, and direct. The interface should feel like a trustworthy tool:
high contrast, restrained colour, clear hierarchy, and no decorative noise.

## Material 3

- Use `SmokeTheme` and the shared tokens in `:core:designsystem`.
- Use Material 3 components before custom primitives.
- Support light and dark themes. Dynamic colour is intentionally off so the
  product keeps its identity.
- Use edge-to-edge layout and window insets.
- Use adaptive spacing for phone and tablet widths.
- Keep primary actions full-width and easy to reach.

## Accessibility

- Every interactive icon needs a content description or an explicit null when
  the adjacent text already names the action.
- Touch targets are at least 48dp.
- Text must survive font scaling.
- Test the auth, home, and settings screens with TalkBack and large text.
- Never rely on colour alone to communicate errors or status.

## Content

- Use sentence case.
- Keep error copy actionable and specific.
- No analytics or tracking calls are part of the base.
