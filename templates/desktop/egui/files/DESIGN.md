---
version: "alpha"
name: Smokeforge egui
description: "CraftAnything-inspired near-black technical shell with compact density, white primary actions, and a blue selection accent."
colors:
  background: "#0E0F11"
  panel: "#121316"
  raised: "#17181B"
  border: "#24262B"
  text: "#E7E8EA"
  secondary: "#92959B"
  accent: "#4D8DFF"
  success: "#4ADE80"
  agent: "#7C6CF6"
typography:
  body:
    fontFamily: "Inter, Noto Sans, system-ui, sans-serif"
    fontSize: "12px"
    fontWeight: 400
  title:
    fontFamily: "Inter, Noto Sans, system-ui, sans-serif"
    fontSize: "13px"
    fontWeight: 600
  data:
    fontFamily: "Inter, Noto Sans, system-ui, sans-serif"
    fontSize: "28px"
    fontWeight: 600
rounded:
  control: "6px"
  panel: "8px"
spacing:
  topbar: "38px"
  sidebar: "208px"
  status: "28px"
---

# Design System: Smokeforge egui

**Creative North Star: "The Instrument Panel."**

Near-black neutral surfaces, 1px dividers, compact type, a fixed icon rail and
title bar, full tiled content, and a right-hand inspector. White is the primary
action; blue is selection; green is status; purple is reserved for agent/activity
signals. No card grid, no gradients, no decorative shadows.

## Rules

- Centralize tokens in `crates/app/src/theme.rs`; no component hex colors.
- Dark is the default; light and system themes remain supported.
- Title bar content is draggable; controls are no-drag.
- Every icon-only control has an accessible name.
- Motion is limited to short panel/tab transitions and respects reduced motion.
- Use Lucide SVGs embedded through egui_extras; do not add an egui-0.36 icon crate.
