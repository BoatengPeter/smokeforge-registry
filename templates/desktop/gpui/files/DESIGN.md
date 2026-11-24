---
version: "2"
name: Smokeforge desktop (GPUI)
description: "Quiet, precise desktop UI: neutral surfaces, one blue accent, Inter, light and dark themes."
source_of_truth: crates/ui/themes/smokeforge.json
themes: ["Smokeforge Light", "Smokeforge Dark"]
typography: { family: "Inter (embedded, assets/fonts)", base: "16px = 1rem" }
radius: { general: 6, large: 10 }
---

# Design system

The look follows one idea: **content and hierarchy carry the interface;
decoration supports them.** Neutral surfaces, hairline borders, one accent
colour, a single UI font, and consistent rem-based spacing. Colour is spent
only where it changes what the user notices or does. This follows the GPUI Kit
Design Guides (`.agents/skills/gpui-kit-design-guides`); read them first.

## Where things live

| Layer | File | Owns |
| --- | --- | --- |
| Theme | `crates/ui/themes/smokeforge.json` | every colour, the font family, base size, radii, shadows — light and dark |
| Theme install | `crates/ui/src/theme.rs` | registers Inter and the themes once at startup; contrast tests |
| Components | `crates/ui/src/components/` | `PageHeader`, `SectionCard`, `StatCard`, `EmptyState`, `StatusPage`, `FeatureRow`, `Inset` (`callout`), `ValueStepper`, `WizardWindow` |
| Shell | `crates/app/src/shell.rs` | title bar, sidebar, content column, status bar, command palette |
| Pages | `crates/app/src/pages/` | one page per file; compose GPUI Kit and `crates/ui` components |
| States | `crates/app/src/pages/states.rs`, `docs/STATES.md` | error, empty, degraded, confirmation |

## Colour

Read colours from `cx.theme()` by semantic role; **no hex, `rgb`, or `hsla` in
application code**. The theme JSON maps the shared Smokeforge palette onto GPUI
Kit's roles; the egui template uses the same hex values.

| Palette role | Light | Dark | GPUI Kit keys |
| --- | --- | --- | --- |
| canvas | `#F5F5F6` | `#161619` | `background`, `tab.active.background` |
| surface | `#FFFFFF` | `#1F1F23` | `group_box`, `sidebar`, `title_bar`, `status_bar`, `list`, `table` |
| raised | `#FFFFFF` | `#27272C` | `popover` |
| field | `#F4F4F5` | `#27272C` | `muted`, `secondary`, `skeleton` |
| hover / pressed | `#ECECEE` / `#E2E2E5` | `#2C2C32` / `#35353C` | `accent`, `list.hover`, `secondary.hover` / `secondary.active` |
| selected | `#E8F0FE` | `#1E2C48` | `list.active`, `sidebar.accent`, `table.active` |
| border / strong | `#E4E4E7` / `#D1D1D6` | `#2F2F35` / `#44444C` | `border` / `input.border`, scrollbar thumb |
| text / secondary | `#18181B` / `#5C5C63` | `#F4F4F5` / `#A6A6AF` | `foreground` / `muted.foreground`, `sidebar.foreground` |
| accent / hover | `#2563EB` / `#1D4ED8` | same | `primary` |
| accent text / focus | `#1D4ED8` / `#2563EB` | `#6BA6FF` | `link`, `caret` / `ring` |
| danger fill | `#DC2626` | `#DC2626` | `danger` |

Rules:

- **One primary button per area** (`.primary()`); the rest are `outline`,
  `ghost`, or `danger`.
- **Tags and badges stay neutral** (`Tag::secondary()`) unless the state truly
  carries meaning. Active records use `Tag::success()`; drafts and archived are
  neutral. Never colour alone.
- Contrast is enforced by `crates/ui/src/theme.rs` tests (WCAG AA 4.5:1 for
  text pairs, 3:1 for focus rings). **If you change a colour, run
  `cargo test -p <crate>_ui`.** A failing contrast test means the colour is
  wrong.

## Type, spacing, radius

- Inter is embedded and the base font is 16px, so `text_sm` is 14px body text. Because GPUI Kit uses the base
  font as the rem, **use rem helpers** (`text_sm()`, `text_xl()`, `p_4()`,
  `gap_3()`, `size_4()`, `w_60()`), never `px(...)`. Changing `font.size` in the
  theme zooms the whole app consistently, which is how interface size works:
  `ui::theme::zoom` (`DEFAULT 1.0`, `RANGE 0.7–2.0`, `STEP 0.1`) scales the
  font size at run time. Never change a token to make "everything bigger".
- Page title `text_xl` semibold (via `PageHeader`); section title `text_sm`
  semibold (via `SectionCard`); body `text_sm`; captions `text_xs`; large
  numbers `text_2xl` semibold (via `StatCard`).
- Spacing relationships: parts of one control `gap_1`, related controls
  `gap_2`, groups `gap_3`–`gap_4`, sections `gap_6`, page bands `py_6`/`px_8`.
- Radii from the theme: `cx.theme().radius` (6) for controls,
  `cx.theme().radius_lg` (10) for cards and dialogs. Do not nest cards.

## Icons

Use GPUI Kit's Lucide `IconName`s (`gpui_kit::component::IconName`). The
default set ships ~100 icons; check `default-icons.txt` in the pinned
`gpui-kit-assets` source before choosing one. To add an icon outside that set,
provide it through an application `AssetSource` as the gpui-kit-assets README
describes; never inline SVG strings in pages.

## Extending the system

See `docs/UI.md` for recipes and `docs/STATES.md` for states. When you add a
theme key, component, page, or state, update this file or the matching doc in
the same change.
