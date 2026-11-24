---
version: "2"
name: Smokeforge desktop
description: "Quiet, precise desktop UI: neutral surfaces, one blue accent, Inter, light and dark themes with a high-contrast variant."
themes: [light, dark, high-contrast variants of each]
source_of_truth: crates/app/src/theme.rs
typography:
  family: "Inter (embedded, assets/fonts)"
  small: "12px regular"
  body: "14px regular"
  button: "14px regular"
  heading: "15px semibold"
  title: "22px semibold"
  display: "28px semibold"
  monospace: "13px"
spacing: [4, 8, 12, 16, 24, 32]
radius: { sm: 4, md: 6, lg: 10 }
size: { control: 32, title_bar: 44, status_bar: 30, sidebar: 220, sidebar_collapsed: 52, icon: 16 }
---

# Design system

The look follows one idea: **content and hierarchy carry the interface;
decoration supports them.** Neutral surfaces, hairline borders, one accent
colour, a single UI font, and generous but consistent spacing. Colour is spent
only where it changes what the user notices or does.

Everything visual comes from `crates/app/src/theme.rs` (tokens) and
`crates/app/src/icons.rs` (icons). Components in `crates/app/src/components/`
are the only code that turns tokens into pixels; pages compose components.

## Layers of the system

| Layer | File | Owns | Never contains |
| --- | --- | --- | --- |
| Tokens | `theme.rs` | palettes, type scale, spacing, radii, shadows, fonts, egui `Visuals`/`Style` | widgets, layout |
| Icons | `icons.rs` | every icon, drawn in code on a 16×16 grid | colours (callers pass one) |
| Components | `components/*.rs` | reusable, presentational widgets | database, API, `Action` dispatch |
| Shell | `shell.rs` | title bar, sidebar, status bar, tabs, command palette, confirmations | page content |
| Pages | `views/*.rs` | one page each: data loading, composition, dispatching one `Action` | raw colours, custom painting |
| States | `components/{status_page,notice,confirm_dialog,empty_state}.rs`, `startup.rs` | error, empty, not-found, offline, confirmation screens | business logic |

## Colour

Feature code reads colours with `theme::palette(ui.ctx())`. **No
`Color32::from_rgb` outside `theme.rs`.** The palette roles:

| Role | Light | Dark | Use |
| --- | --- | --- | --- |
| `canvas` | `#F5F5F6` | `#161619` | workspace behind cards |
| `surface` | `#FFFFFF` | `#1F1F23` | title bar, sidebar, status bar, cards |
| `surface_raised` | `#FFFFFF` | `#27272C` | menus, popovers, dialogs |
| `field` | `#F4F4F5` | `#27272C` | text fields, filled controls at rest |
| `hover` / `pressed` | `#ECECEE` / `#E2E2E5` | `#2C2C32` / `#35353C` | interaction states |
| `selected` | `#E8F0FE` | `#1E2C48` | selected rows, active navigation |
| `border` / `border_strong` | `#E4E4E7` / `#D1D1D6` | `#2F2F35` / `#44444C` | hairlines / hovered outlines |
| `text` / `text_secondary` / `text_tertiary` | `#18181B` / `#5C5C63` / `#7C7C84` | `#F4F4F5` / `#A6A6AF` / `#80808A` | body / supporting / hints |
| `accent` / `accent_hover` / `on_accent` | `#2563EB` / `#1D4ED8` / white | same | the primary action |
| `accent_text` / `focus` | `#1D4ED8` / `#2563EB` | `#6BA6FF` / `#6BA6FF` | links, active icons / focus ring |
| `success` / `warning` / `danger` | `#166534` / `#B45309` / `#C81E1E` | `#4ADE80` / `#FBBF24` / `#F87171` | status text |
| `danger_fill` | `#DC2626` | `#DC2626` | destructive fills (white text) |

The GPUI template uses the same values, so apps built on either stack look
like one family.

Rules:

- **One primary action per area.** `ButtonKind::Primary` (accent fill) marks
  the default commit; everything else is `Secondary`, `Ghost`, or `Danger`.
- **Status colour carries meaning, not decoration.** Badges are `Tone::Neutral`
  unless the state truly is success, warning, danger, or informational.
  Archived and draft records are neutral; active records are success.
- **Never colour alone.** Every coloured state also has text or an icon.
- **High contrast** (`Contrast::High`) darkens/lightens text, borders, and focus
  for every theme. Do not special-case it in components; read the palette.

Contrast is enforced by tests in `theme.rs` (WCAG AA 4.5:1 for text on every
surface it lands on, 3:1 for tertiary text, focus rings, and large text) and in
`components/badge.rs` (badge text on its tint). **If you change a colour, run
`cargo test` — a failing contrast test means the colour is wrong, not the test.**

## Typography

Inter regular and semibold are embedded (`assets/fonts`, OFL). Use the five
`TextStyle`s; do not call `.size()` on text in feature code.

| Style | Size | Use |
| --- | --- | --- |
| `theme::title_style()` | 22 semibold | page title (one per page, via `PageHeader`) |
| `TextStyle::Heading` | 15 semibold | section titles (via `section`) |
| `TextStyle::Body` / `Button` | 14 | everything else |
| `TextStyle::Small` | 12 | captions, helper text, badges, status bar |
| `theme::display_style()` | 28 semibold | large numbers in `StatCard` only |

## Spacing, radius, size

- Spacing steps: `space::XS 4`, `S 8`, `M 12`, `L 16`, `XL 24`, `XXL 32`.
  Parts of one control 4, related controls 8, groups 12–16, sections 16–24,
  page bands 32. Equal gaps mean equal relationships.
- Radii: `radius::SM 4` (badges, chips), `MD 6` (buttons, fields, rows),
  `LG 10` (cards, dialogs). Nested shapes use a smaller radius than their parent.
- Controls are `size::CONTROL` (32) tall. Icons are `size::ICON` (16).
- Page content is centred with a 1200px maximum width and a 28px gutter
  (`shell.rs`). Pages never add their own outer margin.
- Do not nest cards. A page is a column of `card`/`section` blocks on the canvas.

## Icons

Icons are line drawings in `icons.rs` (1.5pt stroke at 16pt, rounded to the
grid). Use them through components (`Button::icon`, `IconButton`, `NavItem`,
`EmptyState`, `StatusPage`) or `icons::paint`. Never embed icon PNGs or SVGs for
UI chrome; the app icon and logo in `assets/` are the only images.

## Motion

Hover fades and the switch knob use `theme` animation time (0.12s). No motion
that is not explaining a real state change.

## Extending the system

Read `docs/UI.md` for step-by-step recipes (new token, icon, component, page,
state). The short version:

1. Need a colour? Add a role to `Palette` in **both** `light()` and `dark()`,
   add it to the contrast tests, document it in the table above.
2. Need an icon? Add a variant to `Icon`, list it in `Icon::ALL`, draw it in
   `shapes`. The gallery test checks it fits its box.
3. Need a widget used in more than one place? Add a component under
   `components/`, re-export it from `components/mod.rs`, give it an accessible
   name and a focus ring, and test it.
4. Need a new page or state? Follow `docs/UI.md` and `docs/STATES.md`.
