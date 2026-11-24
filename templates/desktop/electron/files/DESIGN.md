---
version: "alpha"
name: Smokeforge Desktop
description: "The shared Smokeforge palette applied to a calm desktop control surface: charcoal ink, cream paper, tide teal, and a rare coral signal."
colors:
  cream: "#fbf8f3"
  charcoal: "#313436"
  ink: "#272727"
  tide: "#2fabaa"
  coral: "#ff7448"
  card: "#fffdf9"
  sand: "#e7e0d5"
  mist: "#f1ece3"
  bark: "#6f6b64"
  success: "#0f766e"
  warning: "#b45309"
  info: "#0369a1"
typography:
  body:
    fontFamily: "ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif"
    fontSize: "0.875rem"
    fontWeight: 400
    lineHeight: 1.5
  title:
    fontFamily: "ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif"
    fontSize: "1.125rem"
    fontWeight: 600
    lineHeight: 1.3
    letterSpacing: "-0.01em"
  data:
    fontFamily: "ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif"
    fontSize: "1.875rem"
    fontWeight: 600
    lineHeight: 1.1
    letterSpacing: "-0.02em"
rounded:
  control: "8px"
  card: "12px"
  shell: "0px"
spacing:
  page: "24px"
  section: "20px"
  control: "8px"
components:
  button-primary:
    backgroundColor: "{colors.charcoal}"
    textColor: "{colors.cream}"
    rounded: "{rounded.control}"
    padding: "0 14px"
  card:
    backgroundColor: "{colors.card}"
    textColor: "{colors.charcoal}"
    rounded: "{rounded.card}"
    padding: "20px"

---

# Design System: Smokeforge Desktop

## Overview

**Creative North Star: "The Instrument Panel"**

The desktop shell applies the web palette to a well-labeled instrument panel:
quiet enough to stay in the background, precise enough to trust with local data.
Cream paper keeps records and forms legible, charcoal carries primary action,
and the ink sidebar anchors navigation. Tide teal marks selection and focus;
coral appears rarely as a human signal.

- **Density:** 6/10 — productive, not cramped
- **Variance:** 4/10 — disciplined shell with one resizable content edge
- **Motion:** 2/10 — short route and feedback transitions only

## Palette source of truth

- Renderer colors live in `src/renderer/src/styles/globals.css` as semantic CSS
  variables and Tailwind `@theme` mappings.
- Components use semantic utilities such as `bg-background`, `text-muted-foreground`,
  `border-success/30`, `bg-warning/10`, and `bg-chart-1`.
- No component file may contain a raw hex, RGB, HSL, or named Tailwind palette
  class for product color.
- Native window chrome cannot read CSS, so its two colors live in
  `src/shared/design-tokens.ts` and must be changed together with the CSS ink
  tokens.

## Colors

- **Cream:** `#fbf8f3` light work surface and inverse text.
- **Charcoal:** `#313436` primary action, body copy, and sidebar accent.
- **Ink:** `#272727` light sidebar and dark section ground.
- **Tide:** `#2fabaa` selection, focus, links, and the primary chart series.
- **Coral:** `#ff7448` rare signal, spark, and second chart series.
- **Sand:** `#e7e0d5` borders and quiet surfaces.
- **Mist:** `#f1ece3` muted fills.
- **Bark:** `#6f6b64` secondary copy.
- **Success:** deep tide for active, online, and update-ready states.
- **Warning:** amber for draft, offline, and update-available states.
- **Info:** sky for neutral system information.

**The Signal Reserve Rule.** Tide is selection and focus. Coral is a rare human
signal. Green, amber, and sky only communicate system state; never use them
decoratively.

## Typography

The app uses the native UI sans stack so it feels at home on every desktop; the
brand palette, not the web display font, carries continuity with the web app.
Titles use 600 weight and slightly tightened tracking; data uses a 600 weight at
30px. Labels stay sentence case. Avoid all-caps labels and decorative eyebrow
text.

## Layout

The window is a fixed shell with one scrolling content region:

- 38px draggable title bar with traffic lights or native overlay controls.
- 220px expanded sidebar, 60px collapsed rail.
- Page header with title, one-line description, and at most one primary action.
- Content uses a 24px gutter and 20px section rhythm.
- Library uses a resizable list/detail split; the inspector is not a modal.
- Status bar is 32px and carries connection, job, update, and version state.

## Components

- Use generated shadcn/ui primitives. Do not hand-write buttons, dialogs,
  inputs, selects, switches, or commands.
- Cards are containers for real information, not the default for every block.
- Tables use quiet borders, compact row rhythm, and a selected-row background.
- Destructive actions are text buttons or icon buttons, never a red card.
- Empty states explain the next action.
- Toasts are bottom-right and short; use action toasts only for undo.
- App-specific primitives beside shadcn live in `components/ui` too:
  `WizardWindow` (floating multi-step window), `FeatureRow`, `Segmented`
  (2–4 options), `ValueStepper` (− value +), `Inset` (`quiet` / one `callout`
  per page). See `docs/UI.md` › Welcome tour.
- Interface size scales the whole page (`zoom` setting); never change tokens
  to make "everything bigger".

## Motion

- Route content fades and rises 4px over 160ms with `ease-out`.
- Sidebar width and resizable panes transition only through their layout
  primitives or CSS width, never a spring.
- Respect reduced motion; do not animate idle content, charts, or table rows.
- No gradients as decoration. No glassmorphism. No neon glows.

## Accessibility

- Every action is keyboard reachable; focus rings use `--ring`.
- Do not remove native title-bar controls.
- Keep touch and click targets at least 32px in dense desktop surfaces.
- Verify at 760px minimum window width and at 1440px wide.
- Status must be conveyed with text as well as color.

## Do's and Don'ts

### Do

- Pull colors from CSS variables and semantic Tailwind classes.
- Keep the sidebar dark in both themes; it anchors the application.
- Use charcoal for primary action and tide for selection and focus.
- Keep coral rare; one signal per view at most.
- Use one primary action per page header.
- Let Library rows carry density; give forms and inspector fields room.

### Don't

- Don't use raw hex values in components or one-off color classes.
- Don't use purple gradients, neon, or glass panels.
- Don't turn every section into a same-sized card grid.
- Don't put a rounded icon tile above every heading.
- Don't use uppercase navigation or section labels.
